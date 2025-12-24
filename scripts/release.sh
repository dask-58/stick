#!/bin/bash

set -e

VERSION="0.1.0"
TARGETS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
)

mkdir -p releases

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    
    cargo build --release --target "$target" 2>/dev/null || {
        echo "  Skipping $target (toolchain not installed)"
        continue
    }
    
    tar -czvf "releases/stick-$target.tar.gz" \
        -C "target/$target/release" stick
    
    echo "  SHA256: $(shasum -a 256 "releases/stick-$target.tar.gz" | cut -d' ' -f1)"
done

echo ""
echo "Done! Upload releases/*.tar.gz to GitHub Releases."
echo "Then update Formula/stick.rb with the SHA256 hashes above."
