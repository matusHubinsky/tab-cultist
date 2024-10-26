

// TODO: not tested yet
// TODO: not working YET

use std::fs;
use std::fs::File;
use std::io::{self, BufRead, };


fn string_to_bool(condition: &str) -> Result<bool, &'static str> {
    match condition {
        "true" | "t" => Ok(true),
        "false" | "f" => Ok(false),
        _ => Err("Unknown boolean type: {}",),
    }
}


pub struct Settings {
    pub name: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub font: String,
    pub location: String,
    pub font_small_size: u32,
    pub font_medium_size: u32,
    pub font_big_size: u32,
    pub lines: u32,
    pub note_size: u32,
}


impl Settings {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            title: String::new(),
            width: 0,
            height: 0,
            fullscreen: true,
            font: String::new(),
            location: String::new(),
            font_small_size: 0,
            font_medium_size: 0,
            font_big_size: 0,
            lines: 0,
            note_size: 0,
        }
    }

    pub fn load() -> io::Result<Settings> { 
        let file;

        // find the config.txt
        match fs::metadata("./config.txt") {
            Ok(_) => {
                file = File::open("./config.txt")?;
            }
            Err(_) => {
                file = File::open("/usr/share/tab-cultist/config.txt")?;
            }
        }

        let reader = io::BufReader::new(file);
        let mut settings = Settings::new();

        for line_res in reader.lines() {
            let line = line_res?;
            // split by space
            let words: Vec<&str> = line.split_whitespace().collect();

            if words.len() > 2 {
                match words[0] {
                    "name" => settings.name = String::from(words[2]),
                    "title" => settings.title = String::from(words[2]),
                    "width" => settings.width =  words[2].parse::<u32>().unwrap(),
                    "height" => settings.height =words[2].parse::<u32>().unwrap(),
                    "fullscreen" => settings.fullscreen = string_to_bool(words[2]).unwrap(),
                    "font" => settings.font = String::from(words[2]),
                    "location" => settings.location = String::from(words[2]),
                    "font_small_size" => settings.font_small_size =words[2].parse::<u32>().unwrap(),
                    "font_medium_size" => settings.font_medium_size =words[2].parse::<u32>().unwrap(),
                    "font_big_size" => settings.font_big_size =words[2].parse::<u32>().unwrap(),
                    "lines" => settings.lines =words[2].parse::<u32>().unwrap(),
                    "note_size" => settings.note_size =words[2].parse::<u32>().unwrap(),
                    "next_song" => continue,
                    "previous_song" => continue,
                    "toggle_fullscreen" => continue,
                    "close" => continue,
                    "render_scale_quality" => continue,
                    "logo" => continue,
                    "icon" => continue,
                    "database" => continue,
                    " " => continue,
                    _ => println!("Err: {}: {}", words[0], words[2]),
                }
            } 
        }

        return Ok(settings);
    }
}