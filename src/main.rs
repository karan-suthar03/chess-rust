mod engine;

use std::collections::{HashMap, HashSet};
use macroquad::prelude::*;
use crate::engine::core::Piece;
use crate::engine::main_board::MainBoard;


const CELL_SIZE: f32 = 100f32/4f32;

struct Selection{
    cell: (u32,u32),
    moves: HashSet<(u32,u32)>,
}

impl Selection {
    fn new(x:u32,y:u32) -> Self {
        Selection{cell: (x,y),moves: HashSet::new()}
    }
}

struct Game<'a>{
    board:MainBoard,
    texture_map: &'a HashMap<Piece, Texture2D>,
    selected:Option<Selection>,
}

impl<'a> Game<'a>{
    fn new(texture_map: &'a HashMap<Piece, Texture2D>) -> Game<'a>{
        let fen_board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = MainBoard::new_from_fen(fen_board);
        Game{
            board,
            texture_map,
            selected:None
        }
    }
    fn draw(&self){

        let drawable = self.board.get_drawable();

        for y in 0..8{
            for x in 0..8{
                let piece = drawable[7-y][x];

                let mut color = if (x + y) % 2 == 0 {
                    DARKGRAY
                } else {
                    WHITE
                };

                if self.selected.is_some() {
                    if self.selected.as_ref().unwrap().cell.0 == x as u32 && self.selected.as_ref().unwrap().cell.1 == y as u32 {
                        color = BLUE;
                    }else { 
                        let selection = self.selected.as_ref().unwrap();
                        
                        if selection.moves.contains(&(x as u32, y as u32)) {
                            color = RED;
                        }
                        
                    }
                }

                let x = x as f32;
                let y = y as f32;

                draw_rectangle((x -4.0) * CELL_SIZE, (y-4.0) * CELL_SIZE, CELL_SIZE, CELL_SIZE, color);

                if piece.is_piece() {
                    draw_texture_ex(
                        self.texture_map.get(&piece).unwrap(),
                        (x -4.0) * CELL_SIZE,(y-4.0) * CELL_SIZE,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(CELL_SIZE, CELL_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

    }

    fn update(&mut self, input: &Input){
        let world = input.world_pos;

        let bx = (world.x / CELL_SIZE + 4.0).floor() as i32;
        let by = (world.y / CELL_SIZE + 4.0).floor() as i32;

        if bx < 0 || bx >= 8 || by < 0 || by >= 8 {
            return;
        }

        let board_x = bx as u32;
        let board_y = 7 - by as u32;

        if input.clicked {
            if self.selected.is_none() {
                let mut selection = Selection::new(board_x, board_y);
                selection.moves = self.board.moves_for(board_x, board_y);
                self.selected = Some(selection);
            }else {
                if self.selected.as_ref().unwrap().cell.0 == board_x && self.selected.as_ref().unwrap().cell.1 == board_y {
                    self.selected = None;
                }else {
                    let mut selection = Selection::new(board_x, board_y);
                    selection.moves = self.board.moves_for(board_x, board_y);
                    self.selected = Some(selection);
                }
            }
        }
    }
}

fn normalize_camera(){
    let w = screen_width();
    let h = screen_height();

    let aspect = w / h;

    let zoom = if aspect >= 1.0 {
        vec2(0.01 / aspect, -0.01)
    } else {
        vec2(0.01, -0.01 * aspect)
    };
    let camera = Camera2D {
        zoom,
        ..Default::default()
    };
    set_camera(&camera);
}

async fn load_textures(texture_map: &mut HashMap<Piece, Texture2D>){
    let mut my_vec1 = Vec::new();
    my_vec1.push(Piece::King(engine::core::Color::White));
    my_vec1.push(Piece::Queen(engine::core::Color::White));
    my_vec1.push(Piece::Bishop(engine::core::Color::White));
    my_vec1.push(Piece::Knight(engine::core::Color::White));
    my_vec1.push(Piece::Rook(engine::core::Color::White));
    my_vec1.push(Piece::Pawn(engine::core::Color::White));
    my_vec1.push(Piece::King(engine::core::Color::Black));
    my_vec1.push(Piece::Queen(engine::core::Color::Black));
    my_vec1.push(Piece::Bishop(engine::core::Color::Black));
    my_vec1.push(Piece::Knight(engine::core::Color::Black));
    my_vec1.push(Piece::Rook(engine::core::Color::Black));
    my_vec1.push(Piece::Pawn(engine::core::Color::Black));
    let texture = load_texture("assets/pieces.png").await.unwrap();

    for (i , piece) in my_vec1.iter().enumerate(){
        let my_texture = render_target(120, 120);
        my_texture.texture.set_filter(FilterMode::Linear);
        set_camera(&Camera2D {
            render_target: Some(my_texture.clone()),
            zoom: vec2(2.0 / 150.0, -2.0 / 150.0),
            target: vec2(67.0, 67.0),
            ..Default::default()
        });

        let i = i % 6;

        draw_texture_ex(
            &texture,
            0.0, 0.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(170.0 * i as f32, if piece.is_white() { 0.0 }else { 170.0 }, 161.0, 161.0)),
                ..Default::default()
            },
        );
        set_default_camera();
        texture_map.insert(*piece, my_texture.texture);
    }
}

struct Input{
    clicked: bool,
    world_pos: Vec2,
}

fn mouse_to_world() -> Vec2 {
    let (mx, my) = mouse_position();

    let w = screen_width();
    let h = screen_height();

    let nx = (mx / w) * 2.0 - 1.0;
    let ny = 1.0 - (my / h) * 2.0;

    let aspect = w / h;

    let zoom = if aspect >= 1.0 {
        vec2(0.01 / aspect, -0.01)
    } else {
        vec2(0.01, -0.01 * aspect)
    };

    vec2(nx / zoom.x, ny / zoom.y)
}

impl Input{
    fn new() -> Input{
        Input{
            clicked:false,
            world_pos: mouse_to_world(),
        }
    }

    fn gather(&mut self){
        self.clicked = is_mouse_button_pressed(MouseButton::Left);
        self.world_pos = mouse_to_world();
    }
}

async fn run_game(){
    let mut texture_map = HashMap::new();
    load_textures(&mut texture_map).await;
    let mut game = Game::new(&texture_map);
    let mut input = Input::new();
    loop {
        normalize_camera();
        input.gather();
        game.update(&input);
        game.draw();
        next_frame().await;
    }
}
fn run(){
    macroquad::Window::new("chess hai ye",run_game());
}
fn main() {
    let test = false;
    if test {
        let fen_board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let main_board = MainBoard::new_from_fen(fen_board);
        main_board.display();
    }else{
        run();
    }
}
