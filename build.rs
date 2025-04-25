use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // #include "../../__external/llama.cpp/ggml/include/gguf.h"
    // #include "../../__external/llama.cpp/ggml/include/ggml.h"

    println!("cargo:rerun-if-changed=../__external/llama.cpp/ggml/src/ggml.c");
    println!("cargo:rerun-if-changed=../__external/llama.cpp/ggml/src/gguf.cpp");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_path).expect("Failed to create OUT_DIR"); // 👈 Add this line

    println!("Writing bindings to: {:?}", out_path.join("bindings.rs"));

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // cc::Build::new()
    //     .cpp(true)
    //     .warnings(false)
    //     .include("../__external/llama.cpp/ggml/include")
    //     .file("../__external/llama.cpp/ggml/src/ggml.c")
    //     .file("../__external/llama.cpp/ggml/src/gguf.cpp")
    //     .include("../__external/llama.cpp/src/llama.cpp")
    //     .flag_if_supported("-O3")
    //     .compile("ggml_gguf");

    // Compile C code (ggml.c)
    cc::Build::new()
        .warnings(false)
        .include("../__external/llama.cpp/ggml/include")
        .include("../__external/llama.cpp/ggml/src/")
        .file("../__external/llama.cpp/ggml/src/ggml.c")
        .file("../__external/llama.cpp/ggml/src/ggml-quants.c")
        .compile("ggml");

    // Compile C++ code (gguf.cpp)
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .include("../__external/llama.cpp/ggml/include")
        .file("../__external/llama.cpp/ggml/src/gguf.cpp")
        .file("../__external/llama.cpp/ggml/src/ggml-threading.cpp")
        .flag_if_supported("-std=c++17") // <--- strong modern standard
        .flag_if_supported("-O3") // optimize hard
        .flag_if_supported("-fpermissive") // <--- allow slight loose C++ typing
        .compile("gguf");

    // Tell cargo to link
    println!("cargo:rustc-link-arg=-Wl,--start-group");
    println!("cargo:rustc-link-lib=static=ggml");
    println!("cargo:rustc-link-lib=static=gguf");
    println!("cargo:rustc-link-arg=-Wl,--end-group");
    // C++ runtime
    println!("cargo:rustc-link-lib=dylib=stdc++");
    // println!("cargo:rustc-link-arg=-Wl,--start-group");
    // println!("cargo:rustc-link-arg=-Wl,--end-group");
    // // C++ runtime
    // println!("cargo:rustc-link-lib=dylib=stdc++");
}

// let full_path = std::fs::canonicalize("../../external/llama.cpp/build/ggml/src")
// .expect("Could not resolve path to libggml-base.a");
// println!("cargo:rustc-link-search=native={}", full_path.display());
// println!("cargo:rustc-link-lib=static=ggml-base");

// println!("cargo:rustc-link-lib=static=llrs_shim");
