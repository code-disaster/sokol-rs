//! sokol::audio - cross-platform audio-streaming API
//!
//! A Rust API to the [sokol_audio.h](https://github.com/floooh/sokol/blob/master/sokol_audio.h)
//! header-only C library.

pub mod ffi {
    use std::os::raw::c_int;
    use std::os::raw::c_void;
    use std::ptr::null;

    #[repr(C)]
    #[derive(Debug)]
    pub struct SAudioDesc {
        sample_rate: c_int,
        num_channels: c_int,
        buffer_frames: c_int,
        packet_frames: c_int,
        num_packets: c_int,
        stream_cb: *const c_void,
        stream_userdata_cb: Option<unsafe extern fn(*mut f32, c_int, c_int, *mut c_void)>,
        user_data: *mut c_void,
    }

    extern {
        pub fn saudio_setup(desc: *const SAudioDesc);
        pub fn saudio_shutdown();
        pub fn saudio_isvalid() -> bool;
        pub fn saudio_sample_rate() -> c_int;
        //pub fn saudio_buffer_size() -> c_int;
        pub fn saudio_channels() -> c_int;
        pub fn saudio_expect() -> c_int;
        pub fn saudio_push(frames: *const f32, num_frames: c_int) -> c_int;
    }

    pub fn saudio_make_desc(desc: super::SAudioDesc) -> SAudioDesc {
        let app_ptr = unsafe {
            super::super::app::ffi::sapp_get_userdata()
        };

        SAudioDesc {
            sample_rate: desc.sample_rate,
            num_channels: desc.num_channels,
            buffer_frames: desc.buffer_frames,
            packet_frames: desc.packet_frames,
            num_packets: desc.num_packets,
            stream_cb: null(),
            stream_userdata_cb: if desc.use_stream_cb {
                Some(super::super::app::ffi::stream_userdata_cb)
            } else {
                None
            },
            user_data: app_ptr,
        }
    }
}

#[derive(Default, Debug)]
pub struct SAudioDesc {
    pub sample_rate: i32,
    pub num_channels: i32,
    pub buffer_frames: i32,
    pub packet_frames: i32,
    pub num_packets: i32,
    pub use_stream_cb: bool,
}

pub fn saudio_setup(desc: SAudioDesc) {
    unsafe {
        ffi::saudio_setup(&ffi::saudio_make_desc(desc))
    }
}

pub fn saudio_shutdown() {
    unsafe {
        ffi::saudio_shutdown();
    }
}

pub fn saudio_isvalid() -> bool {
    unsafe {
        ffi::saudio_isvalid()
    }
}

pub fn saudio_sample_rate() -> i32 {
    unsafe {
        ffi::saudio_sample_rate()
    }
}

/*pub fn saudio_buffer_size() -> i32 {
    unsafe {
        ffi::saudio_buffer_size()
    }
}*/

pub fn saudio_channels() -> i32 {
    unsafe {
        ffi::saudio_channels()
    }
}

pub fn saudio_expect() -> i32 {
    unsafe {
        ffi::saudio_expect()
    }
}

pub fn saudio_push(frames: &[f32], num_frames: i32) -> i32 {
    unsafe {
        ffi::saudio_push(frames.as_ptr(), num_frames)
    }
}
