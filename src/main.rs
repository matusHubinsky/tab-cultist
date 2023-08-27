
use sdl2::video::FullscreenType;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseState, MouseUtil};
use std::time::{Duration, Instant};

use std::path::Path;
use std::env;

pub mod graphics;
pub mod texture;
pub mod data;

use crate::data::Song;
use crate::data::Album;
use crate::data::Window;
use crate::data::Database;
use crate::data::Runner;

mod constants {
    pub const FRAME_TIME: f32 = 0.02;
}


fn main() -> Result<(), String> {
    println!("Hail Satan!");
    if let Ok(current_dir) = env::current_dir() {
        if let Some(dir) = current_dir.to_str() {
            println!("From: {}", dir);
        }
    }

    let mut database = Database::new("database/".to_string());    

    // album.index = (album.index + 1) % album.songs.len();
    let mut song = Song::new();
    Song::load(Database::next(&mut database), &mut song);
    
    let mut window = Window::new();

    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Initialize Font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    // let font_path: &Path = Path::new(&"./font/VT323/VT323-Regular.ttf");
    let font_path: &Path = Path::new(&"./font/Roboto_Mono/RobotoMono-VariableFont_wght.ttf");
    let font_big = ttf_context.load_font(font_path, 128)?;
    let font_small = ttf_context.load_font(font_path, 32)?;
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    // Create a window
    let sdl_window = video_subsystem
        .window("Tab Cultist", window.width, window.height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = sdl_window.into_canvas().build()
        .expect("could not make a canvas");
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "1");

    // Initializa Texture Creator
    let texture_creator = canvas.texture_creator();
    let mut tex_man = texture::TextureManager::new(&texture_creator);

    let mut fullscreen = false;

    let mut runner = Runner::new();
    let mut i = 0;
    let shift = 24;

    // Main event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        
        let frame_start_time = Instant::now();

        // Handle events
        for event in event_pump.poll_iter() {
            // TODO: render on click
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    // Toggle fullscreen
                    fullscreen = !fullscreen;
                    // Set fullscreen mode
                    if fullscreen {
                        canvas.window_mut().set_fullscreen(FullscreenType::Desktop).unwrap();
                    } else {
                        canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap();
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    runner.x = 60;
                    runner.y = 80;
                    runner.play = true;
                    runner.show = true;
                    i = 0;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    runner.play = !runner.play;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    Song::load(Database::next(&mut database), &mut song)?;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    Song::load(Database::prev(&mut database), &mut song)?;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    todo!();
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    todo!();
                }
                Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(width, height) => {
                            if width <= 600 {
                                window.width = 600;
                            } else {
                                window.width = width as u32;
                            }

                            if height <= 400 {
                                window.height = 400; 
                            } else {
                                window.height = height as u32;
                            }

                            println!("Window resized: {}x{}, {}x{}", width, height, window.width, window.height);
                            graphics::render(&mut canvas, &mut tex_man, &texture_creator, &font_big, &font_small, &mut song, &mut window, &runner)?;                        
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let elapsed_frame_time = frame_start_time.elapsed();
        if elapsed_frame_time < Duration::from_secs_f32(constants::FRAME_TIME) {
            let mouse_state: MouseState = event_pump.mouse_state();
            window.mouse_x = mouse_state.x() as u32;
            window.mouse_y = mouse_state.y() as u32;
            graphics::render(&mut canvas, &mut tex_man, &texture_creator, &font_big, &font_small, &mut song, &mut window, &runner)?;
            std::thread::sleep(Duration::from_secs_f32(constants::FRAME_TIME) - elapsed_frame_time / 2);
                        
            if runner.play {
                match song.tabs.notes.chars().nth(i) {
                    None => runner.play = false,
                    Some('1') => runner.x += shift,
                    Some('2') => runner.x += shift / 2,
                    Some('4') => runner.x += shift / 4,
                    Some('8') => runner.x += shift / 8,
                    Some('-') => runner.x += shift,
                    Some('|') => runner.x += 0,
                    _ => println!("Error: Wrong value {} at Song tabs!", song.tabs.notes.chars().nth(i).unwrap()),
                }             
                i += 1;
                
                if runner.x > window.width as i32 { 
                    runner.y += 175;
                    runner.x = 60;
                }
            }
        }
    }

    println!("...from depths of hell I raise.");
    return Ok(());
}
