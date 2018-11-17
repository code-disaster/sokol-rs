extern crate imgui_sys;
extern crate sokol;
extern crate sokol_imgui;

use imgui_sys::igSetNextWindowPos;
use imgui_sys::igShowDemoWindow;
use imgui_sys::ImGuiCond;
use imgui_sys::ImVec2;

use sokol::app::*;
use sokol::gfx::*;
use sokol::time::*;
use sokol_imgui::app::*;
use sokol_imgui::gfx::*;

struct ImGuiDemo {
    pass_action: SgPassAction,
    ui: SAppImGui,
    ui_renderer: SgImGui,
    frame_time: u64,
}

impl SApp for ImGuiDemo {
    fn sapp_init(&mut self) {
        stm_setup();

        sg_setup(&SgDesc {
            ..Default::default()
        });

        self.ui = sapp_imgui_setup();
        self.ui_renderer = sg_imgui_setup(1 << 16);
    }

    fn sapp_frame(&mut self) {
        let laptime = stm_laptime(&mut self.frame_time);

        sapp_imgui_new_frame(&mut self.ui, stm_sec(laptime) as f32);
        self.show_demo_window();

        sg_begin_default_pass(&self.pass_action, sapp_width(), sapp_height());
        sg_imgui_draw(&self.ui_renderer);
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_imgui_shutdown(&self.ui_renderer);
        sg_shutdown();
    }

    fn sapp_event(&mut self, event: SAppEvent) {
        sapp_imgui_event(&mut self.ui, &event);
    }
}

impl ImGuiDemo {
    fn show_demo_window(&mut self) {
        unsafe {
            igSetNextWindowPos(
                ImVec2 { x: 60.0, y: 20.0 },
                ImGuiCond::FirstUseEver,
                ImVec2 { x: 0.0, y: 0.0 },
            );

            let mut show = true;
            igShowDemoWindow(&mut show);
        }
    }
}

fn main() {
    let app = ImGuiDemo {
        pass_action: SgPassAction {
            colors: vec!(
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.2, 0.2, 0.2, 1.0],
                }
            ),
            ..Default::default()
        },
        ui: Default::default(),
        ui_renderer: Default::default(),
        frame_time: 0,
    };

    let title = format!("imgui-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        app,
        SAppDesc {
            width: 1280,
            height: 960,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
