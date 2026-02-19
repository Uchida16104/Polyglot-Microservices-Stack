#!/usr/bin/env bash
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

echo "==> Installing system dependencies..."
apt-get update -qq 2>/dev/null || true
apt-get install -y --no-install-recommends \
    build-essential g++ wget curl xz-utils ca-certificates \
    2>/dev/null || true

echo "==> Installing .NET 8 SDK..."
wget -qO /tmp/dotnet-install.sh https://dot.net/v1/dotnet-install.sh
chmod +x /tmp/dotnet-install.sh
/tmp/dotnet-install.sh --channel 8.0 --install-dir /opt/render/project/dotnet 2>/dev/null || true
export PATH="/opt/render/project/dotnet:$PATH"

echo "==> Installing Zig 0.13.0..."
ZIG_VERSION="0.13.0"
ZIG_DIR="/opt/render/project/zig"
if [ ! -f "${ZIG_DIR}/zig" ]; then
    wget -qO /tmp/zig.tar.xz \
        "https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz"
    tar -xf /tmp/zig.tar.xz -C /tmp/
    mv "/tmp/zig-linux-x86_64-${ZIG_VERSION}" "${ZIG_DIR}"
    rm -f /tmp/zig.tar.xz
fi
export PATH="${ZIG_DIR}:$PATH"

echo "==> Compiling C++ shared library..."
g++ -O2 -shared -fPIC -o libcompute.so cpp/compute.cpp || echo "C++ compile warning (non-fatal)"

echo "==> Building C# project..."
/opt/render/project/dotnet/dotnet build csharp/Processor.csproj -c Release 2>/dev/null \
    || echo "dotnet build warning (non-fatal)"

echo "==> Building Rust release binary..."
cargo build --release

echo "==> Copying assets to release directory..."
mkdir -p target/release/assets
cp -r cpp zig mojo fstar dafny csharp target/release/assets/ 2>/dev/null || true
cp libcompute.so target/release/ 2>/dev/null || true

echo "==> Build complete."
