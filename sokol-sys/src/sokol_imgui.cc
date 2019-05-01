#include <sokol_app.h>
#include <sokol_gfx.h>

#define IMGUI_DISABLE_OBSOLETE_FUNCTIONS
#include <imgui.h>

#define SOKOL_IMGUI_IMPL
#include <sokol_imgui.h>

#define SOKOL_GFX_IMGUI_IMPL
#include <sokol_gfx_imgui.h>

extern "C" void simgui_show_demo_window(bool* p_open) {
    ImGui::ShowDemoWindow(p_open);
}

static sg_imgui_t sg_imgui_ctx;

extern "C" void sg_imgui_wrap_init() {
    sg_imgui_init(&sg_imgui_ctx);
}

extern "C" void sg_imgui_wrap_discard() {
    sg_imgui_discard(&sg_imgui_ctx);
}

extern "C" void sg_imgui_wrap_draw(bool* ctx) {
    sg_imgui_ctx.buffers.open = ctx[0];
    sg_imgui_ctx.images.open = ctx[1];
    sg_imgui_ctx.shaders.open = ctx[2];
    sg_imgui_ctx.pipelines.open = ctx[3];
    sg_imgui_ctx.passes.open = ctx[4];
    sg_imgui_ctx.capture.open = ctx[5];

    sg_imgui_draw(&sg_imgui_ctx);
}
