use sokol::app::*;
use sokol::gfx::*;

struct Clear {}

impl SappCallbacks for Clear {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });
    }

    fn sapp_frame(&mut self) {
        let pass_action =
            SgPassAction::color(
                SgColorAttachmentAction::clear(
                    [0.5, 0.0, 0.25, 1.0]
                )
            );

        sg_begin_default_pass(&pass_action, sapp_width(), sapp_height());
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_shutdown();
    }
}

fn main() {
    let clear_app = Clear {};

    let title = format!("clear-sapp.rs ({:?})", sg_api());

    let exit_code = sapp_main(
        clear_app,
        SappDesc {
            width: 800,
            height: 600,
            window_title: title,
            ..Default::default()
        });

    std::process::exit(exit_code);
}
