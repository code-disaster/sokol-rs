/*!

This crate provides a renderer backend and user input handler to
[imgui-sys](https://crates.io/crates/imgui-sys). It is aimed to ease integration of
[Dear ImGui](https://github.com/ocornut/imgui) for applications using the sokol API.

*/

extern crate imgui_sys;
extern crate sokol;

pub mod app;
pub mod gfx;
