#!/bin/bash

TARGET=aarch64-apple-darwin
#TARGET=x86_64-apple-darwin

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cd ${DIR}/..

cargo build --target ${TARGET}

mkdir target/macos &> /dev/null
mkdir target/macos/Lightswitch.app/  &> /dev/null
mkdir target/macos/Lightswitch.app/Contents &> /dev/null
mkdir target/macos/Lightswitch.app/Contents/Resources &> /dev/null
mkdir target/macos/Lightswitch.app/Contents/MacOS &> /dev/null

cp ${DIR}/../target/${TARGET}/debug/lightswitch ${DIR}/../target/macos/Lightswitch.app/Contents/MacOS/
cp ${DIR}/Info.plist ${DIR}/../target/macos/Lightswitch.app/Contents/
cp ${DIR}/AppIcon.icns ${DIR}/../target/macos/Lightswitch.app/Contents/Resources/

hdiutil create ${DIR}/../target/macos/Lightswitch.dmg \
    -volname "Lightswitch keyboard switcher" \
    -srcfolder ${DIR}/../target/macos/Lightswitch.app \
    -ov