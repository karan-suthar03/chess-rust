use std::collections::{HashMap, HashSet};
use crate::engine::core::{Color, Piece};

#[derive(Debug)]
pub struct MainBoard{
    board: [Piece; 64],
}

fn to_2d_index(index: usize) -> (u32,u32){
    let rank = index / 8;
    let file = index % 8;
    (file as u32, rank as u32)
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

    pub(crate) fn get_drawable(&self) -> [[Piece;8];8] {
        let mut board_2d = [[Piece::None; 8]; 8];

        for (i, cell) in self.board.iter().enumerate() {
            let rank = i / 8;
            let file = i % 8;

            board_2d[7 - rank][file] = *cell;
        }
        board_2d
    }

    pub(crate) fn moves_for(&mut self, p0: u32, p1: u32) -> HashSet<(u32, u32)> {
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

    fn get_pawn_moves(&mut self, color: Color, set: &mut HashSet<(u32, u32)>,index:usize) {
        if color == Color::White {
            set.insert(to_2d_index(index+8));
            set.insert(to_2d_index(index+16));
        }else {
            set.insert(to_2d_index(index-8));
            set.insert(to_2d_index(index-16));
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