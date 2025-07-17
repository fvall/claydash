fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Make sure we re-run the build script if the clay.h file changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/external/clay/clay.h");
    println!("cargo:rerun-if-changed=src/external/raylib/raylib.h");
    println!("cargo:rerun-if-changed=src/external/raylib/raymath.h");
    println!("cargo:rustc-link-search=raylib-5.5_linux_amd64/lib");
    println!("cargo:rustc-link-lib=dl");
    match std::env::var("CARGO_FEATURE_HOT_RELOAD") {
        Err(_) => {
            println!("cargo:rustc-link-lib=static=raylib");
        }
        Ok(num) => {
            if num == "0" {
                println!("cargo:rustc-link-lib=static=raylib");
            } else {
                println!("cargo:rustc-link-arg=-Wl,-rpath,raylib-5.5_linux_amd64/lib");
                println!("cargo:rustc-link-lib=raylib");
            }
        }
    }

    if target_os == "windows" {
        panic!("I do not develop on windows");
        cc::Build::new()
            .file("clay.cpp")
            .warnings(false)
            .std("c++20")
            .compile("clay");
    } else {
        cc::Build::new()
            .file("src/external/clay/clay.c")
            .warnings(false)
            .compile("clay");
    }

    let clay = bindgen::Builder::default()
        .header("src/external/clay/clay.h")
        .generate()
        .expect("Unable to generate clay.h bindings");

    let out_path = std::path::Path::new("src/bindings/");
    clay.write_to_file(out_path.join("clay.rs"))
        .expect("Couldn't write clay.rs bindings!");

    let raylib = bindgen::Builder::default()
        .header("src/external/raylib/raylib.h")
        .generate()
        .expect("Unable to generate raylib.h bindings");

    raylib
        .write_to_file(out_path.join("raylib.rs"))
        .expect("Couldn't write raylib.rs bindings!");

    let raymath = bindgen::Builder::default()
        .header("src/external/raylib/raymath.h")
        .generate()
        .expect("Unable to generate raymath.h bindings");

    raymath
        .write_to_file(out_path.join("raymath.rs"))
        .expect("Couldn't write raymath.rs bindings!");
}
