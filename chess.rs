use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
struct ChessPiece {
    piece: Piece,
    color: Color,
}

type Board = [[Option<ChessPiece>; 8]; 8];

struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    fn new() -> Self {
        let mut board: Board = [[None; 8]; 8];

        // Place pawns
        for i in 0..8 {
            board[1][i] = Some(ChessPiece { piece: Piece::Pawn, color: Color::Black });
            board[6][i] = Some(ChessPiece { piece: Piece::Pawn, color: Color::White });
        }

        // Place other pieces
        let back_rank = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::Queen,
            Piece::King,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for (i, &piece) in back_rank.iter().enumerate() {
            board[0][i] = Some(ChessPiece { piece, color: Color::Black });
            board[7][i] = Some(ChessPiece { piece, color: Color::White });
        }

        Game {
            board,
            turn: Color::White,
        }
    }

    fn display(&self) {
        for row in &self.board {
            for square in row {
                match square {
                    Some(ChessPiece { piece, color }) => {
                        let symbol = match (piece, color) {
                            (Piece::Pawn, Color::White) => "P",
                            (Piece::Pawn, Color::Black) => "p",
                            (Piece::Rook, Color::White) => "R",
                            (Piece::Rook, Color::Black) => "r",
                            (Piece::Knight, Color::White) => "N",
                            (Piece::Knight, Color::Black) => "n",
                            (Piece::Bishop, Color::White) => "B",
                            (Piece::Bishop, Color::Black) => "b",
                            (Piece::Queen, Color::White) => "Q",
                            (Piece::Queen, Color::Black) => "q",
                            (Piece::King, Color::White) => "K",
                            (Piece::King, Color::Black) => "k",
                        };
                        print!("{} ", symbol);
                    }
                    None => print!(". "),
                }
            }
            println!();
        }
        println!();
    }

    fn get_ai_move(&self) -> Option<((usize, usize), (usize, usize))> {
        let mut moves = vec![];

        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    if piece.color == self.turn {
                        // Generate basic moves based on piece type
                        let directions = match piece.piece {
                            Piece::Pawn => vec![(1, 0), (-1, 0)],
                            Piece::Rook => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
                            Piece::Bishop => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
                            Piece::Queen => vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)],
                            Piece::King => vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)],
                            Piece::Knight => vec![(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)],
                        };

                        for &(di, dj) in &directions {
                            let ni = i as isize + di;
                            let nj = j as isize + dj;

                            if ni >= 0 && ni < 8 && nj >= 0 && nj < 8 {
                                let ni = ni as usize;
                                let nj = nj as usize;
                                if self.board[ni][nj].is_none() || self.board[ni][nj].unwrap().color != piece.color {
                                    moves.push(((i, j), (ni, nj)));
                                }
                            }
                        }
                    }
                }
            }
        }

        if moves.is_empty() {
            None
        } else {
            // Select the first valid move for simplicity (basic AI)
            Some(moves[0])
        }
    }

    fn make_move(&mut self, mv: ((usize, usize), (usize, usize))) {
        let ((from_x, from_y), (to_x, to_y)) = mv;
        self.board[to_x][to_y] = self.board[from_x][from_y];
        self.board[from_x][from_y] = None;
    }

    fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn is_checkmate(&self) -> bool {
        // Basic checkmate detection placeholder (can be expanded)
        !self.get_ai_move().is_some()
    }

    fn play(&mut self, game_limit: u64, move_limit: usize) {
        let start_time = Instant::now();
        let mut move_count = 0;

        loop {
            if start_time.elapsed().as_secs() >= game_limit {
                println!("Game over! Time limit of {} seconds reached.", game_limit);
                break;
            }

            if move_count >= move_limit {
                println!("Game over! Move limit of {} moves reached.", move_limit);
                break;
            }

            self.display();

            if self.is_checkmate() {
                println!("Checkmate! {:?} wins!", match self.turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                });
                break;
            }

            if let Some(mv) = self.get_ai_move() {
                self.make_move(mv);
                self.switch_turn();
                move_count += 1;
            } else {
                println!("Game over! No more moves for {:?}", self.turn);
                break;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    let game_limit = 300; // 5 minutes in seconds
    let move_limit = 40;  // 20 moves per side
    game.play(game_limit, move_limit);
}

