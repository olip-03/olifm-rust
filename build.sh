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

# build site map
echo "[" > directory_structure.json
find ./web/content -type f -o -type d | while read -r path; do
    if [ -d "$path" ]; then
        type="directory"
    else
        type="file"
    fi
    size=$(stat -c%s "$path" 2>/dev/null || echo 0)
    echo "  {\"path\":\"$path\",\"type\":\"$type\",\"size\":$size}," >> directory_structure.json
done
sed -i '$ s/,$//' directory_structure.json  # Remove last comma
echo "]" >> directory_structure.json
