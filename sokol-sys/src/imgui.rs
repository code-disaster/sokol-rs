pub mod ffi {
    use std::os::raw::c_char;

    extern {
        pub fn ig_begin_main_menu_bar() -> bool;
        pub fn ig_end_main_menu_bar();

        pub fn ig_begin_menu(label: *const c_char, enabled: bool) -> bool;
        pub fn ig_menu_item(label: *const c_char, shortcut: *const c_char, p_selected: *mut bool, enabled: bool);
        pub fn ig_end_menu();

        pub fn ig_show_demo_window(is_open: *mut bool);
    }
}
