use std::collections::{HashMap, HashSet};
use crate::engine::core::{Color, Piece};

#[derive(Debug)]
pub struct MainBoard{
    board: [Piece; 64],
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Pos2d {
    pub(crate) rank:u8,
    pub(crate) file:u8
}

fn to_2d_index(index: usize) -> Pos2d {
    let rank = (index / 8) as u8;
    let file = (index % 8) as u8;
    Pos2d {
        rank ,
        file
    }
}

fn from_2d_index(pos2d: Pos2d) -> usize{
    (pos2d.file + pos2d.rank*8) as usize
}

impl MainBoard {
    pub fn new() -> MainBoard{
        let mut board = [Piece::None; 64];
        board[0] = Piece::Pawn(Color::White);
        board[7] = Piece::Pawn(Color::White);
        MainBoard{
            board,
        }
    }

    pub fn new_from_fen(fen: &str) -> MainBoard{
        let mut board = [Piece::None; 64];
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
        println!("fen board: {}", fen_board);

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
                    board[rank*8+file] = *map.get(&ch).unwrap_or(&Piece::None);
                    file += 1;
                }
            }
        }

        MainBoard{
            board,
        }
    }

    pub(crate) fn make_move(&mut self, from: (u32, u32), to: (u32, u32)) {
        let (file, rank) = from;
        let from = Pos2d{
            file:file as u8,
            rank:rank as u8
        };
        let (file, rank) = to;
        let to = Pos2d{
            file:file as u8,
            rank:rank as u8
        };
        let from_index = from_2d_index(from);
        let to_index = from_2d_index(to);
        let piece = self.board[from_index];
        self.board[from_index] = Piece::None;
        self.board[to_index] = piece;
    }

    pub(crate) fn get_drawable(&self) -> [[Piece;8];8] {
        let mut board_2d = [[Piece::None; 8]; 8];

        for (i, cell) in self.board.iter().enumerate() {
            let rank = i / 8;
            let file = i % 8;

            board_2d[7 - rank][file] = *cell;
        }
        board_2d
    }

    pub(crate) fn moves_for(&mut self, p0: u32, p1: u32) -> HashSet<Pos2d> {
        let index = (p1 * 8 + p0) as usize;
        let piece = self.board[index];
        let mut set = HashSet::new();
        match piece {
            Piece::Pawn(color) => {
                self.get_pawn_moves(color, &mut set,index);
            }
            _ =>{}
        }
        set
    }

    fn get_pawn_moves(&mut self, color: Color, set: &mut HashSet<Pos2d>,index:usize) {
        let multiplier:i32;
        match color {
            Color::White => {
                multiplier = 1;

            },
            Color::Black => {
                multiplier = -1;
            }
        }

        let pos2d = to_2d_index(index);

        let new_rank = pos2d.rank as i32 +(1*multiplier);
        if new_rank < 0 || new_rank > 7 {
            return;
        }

        println!("pawn moves file {} rank {}", pos2d.file, pos2d.rank);
        let new_pos_2d = Pos2d{
            file: pos2d.file,
            rank: (pos2d.rank as i32 +(1*multiplier)) as u8
        };

        let piece = self.board[from_2d_index(new_pos_2d)];
        if !piece.is_piece() {
            set.insert(new_pos_2d);
        }

        // two moves for the first time of a pawn
        if (color == Color::White && pos2d.rank == 1) || (color == Color::Black && pos2d.rank == 6) {
            let new_pos_2d = Pos2d{
                rank: (new_pos_2d.rank as i32 + multiplier) as u8,
                ..new_pos_2d
            };
            let piece = self.board[from_2d_index(new_pos_2d)];
            if !piece.is_piece() {
                set.insert(new_pos_2d);
            }
        }
        {
            let to_position_2d = Pos2d{
                file:new_pos_2d.file - 1,
                ..new_pos_2d
            };
            let index = from_2d_index(to_position_2d);
            let piece = self.board[index];

            match piece.color() {
                Some(piece_color) =>{
                    if piece_color != color {
                        set.insert(to_position_2d);
                    }
                }
                _ =>{}
            }
        }
        {
            let to_position_2d = Pos2d{
                file:new_pos_2d.file + 1,
                ..new_pos_2d
            };
            let index = from_2d_index(to_position_2d);
            let piece = self.board[index];

            match piece.color() {
                Some(piece_color) =>{
                    if piece_color != color {
                        set.insert(to_position_2d);
                    }
                }
                _ =>{}
            }
        }
    }

    pub fn display(&self) {
        for (i, cell) in self.board.iter().rev().enumerate() {
            print!(" {:}", cell);
            if (i + 1) % 8 == 0 {
                println!();
            }
        }
    }
}