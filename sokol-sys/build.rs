extern crate cc;

use std::env;

fn main() {
    let mut build = cc::Build::new();
    let tool = build.try_get_compiler();

    let is_debug = env::var("DEBUG").ok().is_some();

    let is_msvc = match &tool {
        Ok(tool) => {
            tool.is_like_msvc()
        }
        Err(_) => {
            false
        }
    };

    //
    // include paths
    //
    build
        .include("external/sokol");

    //
    // MacOS: need ARC, so compile sokol.m with -fobjc-arc
    //
    if cfg!(target_os = "macos") {
        build
            .flag("-fobjc-arc")
            .file("src/sokol.m");
    } else {
        build
            .file("src/sokol.c");
    }

    //
    // select sokol_gfx renderer, defaults to:
    // - Windows: D3D11 with MSVC, GLCORE33 otherwise
    // - MacOS: Metal
    // - Linux: GLCORE33
    //
    if cfg!(target_os = "windows") && is_msvc {
        build
            .flag("-DSOKOL_D3D11")
            .flag("-DSOKOL_D3D11_SHADER_COMPILER");
        println!("cargo:rustc-cfg=gfx=\"d3d11\"");
    } else if cfg!(target_os = "macos") {
        build.flag("-DSOKOL_METAL");
        println!("cargo:rustc-cfg=gfx=\"metal\"");
    } else {
        build.flag("-DSOKOL_GLCORE33");
        println!("cargo:rustc-cfg=gfx=\"glcore33\"");
    }

    //
    // silence some warnings
    //
    build
        .flag_if_supported("-Wno-unused-parameter");

    //
    // x86_64-pc-windows-gnu: additional compile/link flags
    //
    if cfg!(target_os = "windows") {
        if !is_msvc {
            build
                .flag("-D_WIN32_WINNT=0x0601")
                .flag_if_supported("-Wno-cast-function-type")
                .flag_if_supported("-Wno-sign-compare")
                .flag_if_supported("-Wno-unknown-pragmas");

            println!("cargo:rustc-link-lib=static=gdi32");
            println!("cargo:rustc-link-lib=static=ole32");
        }
    }

    if is_debug {
        build
            .flag("-D_DEBUG")
            .flag("-DSOKOL_DEBUG");
    }

    build
        .compile("sokol-sys");

    //
    // MacOS: frameworks
    //
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
    }

    //
    // Linux: libs
    //
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=X11");
        println!("cargo:rustc-link-lib=dylib=asound");
    }
}
