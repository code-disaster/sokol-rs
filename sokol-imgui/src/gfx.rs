pub mod ffi {
    extern {
        pub fn sg_imgui_wrap_init();
        pub fn sg_imgui_wrap_discard();
        pub fn sg_imgui_wrap_draw(ctx: *mut super::SgImGui);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SgImGui {
    pub buffers: bool,
    pub images: bool,
    pub shaders: bool,
    pub pipelines: bool,
    pub passes: bool,
    pub capture: bool,
}

pub fn sg_imgui_init() {
    unsafe {
        ffi::sg_imgui_wrap_init();
    }
}

pub fn sg_imgui_discard() {
    unsafe {
        ffi::sg_imgui_wrap_discard();
    }
}

pub fn sg_imgui_draw(ctx: &mut SgImGui) {
    unsafe {
        ffi::sg_imgui_wrap_draw(ctx);
    }
}
