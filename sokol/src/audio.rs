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
        stream_cb: Option<extern fn(*mut f32, c_int, c_int)>,
    }

    impl SAudioDesc {
        pub fn make(desc: &super::SAudioDesc) -> SAudioDesc {
            SAudioDesc {
                sample_rate: desc.sample_rate,
                num_channels: desc.num_channels,
                buffer_frames: desc.buffer_frames,
                packet_frames: desc.packet_frames,
                num_packets: desc.num_packets,
                stream_cb: None,
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
    }

    /*#[no_mangle]
    extern fn stream_cb(buffer: *mut f32, num_frames: c_int, num_channels: c_int) {
        let arr = unsafe {
            let len = num_frames * num_channels;
            from_raw_parts_mut(buffer, len as usize)
        };

        super::SAudioImpl::get().stream_cb(arr, num_frames, num_channels);
    }*/
}

#[derive(Default)]
pub struct SAudioDesc {
    pub sample_rate: i32,
    pub num_channels: i32,
    pub buffer_frames: i32,
    pub packet_frames: i32,
    pub num_packets: i32,
}

pub trait SAudioCallbacks {
    fn saudio_stream(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32);
}

/*struct SAudioImpl {
    callbacks: Box<&'static mut SAudioCallbacks>,
}

impl SAudioImpl {
    fn new<S: SAudioCallbacks + 'static>(callbacks: &'static mut S) -> SAudioImpl {
        let x: Box<&'static mut SAudioCallbacks> = Box::new(callbacks);
        SAudioImpl {
            callbacks: x,
        }
    }

    pub fn stream_cb(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32) {
        self.callbacks.saudio_stream(buffer, num_frames, num_channels);
    }

    pub fn get() -> &'static mut SAudioImpl {
        let audio = unsafe {
            let audio_ptr = ffi::saudio_get_user_ptr() as *mut SAudioImpl;
            &mut *audio_ptr
        };

        audio
    }
}*/

pub fn saudio_setup/*<S: SAudioCallbacks + 'static>(callbacks: &'static mut S,*/(desc: &SAudioDesc) {
    /*match callbacks {
        Some(callbacks) => unsafe {
        let audio = SAudioImpl::new(callbacks);
        let audio_ptr = &audio as *const SAudioImpl;
        ffi::saudio_set_user_ptr(audio_ptr as *mut c_void);
        mem::forget(audio);
    },
        None => unsafe {
            ffi::saudio_set_user_ptr(ptr::null_mut());
        },
    }*/

    unsafe {
        ffi::saudio_setup(&ffi::SAudioDesc::make(desc));
    }
}

pub fn saudio_shutdown() {
    unsafe {
        ffi::saudio_shutdown();
    }

    /*unsafe {
        let audio_ptr = ffi::saudio_get_user_ptr();
        if audio_ptr == ptr::null_mut() {
            let _audio = SAudioImpl::get();
        }
    }*/
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
