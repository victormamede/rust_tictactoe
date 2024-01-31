use std::io;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    Cross,
    Circle,
}

enum GameState {
    Winner(Player),
    Draw,
    Ongoing,
}

struct Board([Option<Player>; 9]);

impl Board {
    fn get_identifier(&self, index: usize) -> &'static str {
        let num_string: [&str; 9] = ["0", "1", "2", "3", "4", "5", "6", "7", "8"];
        match self.0[index] {
            Some(player) => match player {
                Player::Cross => "X",
                Player::Circle => "O",
            },
            None => num_string[index],
        }
    }
}

struct TicTacToe {
    board: Board,
    current_player: Player,
}

#[derive(Debug, Clone)]
struct InvalidMoveError;

impl TicTacToe {
    fn new() -> TicTacToe {
        TicTacToe {
            board: Board([None, None, None, None, None, None, None, None, None]),
            current_player: Player::Cross,
        }
    }

    fn play(&mut self, space: usize) -> Result<(), InvalidMoveError> {
        if self.board.0[space].is_some() {
            return Err(InvalidMoveError);
        }

        self.board.0[space] = Some(self.current_player);

        self.current_player = match self.current_player {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        };

        Ok(())
    }

    fn get_state(&self) -> GameState {
        if self.is_win(Player::Cross) {
            return GameState::Winner(Player::Cross);
        }
        if self.is_win(Player::Circle) {
            return GameState::Winner(Player::Circle);
        }
        if self.board.0.iter().all(|cell| cell.is_some()) {
            return GameState::Draw;
        }

        GameState::Ongoing
    }

    fn is_win(&self, player: Player) -> bool {
        let win_conditions = vec![
            // rows
            [true, true, true, false, false, false, false, false, false],
            [false, false, false, true, true, true, false, false, false],
            [false, false, false, false, false, false, true, true, true],
            // cols
            [true, false, false, true, false, false, true, false, false],
            [false, true, false, false, true, false, false, true, false],
            [false, false, true, false, false, true, false, false, true],
            // diagonals
            [true, false, false, false, true, false, false, false, true],
            [false, false, true, false, true, false, true, false, false],
        ];

        for win_condition in win_conditions {
            let mut bit_board = self.board.0.map(|cell| cell == Some(player));
            for i in 0..9 {
                bit_board[i] = bit_board[i] && win_condition[i];
            }
            if bit_board == win_condition {
                return true;
            }
        }
        false
    }

    fn print(&self) {
        println!(
            " {0} | {1} | {2} \n-----------\n {3} | {4} | {5} \n-----------\n {6} | {7} | {8} ",
            self.board.get_identifier(0),
            self.board.get_identifier(1),
            self.board.get_identifier(2),
            self.board.get_identifier(3),
            self.board.get_identifier(4),
            self.board.get_identifier(5),
            self.board.get_identifier(6),
            self.board.get_identifier(7),
            self.board.get_identifier(8),
        );
    }
}

fn main() {
    let mut game = TicTacToe::new();

    loop {
        match game.get_state() {
            GameState::Ongoing => {
                print!("{esc}c", esc = 27 as char);
                game.print();
                println!("\nYour play: ");
                let mut input_line = String::new();
                io::stdin()
                    .read_line(&mut input_line)
                    .expect("Failed to read line");
                let play = input_line.trim().parse::<usize>();

                match play {
                    Ok(play) => match game.play(play) {
                        Err(InvalidMoveError) => println!("Invalid move"),
                        _ => (),
                    },
                    Err(_) => println!("Invalid move!"),
                }
            }
            GameState::Draw => {
                println!("Draw!");
                break;
            }
            GameState::Winner(winner) => {
                println!("{:?} wins!", winner);
                break;
            }
        }
    }
}
