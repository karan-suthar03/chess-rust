use crate::engine::core::{Color, Piece, Pos2d};
use crate::engine::Engine;
use std::collections::HashSet;

impl Engine {

    fn check_for_check(&mut self, from: &Pos2d, to: &Pos2d) -> bool {
        let from_piece = self.board.get(&from);
        let to_piece = self.board.get(&to);
        
        self.board.set_at(&from,Piece::None);
        self.board.set_at(&to,from_piece);
        
        let mut set = HashSet::new();
        for file in 0..8 {
            for rank in 0..8 {
                let pos = Pos2d{
                    file,
                    rank
                };
                let piece = self.board.get(&pos);
                match piece.color() {
                    Some(color) => {
                        if color != from_piece.color().unwrap() {
                            self.sudo_legal_moves_for(&pos, &mut set);
                        }
                    }
                    None => {}
                }
            }
        }
        
        let mut is_check = false;
        for mov in set.iter() {
            let piece = self.board.get(&mov);
            match piece {
                Piece::King(color) => {
                    if color == from_piece.color().unwrap() {
                        is_check = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        self.board.set_at(&from,from_piece);
        self.board.set_at(&to,to_piece);
        
        is_check
    }
    pub fn moves_for(&mut self, new_pos:&Pos2d, set: &mut HashSet<Pos2d>){
        let piece = self.board.get(new_pos);
        if piece.color() != Some(self.turn) {
            return;
        }

        self.sudo_legal_moves_for(new_pos, set);
        let moves: Vec<Pos2d> = set.drain().collect();

        for pos in moves {
            if !self.check_for_check(new_pos, &pos) {
                set.insert(pos);
            }
        }
    }
    
    pub fn sudo_legal_moves_for(&self, new_pos:&Pos2d, set: &mut HashSet<Pos2d>) {
        let piece = self.board.get(new_pos);
        
        if piece.is_piece() {
            match piece {
                Piece::Pawn(color) => {
                    self.get_pawn_moves(color,set,&new_pos);
                }
                Piece::Rook(color) => {
                    self.get_rook_moves(color,set,&new_pos);
                }
                Piece::Bishop(color) => {
                    self.get_bishop_moves(color,set,&new_pos);
                }
                Piece::Queen(color) => {
                    self.get_queen_moves(color,set,&new_pos)
                }
                Piece::Knight(color) => {
                    self.get_knight_moves(color,set,&new_pos);
                }
                _ =>{}
            }
        }
    }

    fn get_pawn_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos2d: &Pos2d) {
        let multiplier:i32;
        match color {
            Color::White => {
                multiplier = 1;

            },
            Color::Black => {
                multiplier = -1;
            }
        }

        let new_rank = pos2d.rank as i32 +(1*multiplier);
        if new_rank < 0 || new_rank > 7 {
            return;
        }

        let new_pos_2d = Pos2d{
            file: pos2d.file,
            rank: (pos2d.rank as i32 +(1*multiplier)) as u8
        };

        let piece = self.board.get(&new_pos_2d);
        if !piece.is_piece() {
            set.insert(new_pos_2d);
            // two moves for the first time of a pawn
            if (color == Color::White && pos2d.rank == 1) || (color == Color::Black && pos2d.rank == 6) {
                let new_pos_2d = Pos2d{
                    rank: (new_pos_2d.rank as i32 + multiplier) as u8,
                    ..new_pos_2d
                };
                let piece = self.board.get(&new_pos_2d);
                if !piece.is_piece() {
                    set.insert(new_pos_2d);
                }
            }
        }
        if new_pos_2d.file != 0 {
            let attack_position_left = Pos2d{
                file:new_pos_2d.file - 1,
                ..new_pos_2d
            };
            let piece = self.board.get(&attack_position_left);

            match piece.color() {
                Some(piece_color) =>{
                    if piece_color != color {
                        set.insert(attack_position_left);
                    }
                }
                _ =>{}
            }
        }
        if new_pos_2d.file != 7 {
            let attack_pos_right = Pos2d{
                file:new_pos_2d.file + 1,
                ..new_pos_2d
            };
            let piece = self.board.get(&attack_pos_right);

            match piece.color() {
                Some(piece_color) =>{
                    if piece_color != color {
                        set.insert(attack_pos_right);
                    }
                }
                _ =>{}
            }
        }
    }

    // need cleanup
    fn get_rook_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos2d: &Pos2d) {
        let rook_dirs = [
            (1, 0),   // up
            (-1, 0),  // down
            (0, 1),   // right
            (0, -1),  // left
        ];

        self.slide_moves(color, set, pos2d, &rook_dirs);
    }


    fn get_bishop_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos2d: &Pos2d) {
        let bishop_dirs = [
            (1, 1),    // top-right
            (1, -1),   // top-left
            (-1, -1),  // bottom-left
            (-1, 1),   // bottom-right
        ];

        self.slide_moves(color, set, pos2d, &bishop_dirs);
    }

    fn slide_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos: &Pos2d, directions: &[(i8, i8)]) {
        for (dr, df) in directions {
            let mut r = pos.rank as i8;
            let mut f = pos.file as i8;

            loop {
                r += dr;
                f += df;

                if r < 0 || r > 7 || f < 0 || f > 7 {
                    break;
                }

                let new_pos = Pos2d {
                    rank: r as u8,
                    file: f as u8,
                };

                let piece = self.board.get(&new_pos);

                match piece.color() {
                    None => {
                        set.insert(new_pos);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos);
                        }
                        break;
                    }
                }
            }
        }
    }

    fn get_queen_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos2d: &Pos2d){
        let queen_dirs = [
            (1, 0),   // up
            (-1, 0),  // down
            (0, 1),   // right
            (0, -1),  // left
            (1, 1),    // top-right
            (1, -1),   // top-left
            (-1, -1),  // bottom-left
            (-1, 1),   // bottom-right
        ];

        self.slide_moves(color, set, pos2d, &queen_dirs);
    }

    fn get_knight_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos2d: &Pos2d){
        let knight_dirs = [
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
            (2, 1),
            (-2, 1),
            (2, -1),
            (-2, -1)
        ];

        for (dir_file, dir_rank) in knight_dirs {
            let new_rank = pos2d.rank as i8 + dir_rank;
            let new_file = pos2d.file as i8 + dir_file;
            if (new_file > 7 || new_file < 0) || (new_rank > 7 || new_rank < 0) {
                continue;
            }

            let new_pos = Pos2d{
                file:new_file as u8,
                rank:new_rank as u8,
            };

            let piece = self.board.get(&new_pos);

            match piece.color() {
                None => {
                    set.insert(new_pos);
                }
                Some(piece_color) => {
                    if piece_color != color {
                        set.insert(new_pos);
                    }
                }
            }
        }
    }
}