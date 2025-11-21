#!/bin/bash
set -e

cd "$(dirname "$0")"

echo "Building WASM..."
cargo build -p ac-pcap-web --release --target wasm32-unknown-unknown

echo "Generating JS bindings..."
wasm-bindgen \
    --target web \
    --out-dir pkg \
    --no-typescript \
    ../../target/wasm32-unknown-unknown/release/ac_pcap_web.wasm

echo "Copying index.html..."
cp index.html pkg/

echo ""
echo "Build complete! Files in crates/web/pkg/"
echo ""
echo "To test locally:"
echo "  cd crates/web/pkg && python3 -m http.server 8080"
echo "  Then open http://localhost:8080"
