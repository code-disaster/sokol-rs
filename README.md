# sokol-rs

Rust bindings to the [sokol](https://github.com/floooh/sokol) header-only cross-platform libraries.

## status

C library | Rust module | status | notes
:---: | :---: | :---: | ---
[sokol_app.h](https://github.com/floooh/sokol/blob/master/sokol_app.h) | sokol::app | done |
[sokol_args.h](https://github.com/floooh/sokol/blob/master/sokol_args.h) | n/a | n/a | low priority - there are many cmdline parsers for Rust already
[sokol_audio.h](https://github.com/floooh/sokol/blob/master/sokol_audio.h) | `sokol::audio` | done | callback API via trait in `sokol::app`
[sokol_gfx.h](https://github.com/floooh/sokol/blob/master/sokol_gfx.h) | `sokol::gfx` | mostly done | missing: separate resource management, render contexts
[sokol_time.h](https://github.com/floooh/sokol/blob/master/sokol_time.h) | `sokol::time` | done |

## remarks

- I didn't bother separating the libraries as much as the C version does: the `gfx` (window/render context creation) and `audio` (stream callback) modules are hard-wired to the `app` module.
- `gfx` is configured to use the "native" render API on each platform:
  - Windows (MSVC): Direct3D 11
  - Windows (not MSVC, tested with `x86_64-pc-windows-gnu`): OpenGL 3.3
  - MacOS: Metal
  - Linux: OpenGL 3.3
- `x86_64-pc-windows-gnu` uses GL33 because sokol_gfx fails to compile for Direct3D 11 with gcc on MinGW64 right now. I didn't spend much time to figure out why.
