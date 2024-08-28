use tic_tac_toe::TicTacToe;

mod tic_tac_toe;

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
    let game = TicTacToe::new();
    game.start();
}
