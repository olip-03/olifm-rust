#!/bin/bash

set -e

WEB_CONTENT_DIR="./web/content"
CONTENT_DIR="./content"
WEB_DIR="./web"

# Sanitize /web/content if it exists
if [ -d "$WEB_CONTENT_DIR" ]; then
    rm -rf "${WEB_CONTENT_DIR:?}/"*
else
    mkdir -p "$WEB_CONTENT_DIR"
fi

# Copy /content folder to /web
cp -r "$CONTENT_DIR/." "$WEB_CONTENT_DIR/"

# Build wasm-pack in /web directory
echo "Building WASM"
(
    cd "$WEB_DIR"
    wasm-pack build --release --target web
)
echo ""

# Build helper project
echo "Building helper project"
cargo build --release

# Check if olifm-helper exists and remove it
if [ -e ./olifm-helper ]; then
    rm ./olifm-helper
fi

# Move the binary
mv ./target/release/olifm-master ./olifm-helper

# Build site map
echo "Building site map"
./olifm-helper --content ./web/content --out ./web
