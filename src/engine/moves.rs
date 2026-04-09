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
        if pos2d.rank != 7 {
            'rankUp: for rank in (pos2d.rank + 1)..8 {
                let new_pos_2d = Pos2d {
                    rank,
                    ..*pos2d
                };
                let piece = self.board.get(&new_pos_2d);

                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'rankUp;
                    }
                }
            }
        }
        if pos2d.rank != 0 {
            'rankDown : for rank in (0..pos2d.rank).rev() {
                let new_pos_2d = Pos2d {
                    rank,
                    ..*pos2d
                };
                let piece = self.board.get(&new_pos_2d);
                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'rankDown;
                    }
                }
            }
        }

        if pos2d.file != 7 {
            'fileRight : for file in (pos2d.file + 1)..8 {
                let new_pos_2d = Pos2d {
                    file,
                    ..*pos2d
                };
                let piece = self.board.get(&new_pos_2d);
                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'fileRight;
                    }
                }
            }
        }
        if pos2d.file != 0 {
            'fileLeft : for file in (0..pos2d.file).rev(){
                let new_pos_2d = Pos2d {
                    file,
                    ..*pos2d
                };
                let piece = self.board.get(&new_pos_2d);
                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'fileLeft;
                    }
                }
            }
        }
    }


    fn get_bishop_moves(&self, color: Color, set: &mut HashSet<Pos2d>, pos2d: &Pos2d) {
        if pos2d.rank != 7 && pos2d.file != 7 {
            'topRight: for delta in 1..8 {
                if (pos2d.rank + delta) > 7 || (pos2d.file + delta) > 7{
                    break 'topRight;
                }
                let new_pos_2d = Pos2d {
                    rank: pos2d.rank + delta,
                    file: pos2d.file + delta
                };
                let piece = self.board.get(&new_pos_2d);

                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'topRight;
                    }
                }
            }
        }

        if pos2d.rank != 7 && pos2d.file != 0 {
            'topLeft: for delta in 1..8 {
                if (pos2d.rank + delta) > 7 || (pos2d.file as i8 - delta as i8) < 0{
                    break 'topLeft;
                }
                let new_pos_2d = Pos2d {
                    rank:pos2d.rank + delta,
                    file:(pos2d.file - delta)
                };
                let piece = self.board.get(&new_pos_2d);

                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'topLeft;
                    }
                }
            }
        }

        if pos2d.rank != 0 && pos2d.file != 0 {
            'bottomLeft: for delta in 1..8 {
                if (pos2d.rank as i8 - delta as i8) < 0 || (pos2d.file as i8 - delta as i8) < 0{
                    break 'bottomLeft;
                }
                let new_pos_2d = Pos2d {
                    rank:pos2d.rank - delta,
                    file:(pos2d.file - delta)
                };
                let piece = self.board.get(&new_pos_2d);

                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'bottomLeft;
                    }
                }
            }
        }

        if pos2d.rank != 0 && pos2d.file != 7 {
            'bottomRight: for delta in 1..8 {
                if (pos2d.rank as i8 - delta as i8) < 0 || (pos2d.file + delta) > 7{
                    break 'bottomRight;
                }
                let new_pos_2d = Pos2d {
                    rank:pos2d.rank - delta,
                    file:pos2d.file + delta
                };
                let piece = self.board.get(&new_pos_2d);

                match piece.color() {
                    None => {
                        set.insert(new_pos_2d);
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(new_pos_2d);
                        }
                        break 'bottomRight;
                    }
                }
            }
        }
    }
}