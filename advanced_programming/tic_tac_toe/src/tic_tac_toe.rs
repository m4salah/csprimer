use std::{
    fmt,
    io::{self, BufRead, Lines, StdinLock},
};

enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => f.write_str("X"),
            Player::O => f.write_str("O"),
        }
    }
}

pub struct TicTacToe {
    board: [u8; 9],
    x: Vec<u8>,
    o: Vec<u8>,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
            x: Vec::new(),
            o: Vec::new(),
        }
    }

    // TODO: How to make iterator more generic to accept any iterator?
    fn add_move(&mut self, iterator: &mut Lines<StdinLock<'static>>, player: Player) {
        let valid = self.valid_inputs();
        let input = loop {
            println!("User {player}: Enter one of numbers on board:");
            let Some(Ok(input)) = iterator.next() else {
                println!("Invalid input. Please try again.");
                continue;
            };
            let Ok(n) = input.parse::<u8>() else {
                println!("Invalid input. Please try again.");
                continue;
            };
            if !valid.contains(&n) {
                println!("Please enter valid input from the valid numbers");
                continue;
            };
            break n;
        };
        match player {
            Player::X => {
                self.x.push(input);
                self.board[input as usize - 1] = 1;
            }
            Player::O => {
                self.o.push(input);
                self.board[input as usize - 1] = 2;
            }
        }
    }

    fn check_tie(&self) -> bool {
        let valid = self.valid_inputs();
        if valid.is_empty() {
            return true;
        }
        return false;
    }

    pub fn start(mut self) {
        // for each player
        //  - print the board
        //  - take the input
        //  - check if is the winner
        //  - check if it's a tie
        //  - repeat until we have a winner or tie.

        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        loop {
            self.print_board();
            self.add_move(&mut iterator, Player::X);
            if self.is_x_winner() {
                self.print_board();
                println!("User {} won! yay!", Player::X);
                break;
            }
            if self.check_tie() {
                self.print_board();
                println!("It's a tie!");
                break;
            }

            self.print_board();
            self.add_move(&mut iterator, Player::O);
            if self.is_o_winner() {
                self.print_board();
                println!("User {} won! yay!", Player::O);
                break;
            }
            if self.check_tie() {
                self.print_board();
                println!("It's a tie!");
                break;
            }
        }
    }

    fn is_x_winner(&self) -> bool {
        if Self::is_winner(&self.x) {
            return true;
        }
        return false;
    }

    fn is_o_winner(&self) -> bool {
        if Self::is_winner(&self.o) {
            return true;
        }
        return false;
    }

    fn is_winner(user_inputes: &[u8]) -> bool {
        let winning_conditions = [
            // Diagonal condition
            [3, 5, 7],
            [1, 5, 9],
            // Horizontal condition
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            // Vertical condition
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
        ];
        for condition in winning_conditions {
            if condition.into_iter().all(|x| user_inputes.contains(&x)) {
                return true;
            }
        }
        return false;
    }

    fn valid_inputs(&self) -> Vec<u8> {
        let valid = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        valid
            .into_iter()
            .filter(|x| !self.x.contains(x) && !self.o.contains(x))
            .collect()
    }

    //  Print the board
    fn print_board(&self) {
        // clear the terminal
        // https://stackoverflow.com/a/75533951
        print!("{esc}c", esc = 27 as char);
        let sep = "---+---+---";
        for (i, elem) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                println!();
                println!("{sep}");
            }
            if elem == &1 {
                print!(" X ");
            } else if elem == &2 {
                print!(" O ");
            } else {
                print!(" {} ", i + 1);
            }
            if (i + 1) % 3 != 0 {
                print!("|");
            }
        }
        println!();
    }
}

#[test]
fn test_is_winner() {
    // Horizontal
    assert!(TicTacToe::is_winner(&[1, 2, 3]));
    assert!(TicTacToe::is_winner(&[4, 5, 6]));

    // Diagonal
    assert!(TicTacToe::is_winner(&[1, 5, 9]));
    assert!(TicTacToe::is_winner(&[3, 5, 7]));

    // Vertical
    assert!(TicTacToe::is_winner(&[1, 4, 7]));
}
