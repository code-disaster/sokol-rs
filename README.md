# sokol-rs

_Experimental_ Rust bindings to the [sokol](https://github.com/floooh/sokol) header-only cross-platform libraries.

> Work in progress! I'm adding wrapper functions as I go, while trying to port some of the [sokol samples](https://github.com/floooh/sokol-samples). Also, regarding the Rust language, I'm still a greenhorn, so I may adjust APIs frequently, or break things.

## build

As of today (Nov 2018), this version is configured to build with the _Rust 2018 edition_, so you'll need a Rust beta toolchain. It won't compile on _stable_ right now.

## status

library | Rust bindings | notes
--- | --- | ---
[sokol_app.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | done |
[sokol_args.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | n/a | low priority - there are many cmdline parsers for Rust already
[sokol_audio.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | mostly done | missing: callback API
[sokol_gfx.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | mostly done | missing: separate resource management, render contexts
[sokol_time.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | done |

## remarks

- I didn't bother separating the libraries as much as the C version does. Right now, sokol_gfx requires (calls functions of) sokol_app.
- sokol_gfx is configured to use the "native" render API on each platform:
  - Windows (MSVC): Direct3D 11
  - Windows (not MSVC, tested with `x86_64-pc-windows-gnu`): OpenGL 3.3
  - MacOS: Metal
  - Linux: OpenGL 3.3
- It uses GL with `x86_64-pc-windows-gnu ` because sokol_gfx fails to compile for Direct3D 11 with gcc on MinGW64 right now. I didn't try to figure out why.
- There may be a compile-time switch to opt-in for an render API when I learn how to `[feature]` properly.
