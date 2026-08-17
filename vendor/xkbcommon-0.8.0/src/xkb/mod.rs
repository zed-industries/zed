#![warn(clippy::all)]
#![allow(
    clippy::similar_names,
    clippy::wildcard_imports,
    clippy::cast_sign_loss,
    clippy::too_many_arguments
)]
pub mod compose;
pub mod ffi;
pub mod keysyms;

#[cfg(feature = "x11")]
pub mod x11;

pub use self::compose::*;
use crate::xkb::ffi::*;

#[cfg(feature = "wayland")]
use memmap2::MmapOptions;
#[cfg(feature = "wayland")]
use std::os::unix::io::OwnedFd;

use libc::{self, c_char, c_int, c_uint};
use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::fs;
use std::io::Read;
use std::iter::Iterator;
use std::mem;
use std::os::raw;
use std::path::Path;
use std::ptr::{null, null_mut};
use std::slice;
use std::str;

/// A number used to represent a physical key on a keyboard.
///
/// A standard PC-compatible keyboard might have 102 keys. An appropriate
/// keymap would assign each of them a keycode, by which the user should
/// refer to the key throughout the library.
///
/// Historically, the X11 protocol, and consequentially the XKB protocol,
/// assign only 8 bits for keycodes. This limits the number of different
/// keys that can be used simultaneously in a single keymap to 256
/// (disregarding other limitations). This library does not share this limit;
/// keycodes beyond 255 ('extended keycodes') are not treated specially.
/// Keymaps and applications which are compatible with X11 should not use
/// these keycodes.
///
/// The values of specific keycodes are determined by the keymap and the
/// underlying input system. For example, with an X11-compatible keymap
/// and Linux evdev scan codes (see linux/input.h), a fixed offset is used:
///
/// ```no_run
/// # use xkbcommon::xkb::keysyms::KEY_A;
/// # use xkbcommon::xkb::Keycode;
/// let keycode_A: Keycode = Keycode::new(KEY_A as u32 + 8);
/// ```
///
/// See `xkb::keycode_is_legal_ext()` and `xkb::keycode_is_legal_x11()`
pub use xkeysym::KeyCode as Keycode;

/// A number used to represent the symbols generated from a key on a keyboard.
///
/// A key, represented by a keycode, may generate different symbols according
/// to keyboard state. For example, on a QWERTY keyboard, pressing the key
/// labled \<A\> generates the symbol 'a'. If the Shift key is held, it
/// generates the symbol 'A'. If a different layout is used, say Greek,
/// it generates the symbol 'α'. And so on.
///
/// Each such symbol is represented by a keysym. Note that keysyms are
/// somewhat more general, in that they can also represent some "function",
/// such as "Left" or "Right" for the arrow keys. For more information,
/// see:
/// <http://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html#keysym_encoding>
///
/// Specifically named keysyms can be found in the
/// xkbcommon/xkbcommon-keysyms.h header file. Their name does not include
/// the `xkb::KEY_` prefix.
///
/// Besides those, any Unicode/ISO 10646 character in the range U0100 to
/// U10FFFF can be represented by a keysym value in the range 0x01000100 to
/// 0x0110FFFF. The name of Unicode keysyms is "`U<codepoint>`", e.g. "UA1B2".
///
/// The name of other unnamed keysyms is the hexadecimal representation of
/// their value, e.g. "0xabcd1234".
///
/// Keysym names are case-sensitive.
pub use xkeysym::Keysym;

/// Index of a keyboard layout.
///
/// The layout index is a state component which detemines which _keyboard
/// layout_ active. These may be different alphabets, different key
/// arrangements, etc.
///
/// Layout indices are consecutive. The first layout has index 0.
///
/// Each layout is not required to have a name, and the names are not
/// guaranteed to be unique (though they are usually provided and unique).
/// Therefore, it is not safe to use the name as a unique identifier for a
/// layout. Layout names are case-sensitive.
///
/// Layouts are also called "groups" by XKB.
pub type LayoutIndex = u32;
/// A mask of layout indices
pub type LayoutMask = u32;

/// Index of a shift level.
///
/// Any key, in any layout, can have several _shift levels_  Each
/// shift level can assign different keysyms to the key. The shift level
/// to use is chosen according to the current keyboard state; for example,
/// if no keys are pressed, the first level may be used; if the Left Shift
/// key is pressed, the second; if Num Lock is pressed, the third; and
/// many such combinations are possible (see `ModIndex`).
///
/// Level indices are consecutive. The first level has index 0.
pub type LevelIndex = u32;

/// Index of a modifier.
///
/// A modifier is a state component which changes the way keys are
/// interpreted. A keymap defines a set of modifiers, such as Alt, Shift,
/// Num Lock or Meta, and specifies which keys may activate which
/// modifiers (in a many-to-many relationship, i.e. a key can activate
/// several modifiers, and a modifier may be activated by several keys.
/// Different keymaps do this differently).
///
/// When retrieving the keysyms for a key, the active modifier set is
/// consulted; this detemines the correct shift level to use within the
/// currently active layout (see `LevelIndex`).
///
/// Modifier indices are consecutive. The first modifier has index 0.
///
/// Each modifier must have a name, and the names are unique. Therefore, it
/// is safe to use the name as a unique identifier for a modifier.
/// Modifier names are case-sensitive.
pub type ModIndex = u32;
/// A mask of modifier indices.
pub type ModMask = u32;

/// Index of a keyboard LED.
///
/// LEDs are logical objects which may be  active or  inactive. They
/// typically correspond to the lights on the keyboard. Their state is
/// determined by the current keyboard state.
///
/// LED indices are non-consecutive. The first LED has index 0.
///
/// Each LED must have a name, and the names are unique. Therefore,
/// it is safe to use the name as a unique identifier for a LED. The names
/// of some common LEDs are provided in the xkbcommon/xkbcommon-names.h
/// header file. LED names are case-sensitive.
///
/// # Warning
///
/// A given keymap may specify an exact index for a given LED.
/// Therefore, LED indexing is not necessarily sequential, as opposed to
/// modifiers and layouts. This means that when iterating over the LEDs
/// in a keymap using e.g. `xkb_keymap_num_leds()`, some indices might be
/// invalid. Given such an index, functions like `xkb_keymap_led_get_name()`
/// will return `NULL`, and `xkb_state_led_index_is_active()` will return -1.
///
/// LEDs are also called "indicators" by XKB.
pub type LedIndex = u32;
/// A mask of LED indices.
pub type LedMask = u32;

pub const KEYCODE_INVALID: u32 = 0xffff_ffff;
pub const LAYOUT_INVALID: u32 = 0xffff_ffff;
pub const LEVEL_INVALID: u32 = 0xffff_ffff;
pub const MOD_INVALID: u32 = 0xffff_ffff;
pub const LED_INVALID: u32 = 0xffff_ffff;

pub const KEYCODE_MAX: u32 = 0xffff_fffe;

pub type KeysymFlags = u32;
pub const KEYSYM_NO_FLAGS: u32 = 0;
pub const KEYSYM_CASE_INSENSITIVE: u32 = 1 << 0;

/// Flags for context creation.
pub type ContextFlags = u32;
/// Do not apply any context flags.
pub const CONTEXT_NO_FLAGS: u32 = 0;
/// Create this context with an empty include path.
pub const CONTEXT_NO_DEFAULT_INCLUDES: u32 = 1 << 0;
/// Don't take RMLVO names from the environment.
pub const CONTEXT_NO_ENVIRONMENT_NAMES: u32 = 1 << 1;

#[repr(C)]
pub enum LogLevel {
    Critical = 10,
    Error = 20,
    Warning = 30,
    Info = 40,
    Debug = 50,
}

/// Flags for keymap compilation.
pub type KeymapCompileFlags = u32;
/// Do not apply any flags.
pub const KEYMAP_COMPILE_NO_FLAGS: u32 = 0;

/// The possible keymap formats.
pub type KeymapFormat = u32;
/// The current/classic XKB text format, as generated by xkbcomp -xkb.
pub const KEYMAP_FORMAT_TEXT_V1: u32 = 1;
/// Get the keymap as a string in the format from which it was created.
pub const KEYMAP_FORMAT_USE_ORIGINAL: u32 = 0xffff_ffff;

/// Specifies the direction of the key (press / release).
#[repr(C)]
pub enum KeyDirection {
    /// the key was released
    Up,
    /// the key was pressed
    Down,
}

/// Modifier and layout types for state objects. This enum is bitmaskable,
/// e.g. `(xkb::STATE_MODS_DEPRESSED | xkb::STATE_MODS_LATCHED)` is valid to
/// exclude locked modifiers.
///
/// In XKB, the DEPRESSED components are also known as 'base'.
pub type StateComponent = u32;
/// Depressed modifiers, i.e. a key is physically holding them.
pub const STATE_MODS_DEPRESSED: u32 = 1 << 0;
/// Latched modifiers, i.e. will be unset after the next non-modifier
///  key press.
pub const STATE_MODS_LATCHED: u32 = 1 << 1;
/// Locked modifiers, i.e. will be unset after the key provoking the
///  lock has been pressed again.
pub const STATE_MODS_LOCKED: u32 = 1 << 2;
/// Effective modifiers, i.e. currently active and affect key
///  processing (derived from the other state components).
///  Use this unless you explictly care how the state came about.
pub const STATE_MODS_EFFECTIVE: u32 = 1 << 3;
/// Depressed layout, i.e. a key is physically holding it.
pub const STATE_LAYOUT_DEPRESSED: u32 = 1 << 4;
/// Latched layout, i.e. will be unset after the next non-modifier
///  key press.
pub const STATE_LAYOUT_LATCHED: u32 = 1 << 5;
/// Locked layout, i.e. will be unset after the key provoking the lock
///  has been pressed again.
pub const STATE_LAYOUT_LOCKED: u32 = 1 << 6;
/// Effective layout, i.e. currently active and affects key processing
///  (derived from the other state components).
///  Use this unless you explictly care how the state came about.
pub const STATE_LAYOUT_EFFECTIVE: u32 = 1 << 7;
/// LEDs (derived from the other state components).
pub const STATE_LEDS: u32 = 1 << 8;

/// Match flags for `xkb_state_mod_indices_are_active` and
/// `xkb_state_mod_names_are_active`, specifying how the conditions for a
/// successful match. `xkb::STATE_MATCH_NON_EXCLUSIVE` is bitmaskable with
/// the other modes.
pub type StateMatch = u32;
///Returns true if any of the modifiers are active.
pub const STATE_MATCH_ANY: u32 = 1 << 0;
///Returns true if all of the modifiers are active.
pub const STATE_MATCH_ALL: u32 = 1 << 1;
/// Makes matching non-exclusive, i.e. will not return false if a
///  modifier not specified in the arguments is active.
pub const STATE_MATCH_NON_EXCLUSIVE: u32 = 1 << 16;

pub const MOD_NAME_SHIFT: &str = "Shift";
pub const MOD_NAME_CAPS: &str = "Lock";
pub const MOD_NAME_CTRL: &str = "Control";
pub const MOD_NAME_ALT: &str = "Mod1";
pub const MOD_NAME_NUM: &str = "Mod2";
pub const MOD_NAME_MOD3: &str = "Mod3";
pub const MOD_NAME_LOGO: &str = "Mod4";
pub const MOD_NAME_ISO_LEVEL3_SHIFT: &str = "Mod5";
pub const LED_NAME_CAPS: &str = "Caps Lock";
pub const LED_NAME_NUM: &str = "Num Lock";
pub const LED_NAME_SCROLL: &str = "Scroll Lock";

/// Test whether a value is a valid extended keycode.
/// See `xkb_keycode_t`.
#[must_use]
pub fn keycode_is_legal_ext(key: u32) -> bool {
    key <= KEYCODE_MAX
}

/// Names to compile a keymap with, also known as RMLVO.
///
/// The names are the common configuration values by which a user picks
/// a keymap.
///
/// If the entire struct is NULL, then each field is taken to be NULL.
/// You should prefer passing NULL instead of choosing your own defaults.
#[must_use]
pub fn keycode_is_legal_x11(key: u32) -> bool {
    (8..=255).contains(&key)
}

/// Get the name of a keysym.
#[must_use]
pub fn keysym_get_name(keysym: Keysym) -> String {
    unsafe {
        const BUF_LEN: usize = 64;
        let buf: &mut [c_char] = &mut [0; BUF_LEN];
        let ptr = &mut buf[0] as *mut c_char;
        let len = xkb_keysym_get_name(keysym.raw(), ptr, BUF_LEN);
        if len <= 0 {
            return String::new();
        }
        let slice: &[u8] = slice::from_raw_parts(ptr as *const _, (len as usize).min(BUF_LEN));
        String::from_utf8_unchecked(slice.to_owned())
    }
}

/// Get a keysym from its name.
///
///  name The name of a keysym. See remarks in `xkb_keysym_get_name()`;
/// this function will accept any name returned by that function.
///  flags A set of flags controlling how the search is done. If
/// invalid flags are passed, this will fail with `xkb::KEY_NoSymbol`.
///
/// If you use the `xkb::KEYSYM_CASE_INSENSITIVE` flag and two keysym names
/// differ only by case, then the lower-case keysym is returned. For
/// instance, for `KEY_a` and `KEY_A`, this function would return `KEY_a` for
/// the case-insensitive search. If this functionality is needed, it is
/// recommended to first call this function without this flag; and if that
/// fails, only then to try with this flag, while possibly warning the user
/// he had misspelled the name, and might get wrong results.
///
/// Returns The keysym. If the name is invalid, returns `xkb::KEY_NoSymbol`.
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn keysym_from_name(name: &str, flags: KeysymFlags) -> Keysym {
    unsafe {
        let cname = CString::new(name.as_bytes().to_owned()).unwrap();
        Keysym::new(xkb_keysym_from_name(cname.as_ptr(), flags))
    }
}

/// Get the Unicode/UTF-8 representation of a keysym.
///
/// Prefer not to use this function on keysyms obtained from an
/// `xkb_state`. In this case, use `xkb_state_key_get_utf8()` instead.
#[must_use]
pub fn keysym_to_utf8(keysym: Keysym) -> String {
    unsafe {
        let buf: &mut [c_char] = &mut [0; 8];
        let ptr = &mut buf[0] as *mut c_char;
        match xkb_keysym_to_utf8(keysym.raw(), ptr, 8) {
            0 => String::from(""),
            -1 => {
                panic!("Key doesn't fit in buffer")
            }
            len => {
                let slice: &[u8] = slice::from_raw_parts(ptr as *const _, len as usize - 1);
                String::from_utf8_unchecked(slice.to_owned())
            }
        }
    }
}

/// Get the Unicode/UTF-32 representation of a keysym.
///
/// Returns The Unicode/UTF-32 representation of keysym, which is also
/// compatible with UCS-4. If the keysym does not have a Unicode
/// representation, returns 0.
///
/// Prefer not to use this function on keysyms obtained from an
/// `xkb_state`. In this case, `use xkb_state_key_get_utf32()` instead.
#[must_use]
pub fn keysym_to_utf32(keysym: Keysym) -> u32 {
    unsafe { xkb_keysym_to_utf32(keysym.raw()) }
}

/// Get the keysym corresponding to a Unicode/UTF-32 codepoint.
///
/// Returns the keysym corresponding to the specified Unicode codepoint,
/// or `KEY_NoSymbol` if there is none.
///
/// This function is the inverse of `keysym_to_utf32`. In cases where a
/// single codepoint corresponds to multiple keysyms, returns the keysym
/// with the lowest value.
///
/// Unicode codepoints which do not have a special (legacy) keysym
/// encoding use a direct encoding scheme. These keysyms don't usually
/// have an associated keysym constant (`XKB_KEY_*`).
///
/// For noncharacter Unicode codepoints and codepoints outside of the
/// defined Unicode planes this function returns `KEY_NoSymbol`.
#[must_use]
pub fn utf32_to_keysym(ucs: u32) -> Keysym {
    unsafe { xkb_utf32_to_keysym(ucs) }.into()
}

/// Top level library context object.
///
/// The context contains various general library data and state, like
/// logging level and include paths.
///
/// Objects are created in a specific context, and multiple contexts may
/// coexist simultaneously. Objects from different contexts are completely
/// separated and do not share any memory or state.
pub struct Context {
    ptr: *mut xkb_context,
}

impl Context {
    /// contruct a context from a raw ffi pointer. This context must already been
    /// referenced as `xkb_context_unref` will be called at drop time
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw_ptr(ptr: *mut xkb_context) -> Context {
        Context { ptr }
    }

    /// get the raw pointer from this context
    #[must_use]
    pub fn get_raw_ptr(&self) -> *mut xkb_context {
        self.ptr
    }

    /// Create a new context.
    ///
    ///  flags Optional flags for the context, or 0.
    ///
    /// The user may set some environment variables to affect default values in
    /// the context.
    #[must_use]
    pub fn new(flags: ContextFlags) -> Context {
        unsafe {
            Context {
                ptr: xkb_context_new(flags),
            }
        }
    }

    /// append a new entry to the context's include path
    /// returns true on success, or false if the include path could not be added
    /// or is inaccessible
    pub fn include_path_append(&mut self, path: &Path) -> bool {
        path.to_str().map_or(false, |s| unsafe {
            let cstr = CString::from_vec_unchecked(s.as_bytes().to_owned());
            xkb_context_include_path_append(self.ptr, cstr.as_ptr()) == 1
        })
    }

    /// Append the default include paths to the context's include path.
    ///
    /// Returns true on success.
    pub fn include_path_append_default(&mut self) -> bool {
        unsafe { xkb_context_include_path_append_default(self.ptr) == 1 }
    }

    /// Reset the context's include path to the default.
    ///
    /// Removes all entries from the context's include path, and inserts the
    /// default paths.
    ///
    /// Returns true on success.yy
    pub fn include_path_reset_defaults(&mut self) -> bool {
        unsafe { xkb_context_include_path_reset_defaults(self.ptr) == 1 }
    }

    /// Remove all entries from the context's include path.
    pub fn include_path_clear(&mut self) {
        unsafe {
            xkb_context_include_path_clear(self.ptr);
        }
    }

    /// get an iterator on the include paths of this context
    #[must_use]
    pub fn include_paths(&self) -> ContextIncludePaths {
        unsafe {
            ContextIncludePaths {
                context: self,
                ind: 0,
                len: xkb_context_num_include_paths(self.ptr),
            }
        }
    }

    /// Set the current logging level.
    ///
    /// The default level is `xkb::LogLevel::Error`. The environment variable
    /// `XKB_LOG_LEVEL`, if set in the time the context was created, overrides the
    /// default value. It may be specified as a level number or name.
    pub fn set_log_level(&mut self, level: LogLevel) {
        unsafe {
            xkb_context_set_log_level(self.ptr, mem::transmute(level));
        }
    }

    #[must_use]
    pub fn get_log_level(&self) -> LogLevel {
        unsafe { mem::transmute(xkb_context_get_log_level(self.ptr)) }
    }

    /// Sets the current logging verbosity.
    ///
    /// The library can generate a number of warnings which are not helpful to
    /// ordinary users of the library. The verbosity may be increased if more
    /// information is desired (e.g. when developing a new keymap).
    ///
    /// The default verbosity is 0. The environment variable `XKB_LOG_VERBOSITY`,
    /// if set in the time the context was created, overrides the default value.
    ///
    /// verbosity can be set from 1 to 10, higher values being more verbose.
    /// 0 would result in no verbose messages being logged.
    ///
    /// Most verbose messages are of level `xkb::LogLevel::Warning` or lower.
    pub fn set_log_verbosity(&mut self, verbosity: i32) {
        unsafe {
            xkb_context_set_log_verbosity(self.ptr, verbosity as c_int);
        }
    }

    #[must_use]
    pub fn get_log_verbosity(&self) -> i32 {
        unsafe { xkb_context_get_log_verbosity(self.ptr) as i32 }
    }
}

impl Clone for Context {
    fn clone(&self) -> Context {
        unsafe {
            Context {
                ptr: xkb_context_ref(self.ptr),
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            xkb_context_unref(self.ptr);
        }
    }
}

/// Iterator to a Context include paths
pub struct ContextIncludePaths<'a> {
    context: &'a Context,
    ind: c_uint,
    len: c_uint,
}

impl<'a> Iterator for ContextIncludePaths<'a> {
    type Item = &'a Path;
    fn next(&mut self) -> Option<&'a Path> {
        if self.ind == self.len {
            None
        } else {
            unsafe {
                let ptr = xkb_context_include_path_get(self.context.ptr, self.ind);
                self.ind += 1;
                let cstr = CStr::from_ptr(ptr);
                Some(Path::new(str::from_utf8_unchecked(cstr.to_bytes())))
            }
        }
    }
}

#[test]
fn check_include_paths() {
    let mut c = Context::new(CONTEXT_NO_DEFAULT_INCLUDES);
    let test_path = Path::new("/");
    assert_eq!(true, c.include_path_append(&test_path));
    assert_eq!(test_path, c.include_paths().nth(0).unwrap());
}

/// Compiled keymap object.
///
/// The keymap object holds all of the static keyboard information obtained
/// from compiling XKB files.
///
/// A keymap is immutable after it is created (besides reference counts, etc.);
/// if you need to change it, you must create a new one.
pub struct Keymap {
    ptr: *mut xkb_keymap,
}

impl Keymap {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw_ptr(ptr: *mut xkb_keymap) -> Keymap {
        Keymap { ptr }
    }

    #[must_use]
    pub fn get_raw_ptr(&self) -> *mut xkb_keymap {
        self.ptr
    }

    /// Create a keymap from RMLVO names.
    ///
    /// The primary keymap entry point: creates a new XKB keymap from a set of
    /// RMLVO (Rules + Model + Layouts + Variants + Options) names.
    ///
    /// __context__
    ///  The context in which to create the keymap.
    ///
    /// __rules__
    ///  The rules file to use. The rules file describes how to interpret
    ///  the values of the model, layout, variant and options fields.
    ///
    ///  If empty string "", a default value is used.
    ///  If the `XKB_DEFAULT_RULES` environment variable is set, it is used
    ///  as the default. Otherwise the system default is used.
    ///
    /// __model__
    ///  The keyboard model by which to interpret keycodes and LEDs.
    ///
    ///  If empty string "", a default value is used.
    ///  If the `XKB_DEFAULT_MODEL` environment variable is set, it is used
    ///  as the default. Otherwise the system default is used.
    ///
    /// __layout__
    ///  A comma separated list of layouts (languages) to include in the
    ///  keymap.
    ///
    ///  If empty string "", a default value is used.
    ///  If the `XKB_DEFAULT_LAYOUT` environment variable is set, it is used
    ///  as the default. Otherwise the system default is used.
    ///
    /// __variant__
    ///  A comma separated list of variants, one per layout, which may
    ///  modify or augment the respective layout in various ways.
    ///
    ///  If empty string "", and a default value is also used
    ///  for the layout, a default value is used. Otherwise no variant is
    ///  used.
    ///  If the `XKB_DEFAULT_VARIANT` environment variable is set, it is used
    ///  as the default. Otherwise the system default is used.
    ///
    /// __options__
    ///  A comma separated list of options, through which the user specifies
    ///  non-layout related preferences, like which key combinations are used
    ///  for switching layouts, or which key is the Compose key.
    ///
    ///  If `None`, a default value is used. If `Some("")` (empty string), no
    ///  options are used.
    ///  If the `XKB_DEFAULT_OPTIONS` environment variable is set, it is used
    ///  as the default. Otherwise the system default is used.
    ///
    /// __flags__
    ///  Optional flags for the keymap, or 0.
    ///
    /// Returns a keymap compiled according to the `RMLVO` names, or `None` if
    /// the compilation failed.
    #[allow(clippy::missing_panics_doc)]
    pub fn new_from_names<S: Borrow<str> + ?Sized>(
        context: &Context,
        rules: &S,
        model: &S,
        layout: &S,
        variant: &S,
        mut options: Option<String>,
        flags: KeymapCompileFlags,
    ) -> Option<Keymap> {
        let crules = CString::new(rules.borrow().as_bytes()).unwrap();
        let cmodel = CString::new(model.borrow().as_bytes()).unwrap();
        let clayout = CString::new(layout.borrow().as_bytes()).unwrap();
        let cvariant = CString::new(variant.borrow().as_bytes()).unwrap();
        let poptions = match &mut options {
            None => null(),
            Some(s) => {
                s.push('\0');
                s.as_ptr().cast()
            }
        };
        let rule_names = xkb_rule_names {
            rules: crules.as_ptr(),
            model: cmodel.as_ptr(),
            layout: clayout.as_ptr(),
            variant: cvariant.as_ptr(),
            options: poptions,
        };
        unsafe {
            let pkeymap = xkb_keymap_new_from_names(context.ptr, &rule_names, flags);
            if pkeymap.is_null() {
                None
            } else {
                Some(Keymap { ptr: pkeymap })
            }
        }
    }

    ///  Create a keymap from a keymap file.
    ///
    ///  Returns `None` if compilation fails.
    ///
    ///  The file must contain a complete keymap. For example, in the
    ///  `XKB_KEYMAP_FORMAT_TEXT_V1` format, this means the file must contain one
    ///  top level `%xkb_keymap` section, which in turn contains other required
    ///  sections.
    ///
    ///  bindings implementation get the content in a `String`
    ///  and call `new_from_string()`.
    pub fn new_from_file(
        context: &Context,
        file: &mut fs::File,
        format: KeymapFormat,
        flags: KeymapCompileFlags,
    ) -> Option<Keymap> {
        let mut string = String::new();
        file.read_to_string(&mut string)
            .ok()
            .and_then(|_| Keymap::new_from_string(context, string, format, flags))
    }

    ///  Create a keymap from a keymap string.
    ///
    ///  This is just like `xkb_keymap_new_from_file()`, but instead of a file, gets
    ///  the keymap as one enormous string.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new_from_string(
        context: &Context,
        string: String,
        format: KeymapFormat,
        flags: KeymapCompileFlags,
    ) -> Option<Keymap> {
        unsafe {
            let cstr = CString::new(string.into_bytes()).unwrap();
            let ptr = xkb_keymap_new_from_string(context.ptr, cstr.as_ptr(), format, flags);
            if ptr.is_null() {
                None
            } else {
                Some(Keymap { ptr })
            }
        }
    }

    #[cfg(feature = "wayland")]
    /// Create a keymap from a file descriptor.
    /// The file is mapped to memory and the keymap is created from the mapped memory buffer.
    ///
    /// # Safety
    /// The file descriptor must be valid and all safety concerns of mapping files to memory
    /// apply here.
    #[allow(clippy::missing_panics_doc)]
    pub unsafe fn new_from_fd(
        context: &Context,
        fd: OwnedFd,
        size: usize,
        format: KeymapFormat,
        flags: KeymapCompileFlags,
    ) -> std::io::Result<Option<Keymap>> {
        let map = MmapOptions::new()
            .len(size as usize)
            // Starting in version 7 of the wl_keyboard protocol, the keymap must be mapped using MAP_PRIVATE.
            .map_copy_read_only(&fs::File::from(fd))?;
        let ptr =
            xkb_keymap_new_from_buffer(context.ptr, map.as_ptr().cast(), size - 1, format, flags);
        if ptr.is_null() {
            Ok(None)
        } else {
            Ok(Some(Keymap { ptr }))
        }
    }

    /// Get the compiled keymap as a string.
    ///
    ///  keymap The keymap to get as a string.
    ///  format The keymap format to use for the string. You can pass
    /// in the special value `xkb::KEYMAP_USE_ORIGINAL_FORMAT` to use the format
    /// from which the keymap was originally created.
    ///
    /// Returns The keymap as a NUL-terminated string, or `NULL` if unsuccessful.
    ///
    /// The returned string may be fed back into `xkb_map_new_from_string()` to get
    /// the exact same keymap (possibly in another process, etc).
    ///
    /// The returned string is dynamically allocated and should be freed by the
    /// caller.
    #[must_use]
    pub fn get_as_string(&self, format: KeymapFormat) -> String {
        unsafe {
            let ffistr = xkb_keymap_get_as_string(self.ptr, format);
            let cstr = CStr::from_ptr(ffistr);
            let res = String::from_utf8_unchecked(cstr.to_bytes().to_owned());
            libc::free(ffistr.cast());
            res
        }
    }

    /// Get the minimum keycode in the keymap.
    #[must_use]
    pub fn min_keycode(&self) -> Keycode {
        Keycode::new(unsafe { xkb_keymap_min_keycode(self.ptr) })
    }

    /// Get the maximum keycode in the keymap.
    #[must_use]
    pub fn max_keycode(&self) -> Keycode {
        Keycode::new(unsafe { xkb_keymap_max_keycode(self.ptr) })
    }

    #[allow(unused_variables)]
    unsafe extern "C" fn callback<F>(
        pkeymap: *mut ffi::xkb_keymap,
        key: ffi::xkb_keycode_t,
        data: *mut raw::c_void,
    ) where
        F: FnMut(&Keymap, Keycode),
    {
        let mut data_box: Box<(&Keymap, F)> = mem::transmute(Box::from_raw(data));
        {
            let (keymap, ref mut closure) = *data_box;
            closure(keymap, key.into());
        }
        let _ = Box::into_raw(data_box);
    }

    /// Run a specified closure for every valid keycode in the keymap.
    pub fn key_for_each<F>(&self, closure: F)
    where
        F: FnMut(&Keymap, Keycode),
    {
        let data_box = Box::new((self, closure));
        let data_ptr = Box::into_raw(data_box).cast();

        unsafe {
            ffi::xkb_keymap_key_for_each(self.get_raw_ptr(), Self::callback::<F>, data_ptr);
            mem::drop(Box::from_raw(data_ptr.cast::<(&Keymap, F)>()));
        }
    }

    /// Get an iterator to the modifiers of this keymap
    #[must_use]
    pub fn mods(&self) -> KeymapMods {
        unsafe {
            KeymapMods {
                keymap: self,
                ind: 0,
                len: xkb_keymap_num_mods(self.ptr),
            }
        }
    }

    /// Get the number of modifiers in the keymap.
    #[must_use]
    pub fn num_mods(&self) -> ModIndex {
        unsafe { xkb_keymap_num_mods(self.ptr) }
    }

    /// Get the name of a modifier by index.
    ///
    /// Returns The name. If the index is invalid, returns "".
    #[must_use]
    pub fn mod_get_name(&self, idx: ModIndex) -> &str {
        unsafe {
            let ptr = xkb_keymap_mod_get_name(self.ptr, idx);
            if ptr.is_null() {
                ""
            } else {
                let cstr = CStr::from_ptr(ptr);
                str::from_utf8_unchecked(cstr.to_bytes())
            }
        }
    }

    /// Get the index of a modifier by name.
    ///
    /// Returns The index. If no modifier with this name exists, returns
    /// `xkb::MOD_INVALID`.
    #[allow(clippy::missing_panics_doc)]
    pub fn mod_get_index<S: Borrow<str> + ?Sized>(&self, name: &S) -> ModIndex {
        unsafe {
            let cstr = CString::new(name.borrow().as_bytes()).unwrap();
            xkb_keymap_mod_get_index(self.ptr, cstr.as_ptr())
        }
    }

    /// Returns an iterator to the layouts in this keymap
    #[must_use]
    pub fn layouts(&self) -> KeymapLayouts {
        unsafe {
            KeymapLayouts {
                keymap: self,
                ind: 0,
                len: xkb_keymap_num_layouts(self.ptr),
            }
        }
    }

    /// Get the number of layouts in the keymap.
    #[must_use]
    pub fn num_layouts(&self) -> LayoutIndex {
        unsafe { xkb_keymap_num_layouts(self.ptr) }
    }

    /// Get the name of a layout by index.
    ///
    /// Returns The name. If the index is invalid, or the layout does not have
    /// a name, returns "".
    #[must_use]
    pub fn layout_get_name(&self, idx: LayoutIndex) -> &str {
        unsafe {
            let ptr = xkb_keymap_layout_get_name(self.ptr, idx);
            if ptr.is_null() {
                ""
            } else {
                let cstr = CStr::from_ptr(ptr);
                str::from_utf8_unchecked(cstr.to_bytes())
            }
        }
    }

    /// Find the name of the key with the given keycode.
    /// This function always returns the canonical name of the key (see description in [Keycode]).
    pub fn key_get_name(&self, key: Keycode) -> Option<&str> {
        unsafe {
            let ptr = xkb_keymap_key_get_name(self.ptr, key.into());
            if ptr.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(ptr);
                Some(str::from_utf8_unchecked(cstr.to_bytes()))
            }
        }
    }

    /// Find the keycode of the key with the given name.
    /// The name can be either a canonical name or an alias.
    pub fn key_by_name<S: Borrow<str> + ?Sized>(&self, name: &S) -> Option<Keycode> {
        unsafe {
            let cstr = CString::new(name.borrow().as_bytes()).unwrap();
            let code = xkb_keymap_key_by_name(self.ptr, cstr.as_ptr());
            if code == XKB_KEYCODE_INVALID {
                None
            } else {
                Some(Keycode::new(code))
            }
        }
    }

    /// Get the index of a layout by name.
    ///
    /// Returns The index. If no layout exists with this name, returns
    /// `xkb::LAYOUT_INVALID`. If more than one layout in the keymap has this name,
    /// returns the lowest index among them.
    #[allow(clippy::missing_panics_doc)]
    pub fn layout_get_index<S: Borrow<str> + ?Sized>(&self, name: &S) -> LayoutIndex {
        unsafe {
            let cstr = CString::new(name.borrow().as_bytes()).unwrap();
            xkb_keymap_layout_get_index(self.ptr, cstr.as_ptr())
        }
    }

    /// Returns an iterator to the leds in this keymap
    #[must_use]
    pub fn leds(&self) -> KeymapLeds {
        unsafe {
            KeymapLeds {
                keymap: self,
                ind: 0,
                len: xkb_keymap_num_leds(self.ptr),
            }
        }
    }

    /// Get the number of LEDs in the keymap.
    ///
    /// # warning
    /// The range `[0..num_leds())` includes all of the LEDs
    /// in the keymap, but may also contain inactive LEDs. When iterating over
    /// this range, you need the handle this case when calling functions such as
    /// `led_get_name()` or `led_index_is_active()`.
    #[must_use]
    pub fn num_leds(&self) -> LedIndex {
        unsafe { xkb_keymap_num_leds(self.ptr) }
    }

    /// Get the name of a LED by index.
    ///
    /// Returns the name. If the index is invalid, returns `""`.
    #[must_use]
    pub fn led_get_name(&self, idx: LedIndex) -> &str {
        unsafe {
            let ptr = xkb_keymap_led_get_name(self.ptr, idx);
            if ptr.is_null() {
                ""
            } else {
                let cstr = CStr::from_ptr(ptr);
                str::from_utf8_unchecked(cstr.to_bytes())
            }
        }
    }

    /// Get the index of a LED by name.
    ///
    /// Returns The index. If no LED with this name exists, returns
    /// `xkb::LED_INVALID`.
    #[allow(clippy::missing_panics_doc)]
    pub fn led_get_index<S: Borrow<str> + ?Sized>(&self, name: &S) -> LedIndex {
        unsafe {
            let cstr = CString::new(name.borrow().as_bytes()).unwrap();
            xkb_keymap_led_get_index(self.ptr, cstr.as_ptr())
        }
    }

    /// Get the number of layouts for a specific key.
    ///
    /// This number can be different `from num_layouts()`, but is always
    /// smaller. It is the appropriate value to use when iterating over the
    /// layouts of a key.
    #[must_use]
    pub fn num_layouts_for_key(&self, key: Keycode) -> LayoutIndex {
        unsafe { xkb_keymap_num_layouts_for_key(self.ptr, key.raw()) }
    }

    /// Get the number of shift levels for a specific key and layout.
    ///
    /// If layout is out of range for this key (that is, larger or equal to
    /// the value returned by `num_layouts_for_key()`), it is brought
    /// back into range in a manner consistent with `State::key_get_layout()`.
    #[must_use]
    pub fn num_levels_for_key(&self, key: Keycode, layout: LayoutIndex) -> LevelIndex {
        unsafe { xkb_keymap_num_levels_for_key(self.ptr, key.into(), layout) }
    }

    /// Get the keysyms obtained from pressing a key in a given layout and
    /// shift level.
    ///
    /// This function is like `xkb_state_key_get_syms()`, only the layout and
    /// shift level are not derived from the keyboard state but are instead
    /// specified explicitly.
    ///
    /// If layout is out of range for this key (that is, larger or equal to
    /// the value returned by `num_layouts_for_key()`), it is brought
    /// back into range in a manner consistent with `State::key_get_layout()`.
    #[must_use]
    pub fn key_get_syms_by_level(
        &self,
        key: Keycode,
        layout: LayoutIndex,
        level: LevelIndex,
    ) -> &[Keysym] {
        unsafe {
            let mut syms_out: *const Keysym = null_mut();
            let len = xkb_keymap_key_get_syms_by_level(
                self.ptr,
                key.raw(),
                layout,
                level,
                &mut syms_out as *mut *const Keysym as *mut *const xkeysym::RawKeysym,
            );
            if syms_out.is_null() {
                &[]
            } else {
                slice::from_raw_parts(syms_out, len as usize)
            }
        }
    }

    /// Determine whether a key should repeat or not.
    ///
    /// A keymap may specify different repeat behaviors for different keys.
    /// Most keys should generally exhibit repeat behavior; for example, holding
    /// the 'a' key down in a text editor should normally insert a single 'a'
    /// character every few milliseconds, until the key is released. However,
    /// there are keys which should not or do not need to be repeated. For
    /// example, repeating modifier keys such as Left/Right Shift or Caps Lock
    /// is not generally useful or desired.
    #[must_use]
    pub fn key_repeats(&self, key: Keycode) -> bool {
        unsafe { xkb_keymap_key_repeats(self.ptr, key.into()) != 0 }
    }
}

impl Clone for Keymap {
    fn clone(&self) -> Keymap {
        unsafe {
            Keymap {
                ptr: xkb_keymap_ref(self.ptr),
            }
        }
    }
}

impl Drop for Keymap {
    fn drop(&mut self) {
        unsafe {
            xkb_keymap_unref(self.ptr);
        }
    }
}

/// iterator to the modifiers in a Keymap
pub struct KeymapMods<'a> {
    keymap: &'a Keymap,
    ind: ModIndex,
    len: ModIndex,
}

impl<'a> Iterator for KeymapMods<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.ind == self.len {
            None
        } else {
            unsafe {
                let ptr = xkb_keymap_mod_get_name(self.keymap.ptr, self.ind);
                self.ind += 1;
                let cstr = CStr::from_ptr(ptr);
                Some(str::from_utf8_unchecked(cstr.to_bytes()))
            }
        }
    }
}

/// iterator to the layouts in Keymap
pub struct KeymapLayouts<'a> {
    keymap: &'a Keymap,
    ind: LayoutIndex,
    len: LayoutIndex,
}

impl<'a> Iterator for KeymapLayouts<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.ind == self.len {
            None
        } else {
            unsafe {
                let ptr = xkb_keymap_layout_get_name(self.keymap.ptr, self.ind);
                self.ind += 1;
                let cstr = CStr::from_ptr(ptr);
                Some(str::from_utf8_unchecked(cstr.to_bytes()))
            }
        }
    }
}

/// iterator to the leds in a Keymap
pub struct KeymapLeds<'a> {
    keymap: &'a Keymap,
    ind: LedIndex,
    len: LedIndex,
}

impl<'a> Iterator for KeymapLeds<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.ind == self.len {
            None
        } else {
            unsafe {
                let ptr = xkb_keymap_led_get_name(self.keymap.ptr, self.ind);
                self.ind += 1;
                let cstr = CStr::from_ptr(ptr);
                Some(str::from_utf8_unchecked(cstr.to_bytes()))
            }
        }
    }
}

/// Keyboard state object.
///
/// State objects contain the active state of a keyboard (or keyboards), such
/// as the currently effective layout and the active modifiers. It acts as a
/// simple state machine, wherein key presses and releases are the input, and
/// key symbols (keysyms) are the output.
pub struct State {
    ptr: *mut xkb_state,
}

impl State {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_raw_ptr(ptr: *mut xkb_state) -> State {
        State { ptr }
    }

    #[must_use]
    pub fn get_raw_ptr(&self) -> *mut xkb_state {
        self.ptr
    }

    /// Create a new keyboard state object from a keymap.
    #[must_use]
    pub fn new(keymap: &Keymap) -> State {
        unsafe {
            State {
                ptr: xkb_state_new(keymap.ptr),
            }
        }
    }

    /// Get the keymap which a keyboard state object is using.
    ///
    /// Returns the keymap which was passed to `xkb_state_new()` when creating
    /// this state object.
    ///
    /// This keymap can safely be used beyond the lifetime of this state
    #[must_use]
    pub fn get_keymap(&self) -> Keymap {
        unsafe {
            let keymap = xkb_state_get_keymap(self.ptr);
            xkb_keymap_ref(keymap);
            Keymap::from_raw_ptr(keymap)
        }
    }

    /// Update the keyboard state to reflect a given key being pressed or
    /// released.
    ///
    /// This entry point is intended for programs which track the keyboard state
    /// explictly (like an evdev client). If the state is serialized to you by
    /// a master process (like a Wayland compositor) using functions like
    /// `xkb_state_serialize_mods()`, you should use `xkb_state_update_mask()`
    /// instead. The two functins should not generally be used together.
    ///
    /// A series of calls to this function should be consistent; that is, a call
    /// with `xkb::KEY_DOWN` for a key should be matched by an `xkb::KEY_UP`; if
    /// a key is pressed twice, it should be released twice; etc. Otherwise (e.g.
    /// due to missed input events), situations like "stuck modifiers" may occur.
    ///
    /// This function is often used in conjunction with the function
    /// `xkb_state_key_get_syms()` (or `xkb_state_key_get_one_sym()`), for
    /// example, when handling a key event. In this case, you should prefer to
    /// get the keysyms *before* updating the key, such that the keysyms reported
    /// for the key event are not affected by the event itself. This is the
    /// conventional behavior.
    ///
    /// Returns A mask of state components that have changed as a result of
    /// the update. If nothing in the state has changed, returns 0.
    pub fn update_key(&mut self, key: Keycode, direction: KeyDirection) -> StateComponent {
        unsafe { xkb_state_update_key(self.ptr, key.into(), mem::transmute(direction)) }
    }

    /// Update a keyboard state from a set of explicit masks.
    ///
    /// This entry point is intended for window systems and the like, where a
    /// master process holds an `xkb_state`, then serializes it over a wire
    /// protocol, and clients then use the serialization to feed in to their own
    /// `xkb_state`.
    ///
    /// All parameters must always be passed, or the resulting state may be
    /// incoherent.
    ///
    /// The serialization is lossy and will not survive round trips; it must only
    /// be used to feed slave state objects, and must not be used to update the
    /// master state.
    ///
    /// If you do not fit the description above, you should use
    /// `xkb_state_update_key()` instead. The two functions should not generally be
    /// used together.
    ///
    /// Returns a mask of state components that have changed as a result of
    /// the update. If nothing in the state has changed, returns 0.
    pub fn update_mask(
        &mut self,
        depressed_mods: ModMask,
        latched_mods: ModMask,
        locked_mods: ModMask,
        depressed_layout: LayoutIndex,
        latched_layout: LayoutIndex,
        locked_layout: LayoutIndex,
    ) -> StateComponent {
        unsafe {
            xkb_state_update_mask(
                self.ptr,
                depressed_mods,
                latched_mods,
                locked_mods,
                depressed_layout,
                latched_layout,
                locked_layout,
            )
        }
    }

    /// Get the keysyms obtained from pressing a particular key in a given
    /// keyboard state.
    ///
    /// Get the keysyms for a key according to the current active layout,
    /// modifiers and shift level for the key, as determined by a keyboard
    /// state.
    ///
    /// # Arguments
    /// * `state`: The keyboard state object.
    /// * `key`: The keycode of the key.
    ///
    /// # Return
    /// * `syms_out`: An immutable array of keysyms corresponding the
    /// key in the given keyboard state.
    ///
    /// As an extension to XKB, this function can return more than one keysym.
    /// If you do not want to handle this case, you should use
    /// `xkb_state_key_get_one_sym()`, which additionally performs transformations
    /// which are specific to the one-keysym case.
    #[must_use]
    pub fn key_get_syms(&self, key: Keycode) -> &[Keysym] {
        unsafe {
            let mut syms_out: *const Keysym = null_mut();
            let len = xkb_state_key_get_syms(
                self.ptr,
                key.into(),
                &mut syms_out as *mut *const Keysym as *mut *const xkeysym::RawKeysym,
            );
            if syms_out.is_null() {
                &[]
            } else {
                slice::from_raw_parts(syms_out, len as usize)
            }
        }
    }

    /// Get the Unicode/UTF-8 string obtained from pressing a particular key
    /// in a given keyboard state.
    #[must_use]
    pub fn key_get_utf8(&self, key: Keycode) -> String {
        unsafe {
            const BUF_LEN: usize = 64;
            let buf: &mut [c_char] = &mut [0; BUF_LEN];
            let ptr = &mut buf[0] as *mut c_char;
            let ret = xkb_state_key_get_utf8(self.ptr, key.into(), ptr, BUF_LEN);
            // ret is similar to the return value of snprintf.
            // it may be negative on unspecified errors, or >64 if the buffer is too small.
            let len = ret.max(0).min(BUF_LEN as i32);
            let slice: &[u8] = slice::from_raw_parts(ptr as *const _, len as usize);
            String::from_utf8_unchecked(slice.to_owned())
        }
    }

    /// Get the Unicode/UTF-32 codepoint obtained from pressing a particular
    /// key in a a given keyboard state.
    ///
    /// Returns The UTF-32 representation for the key, if it consists of only
    /// a single codepoint. Otherwise, returns 0.
    #[must_use]
    pub fn key_get_utf32(&self, key: Keycode) -> u32 {
        unsafe { xkb_state_key_get_utf32(self.ptr, key.into()) }
    }

    /// Get the single keysym obtained from pressing a particular key in a
    /// given keyboard state.
    ///
    /// This function is similar to `xkb_state_key_get_syms()`, but intended
    /// for users which cannot or do not want to handle the case where
    /// multiple keysyms are returned (in which case this function is
    /// preferred).
    ///
    /// Returns the keysym. If the key does not have exactly one keysym,
    /// returns `xkb::KEY_NoSymbol`.
    #[must_use]
    pub fn key_get_one_sym(&self, key: Keycode) -> Keysym {
        unsafe { xkb_state_key_get_one_sym(self.ptr, key.into()) }.into()
    }

    /// Get the effective layout index for a key in a given keyboard state.
    ///
    /// Returns the layout index for the key in the given keyboard state. If
    /// the given keycode is invalid, or if the key is not included in any
    /// layout at all, returns `xkb::LAYOUT_INVALID`.
    #[must_use]
    pub fn key_get_layout(&self, key: Keycode) -> LayoutIndex {
        unsafe { xkb_state_key_get_layout(self.ptr, key.into()) }
    }

    /// Get the effective shift level for a key in a given keyboard state and
    /// layout.
    ///
    /// Return the shift level index. If the key or layout are invalid,
    /// returns `xkb::LEVEL_INVALID`.
    #[must_use]
    pub fn key_get_level(&self, key: Keycode, layout: LayoutIndex) -> LevelIndex {
        unsafe { xkb_state_key_get_level(self.ptr, key.into(), layout) }
    }

    /// The counterpart to `xkb_state_update_mask` for modifiers, to be used on
    /// the server side of serialization.
    ///
    /// State components other than `xkb::STATE_MODS_*` are ignored.
    /// If `xkb::STATE_MODS_EFFECTIVE` is included, all other state components are
    /// ignored.
    ///
    /// Returns a `ModMask` representing the given components of the
    /// modifier state.
    ///
    /// This function should not be used in regular clients; please use the
    /// `xkb::State::mod_*_is_active` API instead.
    #[must_use]
    pub fn serialize_mods(&self, components: StateComponent) -> ModMask {
        unsafe { xkb_state_serialize_mods(self.ptr, components) }
    }

    #[must_use]
    pub fn serialize_layout(&self, components: StateComponent) -> LayoutIndex {
        unsafe { xkb_state_serialize_layout(self.ptr, components) }
    }

    /// Test whether a modifier is active in a given keyboard state by name.
    #[allow(clippy::missing_panics_doc)]
    pub fn mod_name_is_active<S: Borrow<str> + ?Sized>(
        &self,
        name: &S,
        type_: StateComponent,
    ) -> bool {
        unsafe {
            let cname = CString::new(name.borrow().as_bytes()).unwrap();
            xkb_state_mod_name_is_active(self.ptr, cname.as_ptr(), type_) == 1
        }
    }

    /// Test whether a modifier is active in a given keyboard state by index.
    #[must_use]
    pub fn mod_index_is_active(&self, idx: ModIndex, type_: StateComponent) -> bool {
        unsafe { xkb_state_mod_index_is_active(self.ptr, idx, type_) == 1 }
    }

    /// Test whether a modifier is consumed by keyboard state translation for
    /// a key.
    ///
    /// Some functions, like `xkb_state_key_get_syms()`, look at the state of
    /// the modifiers in the keymap and derive from it the correct shift level
    /// to use for the key. For example, in a US layout, pressing the key
    /// labeled \<A\> while the Shift modifier is active, generates the keysym 'A'.
    /// In this case, the Shift modifier is said to be consumed. However, the
    /// Num Lock modifier does not affect this translation at all, even if it
    /// active, so it is not consumed by this translation.
    ///
    /// It may be desirable for some application to not reuse consumed modifiers
    /// for further processing, e.g. for hotkeys or keyboard shortcuts. To
    /// understand why, consider some requirements from a standard shortcut
    /// mechanism, and how they are implemented:
    ///
    /// 1. The shortcut's modifiers must match exactly to the state. For example,
    ///    it is possible to bind separate actions to \<Alt\>\<Tab\> and to
    ///    \<Alt\>\<Shift\>\<Tab\>. Further, if only \<Alt\>\<Tab\> is bound to
    ///    an action, pressing \<Alt\>\<Shift\>\<Tab\> should not trigger the
    ///    shortcut.
    ///    Effectively, this means that the modifiers are compared using the
    ///    equality operator (==).
    /// 2. Only relevant modifiers are considered for the matching. For example,
    ///    Caps Lock and Num Lock should not generally affect the matching, e.g.
    ///    when matching \<Alt\>\<Tab\> against the state, it does not matter
    ///    whether Num Lock is active or not. These relevant, or significant,
    ///    modifiers usually include Alt, Control, Shift, Super and similar.
    ///    Effectively, this means that non-significant modifiers are masked out,
    ///    before doing the comparison as described above.
    /// 3. The matching must be independent of the layout/keymap. For example,
    ///    the \<Plus\> (+) symbol is found on the first level on some layouts,
    ///    and requires holding Shift on others. If you simply bind the action
    ///    to the \<Plus\> keysym, it would work for the unshifted kind, but
    ///    not for the others, because the match against Shift would fail. If
    ///    you bind the action to \<Shift\>\<Plus\>, only the shifted kind would
    ///    work. So what is needed is to recognize that Shift is used up in the
    ///    translation of the keysym itself, and therefore should not be included
    ///    in the matching.
    ///    Effectively, this means that consumed modifiers (Shift in this example)
    ///    are masked out as well, before doing the comparison.
    ///
    /// `state_modifiers` are the modifiers reported by
    /// `xkb::State::mod_index_is_active()` and similar functions.
    /// `consumed_modifiers` are the modifiers reported by
    /// `xkb::State::mod_index_is_consumed()`.
    /// `significant_modifiers` are decided upon by the application/toolkit/user;
    /// it is up to them to decide whether these are configurable or hard-coded.
    #[must_use]
    pub fn mod_index_is_consumed(&self, key: Keycode, idx: ModIndex) -> bool {
        unsafe { xkb_state_mod_index_is_consumed(self.ptr, key.into(), idx) == 1 }
    }

    /// Remove consumed modifiers from a modifier mask for a key.
    ///
    /// Takes the given modifier mask, and removes all modifiers which are
    /// consumed for that particular key (as in `xkb_state_mod_index_is_consumed()`).
    #[must_use]
    pub fn mod_mask_remove_consumed(&self, key: Keycode, mask: ModMask) -> ModMask {
        unsafe { xkb_state_mod_mask_remove_consumed(self.ptr, key.into(), mask) }
    }

    /// Get the mask of modifiers consumed by translating a given key.
    ///
    /// Returns a mask of the consumed modifiers.
    #[must_use]
    pub fn key_get_consumed_mods(&self, key: Keycode) -> ModMask {
        unsafe { xkb_state_key_get_consumed_mods(self.ptr, key.into()) }
    }

    /// Test whether a layout is active in a given keyboard state by name.
    ///
    /// If multiple layouts in the keymap have this name, the one with the lowest
    /// index is tested.
    #[allow(clippy::missing_panics_doc)]
    pub fn layout_name_is_active<S: Borrow<str> + ?Sized>(
        &self,
        name: &S,
        type_: StateComponent,
    ) -> bool {
        unsafe {
            let cname = CString::new(name.borrow().as_bytes()).unwrap();
            xkb_state_layout_name_is_active(self.ptr, cname.as_ptr(), type_) != 0
        }
    }

    /// Test whether a layout is active in a given keyboard state by index.
    #[must_use]
    pub fn layout_index_is_active(&self, idx: LayoutIndex, type_: StateComponent) -> bool {
        unsafe { xkb_state_layout_index_is_active(self.ptr, idx, type_) != 0 }
    }

    /// Test whether a LED is active in a given keyboard state by name.
    #[allow(clippy::missing_panics_doc)]
    pub fn led_name_is_active<S: Borrow<str> + ?Sized>(&self, name: &S) -> bool {
        unsafe {
            let cname = CString::new(name.borrow().as_bytes()).unwrap();
            xkb_state_led_name_is_active(self.ptr, cname.as_ptr()) != 0
        }
    }

    /// Test whether a LED is active in a given keyboard state by index.
    #[must_use]
    pub fn led_index_is_active(&self, idx: LedIndex) -> bool {
        unsafe { xkb_state_led_index_is_active(self.ptr, idx) != 0 }
    }
}

impl Clone for State {
    fn clone(&self) -> State {
        unsafe {
            State {
                ptr: xkb_state_ref(self.ptr),
            }
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            xkb_state_unref(self.ptr);
        }
    }
}
