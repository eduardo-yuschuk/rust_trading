extern crate sdl2;
//extern crate sdl2_ttf;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::path::Path;
//use sdl2::ttf::
use std::time::Duration;
#[macro_use]
extern crate serde_derive;
extern crate serde;
// use std::fs;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::Read;
// use std::sync::mpsc;
// use std::sync::mpsc::{Receiver, Sender};
// use std::thread;
use std::time::Instant;
// extern crate rand;
// use rand::prelude::random;
//use std::fs::File;
use std::time::SystemTime;
//use std::io::prelude::*;
extern crate configuration;
use configuration::get_text_quotes_file_path;
extern crate common;
use common::Quote;
extern crate text_io;
//use std::io::BufWriter;
use text_io::read_quotes_from_csv;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyboard {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
}

impl Keyboard {
    pub fn empty() -> Keyboard {
        Keyboard {
            left: false,
            right: false,
            up: false,
            down: false,
            space: false,
        }
    }
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (800u32 as i32 - w) / 2;
    let cy = (600u32 as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

////////////////////////////////////////////////////////////////////////////////
/// GRAPHICS LOOP
fn run_graphics(title: &str, width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut begin = Instant::now();
    let mut iterations = 0i32;
    let mut game_state = GameState::initial(width as i32, height as i32);

    // Load a font
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path: &Path = Path::new("quotes_chart/fonts/FreeMono.ttf");
    let mut font = ttf_context.load_font(font_path, 18).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render("Hello Rust!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())
        .unwrap();
    //let mut renderer = window.renderer().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap();

    'running: loop {
        // exit
        let mut keyboard = Keyboard::empty();
        for event in event_pump.poll_iter() {
            //println!("event: {:?}", event);
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => keyboard.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => keyboard.right = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => keyboard.up = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => keyboard.down = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => keyboard.space = true,
                _ => {}
            }
        }
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        //::std::thread::sleep(Duration::new(0, 1u32));
        ////////////////////////////////////////////////////////////////////////
        // drawing
        //let drawing_begin = Instant::now();
        update_game(&mut game_state, &keyboard);
        draw_game(&mut canvas, &mut game_state);

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        let target = get_centered_rect(width, height, 800u32 - padding, 600u32 - padding);

        canvas.copy(&mut texture, None, Some(target)).unwrap();

        canvas.present();
        ////////////////////////////////////////////////////////////////////////
        // fps
        iterations += 1;
        let elapsed = begin.elapsed();
        if elapsed.as_secs() >= 1u64 {
            println!(
                "run_graphics({:?}), iterations: {}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap(),
                iterations
            );
            iterations = 0i32;
            begin = Instant::now();
        }
        ////////////////////////////////////////////////////////////////////////
        //let drawing_elapsed = drawing_begin.elapsed();
        ::std::thread::sleep(Duration::new(
            0,
            1_000_000_000u32 / 60u32 / 2u32,
            //(1_000_000_000u128 / (60u128 + 1u128) - drawing_elapsed.as_nanos()) as u32,
        ));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn center(width: i32, height: i32) -> Position {
        Position {
            x: width / 2i32,
            y: height / 2i32,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub width: i32,
    pub height: i32,
    pub character_position: Position,
}

impl GameState {
    pub fn initial(width: i32, height: i32) -> GameState {
        GameState {
            width: width,
            height: height,
            character_position: Position::center(width, height),
        }
    }
}

fn update_game(game_state: &mut GameState, keyboard: &Keyboard) {
    //println!("keyboard: {:?}", keyboard);
    if keyboard.left {
        game_state.character_position.x -= 10i32;
    }
    if keyboard.right {
        game_state.character_position.x += 10i32;
    }
    if keyboard.up {
        game_state.character_position.y -= 10i32;
    }
    if keyboard.down {
        game_state.character_position.y += 10i32;
    }
}

fn draw_game(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, game_state: &mut GameState) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    // drawing
    let sprite_size = 10i32;
    let rect = sdl2::rect::Rect::new(
        game_state.character_position.x,
        game_state.character_position.y,
        sprite_size as u32,
        sprite_size as u32,
    );

    canvas.fill_rect(rect).unwrap();
}

fn main() {
    let quotes_file_path = get_text_quotes_file_path();

    println!("reading quotes from: \n{}", quotes_file_path);

    println!("binary quotes file created...");

    let mut readed_counter = 0;

    read_quotes_from_csv(
        &get_text_quotes_file_path(),
        &mut |quote: Quote| {
            println!("{:?}", quote);
            readed_counter += 1;
            if readed_counter % 1000000 == 0 {
                println!("{} quotes readed", readed_counter);
            }
        },
        1024,
    );

    println!("{} quotes readed", readed_counter);

    run_graphics("simple blocks game", 800u32, 600u32);
}
