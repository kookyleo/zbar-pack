use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use zbar_pack::{Image, ImageScanner, SymbolType};

fn create_test_image(width: u32, height: u32) -> Vec<u8> {
    // Create a simple gradient pattern
    (0..(width * height))
        .map(|i| ((i % 256) as u8))
        .collect()
}

fn bench_scanner_creation(c: &mut Criterion) {
    c.bench_function("scanner_creation", |b| {
        b.iter(|| {
            let scanner = ImageScanner::new().unwrap();
            black_box(scanner);
        });
    });
}

fn bench_image_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_creation");

    for size in [100, 500, 1000, 2000].iter() {
        let data = create_test_image(*size, *size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let image = Image::from_gray(&data, size, size).unwrap();
                black_box(image);
            });
        });
    }

    group.finish();
}

fn bench_image_scanning(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_scanning");

    for size in [100, 500, 1000].iter() {
        let data = create_test_image(*size, *size);
        let mut scanner = ImageScanner::new().unwrap();
        scanner.set_config(SymbolType::QRCODE, 0, 1).unwrap();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let image = Image::from_gray(&data, size, size).unwrap();
            b.iter(|| {
                let symbols = scanner.scan_image(&image).unwrap();
                black_box(symbols);
            });
        });
    }

    group.finish();
}

fn bench_codec_configuration(c: &mut Criterion) {
    let mut scanner = ImageScanner::new().unwrap();

    c.bench_function("configure_qrcode", |b| {
        b.iter(|| {
            scanner.set_config(SymbolType::QRCODE, 0, 1).unwrap();
        });
    });

    c.bench_function("configure_ean", |b| {
        b.iter(|| {
            scanner.set_config(SymbolType::EAN13, 0, 1).unwrap();
        });
    });
}

fn bench_full_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");

    for size in [100, 500].iter() {
        let data = create_test_image(*size, *size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut scanner = ImageScanner::new().unwrap();
                scanner.set_config(SymbolType::QRCODE, 0, 1).unwrap();
                let image = Image::from_gray(&data, size, size).unwrap();
                let symbols = scanner.scan_image(&image).unwrap();
                black_box(symbols);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_scanner_creation,
    bench_image_creation,
    bench_image_scanning,
    bench_codec_configuration,
    bench_full_pipeline
);
criterion_main!(benches);
