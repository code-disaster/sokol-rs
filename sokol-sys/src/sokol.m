#define SOKOL_IMPL
#define SOKOL_NO_ENTRY
#define SOKOL_NO_DEPRECATED
#include <sokol_app.h>
#include <sokol_audio.h>
#include <sokol_gfx.h>
#include <sokol_time.h>

SOKOL_API_DECL void* sapp_get_userdata(void) {
    return _sapp.desc.user_data;
}
