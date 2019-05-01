extern crate cc;

use std::env;

use cc::{Build, Tool};

fn build_new() -> (Build, Tool) {
    let build = Build::new();
    let tool = build.try_get_compiler().unwrap();

    (build, tool)
}

fn select_sokol_gfx_renderer(build: &mut Build, is_msvc: bool, is_impl: bool) {
    //
    // select sokol_gfx renderer, defaults to:
    // - Windows: D3D11 with MSVC, GLCORE33 otherwise
    // - MacOS: Metal
    // - Linux: GLCORE33
    //
    if cfg!(target_os = "windows") && is_msvc {
        build.flag("-DSOKOL_D3D11");
    } else if cfg!(target_os = "macos") {
        build.flag("-DSOKOL_METAL");
    } else {
        build.flag("-DSOKOL_GLCORE33");
    }

    if is_impl {
        if cfg!(target_os = "windows") && is_msvc {
            build.flag("-DSOKOL_D3D11_SHADER_COMPILER");
            println!("cargo:rustc-cfg=gfx=\"d3d11\"");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-cfg=gfx=\"metal\"");
        } else {
            println!("cargo:rustc-cfg=gfx=\"glcore33\"");
        }
    }
}

fn make_sokol() {
    let (mut build, tool) = build_new();

    let is_debug = env::var("DEBUG").ok().is_some();
    let is_msvc = tool.is_like_msvc();

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
    // select sokol_gfx renderer
    //
    select_sokol_gfx_renderer(&mut build, is_msvc, true);

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

fn make_sokol_imgui() {
    let (mut build, tool) = build_new();

    let is_msvc = tool.is_like_msvc();

    //
    // include paths
    //
    build
        .include("external/imgui")
        .include("external/sokol")
        .include("external/sokol/util");

    //
    // source files
    //
    build.files(&[
        "src/sokol_imgui.cc",
        "external/imgui/imgui.cpp",
        "external/imgui/imgui_demo.cpp",
        "external/imgui/imgui_draw.cpp",
        "external/imgui/imgui_widgets.cpp"
    ]);

    //
    // oh dear!
    //
    if cfg!(target_os = "windows") && !is_msvc {
        build.file("src/sokol_imgui_patch_mingw.c");
    }

    //
    // select sokol_gfx renderer
    //
    select_sokol_gfx_renderer(&mut build, is_msvc, false);

    build
        .cpp(true)
        .compile("sokol-sys-imgui");
}

fn main() {
    make_sokol();
    make_sokol_imgui();
}
