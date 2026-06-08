#!/bin/bash

TARGETS=(
    "aarch64-apple-darwin"
    "x86_64-apple-darwin"
    "aarch64-apple-ios"
    "aarch64-unknown-linux-gnu"
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
)

SUCCESS=()
FAILED=()

for TARGET in "${TARGETS[@]}"; do
    echo ""
    echo "Building $TARGET..."

    cargo build --target "$TARGET" --release

    if [ $? -eq 0 ]; then
        SUCCESS+=("$TARGET")
    else
        FAILED+=("$TARGET")
    fi
done

echo "Succeeded (${#SUCCESS[@]}):"
for T in "${SUCCESS[@]}"; do echo "  + $T"; done

echo ""
echo "Failed (${#FAILED[@]}):"
for T in "${FAILED[@]}"; do echo "  - $T"; done