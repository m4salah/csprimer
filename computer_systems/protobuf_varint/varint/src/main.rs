#![allow(dead_code)]
#![allow(unreachable_code)]

use std::error::Error;

/// This is the solution in rust for Protobuf varint
/// https://csprimer.com/watch/varint/
fn encode(n: u64) -> Vec<u8> {
    let mut n = n;
    let mut res = Vec::new();
    while n > 0 {
        let mut part = n & 0x7f;
        n >>= 7;
        if n > 0 {
            part |= 1 << 7;
        }
        res.push(part as u8)
    }
    res
}

#[test]
fn test_encoding() {
    let encode_tests = vec![
        (150, vec![0x96, 0x01]),
        (1, vec![0x01]),
        (
            18446744073709551615,
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
    ];
    for (n, expected) in encode_tests {
        assert!(encode(n) == expected);
    }
}

fn decode(mut varn: Vec<u8>) -> u64 {
    let mut res = 0u64;
    varn.reverse();
    for b in varn {
        res <<= 7;
        let part = b & 0x7f;
        res |= part as u64
    }
    res
}

#[test]
fn test_decoding() {
    let encode_tests = vec![
        (150, vec![0x96, 0x01]),
        (1, vec![0x01]),
        (
            18446744073709551615,
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
    ];
    for (expected, n) in encode_tests {
        assert!(decode(n) == expected);
    }
}

#[test]
fn test_encoding_decoding_round_trip() {
    for i in 0..1 << 30 {
        assert_eq!(decode(encode(i)), i);
    }
}
/// The End of protobuf varints integer exercise.

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, World!");
    Ok(())
}
