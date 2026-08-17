#![allow(non_camel_case_types)]

use libc::FILE;
use std::os::raw::{c_char, c_int, c_uint, c_void};

pub enum xkb_context {}

pub enum xkb_keymap {}

pub enum xkb_state {}

pub type xkb_keycode_t = u32;

pub type xkb_keysym_t = u32;

pub type xkb_layout_index_t = u32;

pub type xkb_layout_mask_t = u32;

pub type xkb_level_index_t = u32;

pub type xkb_mod_index_t = u32;

pub type xkb_mod_mask_t = u32;

pub type xkb_led_index_t = u32;

pub type xkb_led_mask_t = u32;

pub const XKB_KEYCODE_INVALID: u32 = 0xffff_ffff;
pub const XKB_LAYOUT_INVALID: u32 = 0xffff_ffff;
pub const XKB_LEVEL_INVALID: u32 = 0xffff_ffff;
pub const XKB_MOD_INVALID: u32 = 0xffff_ffff;
pub const XKB_LED_INVALID: u32 = 0xffff_ffff;

pub const XKB_KEYCODE_MAX: u32 = 0xffff_fffe;

#[must_use]
pub fn xkb_keycode_is_legal_ext(key: u32) -> bool {
    key <= XKB_KEYCODE_MAX
}

#[must_use]
pub fn xkb_keycode_is_legal_x11(key: u32) -> bool {
    (8..=255).contains(&key)
}

#[repr(C)]
pub struct xkb_rule_names {
    pub rules: *const c_char,
    pub model: *const c_char,
    pub layout: *const c_char,
    pub variant: *const c_char,
    pub options: *const c_char,
}

pub type xkb_keysym_flags = u32;
pub const XKB_KEYSYM_NO_FLAGS: u32 = 0;
pub const XKB_KEYSYM_CASE_INSENSITIVE: u32 = 1 << 0;

pub type xkb_context_flags = u32;
pub const XKB_CONTEXT_NO_FLAGS: u32 = 0;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: u32 = 1 << 0;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: u32 = 1 << 1;

#[repr(C)]
pub enum xkb_log_level {
    CRITICAL = 10,
    ERROR = 20,
    WARNING = 30,
    INFO = 40,
    DEBUG = 50,
}

pub type xkb_keymap_compile_flags = u32;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: u32 = 0;

pub type xkb_keymap_format = u32;
pub const XKB_KEYMAP_FORMAT_TEXT_V1: u32 = 1;
pub const XKB_KEYMAP_FORMAT_USE_ORIGINAL: u32 = 0xffff_ffff;

#[repr(C)]
pub enum xkb_key_direction {
    UP,
    DOWN,
}

pub type xkb_state_component = u32;
pub const XKB_STATE_MODS_DEPRESSED: u32 = 1 << 0;
pub const XKB_STATE_MODS_LATCHED: u32 = 1 << 1;
pub const XKB_STATE_MODS_LOCKED: u32 = 1 << 2;
pub const XKB_STATE_MODS_EFFECTIVE: u32 = 1 << 3;
pub const XKB_STATE_LAYOUT_DEPRESSED: u32 = 1 << 4;
pub const XKB_STATE_LAYOUT_LATCHED: u32 = 1 << 5;
pub const XKB_STATE_LAYOUT_LOCKED: u32 = 1 << 6;
pub const XKB_STATE_LAYOUT_EFFECTIVE: u32 = 1 << 7;
pub const XKB_STATE_LEDS: u32 = 1 << 8;

pub type xkb_state_match = u32;
pub const XKB_STATE_MATCH_ANY: u32 = 1 << 0;
pub const XKB_STATE_MATCH_ALL: u32 = 1 << 1;
pub const XKB_STATE_MATCH_NON_EXCLUSIVE: u32 = 1 << 16;

pub type xkb_log_fn_t = unsafe extern "C" fn(
    context: *mut xkb_context,
    level: xkb_log_level,
    format: *const c_char,
    ...
);

pub type xkb_keymap_key_iter_t =
    unsafe extern "C" fn(keymap: *mut xkb_keymap, key: xkb_keycode_t, data: *mut c_void);

#[link(name = "xkbcommon")]
extern "C" {

    pub fn xkb_keysym_get_name(keysym: xkb_keysym_t, buffer: *mut c_char, size: usize) -> c_int;

    pub fn xkb_keysym_from_name(name: *const c_char, flags: xkb_keysym_flags) -> xkb_keysym_t;

    pub fn xkb_keysym_to_utf8(keysym: xkb_keysym_t, buffer: *mut c_char, size: usize) -> c_int;

    pub fn xkb_keysym_to_utf32(keysym: xkb_keysym_t) -> u32;

    pub fn xkb_utf32_to_keysym(ucs: u32) -> xkb_keysym_t;

    pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;

    pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;

    pub fn xkb_context_unref(context: *mut xkb_context);

    pub fn xkb_context_set_user_data(context: *mut xkb_context, user_data: *mut c_void);

    pub fn xkb_context_get_user_data(context: *mut xkb_context) -> *mut c_void;

    pub fn xkb_context_include_path_append(context: *mut xkb_context, path: *const c_char)
        -> c_int;

    pub fn xkb_context_include_path_append_default(context: *mut xkb_context) -> c_int;

    pub fn xkb_context_include_path_reset_defaults(context: *mut xkb_context) -> c_int;

    pub fn xkb_context_include_path_clear(context: *mut xkb_context);

    pub fn xkb_context_num_include_paths(context: *mut xkb_context) -> c_uint;

    pub fn xkb_context_include_path_get(context: *mut xkb_context, index: c_uint) -> *const c_char;

    pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);

    pub fn xkb_context_get_log_level(context: *mut xkb_context) -> xkb_log_level;

    pub fn xkb_context_set_log_verbosity(context: *mut xkb_context, verbosity: c_int);

    pub fn xkb_context_get_log_verbosity(context: *mut xkb_context) -> c_int;

    pub fn xkb_context_set_log_fn(context: *mut xkb_context, log_fn: xkb_log_fn_t);

    pub fn xkb_keymap_new_from_names(
        context: *mut xkb_context,
        names: *const xkb_rule_names,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;

    pub fn xkb_keymap_new_from_file(
        context: *mut xkb_context,
        file: *mut FILE,
        format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;

    pub fn xkb_keymap_new_from_string(
        context: *mut xkb_context,
        s: *const c_char,
        format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;

    pub fn xkb_keymap_new_from_buffer(
        context: *mut xkb_context,
        buffer: *const c_char,
        length: usize,
        format: xkb_keymap_format,
        flags: xkb_keymap_compile_flags,
    ) -> *mut xkb_keymap;

    pub fn xkb_keymap_ref(keymap: *mut xkb_keymap) -> *mut xkb_keymap;

    pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);

    pub fn xkb_keymap_get_as_string(
        keymap: *mut xkb_keymap,
        format: xkb_keymap_format,
    ) -> *mut c_char;

    pub fn xkb_keymap_min_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;

    pub fn xkb_keymap_max_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;

    pub fn xkb_keymap_key_for_each(
        keymap: *mut xkb_keymap,
        iter: xkb_keymap_key_iter_t,
        data: *mut c_void,
    );

    pub fn xkb_keymap_num_mods(keymap: *mut xkb_keymap) -> xkb_mod_index_t;

    pub fn xkb_keymap_mod_get_name(keymap: *mut xkb_keymap, idx: xkb_mod_index_t) -> *const c_char;

    pub fn xkb_keymap_mod_get_index(
        keymap: *mut xkb_keymap,
        name: *const c_char,
    ) -> xkb_mod_index_t;

    pub fn xkb_keymap_num_layouts(keymap: *mut xkb_keymap) -> xkb_layout_index_t;

    pub fn xkb_keymap_layout_get_name(
        keymap: *mut xkb_keymap,
        idx: xkb_layout_index_t,
    ) -> *const c_char;

    pub fn xkb_keymap_layout_get_index(
        keymap: *mut xkb_keymap,
        name: *const c_char,
    ) -> xkb_layout_index_t;

    pub fn xkb_keymap_num_leds(keymap: *mut xkb_keymap) -> xkb_led_index_t;

    pub fn xkb_keymap_led_get_name(keymap: *mut xkb_keymap, idx: xkb_led_index_t) -> *const c_char;

    pub fn xkb_keymap_led_get_index(
        keymap: *mut xkb_keymap,
        name: *const c_char,
    ) -> xkb_led_index_t;

    pub fn xkb_keymap_num_layouts_for_key(
        keymap: *mut xkb_keymap,
        key: xkb_keycode_t,
    ) -> xkb_layout_index_t;

    pub fn xkb_keymap_num_levels_for_key(
        keymap: *mut xkb_keymap,
        key: xkb_keycode_t,
        layout: xkb_layout_index_t,
    ) -> xkb_level_index_t;

    pub fn xkb_keymap_key_get_syms_by_level(
        keymap: *mut xkb_keymap,
        key: xkb_keycode_t,
        layout: xkb_layout_index_t,
        level: xkb_level_index_t,
        syms_out: *mut *const xkb_keysym_t,
    ) -> c_int;

    pub fn xkb_keymap_key_by_name(keymap: *mut xkb_keymap, name: *const c_char) -> xkb_keycode_t;

    pub fn xkb_keymap_key_get_name(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> *const c_char;

    pub fn xkb_keymap_key_repeats(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> c_int;

    pub fn xkb_state_ref(state: *mut xkb_state) -> *mut xkb_state;

    pub fn xkb_state_unref(state: *mut xkb_state);

    pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;

    pub fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;

    pub fn xkb_state_update_key(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        direction: xkb_key_direction,
    ) -> xkb_state_component;

    pub fn xkb_state_update_mask(
        state: *mut xkb_state,
        depressed_mods: xkb_mod_mask_t,
        latched_mods: xkb_mod_mask_t,
        locked_mods: xkb_mod_mask_t,
        depressed_layout: xkb_layout_index_t,
        latched_layout: xkb_layout_index_t,
        locked_layout: xkb_layout_index_t,
    ) -> xkb_state_component;

    pub fn xkb_state_key_get_syms(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        syms_out: *mut *const xkb_keysym_t,
    ) -> c_int;

    pub fn xkb_state_key_get_utf8(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        buffer: *mut c_char,
        size: usize,
    ) -> c_int;

    pub fn xkb_state_key_get_utf32(state: *mut xkb_state, key: xkb_keycode_t) -> u32;

    pub fn xkb_state_key_get_one_sym(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_keysym_t;

    pub fn xkb_state_key_get_layout(
        state: *mut xkb_state,
        key: xkb_keycode_t,
    ) -> xkb_layout_index_t;

    pub fn xkb_state_key_get_level(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        layout: xkb_layout_index_t,
    ) -> xkb_level_index_t;

    pub fn xkb_state_serialize_mods(
        state: *mut xkb_state,
        components: xkb_state_component,
    ) -> xkb_mod_mask_t;

    pub fn xkb_state_serialize_layout(
        state: *mut xkb_state,
        components: xkb_state_component,
    ) -> xkb_layout_index_t;

    pub fn xkb_state_mod_name_is_active(
        state: *mut xkb_state,
        name: *const c_char,
        type_: xkb_state_component,
    ) -> c_int;

    pub fn xkb_state_mod_names_are_active(
        state: *mut xkb_state,
        type_: xkb_state_component,
        match_: xkb_state_match,
        ...
    ) -> c_int;

    pub fn xkb_state_mod_index_is_active(
        state: *mut xkb_state,
        idx: xkb_mod_index_t,
        type_: xkb_state_component,
    ) -> c_int;

    pub fn xkb_state_mod_index_are_active(
        state: *mut xkb_state,
        type_: xkb_state_component,
        match_: xkb_state_match,
        ...
    ) -> c_int;

    pub fn xkb_state_mod_index_is_consumed(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        idx: xkb_mod_index_t,
    ) -> c_int;

    pub fn xkb_state_mod_mask_remove_consumed(
        state: *mut xkb_state,
        key: xkb_keycode_t,
        mask: xkb_mod_mask_t,
    ) -> xkb_mod_mask_t;

    pub fn xkb_state_key_get_consumed_mods(
        state: *mut xkb_state,
        key: xkb_keycode_t,
    ) -> xkb_mod_mask_t;

    pub fn xkb_state_layout_name_is_active(
        state: *mut xkb_state,
        name: *const c_char,
        type_: xkb_state_component,
    ) -> c_int;

    pub fn xkb_state_layout_index_is_active(
        state: *mut xkb_state,
        idx: xkb_layout_index_t,
        type_: xkb_state_component,
    ) -> c_int;

    pub fn xkb_state_led_name_is_active(state: *mut xkb_state, name: *const c_char) -> c_int;

    pub fn xkb_state_led_index_is_active(state: *mut xkb_state, idx: xkb_led_index_t) -> c_int;

}

pub mod compose {
    use super::{xkb_context, xkb_keysym_t};
    use libc::{c_char, c_int, size_t, FILE};

    pub enum xkb_compose_table {}

    pub enum xkb_compose_state {}

    pub type xkb_compose_compile_flags = u32;

    pub type xkb_compose_format = u32;

    pub type xkb_compose_state_flags = u32;

    pub type xkb_compose_status = u32;

    pub type xkb_compose_feed_result = u32;

    #[link(name = "xkbcommon")]
    extern "C" {

        pub fn xkb_compose_table_new_from_locale(
            context: *mut xkb_context,
            locale: *const c_char,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;

        pub fn xkb_compose_table_new_from_file(
            context: *mut xkb_context,
            file: *mut FILE,
            locale: *const c_char,
            format: xkb_compose_format,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;

        pub fn xkb_compose_table_new_from_buffer(
            context: *mut xkb_context,
            buffer: *const c_char,
            length: size_t,
            locale: *const c_char,
            format: xkb_compose_format,
            flags: xkb_compose_compile_flags,
        ) -> *mut xkb_compose_table;

        pub fn xkb_compose_table_ref(table: *mut xkb_compose_table) -> *mut xkb_compose_table;

        pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);

        pub fn xkb_compose_state_new(
            table: *mut xkb_compose_table,
            flags: xkb_compose_state_flags,
        ) -> *mut xkb_compose_state;

        pub fn xkb_compose_state_ref(state: *mut xkb_compose_state) -> *mut xkb_compose_state;

        pub fn xkb_compose_state_unref(state: *mut xkb_compose_state);

        pub fn xkb_compose_state_get_compose_table(
            state: *mut xkb_compose_state,
        ) -> *mut xkb_compose_table;

        pub fn xkb_compose_state_feed(
            state: *mut xkb_compose_state,
            keysym: xkb_keysym_t,
        ) -> xkb_compose_feed_result;

        pub fn xkb_compose_state_reset(state: *mut xkb_compose_state);

        pub fn xkb_compose_state_get_status(state: *mut xkb_compose_state) -> xkb_compose_status;

        pub fn xkb_compose_state_get_utf8(
            state: *mut xkb_compose_state,
            buffer: *mut c_char,
            size: size_t,
        ) -> c_int;

        pub fn xkb_compose_state_get_one_sym(state: *mut xkb_compose_state) -> xkb_keysym_t;

    }
}
