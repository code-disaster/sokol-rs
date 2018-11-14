extern crate sokol;

use sokol::app::SApp;
use sokol::app::sapp_height;
use sokol::app::sapp_main;
use sokol::app::sapp_width;
use sokol::app::SAppDesc;
use sokol::app::SAppEvent;
use sokol::gfx::sg_api;
use sokol::gfx::sg_begin_default_pass;
use sokol::gfx::sg_commit;
use sokol::gfx::sg_end_pass;
use sokol::gfx::sg_setup;
use sokol::gfx::sg_shutdown;
use sokol::gfx::SgAction;
use sokol::gfx::SgColorAttachmentAction;
use sokol::gfx::SgDesc;
use sokol::gfx::SgPassAction;

struct Clear {
    pass_action: SgPassAction,
}

impl SApp for Clear {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });
    }

    fn sapp_frame(&mut self) {
        sg_begin_default_pass(&self.pass_action, sapp_width(), sapp_height());
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_shutdown();
    }

    fn sapp_event(&mut self, _event: SAppEvent) {}
}

fn main() {
    let clear_app = Clear {
        pass_action: SgPassAction {
            colors: vec!(
                SgColorAttachmentAction {
                    action: SgAction::Clear,
                    val: [0.5, 0.0, 0.25, 1.0],
                }
            ),
            ..Default::default()
        }
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
