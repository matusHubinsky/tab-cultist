# tab-cultist
TabCultist is an application written in Rust for displaying guitar or bass tabs. 

# Usage
- **next song**: up arrow
- **previous song**: down arrow

# Instalation
## Linux and MacOS
1. Download `tab-cultist-linux.zip`
2. Unzip 
3. Run `sudo bash install.sh`

# Development
## Linux and MacOS
### Dependencies
Install sdl2 library:

Arch/Manjaro linux
```
sudo pacman -S sdl2 sdl2_ttf sdl2_image
```

Ubuntu/Debian/Kali
```
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev
```

### Instalation unsing **install.sh**:
```
bash install.sh
```
or
```
chmod +x install.sh
./install.sh
```

### Manual installation:
```
git clone https://github.com/matusHubinsky/tab-cultist
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
cargo build --release
cargo run --release
mv target/release/./tab-cultist /usr/bin/
```

# Examples

# Settings
The settings can be changed by editing the `config.txt`
Default settings:
```
[general]
name = tab-cultist
version = 0.1.0

[window]
title = tab-cultist
width = 1920
height = 1080
fullscreen = false

[theme]
font = Roboto_Mono
font_location = ./font/Roboto_Mono/RobotoMono-VariableFont_wght.ttf"
font_small_size = 12
font_medium_size = 32
font_big_size = 128

[song]
lines = 8
note_size = 2
```

# TODO
- automatic calculating lines and displaying tabs
- tab pages

# Done
- basic tabs displaying
- windows scaling
- demo version with runner
- full screen option

# Sources
- logo: https://emojicombos.com/satan
- font: Roboto Mono Variable Font