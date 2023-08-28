# TabCultist

TabCultist is an application written in Rust for displaying guitar or bass tabs. 

# Usage
- **next song**: up arrow
- **previous song**: down arrow

# Instalation
## Linux and MacOS
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
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh &&
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