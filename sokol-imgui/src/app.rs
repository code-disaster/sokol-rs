use std::ptr;

use imgui_sys::*;

use sokol::app::*;

pub fn sapp_imgui_setup() {
    unsafe {
        igCreateContext(None, None);

        let io = &mut *igGetIO();

        ImFontAtlas_AddFontDefault(io.fonts, ptr::null());

        io.key_map[ImGuiKey::Tab as usize] = SAppKeycode::KeyTab as i32;
        io.key_map[ImGuiKey::LeftArrow as usize] = SAppKeycode::KeyLeft as i32;
        io.key_map[ImGuiKey::RightArrow as usize] = SAppKeycode::KeyRight as i32;
        io.key_map[ImGuiKey::UpArrow as usize] = SAppKeycode::KeyUp as i32;
        io.key_map[ImGuiKey::DownArrow as usize] = SAppKeycode::KeyDown as i32;
        io.key_map[ImGuiKey::PageUp as usize] = SAppKeycode::KeyPageUp as i32;
        io.key_map[ImGuiKey::PageDown as usize] = SAppKeycode::KeyPageDown as i32;
        io.key_map[ImGuiKey::Home as usize] = SAppKeycode::KeyHome as i32;
        io.key_map[ImGuiKey::End as usize] = SAppKeycode::KeyEnd as i32;
        io.key_map[ImGuiKey::Delete as usize] = SAppKeycode::KeyDelete as i32;
        io.key_map[ImGuiKey::Backspace as usize] = SAppKeycode::KeyBackspace as i32;
        //io.key_map[ImGuiKey::Space as usize] = SAppKeycode::KeySpace as i32;
        io.key_map[ImGuiKey::Enter as usize] = SAppKeycode::KeyEnter as i32;
        io.key_map[ImGuiKey::Escape as usize] = SAppKeycode::KeyEscape as i32;
        io.key_map[ImGuiKey::A as usize] = SAppKeycode::KeyA as i32;
        io.key_map[ImGuiKey::C as usize] = SAppKeycode::KeyC as i32;
        io.key_map[ImGuiKey::V as usize] = SAppKeycode::KeyV as i32;
        io.key_map[ImGuiKey::X as usize] = SAppKeycode::KeyX as i32;
        io.key_map[ImGuiKey::Y as usize] = SAppKeycode::KeyY as i32;
        io.key_map[ImGuiKey::Z as usize] = SAppKeycode::KeyZ as i32;

        io.ini_filename = ptr::null();
    }
}

pub fn sapp_imgui_event(event: &SAppEvent) {
    let io = unsafe { &mut *igGetIO() };

    match event.event_type {
        SAppEventType::MouseDown => {
            io.mouse_pos.x = event.mouse_x;
            io.mouse_pos.y = event.mouse_y;
            io.mouse_down[event.mouse_button as usize] = true;
        }
        SAppEventType::MouseUp => {
            io.mouse_pos.x = event.mouse_x;
            io.mouse_pos.y = event.mouse_y;
            io.mouse_down[event.mouse_button as usize] = false;
        }
        SAppEventType::MouseMove => {
            io.mouse_pos.x = event.mouse_x;
            io.mouse_pos.y = event.mouse_y;
        }
        SAppEventType::MouseEnter | SAppEventType::MouseLeave => {
            for btn in 0..3 {
                io.mouse_down[btn] = false;
            }
        }
        SAppEventType::MouseScroll => {
            io.mouse_wheel = event.scroll_y;
        }
        SAppEventType::KeyDown => {
            io.keys_down[event.key_code as usize] = true;
        }
        SAppEventType::KeyUp => {
            io.keys_down[event.key_code as usize] = false;
        }
        SAppEventType::Char => {
            unsafe {
                ImGuiIO_AddInputCharacter(event.char_code as ImWchar);
            }
        }
        _ => {}
    }
}
