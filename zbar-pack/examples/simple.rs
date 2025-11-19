use zbar_pack::{Image, ImageScanner, SymbolType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ZBar Pack Example");
    
    // Print version
    let (major, minor) = zbar_pack::version();
    println!("ZBar version: {}.{}", major, minor);
    
    // Create scanner
    let mut scanner = ImageScanner::new()?;
    println!("Scanner created successfully");
    
    // Configure for QR codes
    scanner.set_config(SymbolType::QRCODE, 0, 1)?;
    println!("Scanner configured for QR code detection");
    
    // Create a small test image (10x10 grayscale)
    let width = 10;
    let height = 10;
    let data: Vec<u8> = vec![128; (width * height) as usize];
    
    let image = Image::from_gray(&data, width, height)?;
    println!("Created {}x{} grayscale image", width, height);
    
    // Scan the image
    let symbols = scanner.scan_image(&image)?;
    println!("Scanning completed");
    
    // Print results
    let mut count = 0;
    for symbol in symbols {
        count += 1;
        println!("Symbol {}: Type={:?}, Data={}", 
                 count, symbol.symbol_type(), symbol.data());
    }
    
    if count == 0 {
        println!("No symbols detected (expected for random noise image)");
    }
    
    Ok(())
}
