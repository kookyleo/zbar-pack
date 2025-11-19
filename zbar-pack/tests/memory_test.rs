use zbar_pack::{Image, ImageScanner, SymbolType};

#[test]
fn test_no_memory_leak_scanner() {
    // Create and drop many scanners
    for _ in 0..1000 {
        let scanner = ImageScanner::new().unwrap();
        drop(scanner);
    }
}

#[test]
fn test_no_memory_leak_image() {
    // Create and drop many images
    let data: Vec<u8> = vec![128; 100 * 100];
    for _ in 0..1000 {
        let image = Image::from_gray(&data, 100, 100).unwrap();
        drop(image);
    }
}

#[test]
fn test_no_memory_leak_scanning() {
    let mut scanner = ImageScanner::new().unwrap();
    scanner.set_config(SymbolType::QRCODE, 0, 1).unwrap();

    let data: Vec<u8> = vec![128; 100 * 100];

    // Scan many times
    for _ in 0..1000 {
        let image = Image::from_gray(&data, 100, 100).unwrap();
        let _symbols = scanner.scan_image(&image).unwrap();
    }
}

#[test]
fn test_large_image_handling() {
    let mut scanner = ImageScanner::new().unwrap();

    // Test with large image (4K resolution)
    let width = 3840u32;
    let height = 2160u32;
    let data: Vec<u8> = vec![128; (width * height) as usize];

    let image = Image::from_gray(&data, width, height).unwrap();
    let _symbols = scanner.scan_image(&image).unwrap();
}

#[test]
fn test_concurrent_scanners() {
    use std::thread;

    let handles: Vec<_> = (0..4)
        .map(|_| {
            thread::spawn(|| {
                let mut scanner = ImageScanner::new().unwrap();
                scanner.set_config(SymbolType::QRCODE, 0, 1).unwrap();

                let data: Vec<u8> = vec![128; 100 * 100];
                let image = Image::from_gray(&data, 100, 100).unwrap();

                for _ in 0..100 {
                    let _symbols = scanner.scan_image(&image).unwrap();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
