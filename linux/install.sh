#!/bin/sh

cat logo.txt

echo "From the dephts of hell I raise"
echo "Installing tab-cultist"

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

echo "Moving tab-cultist database and modules to /usr/share/tab-cultist"
cp linux/./icon.png /usr/share/tab-cultist/
cp linux/./logo_screen.png /usr/share/tab-cultist/
cp linux/./tab-cultist.desktop /usr/share/tab-cultist/
cp ./config.txt  /usr/share/tab-cultist/
cp font/ -r /usr/share/tab-cultist/
cp database/ -r /usr/share/tab-cultist/
cp ./logo.txt /usr/share/tab-cultist/

cp config.txt /usr/share/tab-cultist/config.txt

echo "Installation is complete. Have fun."