use sokol::app::SAppEvent;
use sokol::gfx::SgPixelFormat;

pub mod ffi {
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};

    use sokol::app::ffi::SAppEvent;
    use sokol::gfx::SgPixelFormat;

    #[repr(C)]
    #[derive(Debug)]
    pub struct SImGuiDesc {
        max_vertices: i32,
        color_format: SgPixelFormat,
        depth_format: SgPixelFormat,
        sample_count: i32,
        dpi_scale: f32,
        ini_filename: *const c_char,
        no_default_font: bool,
    }

    impl SImGuiDesc {
        pub fn make(desc: super::SImGuiDesc) -> Self {
            let ini_filename = CString::new(desc.ini_filename).unwrap();

            SImGuiDesc {
                max_vertices: desc.max_vertices,
                color_format: desc.color_format,
                depth_format: desc.depth_format,
                sample_count: desc.sample_count,
                dpi_scale: desc.dpi_scale,
                ini_filename: ini_filename.into_raw(),
                no_default_font: desc.no_default_font,
            }
        }
    }

    extern {
        pub fn simgui_setup(desc: *const SImGuiDesc);
        pub fn simgui_new_frame(width: c_int, height: c_int, delta_time: f64);
        pub fn simgui_render();
        pub fn simgui_handle_event(event: *const SAppEvent) -> bool;
        pub fn simgui_shutdown();

        pub fn simgui_show_demo_window(is_open: *mut bool);
    }
}

#[derive(Default, Debug)]
pub struct SImGuiDesc<'a> {
    pub max_vertices: i32,
    pub color_format: SgPixelFormat,
    pub depth_format: SgPixelFormat,
    pub sample_count: i32,
    pub dpi_scale: f32,
    pub ini_filename: &'a str,
    pub no_default_font: bool,
}

pub fn simgui_setup(desc: SImGuiDesc) {
    unsafe {
        ffi::simgui_setup(&ffi::SImGuiDesc::make(desc));
    }
}

pub fn simgui_new_frame(width: i32, height: i32, delta_time: f64) {
    unsafe {
        ffi::simgui_new_frame(width, height, delta_time);
    }
}

pub fn simgui_render() {
    unsafe {
        ffi::simgui_render();
    }
}

pub fn simgui_handle_event(event: &SAppEvent) -> bool {
    unsafe {
        ffi::simgui_handle_event(&sokol::app::ffi::SAppEvent::translate(event))
    }
}

pub fn simgui_shutdown() {
    unsafe {
        ffi::simgui_shutdown();
    }
}

pub fn simgui_show_demo_window(is_open: &mut bool) {
    unsafe {
        ffi::simgui_show_demo_window(is_open);
    }
}
