mod ffi {
    extern {
        pub fn stm_setup();
        pub fn stm_now() -> u64;
        pub fn stm_diff(new: u64, old: u64) -> u64;
        pub fn stm_since(start: u64) -> u64;
        pub fn stm_laptime(last_time: *mut u64) -> u64;
        pub fn stm_sec(ticks: u64) -> f64;
        pub fn stm_ms(ticks: u64) -> f64;
        pub fn stm_us(ticks: u64) -> f64;
        pub fn stm_ns(ticks: u64) -> f64;
    }
}

pub fn stm_setup() {
    unsafe {
        ffi::stm_setup();
    }
}

pub fn stm_now() -> u64 {
    unsafe {
        ffi::stm_now()
    }
}

pub fn stm_diff(new: u64, old: u64) -> u64 {
    unsafe {
        ffi::stm_diff(new, old)
    }
}

pub fn stm_since(start: u64) -> u64 {
    unsafe {
        ffi::stm_since(start)
    }
}

pub fn stm_laptime(last_time: *mut u64) -> u64 {
    unsafe {
        ffi::stm_laptime(last_time)
    }
}

pub fn stm_sec(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_sec(ticks)
    }
}

pub fn stm_ms(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_ms(ticks)
    }
}

pub fn stm_us(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_us(ticks)
    }
}

pub fn stm_ns(ticks: u64) -> f64 {
    unsafe {
        ffi::stm_ns(ticks)
    }
}
