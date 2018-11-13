#define main main_c

#define SOKOL_IMPL
#include "sokol/sokol_app.h"
#include "sokol/sokol_audio.h"
#include "sokol/sokol_gfx.h"
#include "sokol/sokol_time.h"

static void* _sapp_user_ptr = NULL;
static void* _saudio_user_ptr = NULL;

SOKOL_API_DECL void sapp_set_user_ptr(void* ptr) {
    _sapp_user_ptr = ptr;
}

SOKOL_API_DECL void* sapp_get_user_ptr(void) {
    return _sapp_user_ptr;
}

SOKOL_API_DECL void saudio_set_user_ptr(void* ptr) {
    _saudio_user_ptr = ptr;
}

SOKOL_API_DECL void* saudio_get_user_ptr(void) {
    return _saudio_user_ptr;
}
