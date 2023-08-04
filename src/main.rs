use std::fmt;
use std::io::stdin;
use std::io::{self, Write};

fn greeting() {
    println!(
        "
        \nRUST TIC TAC TOE\n\
        --------------\n\
        AI rust tic tac toe
        "
    )
}

/// TODO
/// Generate a board
/// implement a player picker

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
    None,
}

impl Player {
    pub fn player_move(&self) -> &str {
        match self {
            Player::O => "O",
            Player::X => "X",
            Player::None => "-",
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.player_move())
    }
}

fn pick_player(board: &[[Option<Player>; 3]; 3]) -> Player {
    let mut x = 0;
    let mut o = 0;

    for i in board {
        for j in i {
            match j {
                Some(Player::O) => o += 1,
                Some(Player::X) => x += 1,
                _ => continue,
            }
        }
    }
    if x == o {
        Player::X
    } else {
        Player::O
    }
}

fn display_board(board: &[[Option<Player>; 3]; 3]) {
    for i in board {
        println!(
            "              
                       {}           #       {}          #      {}        
                                   #                    #               
                -------------------------------------------------------
            ",
            i[0].unwrap_or(Player::None),
            i[1].unwrap_or(Player::None),
            i[2].unwrap_or(Player::None)
        );
    }
}

fn check_winner(board: &[[Option<Player>; 3]; 3]) -> Option<Player> {
    // Check rows
    for i in 0..3 {
        if let Some(player) = board[i][0] {
            if board[i][1] == Some(player) && board[i][2] == Some(player) {
                return Some(player);
            }
        }
    }

    // Check columns
    for j in 0..3 {
        if let Some(player) = board[0][j] {
            if board[1][j] == Some(player) && board[2][j] == Some(player) {
                return Some(player);
            }
        }
    }

    // Check diagonals
    if let Some(player) = board[0][0] {
        if board[1][1] == Some(player) && board[2][2] == Some(player) {
            return Some(player);
        }
    }
    if let Some(player) = board[0][2] {
        if board[1][1] == Some(player) && board[2][0] == Some(player) {
            return Some(player);
        }
    }

    None
}

fn check_draw(board: &[[Option<Player>; 3]; 3]) -> bool {
    for row in board {
        for cell in row {
            if cell.is_none() {
                return false;
            }
        }
    }
    true
}

fn actions(board: &[[Option<Player>; 3]; 3]) -> Vec<(usize, usize)> {
    let mut possible_moves: Vec<(usize, usize)> = Vec::new();
    for (row, i) in board.iter().enumerate() {
        for (pos, j) in i.iter().enumerate() {
            if j.is_none() {
                let tuple = (row, pos);
                possible_moves.push(tuple);
            }
        }
    }

    possible_moves
}

fn result(board: &[[Option<Player>; 3]; 3], action: (usize, usize)) -> [[Option<Player>; 3]; 3] {
    let mut resulting_board = board.clone();
    let player: Player = pick_player(&board);

    if board[action.0][action.1].is_some() {
        panic!("Value isn't empty")
    }

    resulting_board[action.0][action.1] = Some(player);
    resulting_board
}

fn utility(board: &[[Option<Player>; 3]; 3]) -> f64 {
    let winner = check_winner(board).unwrap_or(Player::None);
    match winner {
        Player::X => 1.0,
        Player::O => -1.0,
        Player::None => 0.0,
    }
}

fn terminal(board: &[[Option<Player>; 3]; 3]) -> bool {
    if check_winner(board).is_some() {
        return true;
    } else if check_draw(board) {
        return true;
    }

    return false;
}

fn min_max(board: &[[Option<Player>; 3]; 3]) -> Option<(usize, usize)> {
    if terminal(board) {
        return None;
    } else {
        if pick_player(board) == Player::X {
            let (_value, best_move) = max_value(board);
            return best_move;
        } else {
            let (_value, best_move) = min_value(board);
            return best_move;
        }
    }
}

fn max_value(board: &[[Option<Player>; 3]; 3]) -> (f64, Option<(usize, usize)>) {
    if terminal(board) {
        return (utility(board), None);
    }

    let mut v = std::f64::NEG_INFINITY;
    let mut best_move: (usize, usize) = (0, 0);

    for action in actions(board) {
        let (ax, _ac) = min_value(&result(board, action));
        if ax > v {
            v = ax;
            best_move = action;
            if v == 1.0 {
                return (v, Some(best_move));
            }
        }
    }

    return (v, Some(best_move));
}

fn min_value(board: &[[Option<Player>; 3]; 3]) -> (f64, Option<(usize, usize)>) {
    if terminal(board) {
        return (utility(board), None);
    }

    let mut v = std::f64::INFINITY;
    let mut best_move: (usize, usize) = (0, 0);

    for action in actions(board) {
        let (ax, _ac) = max_value(&result(board, action));
        if ax < v {
            v = ax;
            best_move = action;
            if v == -1.0 {
                return (v, Some(best_move));
            }
        }
    }

    return (v, Some(best_move));
}

fn read_input() -> (usize, usize) {
    let mut input = String::new();

    print!("Enter two space-separated numbers: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let numbers: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid input, expected a number"))
        .collect();

    if numbers.len() != 2 {
        panic!("Invalid input, please enter exactly two numbers");
    }

    (numbers[0], numbers[1])
}

fn main() {
    let mut board: [[Option<Player>; 3]; 3] =
        [[None, None, None], [None, None, None], [None, None, None]];
    greeting();

    println!("Choose a player X or O");

    let mut player = String::new();
    let _user_player = stdin().read_line(&mut player).unwrap();
    let player = player.trim();

    let mut player_move: Player = Player::None;
    match player {
        "X" => player_move = Player::X,
        "O" => player_move = Player::O,
        _ => panic!("wrong player"),
    }

    loop {
        if player_move == Player::X {
            display_board(&board);
            let (row, pos) = read_input();
            let new_board = result(&board, (row, pos));
            board = new_board;
            if check_winner(&board).is_some() {
                display_board(&board);
                panic!("THERE IS A WINNER");
            }
            let best_move = min_max(&board).unwrap();
            board = result(&board, best_move);
        } else {
            let best_move = min_max(&board).unwrap();
            board = result(&board, best_move);
            display_board(&board);
            let (row, pos) = read_input();
            let new_board = result(&board, (row, pos));
            board = new_board
        }
    }
}
