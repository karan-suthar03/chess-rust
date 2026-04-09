use crate::engine::core::{Color, Piece, Pos2d};
use crate::engine::Engine;
use std::collections::HashSet;

impl Engine {

    pub fn moves_for(&self, new_pos:&Pos2d) -> HashSet<Pos2d> {
        let piece = self.board.get(new_pos);

        let mut set = HashSet::new();
        match piece.color() {
            Some(color) => {
                if color == self.turn {
                    match piece {
                        Piece::Pawn(color) => {
                            self.get_pawn_moves(color, &mut set,&new_pos);
                        }
                        Piece::Rook(color) => {
                            self.get_rook_moves(color, &mut set,&new_pos);
                        }
                        Piece::Bishop(color) => {
                            self.get_bishop_moves(color, &mut set,&new_pos);
                        }
                        Piece::Queen(color) => {
                            self.get_queen_moves(color, &mut set,&new_pos)
                        }
                        _ =>{}
                    }
                }
            }
            None => {}
        }
        set
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
}