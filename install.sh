#!/bin/sh

cat logo.txt

echo "From the dephts of hell I raise"
echo "Installing tab-cultist"

echo "Installing rust..."
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

echo "Compiling tab-cultist..."
cargo build --release

echo "Running tab-cultist..."
cargo run --release

echo "Moving tab-cultist binary to /usr/bin"
mv target/release/./tab-cultist /usr/bin/

echo "Done. Have fun."

