#!/bin/sh

cargo build --release

zip -9 -j tab-cultist-linux.zip linux/* ../target/release/./tab-cultist
zip -9 tab-cultist-linux.zip database/* ./config.txt ./logo.txt ./font/Roboto_Mono/RobotoMono-VariableFont_wght.ttf

