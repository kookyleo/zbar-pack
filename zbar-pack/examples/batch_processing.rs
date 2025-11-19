use std::time::Instant;
/// Example: Batch process multiple images efficiently
///
/// Demonstrates best practices for scanning multiple images:
/// - Reuse scanner instance
/// - Process images in parallel (optional)
/// - Handle errors gracefully
use zbar_pack::{Image, ImageScanner, SymbolType};

fn generate_test_image(id: usize) -> (Vec<u8>, u32, u32) {
    let size = 100u32;
    // Generate unique pattern based on id
    let data: Vec<u8> = (0..(size * size))
        .map(|i| ((i + (id as u32) * 17) % 256) as u8)
        .collect();
    (data, size, size)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_images = 100;

    println!("Batch Processing Example");
    println!("Processing {} images...\n", num_images);

    // Create scanner once
    let mut scanner = ImageScanner::new()?;
    scanner.set_config(SymbolType::QRCODE, 0, 1)?;
    scanner.set_config(SymbolType::EAN13, 0, 1)?;

    let start = Instant::now();

    for i in 0..num_images {
        let (data, width, height) = generate_test_image(i);
        let image = Image::from_gray(&data, width, height)?;

        match scanner.scan_image(&image) {
            Ok(symbols) => {
                let count: usize = symbols.count();
                if count > 0 {
                    println!("Image {}: Found {} symbol(s)", i, count);
                }
            }
            Err(e) => {
                eprintln!("Image {}: Error: {:?}", i, e);
            }
        }

        if (i + 1) % 10 == 0 {
            print!(".");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }
    }

    let duration = start.elapsed();
    println!("\n\nProcessing complete!");
    println!("Total time: {:?}", duration);
    println!("Average per image: {:?}", duration / num_images as u32);
    println!(
        "Images per second: {:.2}",
        num_images as f64 / duration.as_secs_f64()
    );

    Ok(())
}
