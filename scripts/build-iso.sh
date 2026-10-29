#!/bin/bash
set -e

ARCH=$1
if [ -z "$ARCH" ]; then
    echo "Usage: ./build-iso.sh <x86_64|arm64>"
    exit 1
fi

echo "Building SMTH OS for $ARCH..."

rm -rf out/

lb config --architecture=$ARCH --packages=build/config/auto/packages
lb build --architecture=$ARCH

echo "Build complete. Output in out/"
ls -lh out/
