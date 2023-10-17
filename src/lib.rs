use std::{f32::consts, ffi::c_void};

use libobs::libobs_sys::obs_icon_type_OBS_ICON_TYPE_TEXT;
use log::debug;

const MODULE_NAME: &std::ffi::CStr =
    unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"Rusty Studio\0") };

static mut OBS_MODULE_POINTER: *mut libobs::libobs_sys::obs_module_t = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn obs_module_set_pointer(module: *mut libobs::libobs_sys::obs_module_t) {
    unsafe {
        OBS_MODULE_POINTER = module;
    }
}

#[no_mangle]
pub extern "C" fn obs_current_module() -> *mut libobs::libobs_sys::obs_module_t {
    return unsafe { OBS_MODULE_POINTER };
}

#[no_mangle]
pub extern "C" fn obs_module_ver() -> u32 {
    return (libobs::libobs_sys::LIBOBS_API_MAJOR_VER << 24)
        | (libobs::libobs_sys::LIBOBS_API_MINOR_VER << 16)
        | libobs::libobs_sys::LIBOBS_API_PATCH_VER;
}

#[no_mangle]
pub extern "C" fn obs_module_load() -> bool {
    libobs::log::init();
    log::info!("Hello from rusty studio !");

    let source = libobs::libobs_sys::obs_source_info {
        id: "rusty_studio_chat\0".as_ptr() as *const i8,
        type_: libobs::libobs_sys::obs_source_type_OBS_SOURCE_TYPE_INPUT,
        output_flags: libobs::libobs_sys::OBS_SOURCE_VIDEO,
        get_name: Some(source_get_name),
        create: Some(source_create),
        destroy: Some(source_destroy),
        get_width: Some(source_get_width),
        get_height: Some(source_get_height),
        get_defaults: None,
        get_defaults2: None,
        get_properties: None,
        get_properties2: None,
        update: None,
        activate: None,
        deactivate: None,
        show: None,
        hide: None,
        video_tick: None,
        video_render: Some(source_video_render),
        filter_video: None,
        audio_render: None,
        enum_active_sources: None,
        save: None,
        load: None,
        mouse_click: None,
        mouse_move: None,
        mouse_wheel: None,
        focus: None,
        key_click: None,
        filter_remove: None,
        type_data: std::ptr::null_mut() as *mut c_void,
        free_type_data: None,
        enum_all_sources: None,
        transition_start: None,
        transition_stop: None,
        icon_type: obs_icon_type_OBS_ICON_TYPE_TEXT,
        media_play_pause: None,
        media_restart: None,
        media_stop: None,
        media_next: None,
        media_previous: None,
        media_get_duration: None,
        media_get_time: None,
        media_set_time: None,
        media_get_state: None,
        missing_files: None,
        video_get_color_space: None,
        filter_audio: None,
        audio_mix: None,
        version: 1,
        unversioned_id: std::ptr::null(),
    };
    
    unsafe { libobs::libobs_sys::obs_register_source_s(&source, std::mem::size_of_val(&source)) };

    true
}

#[no_mangle]
pub extern "C" fn obs_module_unload() {}

#[no_mangle]
pub extern "C" fn obs_module_name() -> *const std::os::raw::c_char {
    MODULE_NAME.as_ptr()
}

struct Source {}

impl Source {
    fn new() -> Self {
        Self {}
    }

    fn video_render(&mut self) {

    }

    fn width(&mut self) -> u32 {
        0
    }

    fn height(&mut self) -> u32 {
        0
    }
}

extern "C" fn source_get_name(
    type_data: *mut ::std::os::raw::c_void,
) -> *const ::std::os::raw::c_char {
    debug!("source get name");
    "Rusty Studio : Chat\0".as_ptr() as *const i8
}

unsafe extern "C" fn source_create(
    settings: *mut libobs::libobs_sys::obs_data_t,
    source: *mut libobs::libobs_sys::obs_source_t,
) -> *mut ::std::os::raw::c_void {
    debug!("source create");
    let source = Source::new();

    Box::leak(Box::from(source)) as *mut Source as *mut ::std::os::raw::c_void
}

unsafe extern "C" fn source_destroy(data: *mut ::std::os::raw::c_void) {
    debug!("source destroy");
    let b = Box::from_raw(data as *mut Source);
    drop(b)
}

unsafe extern "C" fn source_video_render(data: *mut ::std::os::raw::c_void, _: *mut libobs::libobs_sys::gs_effect_t) {
    debug!("source video render");
    let source = &mut *(data as *mut Source);
    source.video_render();
}


unsafe extern "C" fn source_get_width(data: *mut ::std::os::raw::c_void) -> u32 {
    debug!("source get width");
    let source = &mut *(data as *mut Source);
    source.width()
}

unsafe extern "C" fn source_get_height(data: *mut ::std::os::raw::c_void) -> u32 {
    debug!("source get height");
    let source = &mut *(data as *mut Source);
    source.height()
}