use std::io::{stdin, stdout, Write};

fn main() {
    let mut game_board = [' '; 9];
    let mut turn = 'X';
    loop {
        print_board(&game_board);
        let pos = get_move(&game_board, turn);
        game_board[pos] = turn;
        if check_win(&game_board) {
            println!("{} wins!", turn);
            break;
        } else if game_board.iter().all(|&x| x != ' '){
            println!("The game is draw!");
            break;
        }
        turn = if turn == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &[char; 9]) {
    println!("{} | {} | {}", board[0], board[1], board[2]);
    println!("--|---|--");
    println!("{} | {} | {}", board[3], board[4], board[5]);
    println!("--|---|--");
    println!("{} | {} | {}", board[6], board[7], board[8]);
}

fn get_move(board: &[char; 9], turn: char) -> usize {
    loop {
        print!("{}'s turn, please input your move (0-8): ", turn);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let pos = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if pos > 8 || board[pos] != ' '{
            println!("Invalid move, please try again!");
            continue;
        }
        return pos;
    }
}

fn check_win(board: &[char; 9]) -> bool {
    let win_patterns = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6]
    ];
    for &pattern in &win_patterns {
        if board[pattern[0]] != ' '
                && board[pattern[0]] == board[pattern[1]]
                && board[pattern[1]] == board[pattern[2]] {
            return true;
        }
    }
    false
}
