/// Beep challenge
///
use std::io::{self, Read, Write};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

fn beep() {
    loop {
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone(); // make a mutable copy of termios
                                               // that we will modify
        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let mut reader = io::stdin();
        let mut buffer = [0; 1]; // read exactly one byte
        reader.read_exact(&mut buffer).unwrap();

        if let Some(n) = (buffer[0] as char).to_digit(10) {
            for _ in 0..n {
                io::stdout().write_all(b"\x07").unwrap();
            }
        }
        tcsetattr(stdin, TCSANOW, &termios).unwrap(); // reset the stdin to
    }
}

/// The end of beep challenge
fn main() {
    beep();
}
