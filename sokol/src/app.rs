use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;

pub mod ffi {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_void;
    use std::ffi::CStr;

    const SAPP_MAX_TOUCHPOINTS: usize = 8;
    const _SAPP_MAX_MOUSEBUTTONS: usize = 3;
    const _SAPP_MAX_KEYCODES: usize = 512;

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct SappTouchPoint {
        identifier: usize,
        pos_x: f32,
        pos_y: f32,
        changed: bool,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct SappEvent {
        event_type: super::SappEventType,
        frame_count: u32,
        key_code: u32,
        char_code: u32,
        modifiers: u32,
        mouse_button: super::SappMouseButton,
        mouse_x: f32,
        mouse_y: f32,
        scroll_x: f32,
        scroll_y: f32,
        num_touches: c_int,
        touches: [SappTouchPoint; SAPP_MAX_TOUCHPOINTS],
        window_width: c_int,
        window_height: c_int,
        framebuffer_width: c_int,
        framebuffer_height: c_int,
    }

    #[repr(C)]
    pub struct Desc {
        init_cb: extern fn(),
        frame_cb: extern fn(),
        cleanup_cb: extern fn(),
        event_cb: extern fn(*const SappEvent),
        fail_cb: extern fn(*const c_char),
        width: c_int,
        height: c_int,
        sample_count: c_int,
        swap_interval: c_int,
        high_dpi: bool,
        fullscreen: bool,
        alpha: bool,
        premultiplied_alpha: bool,
        preserve_drawing_buffer: bool,
        window_title: *const c_char,
        html5_canvas_name: *const c_char,
        html5_canvas_resize: bool,
        ios_keyboard_resizes_canvas: bool,
        gl_force_gles2: bool,
        user_cursor: bool,
    }

    extern {
        pub fn main_c(argc: c_int, argv: *const *const c_char) -> c_int;

        pub fn sapp_isvalid() -> bool;
        pub fn sapp_width() -> c_int;
        pub fn sapp_height() -> c_int;
        pub fn sapp_high_dpi() -> bool;
        pub fn sapp_dpi_scale() -> f32;
        pub fn sapp_show_keyboard(visible: bool);
        pub fn sapp_keyboard_shown() -> bool;

        pub fn sapp_gles2() -> bool;

        pub fn sapp_metal_get_device() -> *const c_void;
        pub fn sapp_metal_get_renderpass_descriptor() -> *const c_void;
        pub fn sapp_metal_get_drawable() -> *const c_void;

        pub fn sapp_d3d11_get_device() -> *const c_void;
        pub fn sapp_d3d11_get_device_context() -> *const c_void;
        pub fn sapp_d3d11_get_render_target_view() -> *const c_void;
        pub fn sapp_d3d11_get_depth_stencil_view() -> *const c_void;

        pub fn sapp_set_user_ptr(ptr: *mut c_void);
        pub fn sapp_get_user_ptr() -> *mut c_void;
    }

    #[no_mangle]
    extern "C" fn sokol_main(_argc: c_int, _argv: *const *const c_char) -> Desc {
        let app = super::SappImpl::get();
        let desc = &app.desc;

        let window_title = CString::new(&*desc.window_title).unwrap();
        let canvas_name = CString::new(&*desc.html5_canvas_name).unwrap();

        Desc {
            init_cb,
            frame_cb,
            cleanup_cb,
            event_cb,
            fail_cb,
            width: desc.width,
            height: desc.height,
            sample_count: desc.sample_count,
            swap_interval: desc.swap_interval,
            high_dpi: desc.high_dpi,
            fullscreen: desc.fullscreen,
            alpha: desc.alpha,
            premultiplied_alpha: desc.premultiplied_alpha,
            preserve_drawing_buffer: desc.preserve_drawing_buffer,
            window_title: window_title.into_raw(),
            html5_canvas_name: canvas_name.into_raw(),
            html5_canvas_resize: desc.html5_canvas_resize,
            ios_keyboard_resizes_canvas: desc.ios_keyboard_resizes_canvas,
            gl_force_gles2: desc.gl_force_gles2,
            user_cursor: desc.user_cursor,
        }
    }

    #[no_mangle]
    extern fn init_cb() {
        super::SappImpl::get().init_cb();
    }

    #[no_mangle]
    extern fn frame_cb() {
        super::SappImpl::get().frame_cb();
    }

    #[no_mangle]
    extern fn cleanup_cb() {
        super::SappImpl::get().cleanup_cb();
    }

    #[no_mangle]
    extern fn event_cb(event: *const SappEvent) {
        let e = unsafe {
            &*event
        }.clone();

        super::SappImpl::get().event_cb(super::SappEvent {
            event_type: e.event_type,
            frame_count: e.frame_count,
            key_code: e.key_code,
            char_code: e.char_code,
            modifiers: e.modifiers,
            mouse_button: e.mouse_button,
            mouse_x: e.mouse_x,
            mouse_y: e.mouse_y,
            scroll_x: e.scroll_x,
            scroll_y: e.scroll_y,
            window_width: e.window_width,
            window_height: e.window_height,
            framebuffer_width: e.framebuffer_width,
            framebuffer_height: e.framebuffer_height,
        });
    }

    #[no_mangle]
    extern fn fail_cb(message: *const c_char) {
        let msg = unsafe {
            CStr::from_ptr(message)
        };

        super::SappImpl::get().fail_cb(msg.to_str().unwrap());
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum SappEventType {
    Invalid,
    KeyDown,
    KeyUp,
    Char,
    MouseDown,
    MouseUp,
    MouseScroll,
    MouseMove,
    MouseEnter,
    MouseLeave,
    TouchesBegan,
    TouchesMoved,
    TouchesEnded,
    TouchesCancelled,
    Resized,
    Iconified,
    Restored,
    Suspended,
    Resumed,
    UpdateCursor,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum SappMouseButton {
    Invalid = -1,
    Left = 0,
    Right = 1,
    Middle = 2,
}

pub struct SappEvent {
    pub event_type: SappEventType,
    pub frame_count: u32,
    pub key_code: u32,
    pub char_code: u32,
    pub modifiers: u32,
    pub mouse_button: SappMouseButton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    //pub num_touches: i32,
    //pub touches: [SappTouchPoint; SAPP_MAX_TOUCHPOINTS],
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
}

#[derive(Default)]
pub struct SappDesc {
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub premultiplied_alpha: bool,
    pub preserve_drawing_buffer: bool,
    pub window_title: String,
    pub html5_canvas_name: String,
    pub html5_canvas_resize: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
    pub user_cursor: bool,
}

pub trait SappCallbacks {
    fn sapp_init(&mut self);
    fn sapp_frame(&mut self);
    fn sapp_cleanup(&mut self);
    fn sapp_event(&mut self, event: SappEvent);

    fn sapp_fail(&mut self, msg: &str) {
        print!("{}", msg);
    }
}

struct SappImpl {
    callbacks: Box<SappCallbacks>,
    desc: SappDesc,
}

impl SappImpl {
    fn new<S: SappCallbacks + 'static>(callbacks: S, desc: SappDesc) -> SappImpl {
        SappImpl {
            callbacks: Box::new(callbacks),
            desc,
        }
    }

    pub fn init_cb(&mut self) {
        self.callbacks.sapp_init();
    }

    pub fn frame_cb(&mut self) {
        self.callbacks.sapp_frame();
    }

    pub fn cleanup_cb(&mut self) {
        self.callbacks.sapp_cleanup();
    }

    pub fn event_cb(&mut self, event: SappEvent) {
        self.callbacks.sapp_event(event);
    }

    pub fn fail_cb(&mut self, msg: &str) {
        self.callbacks.sapp_fail(msg);
    }

    pub fn get() -> &'static mut SappImpl {
        let app = unsafe {
            let app_ptr = ffi::sapp_get_user_ptr() as *mut SappImpl;
            &mut *app_ptr
        };

        app
    }
}

pub fn sapp_main<S: SappCallbacks + 'static>(callbacks: S,
                                             desc: SappDesc) -> i32 {
    let app = SappImpl::new(callbacks, desc);

    {
        let app_ptr = &app as *const SappImpl;
        unsafe {
            ffi::sapp_set_user_ptr(app_ptr as *mut c_void);
        }
    }

    // transform command line into (argc, argv) style

    let args: Vec<CString> = env::args().filter_map(|arg| {
        CString::new(arg).ok()
    }).collect();

    let c_args: Vec<*const c_char> = args.iter().map(|arg| {
        arg.as_ptr()
    }).collect();

    // copy desc, invoke native main() function

    unsafe {
        ffi::main_c(c_args.len() as c_int, c_args.as_ptr())
    }
}

pub fn sapp_isvalid() -> bool {
    unsafe {
        ffi::sapp_isvalid()
    }
}

pub fn sapp_width() -> i32 {
    unsafe {
        ffi::sapp_width()
    }
}

pub fn sapp_height() -> i32 {
    unsafe {
        ffi::sapp_height()
    }
}

pub fn sapp_high_dpi() -> bool {
    unsafe {
        ffi::sapp_high_dpi()
    }
}

pub fn sapp_dpi_scale() -> f32 {
    unsafe {
        ffi::sapp_dpi_scale()
    }
}

pub fn sapp_show_keyboard(visible: bool) {
    unsafe {
        ffi::sapp_show_keyboard(visible);
    }
}

pub fn sapp_keyboard_shown() -> bool {
    unsafe {
        ffi::sapp_keyboard_shown()
    }
}

pub fn sapp_gles2() -> bool {
    unsafe {
        ffi::sapp_gles2()
    }
}
