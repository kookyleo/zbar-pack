/// Example: Scan for multiple barcode types simultaneously
///
/// Demonstrates configuring the scanner for multiple barcode formats.

use zbar_pack::{Image, ImageScanner, SymbolType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multi-Codec Scanner Example\n");

    // Create scanner
    let mut scanner = ImageScanner::new()?;

    // Enable multiple codecs
    let codecs = vec![
        (SymbolType::QRCODE, "QR Code"),
        (SymbolType::EAN13, "EAN-13"),
        (SymbolType::EAN8, "EAN-8"),
        (SymbolType::CODE128, "Code 128"),
        (SymbolType::CODE39, "Code 39"),
    ];

    println!("Enabling codecs:");
    for (codec, name) in &codecs {
        scanner.set_config(*codec, 0, 1)?;
        println!("  - {}", name);
    }

    // Create test image
    let width = 200u32;
    let height = 200u32;
    let data: Vec<u8> = vec![128; (width * height) as usize];

    let image = Image::from_gray(&data, width, height)?;

    // Scan
    println!("\nScanning image...");
    let symbols = scanner.scan_image(&image)?;

    let mut count = 0;
    for symbol in symbols {
        count += 1;
        println!("\nSymbol {}:", count);
        println!("  Type: {:?}", symbol.symbol_type());
        println!("  Data: {}", symbol.data());
        println!("  Quality: {}", symbol.quality());
    }

    if count == 0 {
        println!("\nNo symbols detected in test image.");
        println!("Scanner is ready to process real barcode images.");
    }

    // Print scanner configuration
    println!("\nScanner is configured for:");
    for (_codec, name) in &codecs {
        println!("  - {}", name);
    }

    Ok(())
}
