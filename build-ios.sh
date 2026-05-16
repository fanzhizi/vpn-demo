#!/bin/bash
# Build the Rust tunnel-core library for iOS targets.
# Prerequisites:
#   rustup target add aarch64-apple-ios
#   rustup target add aarch64-apple-ios-sim  (for simulator)

set -e

echo "=== Building tunnel-core for iOS (arm64) ==="
cargo build --package tunnel-core --release --target aarch64-apple-ios

echo "=== Building tunnel-core for iOS Simulator (arm64) ==="
cargo build --package tunnel-core --release --target aarch64-apple-ios-sim

echo ""
echo "=== Build complete ==="
echo "Static libraries:"
echo "  Device:    target/aarch64-apple-ios/release/libtunnel_core.a"
echo "  Simulator: target/aarch64-apple-ios-sim/release/libtunnel_core.a"
echo ""
echo "Header file: tunnel-core/include/tunnel_core.h"
echo ""
echo "Next steps:"
echo "  1. Open the Xcode project"
echo "  2. Add libtunnel_core.a to the PacketTunnel extension target"
echo "  3. Add tunnel-core/include to Header Search Paths"
echo "  4. Set the Bridging Header to PacketTunnel/BridgingHeader.h"
