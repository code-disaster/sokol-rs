pub mod ffi {
    use std::os::raw::c_int;
    use std::os::raw::c_void;

    #[repr(C)]
    pub struct SAudioDesc {
        sample_rate: c_int,
        num_channels: c_int,
        buffer_frames: c_int,
        packet_frames: c_int,
        num_packets: c_int,
        stream_cb: Option<unsafe extern fn(*mut f32, c_int, c_int)>,
    }

    impl SAudioDesc {
        pub fn make(desc: &super::SAudioDesc) -> SAudioDesc {
            SAudioDesc {
                sample_rate: desc.sample_rate,
                num_channels: desc.num_channels,
                buffer_frames: desc.buffer_frames,
                packet_frames: desc.packet_frames,
                num_packets: desc.num_packets,
                stream_cb: if desc.use_stream_cb { Some(stream_cb) } else { None },
            }
        }
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

        pub fn saudio_set_user_ptr(ptr: *mut c_void);
        pub fn saudio_get_user_ptr() -> *mut c_void;

        fn stream_cb(buffer: *mut f32, num_frames: c_int, num_channels: c_int);
    }
}

#[derive(Default)]
pub struct SAudioDesc {
    pub sample_rate: i32,
    pub num_channels: i32,
    pub buffer_frames: i32,
    pub packet_frames: i32,
    pub num_packets: i32,
    pub use_stream_cb: bool,
}

pub fn saudio_setup(desc: &SAudioDesc) {
    unsafe {
        ffi::saudio_setup(&ffi::SAudioDesc::make(desc));
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
