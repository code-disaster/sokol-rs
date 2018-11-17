//! sokol_imgui::app - Dear ImGui update & input handler using the sokol::app API

use std::ptr;

use imgui_sys::*;

use sokol::app::*;

#[derive(Default)]
pub struct SAppImGui {
    button_down: [bool; ffi::SAPP_MAX_MOUSEBUTTONS],
    button_up: [bool; ffi::SAPP_MAX_MOUSEBUTTONS],
}

pub fn sapp_imgui_setup() -> SAppImGui {
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

    SAppImGui {
        ..Default::default()
    }
}

pub fn sapp_imgui_new_frame(ui: &mut SAppImGui, dt: f32) {
    unsafe {
        let io = &mut *igGetIO();

        io.display_size.x = sapp_width() as f32;
        io.display_size.y = sapp_height() as f32;

        io.delta_time = dt;

        for idx in 0..ffi::SAPP_MAX_MOUSEBUTTONS {
            if ui.button_down[idx] {
                ui.button_down[idx] = false;
                io.mouse_down[idx] = true;
            } else if ui.button_up[idx] {
                ui.button_up[idx] = false;
                io.mouse_down[idx] = false;
            }
        }

        igNewFrame();
    }
}

pub fn sapp_imgui_event(ui: &mut SAppImGui, event: &SAppEvent) {
    let io = unsafe { &mut *igGetIO() };

    match event.event_type {
        SAppEventType::MouseDown => {
            io.mouse_pos.x = event.mouse_x;
            io.mouse_pos.y = event.mouse_y;
            ui.button_down[event.mouse_button as usize] = true;
        }
        SAppEventType::MouseUp => {
            io.mouse_pos.x = event.mouse_x;
            io.mouse_pos.y = event.mouse_y;
            ui.button_up[event.mouse_button as usize] = true;
        }
        SAppEventType::MouseMove => {
            io.mouse_pos.x = event.mouse_x;
            io.mouse_pos.y = event.mouse_y;
        }
        SAppEventType::MouseEnter | SAppEventType::MouseLeave => {
            for btn in 0..3 {
                ui.button_down[btn] = false;
                ui.button_up[btn] = false;
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
