# sokol-rs

This repository contains source code of the following Rust library crates:

- sokol ([README](sokol/)) - Rust bindings to the [sokol][sokol] header-only, cross-platform libraries.
- sokol-imgui ([README](sokol-imgui/)) - Rust bindings to the `sokol_imgui` and `sokol_gfx_imgui` utility libraries.
- sokol-sys ([README](sokol-sys/)) - system crate to compile native dependencies shared by other crates.


- sokol-stb - a library for easy access to a _subset_ of the [stb][stb] libraries.
- sokol-samples - a number of examples ported from [sokol-samples][sokol-samples].

[imgui]: https://github.com/ocornut/imgui
[sokol]: https://github.com/floooh/sokol
[sokol-samples]: https://github.com/floooh/sokol-samples
[stb]: https://github.com/nothings/stb

## How to build

The current version compiles (and has been tested) with stable Rust (v1.34.1) on Windows (both MSVC and GNU toolchains), MacOS and Linux.

~~~
> git clone --recursive https://github.com/code-disaster/sokol-rs
> cd sokol-rs
> cargo build
~~~

The `sokol-samples` folder contains some examples ported from [sokol-samples/sapp](https://github.com/floooh/sokol-samples/tree/master/sapp).

~~~
> cargo run --bin clear-sapp 
~~~

## About the implementation

### The __SApp__ program loop

In the C version of `sokol_app`, when compiled with `SOKOL_NO_ENTRY`, you call `sapp_run()`, passing callback function pointers for setup, frame updates, and cleanup.

In the Rust version, you call `sokol::app::sapp_run()`. This hands over control to the C library, which then will operate as usual. The C callbacks are implemented by sokol-rs. They are forwarded to your application via the `SApp` trait. User applications implement this trait to power the application loop.

Check the [clear-sapp](https://github.com/code-disaster/sokol-rs/blob/master/sokol-samples/clear-sapp/src/main.rs) sample for a minimal implementation.

### API style and implementation details

I tried to stay as close as possible to the source, while adjusting to the Rust naming conventions, as well as making the public API more convenient in places a direct port would be too cumbersome.

- Function names stay the same. No change here.
- Function signatures are identical _most of the time_. In some rare cases, I moved parameters out of structs and pass them as additional function parameters.
- Type names are renamed, e.g. `sapp_desc` -> `SAppDesc`.
- Some identifiers, like "type", had to be renamed because they clash with reserved keywords in Rust.
- Element names in enums are shortened and changed to CamelCase, e.g. `sapp_event_type::SAPP_EVENTTYPE_RESIZED` becomes `SAppEventType::Resized`. On the plus side, they are all totally type-safe now.
- I tried to stay true to the C99-style struct initializers - check the samples to see what it looks like. Since Rust forces you to initialize __all__ struct members, most of them enable `#[derive(Default)]` so that you are still able to only set the options you are interested in - everything else can be initialized to sokol's default values with `..Default::default()`.
- Arrays in structs, which are all fixed-sized in sokol, are initialized using `Vec<T>` in public declarations. This is because they can be set conveniently with `vec![]`, so you don't have to keep an eye on the array size, and/or spatter `Default::default()` all over the place. _(I tried to be clever and use macro magic to make this part even more convenient while not paying the Vec<> allocation overhead, but I'm not nearly clever enough with Rust... yet.)_

In the Rust library, the `app`, `gfx` and `audio` modules are not as separable as their C counterparts. Essentially, sokol-rs assumes that you use them in conjunction.

- `sg_setup()` uses `app` functions to configure the render backend.
- If `saudio_setup()` is told to use callbacks, the function callback is part of the `SApp` trait (and, as a matter of fact, managed by the `app` module in most parts).

### Status

C library | Rust module | status | notes
:---: | :---: | :---: | ---
[sokol_app.h](https://github.com/floooh/sokol/blob/master/sokol_app.h) | `sokol::app` | done |
[sokol_args.h](https://github.com/floooh/sokol/blob/master/sokol_args.h) | n/a | n/a | _low priority_: there are many cmdline parsers for Rust already
[sokol_audio.h](https://github.com/floooh/sokol/blob/master/sokol_audio.h) | `sokol::audio` | done | callback API via trait in `sokol::app`
[sokol_gfx.h](https://github.com/floooh/sokol/blob/master/sokol_gfx.h) | `sokol::gfx` | mostly done | trace hooks are enabled (and consumed by the `sokol_gfx_imgui` implementation in the sokol-imgui crate)<br><br>_missing_: separate resource management, render contexts, user-provided buffers<br><br>_not implemented_: `sg_query_*_info()` functions  
[sokol_time.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | `sokol::time` | done |

C util library | Rust module | status | notes
:---: | :---: | :---: | ---
sokol_imgui | `sokol_imgui::imgui` | done |
sokol_gl | n/a | n/a | _no plans yet_
sokol_gfx_imgui | `sokol_imgui::gfx` | partially<br>done | `sg_imgui_t` is wrapped opaquely instead of replicated in Rust<br><br>_missing_: `sg_imgui_draw_*_content` and `sg_imgui_draw_*_window` functions

### Remarks

- `gfx` is configured to use the "native" render API on each platform:
  - Windows (MSVC): Direct3D 11
  - Windows (not MSVC, tested with `x86_64-pc-windows-gnu`): OpenGL 3.3
  - MacOS: Metal
  - Linux: OpenGL 3.3
- `x86_64-pc-windows-gnu` uses GL33 because sokol_gfx fails to compile for Direct3D 11 with gcc on MinGW64. I didn't invest much time to figure out why.
