use array2d::Array2D;
use itertools::Itertools;
use std::fmt;

const BOARD_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Piece {
    X,
    O,
}
impl Piece {
    fn other(&self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Winner {
    X,
    O,
    Tie,
}

impl From<Piece> for Winner {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::X => Self::X,
            Piece::O => Self::O,
        }
    }
}

type Board = Array2D<Option<Piece>>;

#[derive(Debug, Clone)]
struct Game {
    board: Board,
    current_piece: Piece,
    winner: Option<Winner>,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum MoveError {
    GameAlreadyOver,
    InvalidPosition {
        row: usize,
        col: usize,
    },
    TileNotEmpty {
        other_piece: Piece,
        row: usize,
        col: usize,
    },
}

fn to_winner(board: &Board) -> Option<Winner> {
    // Check rows
    for (i, mut row) in board.rows_iter().enumerate() {
        let first = board[(i, 0)];
        if first.is_some() && row.all(|&p| p == first) {
            return Some(first.unwrap().into());
        }
    }

    // Check columns
    for (i, mut col) in board.columns_iter().enumerate() {
        let first = board[(0, i)];
        if first.is_some() && col.all(|&p| p == first) {
            return Some(first.unwrap().into());
        }
    }

    // check first diag
    assert_eq!(board.num_rows(), board.num_columns());
    let top_left = board[(0, 0)];
    if top_left.is_some()
        && (1..board.num_rows())
            .map(|i| board[(i, i)])
            .all(|p| p == top_left)
    {
        return Some(top_left.unwrap().into());
    }

    // check second diag
    let max_index = board.num_rows() - 1;
    let top_right = board[(0, max_index)];
    if top_right.is_some()
        && (1..board.num_rows())
            .map(|i| {
                let row = i;
                let col = max_index - i;
                board[(row, col)]
            })
            .all(|p| p == top_right)
    {
        return Some(top_right.unwrap().into());
    }

    // test for tie
    if board.rows_iter().flatten().all(|&p| p.is_some()) {
        return Some(Winner::Tie);
    }

    None
}

impl Game {
    fn new() -> Self {
        Self {
            board: Array2D::filled_with(None, BOARD_SIZE, BOARD_SIZE),
            current_piece: Piece::X,
            winner: None,
        }
    }

    fn is_finished(&self) -> bool {
        self.winner.is_some()
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<(), MoveError> {
        if self.is_finished() {
            return Err(MoveError::GameAlreadyOver);
        }

        if row >= self.board.num_rows() || col >= self.board.num_columns() {
            return Err(MoveError::InvalidPosition { row, col });
        }

        if let Some(piece) = self.board[(row, col)] {
            return Err(MoveError::TileNotEmpty {
                other_piece: piece,
                row,
                col,
            });
        }

        // modify the current state
        self.board[(row, col)] = Some(self.current_piece);
        self.current_piece = self.current_piece.other();
        self.winner = to_winner(&self.board);
        Ok(())
    }

    fn valid_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        for row in 0..self.board.num_rows() {
            for col in 0..self.board.num_columns() {
                if self.board[(row, col)].is_none() {
                    moves.push((row, col));
                }
            }
        }
        moves
    }
}

fn to_char(maybe_piece: &Option<Piece>) -> char {
    match maybe_piece {
        None => ' ',
        Some(Piece::X) => 'X',
        Some(Piece::O) => 'O',
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_board = self
            .board
            .rows_iter()
            .map(|row| row.map(to_char).join("|"))
            .join("\n-----\n");

        write!(f, "{}\nWinner: {:?}", display_board, self.winner)
    }
}

fn next_games(game: &Game) -> Vec<Game> {
    game.valid_moves()
        .iter()
        .flat_map(|(row, col)| {
            let mut game = game.clone();
            let res = game.make_move(*row, *col);
            assert!(res.is_ok(), "{:?}", res.err().unwrap());
            Ok::<Game, MoveError>(game)
        })
        .collect()
}

struct GameTree {
    game: Game,
    edges: Vec<Edge>,
}

impl fmt::Display for GameTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Current State:\n{}\n\n{}",
            self.game,
            self.edges
                .iter()
                .map(|e| {
                    format!(
                        "{}\nO wins: {}\nX wins: {}\nTies: {}",
                        e.child.game, e.x_wins, e.o_wins, e.ties
                    )
                })
                .join("\n\n")
        )
    }
}

struct Edge {
    child: GameTree,
    x_wins: f32,
    o_wins: f32,
    ties: f32,
}

impl GameTree {
    fn from(game: Game) -> Self {
        if game.is_finished() {
            return Self {
                game: game.clone(),
                edges: vec![],
            };
        }

        let edges = next_games(&game)
            .iter()
            .map(|game| {
                let child = GameTree::from(game.clone());
                let x_wins = child.x_wins();
                let o_wins = child.o_wins();
                let ties = child.ties();
                Edge {
                    child,
                    x_wins,
                    o_wins,
                    ties,
                }
            })
            .collect();

        Self {
            game: game.clone(),
            edges,
        }
    }

    fn o_wins(&self) -> f32 {
        if self.game.is_finished() {
            return match self.game.winner {
                Some(Winner::O) => 1.0,
                _ => 0.0,
            };
        }
        assert_ne!(self.edges.len(), 0);
        self.edges.iter().map(|e| e.child.o_wins()).sum::<f32>() / self.edges.len() as f32
    }

    // percentage of the times that x wins
    fn x_wins(&self) -> f32 {
        if self.game.is_finished() {
            return match self.game.winner {
                Some(Winner::X) => 1.0,
                _ => 0.0,
            };
        }
        assert_ne!(self.edges.len(), 0);
        self.edges.iter().map(|e| e.child.x_wins()).sum::<f32>() / self.edges.len() as f32
    }

    // percentage of the times that game ends in tie
    fn ties(&self) -> f32 {
        if self.game.is_finished() {
            return match self.game.winner {
                Some(Winner::Tie) => 1.0,
                _ => 0.0,
            };
        }
        assert_ne!(self.edges.len(), 0);
        self.edges.iter().map(|e| e.child.ties()).sum::<f32>() / self.edges.len() as f32
    }
}

fn main() {
    let game_tree = GameTree::from(Game::new());
    println!("{}", game_tree);
    println!(
        "\n\nX: {}\nO: {}\nTie: {}\nsum: {}",
        game_tree.x_wins(),
        game_tree.o_wins(),
        game_tree.ties(),
        game_tree.x_wins() + game_tree.o_wins() + game_tree.ties()
    );
}
