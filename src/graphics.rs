
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;

use crate::texture::TextureManager;

use crate::data::Song;
use crate::data::Runner;
use crate::data::Window;


fn render_text(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &sdl2::ttf::Font, s: String, x: i32, y: i32, w: u32, h: u32) -> Result<(), String> {
    let surface = font
                .render(&s)
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string())?;
    let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

    canvas.copy(&texture, None, Rect::new(x, y, w, h))?;  
    return Ok(());
}


pub fn render_lines(canvas: &mut WindowCanvas, window: &mut Window, between_lines: i32) -> Result<(), String> {
    let mut shift = 80;
    let line_number = 6; 
    canvas.set_draw_color(Color::RGB(75, 75, 75));

    for _ in 1..8 {
        for j in 0..line_number {
            canvas.draw_line((0, shift + j*between_lines), (window.width as i32, shift + j*between_lines))?;
        }    
        shift += 175;
    }
    
    return Ok(());
}


pub fn render(canvas: &mut WindowCanvas, tex_man: &mut TextureManager<WindowContext>, texture_creator: &TextureCreator<WindowContext>, font_big: &sdl2::ttf::Font, font_small: &sdl2::ttf::Font, song: &mut Song, window: &mut Window, runner: &Runner) -> Result<(), String> {
    let between_lines = 17;

    // Clear the canvas
    canvas.clear();
    
    // Draw a rectangle on the canvas
    canvas.set_draw_color(Color::RGB(5, 5, 5));
    canvas.fill_rect(Rect::new(0, 0, window.width, window.height))?;

    // Render lines
    render_lines(canvas, window, between_lines)?;

    // Artist and song Title
    let name = format!("{} - {}", song.artist, song.title);
    render_text(canvas, &texture_creator, &font_big, name, window.width as i32 / 2 - 200, 5, 400, 48)?;

    // Render Textures for small letters
    let mut textures_chars: Vec<sdl2::render::Texture> = Vec::new();
    for i in 0..=122 {
        let mut char = String::new();
        if ((i >= 'a' as usize) && (i <= 'z' as usize)) || ((i >= 'A' as usize) && (i <= 'Z' as usize))  {
            char = std::char::from_u32(i as u32).unwrap().to_string();
        } else {
            char = std::char::from_u32('a' as u32).unwrap().to_string();
        }

        let surface = font_small
                    .render(&char)
                    .blended(Color::RGBA(255, 255, 255, 255))
                    .map_err(|e| e.to_string())?;
        let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;    
            textures_chars.push(texture);
    }

    // Render Textures for note numbes
    let mut textures_notes: Vec<sdl2::render::Texture> = Vec::new();
    for i in 0..=24 {
        let surface = font_small
            .render(i.to_string().as_str())
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;    
        textures_notes.push(texture);
    }

    // Render Textures for tempo numbers
    let mut textures_tempo: Vec<sdl2::render::Texture> = Vec::new();
    for i in 0..=16 {
        let surface = font_big
            .render(i.to_string().as_str())
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;    
        textures_tempo.push(texture);
    }


    // Render tempo numbers
    let tempo: Vec<&str> = song.tempo.split("/").collect();
    let mut first = 0;
    let mut second = 0;

    if tempo.len() == 2 {
        if let Ok(parsed_first) = tempo[0].parse::<usize>() {
            first = parsed_first;
        }
        if let Ok(parsed_second) = tempo[1].parse::<usize>() {
            second = parsed_second;
        }
    }

    let mut i = 0;
    let shift_x = 10;
    let default_x = 63;
    let default_y = 70;
    let mut note_x = 63;
    let mut note_y = 70; 

    for line in song.tabs.tones.iter() {
        let mut j = 0;
        let mut dont = false;
        for c in line.chars() {

            // when notes reach end of the screen, move to them to the next "line"
            if note_x + 50 > window.width as i32 {
                note_x = 0;
                note_y += 175;
            } 

            if let Some(number) = c.to_digit(10) {
                let k: usize = number.try_into().unwrap();
                if !dont {
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                    canvas.fill_rect(Rect::new(note_x - 4, note_y, 16, 16))?;  
                    dont = false;
                }
                
                canvas.copy(&textures_notes[k], None, Rect::new(note_x, note_y, 10,20))?; 
            
                if j < line.len() - 1 {
                    if let Some(_number) = line.chars().nth(j + 1).and_then(|c| c.to_digit(10)) {
                        dont = true;
                    } 
                }
            }

            if c == '|' && i == 0 {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 75, 75));
                canvas.draw_line(Point::new(note_x, note_y + 10), Point::new(note_x, note_y+8+(5*between_lines)))?;
            }

            note_x += shift_x;
            j += 1;
        }
        note_x = default_x;
        note_y = default_y + (between_lines * (i + 1));
        i += 1;
    }

    // Render strings 
    let mut i = 0;
    for c in song.tuning.chars() {
        let ordinal = c as usize;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(3, 73 + i*between_lines, 12, 8))?;  
        canvas.copy(&textures_chars[ordinal], None, Some(Rect::new(5, 70 + i*between_lines, 10, 20)))?;
        i += 1;
    }

    canvas.copy(&textures_tempo[first], None, Some(Rect::new(30, 80, 25, 50)))?;  
    canvas.copy(&textures_tempo[second], None, Some(Rect::new(30,115, 25, 50)))?;  

    // Update the canvas    
    canvas.present();

    return Ok(());
}