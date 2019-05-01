#include <sokol_app.h>
#include <sokol_gfx.h>

#include <imgui.h>

#define SOKOL_IMGUI_IMPL
#include <sokol_imgui.h>

extern "C" void simgui_show_demo_window(bool* p_open) {
    ImGui::ShowDemoWindow(p_open);
}
