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
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(2 * 6);
        bytes.write_u16::<BigEndian>(self.id).unwrap();
        bytes.write_u16::<BigEndian>(self.flags).unwrap();
        bytes.write_u16::<BigEndian>(self.question_count).unwrap();
        bytes.write_u16::<BigEndian>(self.answer_count).unwrap();
        bytes.write_u16::<BigEndian>(self.authority_count).unwrap();
        bytes.write_u16::<BigEndian>(self.additional_count).unwrap();
        return bytes;
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
        println!("name is: {name:02X?}");

        DNSQuestion {
            name,
            qtype,
            qclass,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.name.len() + 4);
        for byte in self.name.clone() {
            bytes.write_u8(byte).unwrap();
        }
        bytes.write_u16::<BigEndian>(self.qtype).unwrap();
        bytes.write_u16::<BigEndian>(self.qclass).unwrap();
        return bytes;
    }
}

fn main() {
    let hostname = env::args()
        .skip(1)
        .next()
        .expect("Pass the hostname as argument");
    println!("querying {hostname}");

    let socket_fd = socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        None,
    )
    .expect("Failed to open socket file descriptor");

    let header = DNSHeader {
        id: 42,
        flags: 0x0100,
        question_count: 1,
        answer_count: 0,
        authority_count: 0,
        additional_count: 0,
    };
    let question = DNSQuestion::new(&hostname, 1, 1);

    let header = header.as_bytes();

    let question = question.as_bytes();

    // Combine header and question into one message
    let mut message = Vec::new();
    message.extend_from_slice(&header);
    message.extend_from_slice(&question);

    let cloudflare_dns = SockaddrIn::from_str("1.1.1.1:53").unwrap();

    sendto(
        socket_fd.as_raw_fd(),
        &message,
        &cloudflare_dns,
        MsgFlags::empty(),
    )
    .expect("Failed to sendto");

    let mut answer_buf = [0; 1024];
    if let Ok((bytes, Some(addr))) = recvfrom::<SockaddrIn>(socket_fd.as_raw_fd(), &mut answer_buf)
    {
        // TODO: return the to user the dns record, or do something with it.
        println!("Received form {addr} {bytes} bytes");
    }
}
