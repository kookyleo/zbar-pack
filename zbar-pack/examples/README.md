# Examples

This directory contains example programs demonstrating various features of zbar-pack.

## Running Examples

```bash
# Basic usage
cargo run --example simple

# QR code pattern generation
cargo run --example generate_qrcode

# Batch processing
cargo run --example batch_processing

# Multiple codec types
cargo run --example multi_codec
```

## Examples Overview

### simple.rs
Basic introduction to the library. Shows:
- Creating a scanner
- Configuring for QR codes
- Creating an image
- Scanning and iterating results

**Run time**: < 1 second

### generate_qrcode.rs
Demonstrates a minimal QR code pattern generation for testing.

**Note**: For production QR code generation, use a dedicated library like the `qrcode` crate. This example is for testing the scanner.

**Run time**: < 1 second

### batch_processing.rs
Shows best practices for processing multiple images efficiently:
- Scanner reuse
- Performance measurement
- Error handling
- Progress reporting

Processes 100 test images and reports performance metrics.

**Run time**: 1-2 seconds

### multi_codec.rs
Demonstrates multi-format scanning:
- Configuring multiple barcode types
- QR Code, EAN-13, EAN-8, Code 128, Code 39

**Run time**: < 1 second

## Working with Real Images

To scan real barcode images, you'll need to:

1. **Load image data**: Use an image library like `image`:
   ```toml
   [dependencies]
   image = "0.24"
   ```

2. **Convert to grayscale**:
   ```rust
   use image::io::Reader as ImageReader;

   let img = ImageReader::open("barcode.png")?.decode()?;
   let gray = img.to_luma8();
   let (width, height) = gray.dimensions();
   let data = gray.into_raw();

   let image = Image::from_gray(&data, width, height)?;
   ```

3. **Scan the image**:
   ```rust
   let symbols = scanner.scan_image(&image)?;
   ```

## Example: Complete Image Scanner

```rust
use zbar_pack::{ImageScanner, Image, SymbolType};
use image::io::Reader as ImageReader;

fn scan_image_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let img = ImageReader::open(path)?.decode()?;
    let gray = img.to_luma8();
    let (width, height) = gray.dimensions();

    // Create scanner
    let mut scanner = ImageScanner::new()?;
    scanner.set_config(SymbolType::QRCODE, 0, 1)?;

    // Scan
    let image = Image::from_gray(gray.as_raw(), width, height)?;
    for symbol in scanner.scan_image(&image)? {
        println!("Found: {}", symbol.data());
    }

    Ok(())
}
```

## Performance Tips

1. **Reuse scanner**: Create once, scan many images
2. **Use appropriate resolution**: Downsample large images
3. **Convert to grayscale**: Scanner only uses grayscale data
4. **Enable only needed codecs**: Use feature flags or runtime config

## Feature Flags

Build with specific codecs only:

```bash
# Only QR codes
cargo run --example simple --no-default-features --features vendored,codec-qrcode

# QR codes and EAN
cargo run --example multi_codec --no-default-features --features vendored,codec-qrcode,codec-ean
```

## See Also

- [Main README](../../../README.md) - Project overview
- [BENCHMARKING.md](../../../BENCHMARKING.md) - Performance testing
- [CROSS_COMPILE.md](../../../CROSS_COMPILE.md) - Cross-compilation
