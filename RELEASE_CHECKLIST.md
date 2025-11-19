# Release Checklist

Follow this checklist before publishing to crates.io.

## Pre-release

### Code Quality
- [ ] All tests pass: `cargo test --all-features`
- [ ] Clippy is happy: `cargo clippy --all-features -- -D warnings`
- [ ] Formatting is correct: `cargo fmt --all -- --check`
- [ ] Documentation builds: `cargo doc --all-features --no-deps`
- [ ] Examples work: `cargo run --example simple`
- [ ] Benchmarks compile: `cargo bench --no-run`

### Package Size
- [ ] Check package size: `cargo package --list | wc -l`
- [ ] Verify size is under 10MB (crates.io limit)
- [ ] Remove unnecessary files from vendor directory
- [ ] Update `.gitignore` and `Cargo.toml` exclude list

### Documentation
- [ ] README is up-to-date
- [ ] CHANGELOG is updated with version and date
- [ ] API documentation is complete
- [ ] Examples are documented
- [ ] COMPLIANCE.md is current

### Metadata
- [ ] `Cargo.toml` version is correct
- [ ] `Cargo.toml` metadata is complete:
  - [ ] description
  - [ ] repository
  - [ ] homepage
  - [ ] keywords (max 5)
  - [ ] categories (max 5)
  - [ ] license
- [ ] README links work
- [ ] License files are included

### Dependencies
- [ ] Dependencies are minimal
- [ ] Version constraints are appropriate
- [ ] No path dependencies in published crates
- [ ] Optional dependencies are properly gated

## Publishing

### Dry Run
```bash
# Check what will be published
cargo package --list

# Dry run publish
cargo publish --dry-run

# Check package contents
cargo package
tar tzf target/package/zbar-pack-0.1.0.crate | less
```

### Version Bump
```bash
# Update version in Cargo.toml files
# - zbar-pack/Cargo.toml
# - zbar-sys/Cargo.toml
# - zbar-src/Cargo.toml
# - Root Cargo.toml workspace

# Update CHANGELOG.md
# Tag version
git tag -a v0.1.0 -m "Release version 0.1.0"
```

### Publish Order
```bash
# 1. Publish zbar-src first
cd zbar-src
cargo publish

# 2. Wait for it to be available, then publish zbar-sys
cd ../zbar-sys
cargo publish

# 3. Finally publish zbar-pack
cd ../zbar-pack
cargo publish

# 4. Push tags
git push origin v0.1.0
```

## Post-release

- [ ] Verify package on crates.io
- [ ] Test installation: `cargo install --force zbar-pack`
- [ ] Create GitHub release with binaries
- [ ] Update documentation site (if applicable)
- [ ] Announce on:
  - [ ] Reddit /r/rust
  - [ ] This Week in Rust
  - [ ] Twitter/X
  - [ ] Project blog

## Troubleshooting

### Package too large

If vendor directory exceeds size limits:

1. Create separate repository for ZBar source
2. Use git submodule or download in build.rs
3. Strip unnecessary files:
   ```bash
   cd zbar-src/vendor/zbar-0.23.93
   rm -rf .git doc examples test android iphone java perl python qt gtk
   ```

### Build fails on crates.io

- Ensure all build dependencies are available
- Test in clean environment: `docker run --rm -it rust:latest`
- Check build logs on crates.io
- May need to adjust build.rs or dependencies

### Version conflicts

- Ensure workspace versions are synchronized
- Check that path dependencies use correct versions
- Run `cargo update` and test again
