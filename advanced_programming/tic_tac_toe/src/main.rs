use std::io::{self, BufRead};

// Tic tac toe game
// We will need a place to initialize the board
// I will choose a simple 2D array. like this
//  board = [
//     0,0,0,
//     0,0,0,
//     0,0,0,
//  ]
//  i will take input from user1 from 1 to 9 avail all the possible inputs
//  check if there is a winner
//      if there is a winner:
//          close the game and print the winner.
//      else
//  render the board with the new state
//  Take the input from user2 and again avail the possible inputes that user2 has to play.
//  after every input check if the user has won.
//  if there is no valid input close the game and mark it as tie.
//
//
//  How to check the valid inputs
//  take the user1 inputs and user2 inputs and avail the valid inputs
//
//  How to check if a user is a winner?
//  take the board state and check if one of the user is completed three horizontal, vertical or diagonal
//  1. check the diagonal
//      board[2] == board[4] == board[6] or
//      board[1] == board[4] == board[8]
//  2. check horizontal
//      board[0] == board[1] == board[2] or
//      board[3] == board[4] == board[5]
//      board[6] == board[7] == board[8]
//  3. check vaertical
//      board[0] == board[3] == board[6] or
//      board[1] == board[4] == board[7]
//      board[2] == board[5] == board[8]
//
fn main() {
    let mut board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut x = Vec::new();
    let mut o = Vec::new();
    loop {
        print_board(&board);
        let valid = valid_inputs(&x, &o);
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
        x.push(x_input);

        board[x_input as usize - 1] = 1;
        if is_winner(&x) {
            print_board(&board);
            println!("User X won! yay!");
            break;
        }
        print_board(&board);

        let valid = valid_inputs(&x, &o);
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
        o.push(o_input);

        board[o_input as usize - 1] = 2;
        if is_winner(&o) {
            print_board(&board);
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

//  Set[1, 2, 3, 4, 5, 6, 7, 8, 9] - user1 UNION user2;
fn valid_inputs(user1inputes: &[u8], user2inputes: &[u8]) -> Vec<u8> {
    let valid = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    valid
        .into_iter()
        .filter(|x| !user1inputes.contains(x) && !user2inputes.contains(x))
        .collect()
}

//  Print the board
fn print_board(board: &[u8]) {
    println!("-------------");
    for (i, elem) in board.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!();
            println!("-------------");
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
    println!("-------------");
}
