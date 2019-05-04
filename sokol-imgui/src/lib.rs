/*!

Rust bindings to the `sokol_imgui` and `sokol_gfx_imgui` utility libraries for
[sokol](https://github.com/floooh/sokol).

This crate also exposes bindings to a very small subset of the
[Dear ImGui](https://github.com/ocornut/imgui) library.

*/

extern crate sokol;
extern crate sokol_sys as sys;

pub mod gfx;
pub mod imgui;
