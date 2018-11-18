extern crate cc;

fn main() {
    let mut build = cc::Build::new();

    build
        .file("src/stb/stb_vorbis.c")
        .flag_if_supported("-Wno-unused-value")
        .flag_if_supported("-Wno-unused-parameter");

    build
        .compile("sokol-stb");
}
