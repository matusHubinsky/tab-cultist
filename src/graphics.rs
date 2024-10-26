
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;

use sdl2::image::LoadTexture;

use crate::texture::TextureManager;

use crate::data::Song;
use crate::data::Runner;
use crate::data::Window;

use crate::Duration;

const BEGIN_X_SHIFT: i32 = 64;
const BEGIN_Y_SHIFT: i32 = 32;
const X_SHIFT: i32 = 15;
const LINES_SPACING: i32 = 17;

// TODO: rendering 24 as a single number should solve issues with bad spacing
// TODO: don't re-render useless stuff
// TODO: split rendering to functions

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


pub fn render_lines(song: &mut Song, canvas: &mut WindowCanvas, window: &mut Window) -> Result<(), String> {
    let mut shift = 80 + BEGIN_Y_SHIFT;
    let line_number = 6; 
    let line_lenght = (song.tabs.notes.len() / 64) + 2;

    canvas.set_draw_color(Color::RGB(75, 75, 75));

    for _ in 1..line_lenght {
        for j in 0..line_number {
            canvas.draw_line((BEGIN_X_SHIFT, shift + j*LINES_SPACING), (window.width as i32 - BEGIN_X_SHIFT, shift + j*LINES_SPACING))?;
        }    
        shift += 175;
    }
    
    return Ok(());
}


pub fn render_logo(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font_big: &sdl2::ttf::Font, window: &mut Window) -> Result<(), String>  {
    let logo = texture_creator
        .load_texture("linux/logo_screen.png")
        .map_err(|e| e.to_string())?;    

    let text_surface = font_big
        .render("Atush Software")
        .blended(Color::RGB(0xff, 0xff, 0xff))  // Text color
        .map_err(|e| e.to_string())?;
    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string())?;

    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let x: i32 = (window.width/2-256-64).try_into().unwrap();
    let y: i32 = (window.height/6).try_into().unwrap();
    let w: u32 = ("Atush Software".len() * 48).try_into().unwrap();
    
    for i in (0..=0xff).step_by(4) {
        canvas.clear();
        canvas.copy(&logo, None, Rect::new(0, 0, window.width, window.height))?; 
        canvas.copy(&text_texture, None, Rect::new(x, y, w, 104))?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255 - i));
        canvas.fill_rect(Rect::new(0, 0, window.width, window.height))?;
        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    for i in (0..=0xff).step_by(4) {
        canvas.clear();
        canvas.copy(&logo, None, Rect::new(0, 0, window.width, window.height))?; 
        canvas.copy(&text_texture, None, Rect::new(x, y, w, 104))?;
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0 + i));
        canvas.fill_rect(Rect::new(0, 0, window.width, window.height))?;
        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    canvas.clear();
    return Ok(());
}


pub fn render(canvas: &mut WindowCanvas, _tex_man: &mut TextureManager<WindowContext>, texture_creator: &TextureCreator<WindowContext>, font_big: &sdl2::ttf::Font, font_small: &sdl2::ttf::Font, song: &mut Song, window: &mut Window, runner: &Runner) -> Result<(), String> {

    // Clear the canvas
    canvas.clear();
    
    // Draw a rectangle on the canvas
    canvas.set_draw_color(Color::RGB(5, 5, 5));
    canvas.fill_rect(Rect::new(0, 0, window.width, window.height))?;

    // Render lines
    render_lines(song, canvas, window)?;

    // Artist and song Title
    let name = format!("{} - {}", song.artist, song.title);
    let width = (name.len()*24).try_into().unwrap();
    render_text(canvas, &texture_creator, &font_big, name.clone(), window.width as i32 / 2 - ((width/2) as i32), 32, width, 48)?;

    let mut letter = String::new();

    // Render Textures for small letters
    let mut textures_chars: Vec<sdl2::render::Texture> = Vec::new();
    for i in 0..=122 {
        if ((i >= 'a' as usize) && (i <= 'z' as usize)) || ((i >= 'A' as usize) && (i <= 'Z' as usize))  {
            letter = std::char::from_u32(i as u32).unwrap().to_string();
        } else {
            letter = std::char::from_u32('a' as u32).unwrap().to_string();
        }

        let surface = font_small
            .render(&letter)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;    
        textures_chars.push(texture);
    }

    // Render Textures for note numbers
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
    let mut skip: u32 = 0;
    let default_x = 53 + BEGIN_X_SHIFT;
    let default_y = 70 + BEGIN_Y_SHIFT;
    let mut note_x = 63 + BEGIN_X_SHIFT;
    let mut note_y = 70 + BEGIN_Y_SHIFT; 

    for line in song.tabs.tones.iter() {
        let mut j = 0;
        let mut dont = false;
        for c in line.chars() {

            // when notes reach end of the screen, move to them to the next "line"
            if note_x + BEGIN_X_SHIFT > window.width as i32 {
                // render bar line at the end
                if i == 0 {
                    let x = window.width as i32 - BEGIN_X_SHIFT;
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 75, 75));
                    canvas.draw_line(Point::new(x, note_y + 10), Point::new(x, note_y+10+(5*LINES_SPACING)))?;
                }
                note_x = BEGIN_X_SHIFT;
                note_y += 175;
                // render bar line at the begging
                if i == 0 {
                    let x = BEGIN_X_SHIFT;
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 75, 75));
                    canvas.draw_line(Point::new(x, note_y + 10), Point::new(x, note_y+10+(5*LINES_SPACING)))?;
                }
            } 

            // rendering tabs
            if let Some(number) = c.to_digit(10) {
                let mut k: usize = number.try_into().unwrap();

                if !dont || skip > 0 {
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                    canvas.fill_rect(Rect::new(note_x - 4, note_y, 16, 16))?;  
                    dont = false;
                }
                
                if k > 10 {
                    canvas.copy(&textures_notes[k], None, Rect::new(note_x, note_y, 16, 20))?; 
                    println!("Double: {:?}", k);
                } else {
                    canvas.copy(&textures_notes[k], None, Rect::new(note_x, note_y, 10, 20))?; 
                }
            
                if j < line.len() - 1 {
                    if let Some(_number) = line.chars().nth(j + 1).and_then(|c| c.to_digit(10)) {
                        dont = true;
                    } 
                }
            } 

            // rendering bar lines
            if c == '|' && i == 0 {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 75, 75));
                canvas.draw_line(Point::new(note_x, note_y + 10), Point::new(note_x, note_y+8+(5*LINES_SPACING)))?;
            }

            // always draw bar line for the first row
            if i == 0 && i == 0 {
                let x = window.width as i32 - BEGIN_X_SHIFT;
                canvas.set_draw_color(sdl2::pixels::Color::RGB(75, 75, 75));
                canvas.draw_line(Point::new(x, note_y + 10), Point::new(x, note_y+10+(5*LINES_SPACING)))?;
            }

            note_x += X_SHIFT;
            j += 1;
            
            if skip > 0 {
                skip -= 1;
            }
        }

        note_x = default_x;
        note_y = default_y + (LINES_SPACING * (i + 1));
        i += 1;
    }

    // Render strings 
    let mut i = 0;
    for c in song.tuning.chars() {
        let ordinal = c as usize;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(BEGIN_X_SHIFT+3, BEGIN_Y_SHIFT+73 + i*LINES_SPACING, 12, 8))?;  
        canvas.copy(&textures_chars[ordinal], None, Some(Rect::new(BEGIN_X_SHIFT+5, BEGIN_Y_SHIFT+70 + i*LINES_SPACING, 10, 20)))?;
        i += 1;
    }

    // draw tempo notes
    canvas.copy(&textures_tempo[first], None, Some(Rect::new(BEGIN_X_SHIFT+30, 80+BEGIN_Y_SHIFT, 25, 50)))?;  
    canvas.copy(&textures_tempo[second], None, Some(Rect::new(BEGIN_X_SHIFT+30,115+BEGIN_Y_SHIFT, 25, 50)))?;  

    // render runner
    if runner.show {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 0, 0, 120));
        let _ = canvas.fill_rect(Rect::new(runner.x, runner.y - 4, 4, 96));
    }

    // Update the canvas    
    canvas.present();

    // just shut up rust
    let _c = letter;

    return Ok(());
}