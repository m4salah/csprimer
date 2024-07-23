use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

/// function truncate string on n characters
/// taking in considiration the utf8 characters
fn utf8_truncate(n: u8, buf: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let n = n as usize;
    let mut res = Vec::new();
    let mut i = 0;
    loop {
        if i + 1 > n {
            break;
        }
        if i >= buf.len() {
            break;
        }
        if buf[i] & 0b10000000 == 0 {
            // This is one byte character
            res.push(buf[i]);
            i += 1;
        } else if buf[i] & 0b11110000 == 0b11110000 {
            // four bytes utf-8 character
            if i + 4 > n {
                break;
            }
            res.push(buf[i]);
            res.push(buf[i + 1]);
            res.push(buf[i + 2]);
            res.push(buf[i + 3]);
            i += 4;
        } else if buf[i] & 0b11100000 == 0b11100000 {
            // three bytes utf-8 character
            if i + 3 > n {
                break;
            }
            res.push(buf[i]);
            res.push(buf[i + 1]);
            res.push(buf[i + 2]);
            i += 3;
        } else if buf[i] & 0b11000000 == 0b11000000 {
            // two bytes utf-8 character
            if i + 2 > n {
                break;
            }
            res.push(buf[i]);
            res.push(buf[i + 1]);
            i += 2;
        } else {
            panic!(
                "Unexpected utf-8 buf: hex: {:2X?}, bin:{:#10b} i:{i}",
                buf[i], buf[i]
            );
        }
    }
    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let txt = File::open("utf8-truncate/cases")?;
    let mut txt = BufReader::new(txt);
    let mut buffer = Vec::new();
    while txt.read_until(b'\n', &mut buffer)? > 0 {
        let n = buffer[0];
        // remove the new line b'\n'
        buffer.pop();

        let res = utf8_truncate(n, &buffer[1..])?;
        println!("{}", String::from_utf8_lossy(&res));
        // Clear the buffer for the next line
        buffer.clear();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn truncate_full_emoji_at_end() {
        assert_eq!(
            utf8_truncate(28, "Truncate full emoji at end ð¿".as_bytes()).unwrap(),
            Vec::from("Truncate full emoji at end ".as_bytes())
        )
    }

    #[test]
    fn truncate_retain_full_emoji_at_end() {
        assert_eq!(
            utf8_truncate(29, "Retain full emoji at end 😍".as_bytes()).unwrap(),
            Vec::from("Retain full emoji at end 😍".as_bytes())
        )
    }

    #[test]
    fn do_truncate() {
        assert_eq!(
            utf8_truncate(8, "Do truncate".as_bytes()).unwrap(),
            Vec::from("Do trunc".as_bytes())
        )
    }

    #[test]
    fn dont_truncate() {
        assert_eq!(
            utf8_truncate(255, "Don't truncate, ASCII".as_bytes()).unwrap(),
            Vec::from("Don't truncate, ASCII".as_bytes())
        )
    }
}
