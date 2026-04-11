use crate::engine::core::{Color, Move, Piece, Pos2d};
use crate::engine::Engine;
use std::collections::HashSet;

impl Engine {

    fn check_for_check(&mut self, from: &Pos2d, to: &Pos2d) -> bool {
        let from_piece = self.board.get(&from);
        let mut is_pawn = false;
        match from_piece {
            Piece::Pawn(_) => {
                is_pawn = true;
            }
            _ => {}
        }

        if is_pawn && self.en_peasant.is_some() && self.en_peasant.unwrap() == *to {
            self.board.set_at(&Pos2d{
                rank: ((to.rank as i8) + if from.rank < to.rank { -1 } else { 1 }) as u8,
                ..*to
            },Piece::None);
        }
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
            let piece = self.board.get(&mov.to);
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

        if is_pawn && self.en_peasant.is_some() && self.en_peasant.unwrap() == *to {
            self.board.set_at(&Pos2d{
                rank: ((to.rank as i8) + if from.rank < to.rank { -1 } else { 1 }) as u8,
                ..*to
            },Piece::Pawn(from_piece.color().unwrap().flip()));
        }
        self.board.set_at(&from,from_piece);
        self.board.set_at(&to,to_piece);

        is_check
    }
    pub fn moves_for(&mut self, new_pos:&Pos2d, set: &mut HashSet<Move>){
        let piece = self.board.get(new_pos);
        if piece.color() != Some(self.turn) {
            return;
        }

        self.sudo_legal_moves_for(new_pos, set);
        let moves: Vec<Move> = set.drain().collect();

        for pos in moves {
            let is_king_castle = matches!(piece, Piece::King(_))
                && (new_pos.file as i8 - pos.to.file as i8).abs() == 2;

            if is_king_castle {
                let in_check_now = self.check_for_check(new_pos, new_pos);
                let middle_file = if pos.to.file > new_pos.file { new_pos.file + 1 } else { new_pos.file - 1 };
                let middle = Pos2d {
                    file: middle_file,
                    rank: new_pos.rank,
                };
                let through_check = self.check_for_check(new_pos, &middle);
                let end_check = self.check_for_check(new_pos, &pos.to);

                if !in_check_now && !through_check && !end_check {
                    set.insert(pos);
                }
            } else if !self.check_for_check(new_pos, &pos.to) {
                set.insert(pos);
            }
        }
    }

    pub fn sudo_legal_moves_for(&self, new_pos:&Pos2d, set: &mut HashSet<Move>) {
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
                Piece::King(color) => {
                    self.get_king_moves(color,set,&new_pos);
                }
                _ =>{}
            }
        }
    }

    fn get_king_moves(&self, color: Color, set: &mut HashSet<Move>, pos2d: &Pos2d) {
        let all_moves = [
            (-1,-1),
            (0,-1),
            (1,-1),
            (-1,0),
            (0,0),
            (1,0),
            (-1,1),
            (0,1),
            (1,1)
        ];

        for (dir_file, dir_rank) in all_moves {
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
                    set.insert(Move{
                        from:*pos2d,
                        to:new_pos,
                        promotion:None
                    });
                }
                Some(piece_color) => {
                    if piece_color != color {
                        set.insert(Move{
                            from:*pos2d,
                            to:new_pos,
                            promotion:None
                        });
                    }
                }
            }
        }

        let rights = if color == Color::White {
            self.white_castle
        } else {
            self.black_castle
        };


        if rights == 1 || rights == 3 {
            let rook_pos = Pos2d { file: 7, rank: pos2d.rank };
            let f_pos = Pos2d { file: 5, rank: pos2d.rank };
            let g_pos = Pos2d { file: 6, rank: pos2d.rank };

            if self.board.get(&rook_pos) == Piece::Rook(color)
                && self.board.get(&f_pos) == Piece::None
                && self.board.get(&g_pos) == Piece::None
            {
                set.insert(Move {
                    from: *pos2d,
                    to: g_pos,
                    promotion: None,
                });
            }
        }

        if rights == 2 || rights == 3 {
            let rook_pos = Pos2d { file: 0, rank: pos2d.rank };
            let b_pos = Pos2d { file: 1, rank: pos2d.rank };
            let c_pos = Pos2d { file: 2, rank: pos2d.rank };
            let d_pos = Pos2d { file: 3, rank: pos2d.rank };

            if self.board.get(&rook_pos) == Piece::Rook(color)
                && self.board.get(&b_pos) == Piece::None
                && self.board.get(&c_pos) == Piece::None
                && self.board.get(&d_pos) == Piece::None
            {
                set.insert(Move {
                    from: *pos2d,
                    to: c_pos,
                    promotion: None,
                });
            }
        }
    }


    fn get_pawn_moves(&self, color: Color, set: &mut HashSet<Move>, pos2d: &Pos2d) {
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

        let mut vec:Vec<Pos2d> = Vec::new();

        let piece = self.board.get(&new_pos_2d);
        if !piece.is_piece() {
            vec.push(new_pos_2d);
            // two moves for the first time of a pawn
            if (color == Color::White && pos2d.rank == 1) || (color == Color::Black && pos2d.rank == 6) {
                let new_pos_2d = Pos2d{
                    rank: (new_pos_2d.rank as i32 + multiplier) as u8,
                    ..new_pos_2d
                };
                let piece = self.board.get(&new_pos_2d);
                if !piece.is_piece() {
                    vec.push(new_pos_2d);
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
                        vec.push(attack_position_left);
                    }
                }
                _ =>{}
            }
            if self.en_peasant.is_some() && self.en_peasant.unwrap() == attack_position_left {
                vec.push(attack_position_left);
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
                        vec.push(attack_pos_right);
                    }
                }
                _ =>{}
            }
            if self.en_peasant.is_some() && self.en_peasant.unwrap() == attack_pos_right {
                vec.push(attack_pos_right);
            }
        }

        for new_pos in vec {
            let is_promotion_rank = (color == Color::White && new_pos.rank == 7)
                || (color == Color::Black && new_pos.rank == 0);

            if is_promotion_rank {
                let mut my_vec1 = Vec::new();
                my_vec1.push(Piece::Queen(color));
                my_vec1.push(Piece::Bishop(color));
                my_vec1.push(Piece::Knight(color));
                my_vec1.push(Piece::Rook(color));

                for promo in my_vec1 {
                    let new_move = Move{
                        from: *pos2d,
                        to: new_pos,
                        promotion: Some(promo),
                    };
                    set.insert(new_move);
                }
            } else {
                let new_move = Move{
                    from: *pos2d,
                    to: new_pos,
                    promotion: None,
                };
                set.insert(new_move);
            }
        }
    }

    // need cleanup
    fn get_rook_moves(&self, color: Color, set: &mut HashSet<Move>, pos2d: &Pos2d) {
        let rook_dirs = [
            (1, 0),   // up
            (-1, 0),  // down
            (0, 1),   // right
            (0, -1),  // left
        ];

        self.slide_moves(color, set, pos2d, &rook_dirs);
    }


    fn get_bishop_moves(&self, color: Color, set: &mut HashSet<Move>, pos2d: &Pos2d) {
        let bishop_dirs = [
            (1, 1),    // top-right
            (1, -1),   // top-left
            (-1, -1),  // bottom-left
            (-1, 1),   // bottom-right
        ];

        self.slide_moves(color, set, pos2d, &bishop_dirs);
    }

    fn slide_moves(&self, color: Color, set: &mut HashSet<Move>, pos: &Pos2d, directions: &[(i8, i8)]) {
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
                        set.insert(Move{
                            from:*pos,
                            to:new_pos,
                            promotion:None
                        });
                    }
                    Some(piece_color) => {
                        if piece_color != color {
                            set.insert(Move{
                                from:*pos,
                                to:new_pos,
                                promotion:None
                            });
                        }
                        break;
                    }
                }
            }
        }
    }

    fn get_queen_moves(&self, color: Color, set: &mut HashSet<Move>, pos2d: &Pos2d){
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

    fn get_knight_moves(&self, color: Color, set: &mut HashSet<Move>, pos2d: &Pos2d){
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
                    set.insert(Move{
                        from:*pos2d,
                        to:new_pos,
                        promotion:None
                    });
                }
                Some(piece_color) => {
                    if piece_color != color {
                        set.insert(Move{
                            from:*pos2d,
                            to:new_pos,
                            promotion:None
                        });
                    }
                }
            }
        }
    }
}