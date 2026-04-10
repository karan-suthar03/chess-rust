pub mod core;
pub mod board;

mod moves;

use std::collections::{HashMap, HashSet};
use crate::engine::board::Board;
use crate::engine::core::{Color, Piece, Pos2d};

#[derive(Debug)]
pub struct Engine {
    board: Board,
    turn: Color,
    en_peasant: Option<Pos2d>,
}

impl Engine {
    pub fn new() -> Self {
        let board = Board::new();
        Self {
            board,
            turn: Color::White,
            en_peasant: None,
        }
    }

    pub fn get_piece_at(&self, loc_2d: Pos2d) -> Piece {
        self.board.get(&loc_2d)
    }

    pub fn new_from_fen(fen: &str) -> Self {
        let mut board = Board::new();
        let mut map = HashMap::new();
        map.insert('k',Piece::King(Color::Black));
        map.insert('p',Piece::Pawn(Color::Black));
        map.insert('q',Piece::Queen(Color::Black));
        map.insert('r',Piece::Rook(Color::Black));
        map.insert('b',Piece::Bishop(Color::Black));
        map.insert('n',Piece::Knight(Color::Black));
        map.insert('K',Piece::King(Color::White));
        map.insert('P',Piece::Pawn(Color::White));
        map.insert('Q',Piece::Queen(Color::White));
        map.insert('R',Piece::Rook(Color::White));
        map.insert('B',Piece::Bishop(Color::White));
        map.insert('N',Piece::Knight(Color::White));

        let fen_parts: Vec<&str> = fen.split(' ').collect();

        let fen_board = fen_parts[0];

        let mut file = 0;
        let mut rank = 7;

        for ch in fen_board.chars() {
            if ch == '/' {
                file = 0;
                rank-=1;
            }else {
                if ch.is_ascii_digit() {
                    file+= ch.to_digit(10).unwrap() as usize;
                }else{
                    board.set_at(&Pos2d::from_index(rank * 8 + file), *map.get(&ch).unwrap_or(&Piece::None));
                    file += 1;
                }
            }
        }

        let fen_turn = fen_parts[1];
        let turn = match fen_turn {
            "b" => {
                Color::Black
            }
            _ => {
                Color::White
            }
        };

        let fen_en = fen_parts[3];
        let en_peasant = if fen_en != "-" {
            Some(Pos2d::from_string(fen_en))
        }else{
            None
        };

        Self {
            board,
            turn,
            en_peasant
        }
    }

    pub fn make_move(&mut self, from: &Pos2d, to: &Pos2d) {
        let piece = self.board.get(&from);
        let mut is_pawn = false;
        match piece {
            Piece::Pawn(_) => {
                is_pawn = true;
            }
            _ => {}
        }

        self.board.set_at(&from,Piece::None);
        self.board.set_at(&to,piece);

        if is_pawn && self.en_peasant.is_some() && self.en_peasant.unwrap() == *to {
            self.board.set_at(&Pos2d{
                rank: ((to.rank as i8) + if from.rank < to.rank { -1 } else { 1 }) as u8,
                ..*to
            },Piece::None);
            self.en_peasant = None;
        }

        if is_pawn {
            if (from.rank as i8 - to.rank as i8).abs() == 2 {
                if from.rank < to.rank{
                    self.en_peasant = Some(Pos2d{
                        rank:to.rank - 1,
                        ..*to
                    })
                } else {
                    self.en_peasant = Some(Pos2d{
                        rank:to.rank + 1,
                        ..*to
                    })
                }
            }
        }

        self.turn = self.turn.flip();
    }

    // probably change soon
    pub fn display(&self) {
        for (i, cell) in self.board.board.iter().rev().enumerate() {
            print!(" {:}", cell);
            if (i + 1) % 8 == 0 {
                println!();
            }
        }
    }
}

pub trait EngineTestExt {
    fn generate_moves(&mut self) -> HashSet<String>;
}

impl EngineTestExt for Engine {

    // i just realized its too dumb
    fn generate_moves(&mut self) -> HashSet<String> {
        let mut set:HashSet<String> = HashSet::new();

        for file in 0..8 {
            for rank in 0..8 {
                let pos = Pos2d{
                    file,
                    rank
                };

                let mut movs = HashSet::new();
                self.moves_for(&pos, &mut movs);
                for positions in movs.iter() {
                    set.insert(positions.to_string());
                }
            }
        }
        set
    }
}