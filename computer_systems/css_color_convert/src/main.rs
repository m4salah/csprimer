/// The solution for CSS color encoded exercise
/// https://csprimer.com/watch/color-convert/
///
use std::{
    error::Error,
    io::{self, Read},
    str::Chars,
};

use fancy_regex::Regex;
use itertools::Itertools;

fn char_hex_to_decimal(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => panic!("invalid hex char"),
    }
}

fn hex_to_decimal(hex_s: &str) -> u8 {
    hex_s
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| char_hex_to_decimal(c) << 4 * i)
        .sum::<u8>()
}

fn handl_four_eight_hex(mut c_iter: Chars) -> String {
    // extract the length
    let length = match c_iter.try_len() {
        Ok(l) => l,
        Err(e) => match e {
            (_, Some(l)) => l,
            _ => panic!("invalid length"),
        },
    };

    if length == 4 {
        c_iter.map(|c| format!("{c}{c}")).join("")
    } else if length == 8 {
        c_iter.join("")
    } else {
        panic!("not 4 or 8 length")
    }
}

fn handl_three_six_hex(mut c_iter: Chars) -> String {
    // extract the length
    let length = match c_iter.try_len() {
        Ok(l) => l,
        Err(e) => match e {
            (_, Some(l)) => l,
            _ => panic!("invalid length"),
        },
    };

    if length == 3 {
        c_iter.map(|c| format!("{c}{c}")).join("")
    } else if length == 6 {
        c_iter.join("")
    } else {
        panic!("not 3 or 6 length")
    }
}

fn handl_alpha(alpha_hex: &str) -> f32 {
    assert!(alpha_hex.len() == 2, "invalid alpha length");

    let alpha = hex_to_decimal(alpha_hex);

    ((alpha as f32 / 255.0) * 100_000.0).round() / 100_000.0
}

fn hex_to_rgb(h: &str, with_commas: bool) -> String {
    let mut c_iter = h.chars();

    // make sure that it's starts with #
    assert!(c_iter.next() == Some('#'), "");

    // extract the length
    let length = match c_iter.try_len() {
        Ok(l) => l,
        Err(e) => match e {
            (_, Some(l)) => l,
            _ => panic!("invalid length"),
        },
    };

    // validate it's 8 byte or less
    assert!(
        length == 3 || length == 4 || length == 6 || length == 8,
        "invalid css color hex length"
    );

    match length {
        3 | 6 => format!(
            "rgb({})",
            handl_three_six_hex(c_iter.clone())
                .chars()
                .chunks(2)
                .into_iter()
                .map(|mut s| s.join(""))
                .map(|hex_s| hex_to_decimal(&hex_s))
                .join(if with_commas { ", " } else { " " })
        ),
        4 | 8 => format!(
            "rgba({} / {})",
            handl_four_eight_hex(c_iter.clone())
                .chars()
                .take(6)
                .chunks(2)
                .into_iter()
                .map(|mut s| s.join(""))
                .map(|hex_s| hex_to_decimal(&hex_s))
                .join(if with_commas { ", " } else { " " }),
            handl_alpha(&handl_four_eight_hex(c_iter.clone())[6..])
        ),
        _ => panic!("invalid css color hex length"),
    }
}

fn css_color_converter_from_stdin() -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    let re = Regex::new("#[0-9a-fA-F]{3,8}")?;
    let text = String::from_utf8_lossy(&buffer);
    let mut result = text.to_string();

    for i in re.find_iter(&text) {
        let color = &text[i.clone().unwrap().start()..i.unwrap().end()];
        let rgb_color = hex_to_rgb(color, false);
        result = result.replace(color, &rgb_color);
    }
    Ok(())
}

#[test]
fn test_hex_to_rgb() {
    assert!(hex_to_rgb("#00ff00", true) == "rgb(0, 255, 0)");
    assert!(hex_to_rgb("#fe030a", true) == "rgb(254, 3, 10)");
    assert!(hex_to_rgb("#0f0def", true) == "rgb(15, 13, 239)");
    assert!(hex_to_rgb("#000000", true) == "rgb(0, 0, 0)");
    assert!(hex_to_rgb("#123", true) == "rgb(17, 34, 51)");
    assert_eq!(hex_to_rgb("#0000FFC0", true), "rgba(0, 0, 255 / 0.75294)");
    assert_eq!(hex_to_rgb("#00f8", true), "rgba(0, 0, 255 / 0.53333)");
}

fn main() -> Result<(), Box<dyn Error>> {
    css_color_converter_from_stdin()
}
