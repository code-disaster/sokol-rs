extern crate sokol;
extern crate sokol_imgui;

use sokol::app::*;
use sokol::gfx::*;
use sokol::time::*;
use sokol_imgui::imgui::*;

struct ImGuiDemo {
    pass_action: SgPassAction,
    frame_time: u64,
}

impl SApp for ImGuiDemo {
    fn sapp_init(&mut self) {
        stm_setup();

        sg_setup(&SgDesc {
            ..Default::default()
        });

        simgui_setup(SImGuiDesc {
            ..Default::default()
        });
    }

    fn sapp_frame(&mut self) {
        let laptime = stm_laptime(&mut self.frame_time);

        simgui_new_frame(sapp_width(), sapp_height(), stm_sec(laptime));
        self.show_demo_window();

        sg_begin_default_pass(&self.pass_action, sapp_width(), sapp_height());
        simgui_render();
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        simgui_shutdown();
        sg_shutdown();
    }

    fn sapp_event(&mut self, event: SAppEvent) {
        let _handled = simgui_handle_event(&event);
        // here, application would use return value to figure out
        // which events to continue processing
    }
}

impl ImGuiDemo {
    fn show_demo_window(&mut self) {
        /*unsafe {
            igSetNextWindowPos(
                ImVec2 { x: 60.0, y: 20.0 },
                ImGuiCond::FirstUseEver,
                ImVec2 { x: 0.0, y: 0.0 },
            );
        }*/
        let mut show = true;
        simgui_show_demo_window(&mut show);
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
        frame_time: 0,
    };

    let title = format!("imgui-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_run(
        app,
        SAppDesc {
            width: 1280,
            height: 960,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
