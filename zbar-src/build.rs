use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // vendored 模式: 编译内置的 ZBar 源码
    if cfg!(feature = "vendored") {
        build_vendored();
    }
}

fn build_vendored() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let src_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("vendor/zbar-0.23.93");

    println!("cargo:rerun-if-changed={}", src_dir.display());

    // Configure compiler
    let mut build = cc::Build::new();

    // Base compilation options
    build
        .include(src_dir.join("include"))
        .include(src_dir.join("zbar"))
        .define("HAVE_CONFIG_H", None)
        .define("NO_STATS", None) // Disable stats to reduce dependencies
        .warnings(false); // Ignore upstream warnings

    // Target platform settings
    let target = env::var("TARGET").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    // musl build optimization
    if target.contains("musl") {
        build.define("_GNU_SOURCE", None);
        println!("cargo:rustc-link-lib=static=pthread");
    }

    // Windows platform settings
    if target_os == "windows" {
        build.define("_WIN32", None);
        if target_env == "msvc" {
            build.flag("/std:c11");
        } else {
            build.flag("-std=c11");
        }
    } else {
        build.flag("-std=c11");
    }

    // macOS platform settings
    if target_os == "macos" {
        build.define("__APPLE__", None);
    }

    // Core source files
    let core_sources = [
        "zbar/config.c",
        "zbar/convert.c",
        "zbar/decoder.c",
        "zbar/error.c",
        "zbar/image.c",
        "zbar/img_scanner.c",
        "zbar/refcnt.c",
        "zbar/scanner.c",
        "zbar/symbol.c",
    ];

    for src in &core_sources {
        let path = src_dir.join(src);
        if path.exists() {
            build.file(&path);
        } else {
            eprintln!("Warning: Source file not found: {}", path.display());
        }
    }

    // QR code support
    #[cfg(feature = "codec-qrcode")]
    {
        let qrcode_sources = [
            "zbar/qrcode/bch15_5.c",
            "zbar/qrcode/binarize.c",
            "zbar/qrcode/isaac.c",
            "zbar/qrcode/qrdec.c",
            "zbar/qrcode/qrdectxt.c",
            "zbar/qrcode/rs.c",
            "zbar/qrcode/util.c",
        ];
        for src in &qrcode_sources {
            let path = src_dir.join(src);
            if path.exists() {
                build.file(&path);
            }
        }
    }

    // SQ code support
    #[cfg(feature = "codec-sqcode")]
    {
        build.file(src_dir.join("zbar/sqcode.c"));
    }

    // Barcode decoders - conditional compilation
    #[cfg(feature = "codec-ean")]
    build.file(src_dir.join("zbar/decoder/ean.c"));
    #[cfg(feature = "codec-databar")]
    build.file(src_dir.join("zbar/decoder/databar.c"));
    #[cfg(feature = "codec-code128")]
    build.file(src_dir.join("zbar/decoder/code128.c"));
    #[cfg(feature = "codec-code93")]
    build.file(src_dir.join("zbar/decoder/code93.c"));
    #[cfg(feature = "codec-code39")]
    build.file(src_dir.join("zbar/decoder/code39.c"));
    #[cfg(feature = "codec-codabar")]
    build.file(src_dir.join("zbar/decoder/codabar.c"));
    #[cfg(feature = "codec-i25")]
    build.file(src_dir.join("zbar/decoder/i25.c"));
    #[cfg(feature = "codec-qrcode")]
    build.file(src_dir.join("zbar/decoder/qr_finder.c"));
    #[cfg(feature = "codec-sqcode")]
    build.file(src_dir.join("zbar/decoder/sq_finder.c"));

    // Enable all codecs by default (unless minimal specified)
    #[cfg(not(feature = "minimal"))]
    {
        let all_decoder_sources = [
            "zbar/decoder/ean.c",
            "zbar/decoder/databar.c",
            "zbar/decoder/code128.c",
            "zbar/decoder/code93.c",
            "zbar/decoder/code39.c",
            "zbar/decoder/codabar.c",
            "zbar/decoder/i25.c",
            "zbar/decoder/qr_finder.c",
            "zbar/decoder/sq_finder.c",
        ];
        for src in &all_decoder_sources {
            let path = src_dir.join(src);
            if path.exists() {
                build.file(&path);
            }
        }

        // Enable QR code by default
        let qrcode_sources = [
            "zbar/qrcode/bch15_5.c",
            "zbar/qrcode/binarize.c",
            "zbar/qrcode/isaac.c",
            "zbar/qrcode/qrdec.c",
            "zbar/qrcode/qrdectxt.c",
            "zbar/qrcode/rs.c",
            "zbar/qrcode/util.c",
        ];
        for src in &qrcode_sources {
            let path = src_dir.join(src);
            if path.exists() {
                build.file(&path);
            }
        }

        // Enable SQ code by default
        let sqcode_path = src_dir.join("zbar/sqcode.c");
        if sqcode_path.exists() {
            build.file(&sqcode_path);
        }
    }

    // Generate config.h (simplified version)
    let config_h = out_dir.join("config.h");
    std::fs::write(
        &config_h,
        format!(
            r#"
#ifndef CONFIG_H
#define CONFIG_H

/* Enable POSIX features */
#ifndef _WIN32
#define _POSIX_C_SOURCE 200809L
#endif

#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include <time.h>

#define PACKAGE "zbar"
#define VERSION "0.23.93"
#define PACKAGE_VERSION "0.23.93"

/* Version macros */
#define ZBAR_VERSION_MAJOR 0
#define ZBAR_VERSION_MINOR 23
#define ZBAR_VERSION_PATCH 93

/* Disable unnecessary features */
#define NO_STATS 1

/* Timer interface */
#ifndef _WIN32
#define HAVE_SYS_TIME_H 1
#define _POSIX_TIMERS 200809L
#endif

/* Platform-specific definitions */
{}
{}
{}

/* Enable codecs */
{}
{}
{}
{}
{}
{}
{}
{}
{}

#endif /* CONFIG_H */
"#,
            if target_os == "windows" {
                "#define _WIN32 1"
            } else {
                ""
            },
            if target_os == "macos" {
                "#define __APPLE__ 1"
            } else {
                ""
            },
            if target.contains("musl") {
                "#define _GNU_SOURCE 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-ean") || !cfg!(feature = "minimal") {
                "#define ENABLE_EAN 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-databar") || !cfg!(feature = "minimal") {
                "#define ENABLE_DATABAR 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-code128") || !cfg!(feature = "minimal") {
                "#define ENABLE_CODE128 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-code93") || !cfg!(feature = "minimal") {
                "#define ENABLE_CODE93 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-code39") || !cfg!(feature = "minimal") {
                "#define ENABLE_CODE39 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-codabar") || !cfg!(feature = "minimal") {
                "#define ENABLE_CODABAR 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-i25") || !cfg!(feature = "minimal") {
                "#define ENABLE_I25 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-qrcode") || !cfg!(feature = "minimal") {
                "#define ENABLE_QRCODE 1"
            } else {
                ""
            },
            if cfg!(feature = "codec-sqcode") || !cfg!(feature = "minimal") {
                "#define ENABLE_SQCODE 1"
            } else {
                ""
            },
        ),
    )
    .expect("Failed to write config.h");

    build.include(&out_dir);

    // Compile as static library
    build.compile("zbar");

    // Output link information
    println!("cargo:rustc-link-lib=static=zbar");

    // Export include path for downstream crates (like zbar-sys)
    println!(
        "cargo:include={}",
        src_dir.join("include").display()
    );

    // Add system dependencies based on platform
    if target_os == "linux" {
        println!("cargo:rustc-link-lib=m");
        if !target.contains("musl") {
            println!("cargo:rustc-link-lib=pthread");
        }
    } else if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=Foundation");
    } else if target_os == "windows" {
        // Windows requires no additional libraries
    }

    // Log build information
    println!(
        "cargo:warning=ZBar compiled successfully (target: {})",
        target
    );
}
