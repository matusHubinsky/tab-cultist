# tab-cultist
TabCultist is an application written in Rust for displaying guitar or bass tabs. 

# Usage
- **next song**: up arrow
- **previous song**: down arrow

# Instalation

## Linux and MacOS

## Dependencies
Install sdl2 library:

Arch/Manjaro linux
```
sudo pacman -S sdl2 sdl2_ttf sdl2_image
```

Ubuntu/Debian/Kali
```
sudo apt install sdl2 sdl2_ttf sdl2_image
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