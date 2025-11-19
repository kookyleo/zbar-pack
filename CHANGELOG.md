# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-19

### Added
- Initial release of zbar-pack
- Static linking of ZBar 0.23.93 C library
- Safe Rust API wrapper
- Support for vendored (static) and system (dynamic) linking modes
- Feature flags for codec selection
- musl build support for fully static binaries
- Cross-platform support: Linux (glibc/musl), macOS, Windows
- Comprehensive documentation and examples
- LGPL 2.1+ compliance documentation
- CI/CD workflows for testing and releases
- Cross-compilation support for ARM and other architectures
- Performance benchmarks and memory tests

### Features
- `vendored` (default): Compile and statically link bundled ZBar source
- `system`: Use system-installed ZBar library
- `dynamic`: Dynamically link system ZBar library
- `minimal`: Minimal build for custom codec selection
- `all-codecs` (default): Enable all barcode/QR code decoders
- Individual codec features: `codec-qrcode`, `codec-ean`, `codec-code128`, etc.

### Documentation
- README with quick start guide
- COMPLIANCE.md for LGPL licensing details
- CROSS_COMPILE.md for cross-compilation instructions
- BENCHMARKING.md for performance testing guide

### Examples
- simple.rs: Basic scanner usage example

### Known Limitations
- Video input not supported (image-only)
- GUI features not available
- Requires libclang for bindgen during build

[Unreleased]: https://github.com/kookyleo/zbar-pack/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kookyleo/zbar-pack/releases/tag/v0.1.0
