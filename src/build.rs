use std::{env, path::PathBuf, thread};

fn main() {

    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=pxtone/");
    println!("cargo:rerun-if-changed=ogg/");
    
    let pxtone = thread::spawn(|| {
        let src = [
            "pxtone/pxtone.cpp",
        ];

        let mut builder = cc::Build::new();
        let build = builder
            .cpp(true)
            .files(src.iter())
            .include("ogg")
            .include("pxtone")
            .define("pxINCLUDE_OGGVORBIS", "1")
            .flag_if_supported("-fpermissive")
            .flag_if_supported("-Wno-everything"); // don't care about these warnings
        
        build.compile("pxtone");
    });

    let bindings = if let Ok("wasm32-unknown-emscripten") = env::var("TARGET").as_ref().map(|s| s.as_str()) {
        Some(thread::spawn(|| {
            let bindings = bindgen::Builder::default()
                .header("src/wrapper.hpp")
                .blocklist_item("FP_INT_UPWARD")
                .blocklist_item("FP_INT_DOWNWARD")
                .blocklist_item("FP_INT_TOWARDZERO")
                .blocklist_item("FP_INT_TONEARESTFROMZERO")
                .blocklist_item("FP_INT_TONEAREST")
                .blocklist_item("FP_INT_DOWNWARD")
                .blocklist_item("FP_NAN")
                .blocklist_item("FP_INFINITE")
                .blocklist_item("FP_ZERO")
                .blocklist_item("FP_SUBNORMAL")
                .blocklist_item("FP_NORMAL")
                .blocklist_item("std::value")
                .opaque_type("std::.*")
                .clang_args(["--sysroot", format!("{}/upstream/emscripten/cache/sysroot", env::var("EMSDK").unwrap()).as_str()])
                .clang_arg("-fvisibility=default")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .generate()
                .expect("Unable to generate bindings");

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
        }))
    } else {
        Some(thread::spawn(|| {
            let bindings = bindgen::Builder::default()
                .header("src/wrapper.hpp")
                .blocklist_item("FP_INT_UPWARD")
                .blocklist_item("FP_INT_DOWNWARD")
                .blocklist_item("FP_INT_TOWARDZERO")
                .blocklist_item("FP_INT_TONEARESTFROMZERO")
                .blocklist_item("FP_INT_TONEAREST")
                .blocklist_item("FP_INT_DOWNWARD")
                .blocklist_item("FP_NAN")
                .blocklist_item("FP_INFINITE")
                .blocklist_item("FP_ZERO")
                .blocklist_item("FP_SUBNORMAL")
                .blocklist_item("FP_NORMAL")
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .generate()
                .expect("Unable to generate bindings");
    
            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
        }))
    };

    let src = [
        "ogg/bitwise.c",
        "ogg/framing.c",
        "ogg/vorbisfile.c",
        "ogg/lib/info.c",
        "ogg/lib/block.c",
        "ogg/lib/synthesis.c",
        "ogg/lib/sharedbook.c",
        "ogg/lib/codebook.c",
        "ogg/lib/psy.c",
        "ogg/lib/floor0.c",
        "ogg/lib/floor1.c",
        "ogg/lib/registry.c",
        "ogg/lib/mdct.c",
        "ogg/lib/envelope.c",
        "ogg/lib/smallft.c",
        "ogg/lib/bitrate.c",
        "ogg/lib/window.c",
        "ogg/lib/lpc.c",
        "ogg/lib/res0.c",
        "ogg/lib/lsp.c",
        "ogg/lib/mapping0.c",
    ];

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("ogg")
        .flag_if_supported("-fpermissive")
        .flag_if_supported("-Wno-everything"); // don't care about these warnings
    
    build.compile("vorbis");

    if let Some(bindings) = bindings {
        bindings.join().unwrap();   
    }

    pxtone.join().unwrap();
}