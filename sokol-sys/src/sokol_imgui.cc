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

typedef struct {
    sg_imgui_t* content;
    bool buffers;
    bool images;
    bool shaders;
    bool pipelines;
    bool passes;
    bool capture;
} sg_imgui_wrap_t;

extern "C" void sg_imgui_wrap_init(sg_imgui_wrap_t* ctx) {
    ctx->content = (sg_imgui_t*) _sg_imgui_alloc(sizeof(sg_imgui_t));
    sg_imgui_init(ctx->content);
}

extern "C" void sg_imgui_wrap_discard(sg_imgui_wrap_t* ctx) {
    sg_imgui_discard(ctx->content);
    _sg_imgui_free(ctx->content);
}

extern "C" void sg_imgui_wrap_draw(sg_imgui_wrap_t* ctx) {
    ctx->content->buffers.open = ctx->buffers;
    ctx->content->images.open = ctx->images;
    ctx->content->shaders.open = ctx->shaders;
    ctx->content->pipelines.open = ctx->pipelines;
    ctx->content->passes.open = ctx->passes;
    ctx->content->capture.open = ctx->capture;
    sg_imgui_draw(ctx->content);
}
