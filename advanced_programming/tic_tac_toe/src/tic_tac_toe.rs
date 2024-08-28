use std::io::{self, BufRead};

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

    pub fn start(mut self) {
        loop {
            self.print_board();
            let valid = self.valid_inputs();
            if valid.is_empty() {
                println!("It's a tie!");
                break;
            }

            let stdin = io::stdin();
            let mut iterator = stdin.lock().lines();
            let x_input = loop {
                println!("User X: Enter one of numbers on board: ");
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
            self.x.push(x_input);

            self.board[x_input as usize - 1] = 1;
            if Self::is_winner(&self.x) {
                self.print_board();
                println!("User X won! yay!");
                break;
            }
            self.print_board();

            let valid = self.valid_inputs();
            if valid.is_empty() {
                println!("It's a tie!");
                break;
            }

            let o_input = loop {
                println!("User O: Enter one of numbers on board: ");
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
            self.o.push(o_input);

            self.board[o_input as usize - 1] = 2;
            if Self::is_winner(&self.o) {
                self.print_board();
                println!("User O won! yay!");
                break;
            }
        }
    }

    fn is_winner(user_inputes: &[u8]) -> bool {
        //  1. check the diagonal
        //      board[2] == board[4] == board[6] or
        //      board[1] == board[4] == board[8]
        let right_diagonal_win = [3, 5, 7];
        let left_diagonal_win = [1, 5, 9];
        if right_diagonal_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        if left_diagonal_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        //  2. check horizontal
        //      board[0] == board[1] == board[2] or
        //      board[3] == board[4] == board[5]
        //      board[6] == board[7] == board[8]
        let top_horizontal_win = [1, 2, 3];
        let mid_horizontal_win = [4, 5, 6];
        let bottom_horizontal_win = [7, 8, 9];

        if top_horizontal_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        if mid_horizontal_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        if bottom_horizontal_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        //  3. check vaertical
        //      board[0] == board[3] == board[6] or
        //      board[1] == board[4] == board[7]
        //      board[2] == board[5] == board[8]

        let left_vertical_win = [1, 4, 7];
        let mid_vertical_win = [2, 5, 8];
        let right_vertical_win = [3, 6, 9];

        if left_vertical_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        if mid_vertical_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

        if right_vertical_win
            .into_iter()
            .all(|x| user_inputes.contains(&x))
        {
            return true;
        };

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
        println!("------------");
        for (i, elem) in self.board.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                println!();
                println!("------------");
            }
            if elem == &1 {
                print!(" X |");
            } else if elem == &2 {
                print!(" O |");
            } else {
                print!(" {} |", i + 1);
            }
        }
        println!();
        println!("------------");
    }
}
