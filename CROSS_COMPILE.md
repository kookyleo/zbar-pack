# Cross-Compilation Guide

This document describes how to cross-compile zbar-pack for different platforms.

## Prerequisites

### Using cross-rs (Recommended)

```bash
cargo install cross
```

### Manual setup

Install target toolchains:

```bash
# Linux musl targets
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# ARM targets
rustup target add armv7-unknown-linux-musleabihf
rustup target add aarch64-unknown-linux-gnu

# Windows
rustup target add x86_64-pc-windows-gnu

# macOS (on macOS only)
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

## Cross-compilation Examples

### Using cross-rs

```bash
# Linux musl (fully static)
cross build --target x86_64-unknown-linux-musl --release

# ARM64 Linux
cross build --target aarch64-unknown-linux-gnu --release

# ARMv7 Linux
cross build --target armv7-unknown-linux-musleabihf --release
```

### Manual cross-compilation

#### Linux x86_64 musl

```bash
# Install musl toolchain
sudo apt-get install musl-tools

# Build
cargo build --target x86_64-unknown-linux-musl --release
```

#### ARM64 Linux

```bash
# Install cross-compiler
sudo apt-get install gcc-aarch64-linux-gnu

# Build
cargo build --target aarch64-unknown-linux-gnu --release
```

#### Windows from Linux

```bash
# Install MinGW
sudo apt-get install mingw-w64

# Build
cargo build --target x86_64-pc-windows-gnu --release
```

#### macOS universal binary (on macOS)

```bash
# Build for both architectures
cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release

# Create universal binary
lipo -create \
  target/x86_64-apple-darwin/release/libzbar_pack.dylib \
  target/aarch64-apple-darwin/release/libzbar_pack.dylib \
  -output target/universal/libzbar_pack.dylib
```

## Platform-Specific Notes

### musl targets

musl builds are fully static and do not depend on glibc. This makes them ideal for:
- Containers (Alpine Linux)
- Embedded systems
- Portable binaries

### ARM targets

When building for ARM:
- Use hard-float variants (`musleabihf`) for better performance
- Test on actual hardware or QEMU
- Consider enabling ARM-specific optimizations

### Windows

Windows builds support both MSVC and GNU toolchains:
- **MSVC**: Better Windows integration, requires Visual Studio
- **GNU**: MinGW-based, easier cross-compilation from Linux

### macOS

macOS builds require:
- Xcode Command Line Tools (on macOS)
- Proper SDK version targeting via `MACOSX_DEPLOYMENT_TARGET`

## Verification

Check binary properties:

```bash
# Check if binary is static (Linux)
ldd target/x86_64-unknown-linux-musl/release/libzbar_pack.so
# Should output: "not a dynamic executable"

# Check architecture
file target/aarch64-unknown-linux-gnu/release/libzbar_pack.so

# Check symbols
nm target/x86_64-unknown-linux-musl/release/libzbar_pack.so | grep zbar
```

## Troubleshooting

### Bindgen fails

Ensure libclang is available for the target platform:

```bash
export LIBCLANG_PATH=/usr/lib/llvm-14/lib
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/aarch64-linux-gnu"
```

### Linker errors

Check that cross-compilation toolchain is properly installed:

```bash
# Verify linker exists
which aarch64-linux-gnu-gcc

# Check .cargo/config.toml for correct linker path
```

### Missing system libraries

For system mode (`--features system`), ensure target platform has ZBar installed.

## CI/CD Integration

See `.github/workflows/release.yml` for automated cross-compilation setup.
