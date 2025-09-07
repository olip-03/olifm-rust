#!/bin/bash

set -e

WEB_CONTENT_DIR="./web/content"
CONTENT_DIR="./content"
WEB_DIR="./web"

# Sanitize /web/constants if it exists
if [ -d "$WEB_CONTENT_DIR" ]; then
    rm -rf "${WEB_CONTENT_DIR:?}/"*
else
    mkdir -p "$WEB_CONTENT_DIR"
fi

# Copy /content folder to /web/constants
cp -r "$CONTENT_DIR/." "$WEB_CONTENT_DIR/"

# Build wasm-pack in /web directory
(
    cd "$WEB_DIR"
    wasm-pack build --release --target web
)
