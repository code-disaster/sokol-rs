# sokol-rs

[![Crates.io][ci]][cl] [![Docs.rs][di]][dl] ![zlib][li]

[ci]: https://img.shields.io/crates/v/sokol.svg
[cl]: https://crates.io/crates/sokol/

[li]: https://img.shields.io/crates/l/sokol.svg?maxAge=2592000

[di]: https://docs.rs/sokol/badge.svg
[dl]: https://docs.rs/sokol/

Rust bindings to the [sokol](https://github.com/floooh/sokol) header-only, cross-platform libraries.

This crate provides access to `sokol_gfx` (3D-API wrapper), `sokol_app` (application framework wrapper), `sokol_time` (time measurement) and `sokol_audio` (buffer-streaming audio playback). It can serve as an easy-to-use, lean, _almost_ dependency-free entry point to create a graphics or game application.
