/*!

Native bindings to the [sokol](https://github.com/floooh/sokol) header-only,
cross-platform C libraries.

# Example

This is a minimal example of using sokol::app and sokol::gfx to create a window, then clear
its content each frame with a solid color.

~~~
struct ExampleApp {
    pass_action: SgPassAction,
}

impl SappCallbacks for ExampleApp {
    fn sapp_init(&mut self) {
        sg_setup(&SgDesc {
            ..Default::default()
        });
    }

    fn sapp_frame(&mut self) {
        sg_begin_default_pass(&pass_action, sapp_width(), sapp_height());
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        sg_shutdown();
    }
}
~~~

~~~
fn main() {
    let app = ExampleApp {
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

    sapp_main(app, SappDesc {
        window_title: "Example".to_string(),
        ..Default::default()
    });
}
~~~
*/

#[macro_use]
extern crate bitflags;

pub mod app;
pub mod audio;
pub mod gfx;
pub mod time;
