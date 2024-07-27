use byteorder::WriteBytesExt;
/**
* Plan
* 1. conceal function
*   a. get the first 6 bytes you want to encode or less.
*   c. return the f64 of that message in this form to make it NaN
*       0xff 0xff message
*
* 2. extract function
*   a. get the payload from the NaN which is starting from the third bytes to end
*   b. convert the payload to string and return it.
*/
fn conceal(b: &[u8]) -> f64 {
    // get the first 6 bytes of the message or less.
    let end = b.len().min(6) as u8;
    let message = &b[..end as usize];

    // initialize the buf
    let mut buf = Vec::with_capacity(8);

    // write the first byte
    buf.write_u8(0xff).unwrap();

    // write the second byte which is compine the length of the message
    // and ones to make it NaN
    // for example if we 0b11110000 ^ 0b00000110 = 0b11110110
    buf.write_u8(0xf0 ^ end).unwrap();

    // write the message
    for b in message {
        buf.write_u8(*b).unwrap()
    }

    // fill the rest of the buf with zeros.
    for _ in 0..(6 - end) {
        buf.write_u8(0).unwrap();
    }
    return f64::from_be_bytes(buf.as_slice().try_into().unwrap());
}

fn extract(f: f64) -> String {
    // Get the length of the message.
    // Which is the fourth half byte.
    let n = f.to_be_bytes()[1] & 0x0f;

    // the end of the message is equal to the length of the message
    // n + 2 for the first two byte we ignored.
    let end = n + 2;

    let message_bytes = &f.to_be_bytes()[2..end as usize];
    return String::from_utf8_lossy(message_bytes).to_string();
}

#[test]
fn test_conceal_extract() {
    let cases = vec!["hello!", "woo", "hello", "❤️"];
    for case in cases {
        assert_eq!(extract(conceal(case.as_bytes())), case, "Failed at {case}");
    }
}

fn main() {
    let concealed = conceal("hello!".as_bytes());
    println!("Concealed: {concealed}");
    let extracted = extract(concealed);
    println!("Extracted: {}", extracted);

    let concealed = conceal("❤️".as_bytes());
    println!("Concealed: {concealed}");
    let extracted = extract(concealed);
    println!("Extracted: {}", extracted);
}
