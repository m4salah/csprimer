use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    net::Ipv4Addr,
};

use bincode::Options;
use serde::Deserialize;

/// The start of the challenge TCP syn flood

// pcap header file
// https://datatracker.ietf.org/doc/id/draft-gharris-opsawg-pcap-00.html#name-file-header
#[derive(Deserialize, Debug)]
struct Pcap {
    pub magic_number: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub timezone_offset: u32,
    pub timestamp_accuracy: u32,
    pub snaplen: u32,
    pub linktype: u32,
}

// Packet header
// https://datatracker.ietf.org/doc/id/draft-gharris-opsawg-pcap-00.html#name-packet-record
#[derive(Deserialize, Debug)]
struct PacketHeader {
    pub ts_sec: u32,
    pub ts_usec: u32,
    pub incl_len: u32,
    pub orig_len: u32,
}

// our pcap linktype is LINKTYPE_NULL
// so the first byte is the protocol type
// https://www.tcpdump.org/linktypes/LINKTYPE_NULL.html
// https://en.wikipedia.org/wiki/IPv4#Packet_structure
#[derive(Deserialize, Debug)]
struct IPHeader {
    pub ip: u32,
    pub version_ihl: u8,
    pub dscp_ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragmentoffset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub source_addr: Ipv4Addr,
    pub dest_addr: Ipv4Addr,
}

// TCP Header struct
// https://en.wikipedia.org/wiki/Transmission_Control_Protocol#TCP_segment_structure
#[derive(Deserialize, Debug)]
struct TCPHeader {
    pub source_port: u16,
    pub dest_port: u16,
    pub sequence_number: u32,
    pub ack_number: u32,
    pub dataoffset_reserved: u8,
    pub flags: u8,
}

fn tcp_syn_flod() -> Result<(), Box<dyn Error>> {
    let pcap = File::open("syn-flood/synflood.pcap")?;
    let mut pcap = BufReader::new(pcap);

    // GLOBAL HEADER
    // 24 bytes.
    let mut global_header = [0u8; 24];
    pcap.read(&mut global_header)?;
    let pc: Pcap = bincode::deserialize(&global_header)?;
    println!("Pcap: {:?}", pc);

    assert!(pc.magic_number == 0xa1b2c3d4, "invalid magic number");
    assert!(pc.linktype == 0, "invalid linktype");
    assert!(pc.major_version == 2, "invalid major version");
    assert!(pc.minor_version == 4, "invalid minor version");
    // End of the global header

    // Packet header
    //
    // 16 bytes for the packet header
    let mut count = 0;
    let mut initiated = 0;
    let mut acked = 0;
    loop {
        let mut per_packet_header = [0u8; 16];
        match pcap.read_exact(&mut per_packet_header) {
            Ok(_) => (),
            Err(_) => break,
        }
        count += 1;
        let per_packet_header: PacketHeader = bincode::deserialize(&per_packet_header)?;

        assert!(
            per_packet_header.incl_len == per_packet_header.orig_len,
            "the size is invlaid"
        );
        let mut packet = vec![0u8; per_packet_header.incl_len as usize];
        pcap.read_exact(&mut packet)?;

        let per_packet_payload: IPHeader = bincode::deserialize(&packet)?;
        let tcp: TCPHeader = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .with_big_endian()
            .deserialize(&packet[24..])?;

        assert!(per_packet_payload.ip == 2, "invalid ip");
        assert!(per_packet_payload.protocol == 6, "invalid protocol");

        let is_ack = (tcp.flags & 0b00010000) > 0;
        let is_syn = (tcp.flags & 0b00000010) > 0;

        if tcp.dest_port == 80 && is_syn {
            initiated += 1
        }
        if tcp.source_port == 80 && is_ack {
            acked += 1
        }
        println!(
            "{} -> {}{}{}",
            tcp.source_port,
            tcp.dest_port,
            if is_ack { " ACK" } else { "" },
            if is_syn { " SYN" } else { "" }
        )
    }
    println!(
        "{count} packet parsed {initiated} connections, {acked} ({:0.2} acked)",
        acked as f64 / initiated as f64
    );

    Ok(())
}
/// End of the TCP syn flood
fn main() -> Result<(), Box<dyn Error>> {
    tcp_syn_flod()
}
