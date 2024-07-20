use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn utf8_truncate(n: u8, buf: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut n = n as usize;
    if n >= buf.len() {
        return Ok(Vec::from(buf));
    }
    while n > 0 && buf[n] & 0b11000000 == 0b10000000 {
        n -= 1;
    }
    Ok(Vec::from(&buf[..n]))
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
