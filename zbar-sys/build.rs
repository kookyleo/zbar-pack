use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // vendored mode: use zbar-src compiled static library
    #[cfg(feature = "vendored")]
    {
        // Find zbar.h header file
        let zbar_header = find_zbar_header();
        let include_dir = zbar_header.parent().unwrap();

        // Generate bindings using bindgen
        let bindings = bindgen::Builder::default()
            .header(zbar_header.to_string_lossy())
            .clang_arg(format!("-I{}", include_dir.display()))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .allowlist_function("zbar_.*")
            .allowlist_type("zbar_.*")
            .allowlist_var("ZBAR_.*")
            .prepend_enum_name(false)
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings");

        println!("cargo:rustc-link-lib=static=zbar");
    }

    // system/dynamic mode: use pkg-config to find system library
    #[cfg(all(not(feature = "vendored"), feature = "system"))]
    {
        if let Ok(lib) = pkg_config::Config::new()
            .atleast_version("0.10")
            .probe("zbar")
        {
            for path in lib.include_paths {
                println!("cargo:include={}", path.display());
            }
            for path in lib.link_paths {
                println!("cargo:rustc-link-search=native={}", path.display());
            }
            for lib in lib.libs {
                println!("cargo:rustc-link-lib={}", lib);
            }

            // Generate bindings for system library
            let mut builder =
                bindgen::Builder::default().header_contents("wrapper.h", "#include <zbar.h>");

            for path in &lib.include_paths {
                builder = builder.clang_arg(format!("-I{}", path.display()));
            }

            let bindings = builder
                .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                .allowlist_function("zbar_.*")
                .allowlist_type("zbar_.*")
                .allowlist_var("ZBAR_.*")
                .prepend_enum_name(false)
                .generate()
                .expect("Unable to generate bindings");

            bindings
                .write_to_file(out_dir.join("bindings.rs"))
                .expect("Couldn't write bindings");
        } else {
            println!("cargo:warning=pkg-config failed to find zbar, attempting direct link");
            println!("cargo:rustc-link-lib=zbar");
        }
    }

    // Add platform-specific link libraries
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "linux" {
        println!("cargo:rustc-link-lib=m");
        if !target.contains("musl") {
            println!("cargo:rustc-link-lib=pthread");
        }
    } else if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=Foundation");
    }
}

#[cfg(feature = "vendored")]
fn find_zbar_header() -> PathBuf {
    // Try to find zbar-src in various locations
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // 1. Try workspace (development)
    let workspace_path = manifest_dir
        .parent()
        .unwrap()
        .join("zbar-src/vendor/zbar-0.23.93/include/zbar.h");
    if workspace_path.exists() {
        return workspace_path;
    }

    // 2. Try to find in target directory (cargo package)
    // When building a packaged crate, dependencies are extracted to target/package/
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Navigate from OUT_DIR up to find zbar-src in package directory
    for ancestor in out_dir.ancestors() {
        // Check package directory
        if let Some(parent) = ancestor.parent() {
            let package_dir = parent.join("package");
            if package_dir.exists() {
                // Look for zbar-src-* directories
                if let Ok(entries) = std::fs::read_dir(&package_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir()
                            && path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .map(|n| n.starts_with("zbar-src-"))
                                .unwrap_or(false)
                        {
                            let header = path.join("vendor/zbar-0.23.93/include/zbar.h");
                            if header.exists() {
                                return header;
                            }
                        }
                    }
                }
            }
        }
    }

    panic!("Unable to find zbar.h header file. Looked in workspace and package directories.");
}
