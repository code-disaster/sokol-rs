#define SOKOL_IMPL
#define SOKOL_NO_ENTRY
#include "sokol/sokol_app.h"
#include "sokol/sokol_audio.h"
#include "sokol/sokol_gfx.h"
#include "sokol/sokol_time.h"

SOKOL_API_DECL void* sapp_get_userdata(void) {
    return _sapp.desc.user_data;
}
