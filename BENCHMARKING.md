# Benchmarking Guide

This document describes how to run performance tests and interpret results.

## Running Benchmarks

### Basic benchmarks

```bash
cargo bench
```

### Specific benchmark

```bash
cargo bench --bench scan_benchmark
```

### With flamegraph profiling

```bash
cargo install flamegraph
cargo flamegraph --bench scan_benchmark
```

## Benchmark Categories

### 1. Scanner Creation

Measures the overhead of creating a new `ImageScanner` instance.

**Expected performance**: < 1μs

### 2. Image Creation

Tests image allocation and initialization for various sizes.

**Benchmark sizes**: 100x100, 500x500, 1000x1000, 2000x2000

**Expected performance**:
- 100x100: < 10μs
- 1000x1000: < 1ms
- 2000x2000: < 5ms

### 3. Image Scanning

Core scanning performance on gradient patterns (no actual barcodes).

**Expected performance**:
- 100x100: < 100μs
- 500x500: < 1ms
- 1000x1000: < 5ms

### 4. Codec Configuration

Measures the cost of enabling/disabling different codecs.

**Expected performance**: < 100ns

### 5. Full Pipeline

End-to-end benchmark: create scanner, configure, create image, scan.

**Expected performance**:
- 100x100: < 200μs
- 500x500: < 2ms

## Memory Tests

Run memory leak and stress tests:

```bash
cargo test --test memory_test --release
```

### Valgrind memory check (Linux only)

```bash
valgrind --leak-check=full --show-leak-kinds=all \
  cargo test --test memory_test --release
```

### Heaptrack profiling (Linux)

```bash
heaptrack cargo test --test memory_test --release
heaptrack_gui heaptrack.cargo.*.gz
```

## Performance Tips

### 1. Reuse scanners

Creating a scanner once and reusing it is much faster:

```rust
// Good: Create once, reuse
let mut scanner = ImageScanner::new()?;
for image in images {
    scanner.scan_image(&image)?;
}

// Bad: Create every time
for image in images {
    let mut scanner = ImageScanner::new()?;
    scanner.scan_image(&image)?;
}
```

### 2. Image format

Y800 (grayscale) format is fastest. Convert color images to grayscale before scanning.

### 3. Resolution

Lower resolution scans faster. Consider downscaling if high resolution is not needed.

### 4. Codec selection

Enable only needed codecs with features:

```toml
[dependencies]
zbar-pack = { version = "0.1", default-features = false, features = ["vendored", "codec-qrcode"] }
```

## Comparing with C library

Benchmark against native ZBar C library:

```bash
# Install native ZBar
sudo apt-get install libzbar-dev

# Run comparative benchmarks
cargo bench --features system
cargo bench --features vendored
```

## CI Benchmarking

Benchmarks run automatically on CI for performance regression detection.

See `.github/workflows/benchmark.yml` for details.

## Interpreting Results

### Criterion output

```
scanner_creation        time:   [450.23 ns 451.89 ns 453.89 ns]
```

- First number: lower bound
- Second number: mean
- Third number: upper bound

### Performance regression

If you see:

```
Performance has regressed:
  time:   [520.11 ns 525.34 ns 530.89 ns]
  change: [+15.2% +16.3% +17.1%]
```

This indicates a ~16% slowdown compared to baseline.

## Optimization Checklist

- [ ] Profile with `perf` or `flamegraph`
- [ ] Check for unnecessary allocations
- [ ] Verify inlining of hot functions
- [ ] Consider using release mode with debug info for profiling
- [ ] Compare vendored vs system library performance
- [ ] Test on target hardware
