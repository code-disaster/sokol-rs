use std::os::raw::c_void;
use std::ptr::null;

pub mod ffi {
    extern {
        pub fn sg_imgui_wrap_init(ctx: *mut super::SgImGui);
        pub fn sg_imgui_wrap_discard(ctx: *mut super::SgImGui);
        pub fn sg_imgui_wrap_draw(ctx: *mut super::SgImGui);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SgImGui {
    _content: *const c_void,
    pub buffers: bool,
    pub images: bool,
    pub shaders: bool,
    pub pipelines: bool,
    pub passes: bool,
    pub capture: bool,
}

impl SgImGui {
    pub fn new() -> Self {
        SgImGui {
            _content: null(),
            buffers: false,
            images: false,
            shaders: false,
            pipelines: false,
            passes: false,
            capture: false,
        }
    }
}

pub fn sg_imgui_init(ctx: &mut SgImGui) {
    unsafe {
        ffi::sg_imgui_wrap_init(ctx);
    }
}

pub fn sg_imgui_discard(ctx: &mut SgImGui) {
    unsafe {
        ffi::sg_imgui_wrap_discard(ctx);
    }
}

pub fn sg_imgui_draw(ctx: &mut SgImGui) {
    unsafe {
        ffi::sg_imgui_wrap_draw(ctx);
    }
}
