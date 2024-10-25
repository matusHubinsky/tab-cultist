#!/bin/sh

cat logo.txt

echo "From the dephts of hell I raise"
echo "Installing tab-cultist"

cat logo.txt

echo "Checking root priviliges ..."
if [ $EUID != 0 ]; then
    sudo bash "$0" "$@"
    exit $?
else
    echo "I am root"
fi

echo "Moving tab-cultist binary to /usr/bin"
cp ./tab-cultist /usr/bin/

echo "Creating directory 'tab-cultist'"
mkdir /usr/share/tab-cultist
mkdir /usr/share/tab-cultist/font
mkdir /usr/share/tab-cultist/font/Roboto_Mono

echo "Moving tab-cultist database and modules to /usr/share/tab-cultist"
cp linux/./icon.png /usr/share/tab-cultist/
cp linux/./tab-cultist.desktop /usr/share/tab-cultist/
cp ./config.txt  /usr/share/tab-cultist/
cp font/ -r /usr/share/tab-cultist/font/Roboto_Mono/
cp database/ -r /usr/share/tab-cultist/
cp ./logo.txt /usr/share/tab-cultist/

cp config.txt /usr/share/tab-cultist/config.txt

echo "Done. Have fun."