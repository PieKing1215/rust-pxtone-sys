fn main() {
    let src = [
        "pxtone/pxtone.cpp",
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
        .cpp(true)
        .files(src.iter())
        .include("ogg")
        .include("pxtone")
        .define("pxINCLUDE_OGGVORBIS", None)
        .flag_if_supported("-fpermissive");
        
    build.compile("pxtone");
}