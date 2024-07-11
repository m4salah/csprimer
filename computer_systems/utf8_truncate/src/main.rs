use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn utf8_truncate() -> Result<(), Box<dyn Error>> {
    let txt = File::open("utf8-truncate/cases")?;
    let mut txt = BufReader::new(txt);
    let mut buffer = Vec::new();

    while txt.read_until(b'\n', &mut buffer)? > 0 {
        // Remove the newline character if you don't want it in the output
        if let Some(&last) = buffer.last() {
            if last == b'\n' {
                buffer.pop();
            }
        }

        // Print the buffer or process it as needed
        println!("{:?}", buffer);

        // Clear the buffer for the next line
        buffer.clear();
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    utf8_truncate()
}
