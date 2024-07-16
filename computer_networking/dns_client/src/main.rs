#![allow(dead_code)]
use std::{env, os::fd::AsRawFd, str::FromStr};

use byteorder::{BigEndian, WriteBytesExt};
use nix::sys::socket::{
    recvfrom, sendto, socket, AddressFamily, MsgFlags, SockFlag, SockType, SockaddrIn,
};

#[derive(Debug)]
struct DNSHeader {
    id: u16,
    flags: u16,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_count: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(2 * 6);
        bytes.write_u16::<BigEndian>(self.id).unwrap();
        bytes.write_u16::<BigEndian>(self.flags).unwrap();
        bytes.write_u16::<BigEndian>(self.question_count).unwrap();
        bytes.write_u16::<BigEndian>(self.answer_count).unwrap();
        bytes.write_u16::<BigEndian>(self.authority_count).unwrap();
        bytes.write_u16::<BigEndian>(self.additional_count).unwrap();
        return bytes;
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            id: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            flags: u16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            question_count: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            answer_count: u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
            authority_count: u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
            additional_count: u16::from_be_bytes(bytes[10..12].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
struct DNSQuestion {
    name: Vec<u8>,
    qtype: u16,
    qclass: u16,
}

impl DNSQuestion {
    fn new(domain: &str, qtype: u16, qclass: u16) -> Self {
        let mut name = Vec::new();
        for part in domain.split('.') {
            name.push(part.len() as u8);
            name.extend_from_slice(part.as_bytes());
        }
        name.push(0); // Null terminator for the QNAME

        Self {
            name,
            qtype,
            qclass,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.name.clone();
        bytes.write_u16::<BigEndian>(self.qtype).unwrap();
        bytes.write_u16::<BigEndian>(self.qclass).unwrap();
        return bytes;
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let end_of_name_index = bytes.iter().position(|b| *b == 0).unwrap() + 1;

        Self {
            name: Vec::from(&bytes[..end_of_name_index]),
            qtype: u16::from_be_bytes(
                bytes[end_of_name_index..end_of_name_index + 2]
                    .try_into()
                    .unwrap(),
            ),
            qclass: u16::from_be_bytes(
                bytes[end_of_name_index + 2..end_of_name_index + 2 + 2]
                    .try_into()
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug)]
struct DNSAnswer {
    pub name: Vec<u8>,
    pub atype: u16,
    pub qclass: u16,
    pub ttl: u32,
    pub rd_length: u16,
    pub rd_data: Vec<u8>,
}

#[derive(Debug)]
struct DNS {
    header: DNSHeader,
    question: DNSQuestion,
    answer: DNSAnswer,
}

impl DNS {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header = DNSHeader::from_bytes(&bytes[..12]);
        let question = DNSQuestion::from_bytes(&bytes[12..]);
        let answer = DNSAnswer::from_bytes(&bytes[(question.name.len() + 4 + 12)..]);

        Self {
            header,
            question,
            answer,
        }
    }
}

impl DNSAnswer {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let end_of_name_index = bytes.iter().position(|b| *b & 0b11000000 > 0).unwrap() + 2;

        Self {
            name: Vec::from(&bytes[..end_of_name_index]),
            atype: u16::from_be_bytes(
                bytes[end_of_name_index..end_of_name_index + 2]
                    .try_into()
                    .unwrap(),
            ),
            qclass: u16::from_be_bytes(
                bytes[end_of_name_index + 2..end_of_name_index + 2 + 2]
                    .try_into()
                    .unwrap(),
            ),
            ttl: u32::from_be_bytes(
                bytes[end_of_name_index + 2 + 2..end_of_name_index + 2 + 2 + 4]
                    .try_into()
                    .unwrap(),
            ),
            rd_length: u16::from_be_bytes(
                bytes[end_of_name_index + 2 + 2 + 4..end_of_name_index + 2 + 2 + 4 + 2]
                    .try_into()
                    .unwrap(),
            ),
            rd_data: Vec::from(&bytes[end_of_name_index + 2 + 2 + 4 + 2..]),
        }
    }
}

fn main() {
    let hostname = env::args()
        .skip(1)
        .next()
        .expect("Pass the hostname as argument");
    println!("querying... {hostname}");

    let socket_fd = socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    )
    .expect("Failed to open socket file descriptor");

    let header = DNSHeader {
        id: rand::random(),
        flags: 0x0100,
        question_count: 1,
        answer_count: 0,
        authority_count: 0,
        additional_count: 0,
    };
    let question = DNSQuestion::new(&hostname, 1, 1);

    // Combine header and question into one message
    let mut message = Vec::new();
    message.extend_from_slice(&header.to_bytes());
    message.extend_from_slice(&question.to_bytes());

    let cloudflare_dns = SockaddrIn::from_str("1.1.1.1:53").unwrap();

    sendto(
        socket_fd.as_raw_fd(),
        &message,
        &cloudflare_dns,
        MsgFlags::empty(),
    )
    .expect("Failed to sendto");

    let mut answer_buf = [0; 1024];

    while let Ok((bytes, Some(addr))) =
        recvfrom::<SockaddrIn>(socket_fd.as_raw_fd(), &mut answer_buf)
    {
        // validate the receiver address
        if addr != cloudflare_dns {
            continue;
        }
        let dns = DNS::from_bytes(&answer_buf[..bytes]);

        // validate it's the same id we sent
        if dns.header.id != header.id {
            continue;
        }

        println!(
            "{} is on IP: {}",
            hostname,
            dns.answer
                .rd_data
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(".")
        );
        break;
    }
}
