extern crate sokol;
extern crate sokol_imgui;

use sokol::app::*;
use sokol::gfx::*;
use sokol_imgui::*;

struct Clear {
    pass_action: SgPassAction,
    ui_renderer: ImGuiRenderer,
}

impl SApp for Clear {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });

        imgui_create_context();
        self.ui_renderer = imgui_setup();
    }

    fn sapp_frame(&mut self) {
        sg_begin_default_pass(&self.pass_action, sapp_width(), sapp_height());

        imgui_new_frame();
        imgui_draw(&self.ui_renderer);

        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        imgui_shutdown(&self.ui_renderer);
        sg_shutdown();
    }

    fn sapp_event(&mut self, event: SAppEvent) {
        imgui_consume_event(&event);
    }
}

fn main() {
    let clear_app = Clear {
        pass_action: SgPassAction {
            colors: vec!(
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.2, 0.2, 0.2, 1.0],
                }
            ),
            ..Default::default()
        },
        ui_renderer: Default::default(),
    };

    let title = format!("clear-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        clear_app,
        SAppDesc {
            width: 800,
            height: 600,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
