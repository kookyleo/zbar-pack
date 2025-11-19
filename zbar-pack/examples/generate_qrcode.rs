/// Example: Generate a simple QR code pattern for testing
/// This creates a minimal valid QR code structure in memory
///
/// Note: This is a simplified example. For real QR code generation,
/// use a dedicated library like `qrcode` crate.

use zbar_pack::{Image, ImageScanner, SymbolType};

fn create_minimal_qrcode_pattern() -> (Vec<u8>, u32, u32) {
    // Create a simple 21x21 QR code pattern (Version 1)
    // This is a hand-crafted minimal QR code pattern
    let size = 21u32;
    let mut data = vec![255u8; (size * size) as usize]; // White background

    // Helper to set pixel (0 = black, 255 = white)
    let mut set_module = |x: u32, y: u32, black: bool| {
        if x < size && y < size {
            data[(y * size + x) as usize] = if black { 0 } else { 255 };
        }
    };

    // Position detection patterns (finder patterns)
    // Top-left
    for y in 0..7 {
        for x in 0..7 {
            let black = (x == 0 || x == 6 || y == 0 || y == 6) ||
                        (x >= 2 && x <= 4 && y >= 2 && y <= 4);
            set_module(x, y, black);
        }
    }

    // Top-right
    for y in 0..7 {
        for x in 14..21 {
            let black = (x == 14 || x == 20 || y == 0 || y == 6) ||
                        (x >= 16 && x <= 18 && y >= 2 && y <= 4);
            set_module(x, y, black);
        }
    }

    // Bottom-left
    for y in 14..21 {
        for x in 0..7 {
            let black = (x == 0 || x == 6 || y == 14 || y == 20) ||
                        (x >= 2 && x <= 4 && y >= 16 && y <= 18);
            set_module(x, y, black);
        }
    }

    // Timing patterns
    for i in 8..13 {
        set_module(i, 6, i % 2 == 0);
        set_module(6, i, i % 2 == 0);
    }

    // Dark module (always present in QR codes)
    set_module(8, 13, true);

    (data, size, size)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating test QR code pattern...");

    let (data, width, height) = create_minimal_qrcode_pattern();
    println!("Created {}x{} QR code pattern", width, height);

    // Create scanner
    let mut scanner = ImageScanner::new()?;
    scanner.set_config(SymbolType::QRCODE, 0, 1)?;

    // Create image
    let image = Image::from_gray(&data, width, height)?;

    // Scan
    println!("Scanning image...");
    let symbols = scanner.scan_image(&image)?;

    // Print results
    let mut count = 0;
    for symbol in symbols {
        count += 1;
        println!("Found symbol {}:", count);
        println!("  Type: {:?}", symbol.symbol_type());
        println!("  Data: {:?}", symbol.data());
        println!("  Quality: {}", symbol.quality());
    }

    if count == 0 {
        println!("No symbols detected.");
        println!("Note: This is a minimal test pattern and may not be");
        println!("recognized as a valid QR code by the scanner.");
    }

    Ok(())
}
