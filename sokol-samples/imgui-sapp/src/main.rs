#[macro_use]
extern crate sokol;
extern crate sokol_imgui;

use sokol::app::*;
use sokol::gfx::*;
use sokol::time::*;
use sokol_imgui::gfx::*;
use sokol_imgui::imgui::*;

struct ImGuiDemo {
    pass_action: SgPassAction,
    frame_time: u64,
    sg_imgui_ctx: SgImGui,
    imgui_demo_window: bool,
    enable_sg_imgui_menu: bool,
}

impl SApp for ImGuiDemo {
    fn sapp_init(&mut self) {
        stm_setup();

        sg_setup(&SgDesc {
            ..Default::default()
        });

        sg_imgui_init(&mut self.sg_imgui_ctx);
        self.sg_imgui_ctx.capture = true;

        simgui_setup(SImGuiDesc {
            ..Default::default()
        });
    }

    fn sapp_frame(&mut self) {
        let laptime = stm_laptime(&mut self.frame_time);

        simgui_new_frame(sapp_width(), sapp_height(), stm_sec(laptime));

        if imgui_begin_main_menu_bar() {
            if imgui_begin_menu(cstr!("demo"), true) {
                imgui_menu_item(cstr!("ImGui Demo Window"), None, &mut self.imgui_demo_window, true);
                imgui_menu_item(cstr!("Enable sokol-gfx menu"), Some(cstr!("CTRL+D")), &mut self.enable_sg_imgui_menu, true);
                imgui_end_menu()
            }
            if imgui_begin_menu(cstr!("sokol-gfx"), self.enable_sg_imgui_menu) {
                imgui_menu_item(cstr!("Buffers"), None, &mut self.sg_imgui_ctx.buffers, true);
                imgui_menu_item(cstr!("Images"), None, &mut self.sg_imgui_ctx.images, true);
                imgui_menu_item(cstr!("Shader"), None, &mut self.sg_imgui_ctx.shaders, true);
                imgui_menu_item(cstr!("Pipelines"), None, &mut self.sg_imgui_ctx.pipelines, true);
                imgui_menu_item(cstr!("Passes"), None, &mut self.sg_imgui_ctx.passes, true);
                imgui_menu_item(cstr!("Capture"), None, &mut self.sg_imgui_ctx.capture, true);
                imgui_end_menu();
            }
            imgui_end_main_menu_bar();
        }

        self.show_demo_window();
        sg_imgui_draw(&mut self.sg_imgui_ctx);

        sg_begin_default_pass(&self.pass_action, sapp_width(), sapp_height());
        simgui_render();
        sg_end_pass();
        sg_commit();
    }

    fn sapp_cleanup(&mut self) {
        simgui_shutdown();
        sg_imgui_discard(&mut self.sg_imgui_ctx);
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
        if self.imgui_demo_window {
            imgui_show_demo_window(&mut self.imgui_demo_window);
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
        frame_time: 0,
        sg_imgui_ctx: SgImGui::new(),
        imgui_demo_window: true,
        enable_sg_imgui_menu: true,
    };

    let title = format!("imgui-sapp.rs ({:?})", sg_query_backend());

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
