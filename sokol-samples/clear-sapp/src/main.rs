extern crate sokol;

use sokol::app::*;
use sokol::gfx::*;

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
        let g = self.pass_action.colors[0].val[1] + 0.01;
        self.pass_action.colors[0].val[1] = if g > 1.0 { 0.0 } else { g };
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
                    val: [1.0, 0.0, 0.0, 1.0],
                }
            ),
            ..Default::default()
        }
    };

    let title = format!("clear-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_run(
        clear_app,
        SAppDesc {
            width: 400,
            height: 300,
            gl_force_gles2: true,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
