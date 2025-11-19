use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // vendored mode: use zbar-src compiled static library
    #[cfg(feature = "vendored")]
    {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let zbar_src_dir = manifest_dir
            .parent()
            .unwrap()
            .join("zbar-src/vendor/zbar-0.23.93");

        // Generate bindings using bindgen
        let bindings = bindgen::Builder::default()
            .header(zbar_src_dir.join("include/zbar.h").to_string_lossy())
            .clang_arg(format!("-I{}", zbar_src_dir.join("include").display()))
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
