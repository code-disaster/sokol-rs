#define IMGUI_DISABLE_OBSOLETE_FUNCTIONS
#include <imgui.h>

extern "C" bool ig_begin_main_menu_bar() {
    return ImGui::BeginMainMenuBar();
}

extern "C" void ig_end_main_menu_bar() {
    ImGui::EndMainMenuBar();
}

extern "C" bool ig_begin_menu(const char* label, bool enabled) {
    return ImGui::BeginMenu(label, enabled);
}

extern "C" void ig_menu_item(const char* label, const char* shortcut, bool* p_selected, bool enabled) {
    ImGui::MenuItem(label, shortcut, p_selected, enabled);
}

extern "C" void ig_end_menu() {
    ImGui::EndMenu();
}

extern "C" void ig_show_demo_window(bool* p_open) {
    ImGui::ShowDemoWindow(p_open);
}
