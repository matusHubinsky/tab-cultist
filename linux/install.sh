#!/bin/sh

cat logo.txt

echo "From the dephts of hell I raise"
echo "Installing tab-cultist"

echo "Moving tab-cultist binary to /usr/bin"
cp ./tab-cultist /usr/bin/

mkdir /usr/share/tab-cultist
mkdir /usr/share/tab-cultist/font
mkdir /usr/share/tab-cultist/font/Roboto_Mono

cp ./icon.png /usr/share/tab-cultist/
cp ./tab-cultist.desktop /usr/share/tab-cultist/
cp ./config.txt  /usr/share/tab-cultist/
cp font/ -r /usr/share/tab-cultist/font/Roboto_Mono/
cp database/ -r /usr/share/tab-cultist/
cp ./logo.txt /usr/share/tab-cultist/

echo "Done. Have fun."