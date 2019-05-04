# sokol-imgui

Rust bindings to [Dear ImGui](https://github.com/ocornut/imgui) and the associated
[sokol utility libraries](https://github.com/floooh/sokol/tree/master/util).

This crate provides access to `sokol_imgui` (drop-in Dear ImGui renderer/event-handler) and
`sokol_gfx_imgui` (debug-inspection UI). It also contains FFI bindings to a _very small subset_ of the
`Dear ImGui` C++ library.

