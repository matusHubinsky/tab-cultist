
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
pub mod settings;

use crate::data::Song;
use crate::data::Album;
use crate::data::Window;
use crate::data::Database;
use crate::data::Runner;
use crate::settings::Settings;

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
    Song::load(Database::next(&mut database), &mut song)?;
    
    // Load settings
    let settings = Settings::load().unwrap();

    let mut window = Window::new(&settings);

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initialize Font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    let font_path: &Path = Path::new(&settings.location);
    let font_big = ttf_context.load_font(font_path, settings.font_big_size as u16)?;
    // let font_medium = ttf_context.load_font(font_path, 32)?;
    let font_small = ttf_context.load_font(font_path, settings.font_big_size as u16)?;

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

    let mut runner = Runner::new();
    let mut i = 0;

    // Main event loop
    let mut event_pump = sdl_context.event_pump()?;
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
                    window.fullscreen = !window.fullscreen;
                    // Set fullscreen mode
                    if window.fullscreen {
                        canvas.window_mut().set_fullscreen(FullscreenType::Desktop)?;
                    } else {
                        canvas.window_mut().set_fullscreen(FullscreenType::Off)?;
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
            graphics::render(&mut canvas, &mut tex_man, &texture_creator, &font_big, &font_small, &mut song, &mut window, &runner)?;
            std::thread::sleep(Duration::from_secs_f32(constants::FRAME_TIME) - elapsed_frame_time / 2);
        
            window.mouse_x = event_pump.mouse_state().x() as u32;
            window.mouse_y = event_pump.mouse_state().y() as u32;

            runner = Runner::play(runner, &mut song, &mut window, i); 
            i += 1;
        }
    }

    println!("...from depths of hell I raise.");
    return Ok(());
}
