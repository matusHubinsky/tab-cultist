
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::Settings;


pub struct Window {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub mouse_x: u32,
    pub mouse_y: u32,
}


impl Window {
    pub fn new(settings: &Settings) -> Window {
        Window {
            width: settings.width,
            height: settings.height,
            fullscreen: settings.fullscreen,
            mouse_x: 0,
            mouse_y: 0,
        }
    }
}


pub struct Tabs {
    pub tones: [String; 6],
    pub notes: String,
}


impl Tabs {
    pub fn new() -> Self {
        Self {
            tones: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            notes: String::new(),
        }
    }
}


pub struct Song {
    pub artist: String,
    pub title: String,
    pub difficulty: String,
    pub bpm: u8,
    pub tempo: String,
    pub tuning: String,
    pub instrument: String,
    pub tabs: Tabs,
}


impl Song {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            artist: String::new(),
            difficulty: String::new(),
            bpm: 0,
            tempo: String::new(),
            tuning: String::new(),
            instrument: String::new(),
            tabs: Tabs::new(),
        }
    }

    pub fn load(file: String, song: &mut Song) -> Result<(), String> {
        let mut file = File::open(file).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
    
        // Parse the JSON contents
        let json_data = json::parse(&contents).expect("Failed to parse JSON");
    
        // Access the tab information
        song.title = json_data["title"].to_string().replace("\"", "");
        song.artist = json_data["artist"].to_string().replace("\"", "");
        song.difficulty = json_data["difficulty"].to_string().replace("\"", "");
        song.bpm = json_data["bmp"].as_u8().unwrap();
        song.tempo = json_data["tempo"].to_string().replace("\"", "");
        song.tuning = json_data["tuning"].to_string().replace("\"", "");
        song.instrument = json_data["instument"].to_string().replace("\"", "");
    
        // Iterate over the tabs
        let tabs = &json_data["tabs"];
        if tabs.is_array() {
            let mut i = 0;
            let mut tab = Tabs::new();
            for json_tab in tabs.members() {
                if i == tabs.len() - 1 {
                    tab.notes = json_tab.to_string().replace("\"", "");
                } else {
                    tab.tones[i] = json_tab.to_string().replace("\"", "");
                }
                i += 1;
            }
            song.tabs = tab;
        }

        return Ok(());
    }
    
    pub fn print(song: Song) -> Result<(), String> {
        println!("Title: {}", song.title);
        println!("Artist: {}", song.artist);
        println!("Difficulty: {}", song.difficulty);
        println!("BPM: {}", song.bpm);
        println!("Tempo: {}", song.tempo);
        println!("Tuning: {}", song.tuning);
        println!("Instrument: {}", song.instrument);
        println!("e: {}", song.tabs.tones[0]);
        println!("B: {}", song.tabs.tones[1]);
        println!("G: {}", song.tabs.tones[2]);
        println!("D: {}", song.tabs.tones[3]);
        println!("A: {}", song.tabs.tones[4]);
        println!("E: {}", song.tabs.tones[5]);
        return Ok(());
    }
}


pub struct Album {
    pub songs: Vec<Song>,
    pub index: usize,
}


impl Album {
    pub fn new(path: String) -> Self {
        Self {
            // songs: Album::load(path),
            songs: Vec::<Song>::new(),
            index: 0,
        }
    }

    pub fn load(path: String) -> Vec<Song> {
        let mut songs = Vec::new();

        let database = Path::new(path.as_str());

        if database.is_dir() {
            if let Ok(entries) = fs::read_dir(database) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            let mut song = Song::new();
                            Song::load(path.to_str().unwrap().to_string(), &mut song).unwrap();
                            songs.push(song);
                            println!("{}", path.display());
                        }
                    }
                }
            } else {
                eprintln!("Failed to read directory");
            }
        } else {
            eprintln!("Specified path is not a directory");
        }

        return songs;
    }

    pub fn print(album: Album) -> Result<(), String> {
        for i in 0..album.songs.len() {
            println!("Title: {}", album.songs[i].title);
            println!("Artist: {}", album.songs[i].artist);
            println!("Difficulty: {}", album.songs[i].difficulty);
            println!("BPM: {}", album.songs[i].bpm);
            println!("Tempo: {}", album.songs[i].tempo);
            println!("Tuning: {}", album.songs[i].tuning);
            println!("Instrument: {}", album.songs[i].instrument);
            println!("e: {}", album.songs[i].tabs.tones[0]);
            println!("B: {}", album.songs[i].tabs.tones[1]);
            println!("G: {}", album.songs[i].tabs.tones[2]);
            println!("D: {}", album.songs[i].tabs.tones[3]);
            println!("A: {}", album.songs[i].tabs.tones[4]);
            println!("E: {}", album.songs[i].tabs.tones[5]);
        }
        return Ok(());
    }      
}


pub struct Database {
    pub files: Vec<String>,
    pub index: usize,
}


impl Database {
    pub fn new(path: String) -> Self {
        Self {
            files: Self::load(path),
            index: 0,
        }
    }

    pub fn load(path: String) -> Vec<String> {
        let mut files = Vec::<String>::new();
        
        let database = Path::new(path.as_str());

        if database.is_dir() {
            if let Ok(entries) = fs::read_dir(database) {
                println!("Files:");
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            files.push(path.display().to_string());
                            println!("{}", path.display());
                        }
                    }
                }
            } else {
                println!("Failed to read directory");
            }
        } else {
            println!("Specified path is not a directory");
        }

        return files;
    }

    pub fn next(database: &mut Database) -> String { 
        database.index = (database.index + 1) % database.files.len(); 
        return database.files[database.index].clone();
    }

    pub fn prev(database: &mut Database) -> String { 
        if database.index == 0 {
            database.index = database.files.len();
        }
        database.index = (database.index - 1) % database.files.len(); 
        return database.files[database.index].clone();
    }
}


pub struct Runner {
    pub x: i32,
    pub y: i32,
    pub play: bool,
    pub show: bool,
}


impl Runner {
    pub fn new() -> Self {
        Self {
            x: 60,
            y: 80,
            play: false,
            show: false,
        }
    }


    pub fn play(mut self, song: &mut Song, window: &mut Window, i: usize) -> Self {
        let shift = 24;
    
        match song.tabs.notes.chars().nth(i) {
            None => self.play = false,
            Some('-') | Some('1') => self.x += shift,
            Some('2') => self.x += shift / 2,
            Some('4') => self.x += shift / 4,
            Some('8') => self.x += shift / 8,
            Some('9') => self.x += shift / 16,
            Some('|') => self.x += 0,
            _ => println!("Error: Wrong value {} at Song tabs!", song.tabs.notes.chars().nth(i).unwrap()),
        }

        if self.x > window.width as i32 { 
            self.y += 175;
            self.x = 60;
        }
        
        return self;
    }
}

