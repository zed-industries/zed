//! Bindings for [`ANativeActivity`]
//!
//! [`ANativeActivity`]: https://developer.android.com/ndk/reference/group/native-activity#anativeactivity

use super::hardware_buffer_format::HardwareBufferFormat;
use std::{
    ffi::{CStr, OsStr},
    os::{raw::c_void, unix::prelude::OsStrExt},
    path::Path,
    ptr::NonNull,
};

bitflags::bitflags! {
    /// Window flags, as per the Java API at [`android.view.WindowManager.LayoutParams`].
    ///
    /// <https://developer.android.com/ndk/reference/group/native-activity#group___native_activity_1ga2f1398dba5e4a5616b83437528bdb28e>
    ///
    /// [`android.view.WindowManager.LayoutParams`]: https://developer.android.com/reference/android/view/WindowManager.LayoutParams
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct WindowFlags : u32 {
        const ALLOW_LOCK_WHILE_SCREEN_ON = ffi::AWINDOW_FLAG_ALLOW_LOCK_WHILE_SCREEN_ON;
        const DIM_BEHIND = ffi::AWINDOW_FLAG_DIM_BEHIND;
        #[deprecated = "Deprecated. Blurring is no longer supported."]
        const BLUR_BEHIND = ffi::AWINDOW_FLAG_BLUR_BEHIND;
        const NOT_FOCUSABLE = ffi::AWINDOW_FLAG_NOT_FOCUSABLE;
        const NOT_TOUCHABLE = ffi::AWINDOW_FLAG_NOT_TOUCHABLE;
        const NOT_TOUCH_MODAL = ffi::AWINDOW_FLAG_NOT_TOUCH_MODAL;
        #[deprecated = "This constant was deprecated in API level 20. This flag has no effect."]
        const TOUCHABLE_WHEN_WAKING = ffi::AWINDOW_FLAG_TOUCHABLE_WHEN_WAKING;
        const KEEP_SCREEN_ON = ffi::AWINDOW_FLAG_KEEP_SCREEN_ON;
        const LAYOUT_IN_SCREEN = ffi::AWINDOW_FLAG_LAYOUT_IN_SCREEN;
        const LAYOUT_NO_LIMITS = ffi::AWINDOW_FLAG_LAYOUT_NO_LIMITS;
        const FULLSCREEN = ffi::AWINDOW_FLAG_FULLSCREEN;
        #[cfg_attr(feature = "api-level-30", deprecated = "This constant was deprecated in API level 30. This value became API \"by accident\", and shouldn't be used by 3rd party applications.")]
        const FORCE_NOT_FULLSCREEN = ffi::AWINDOW_FLAG_FORCE_NOT_FULLSCREEN;
        #[deprecated = "This constant was deprecated in API level 17. This flag is no longer used."]
        const DITHER = ffi::AWINDOW_FLAG_DITHER;
        const SECURE = ffi::AWINDOW_FLAG_SECURE;
        const SCALED = ffi::AWINDOW_FLAG_SCALED;
        const IGNORE_CHEEK_PRESSES = ffi::AWINDOW_FLAG_IGNORE_CHEEK_PRESSES;
        const LAYOUT_INSET_DECOR = ffi::AWINDOW_FLAG_LAYOUT_INSET_DECOR;
        const ALT_FOCUSABLE_IM = ffi::AWINDOW_FLAG_ALT_FOCUSABLE_IM;
        const WATCH_OUTSIDE_TOUCH = ffi::AWINDOW_FLAG_WATCH_OUTSIDE_TOUCH;
        const SHOW_WHEN_LOCKED = ffi::AWINDOW_FLAG_SHOW_WHEN_LOCKED;
        const SHOW_WALLPAPER = ffi::AWINDOW_FLAG_SHOW_WALLPAPER;
        const TURN_SCREEN_ON = ffi::AWINDOW_FLAG_TURN_SCREEN_ON;
        #[cfg_attr(feature = "api-level-26", deprecated = "This constant was deprecated in API level 26. Use `SHOW_WHEN_LOCKED` instead.")]
        const DISMISS_KEYGUARD = ffi::AWINDOW_FLAG_DISMISS_KEYGUARD;
        const ATTACHED_IN_DECOR = 0x40000000;

        // https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
        const _ = !0;
    }
}

/// A native [`ANativeActivity *`]
///
/// This is either provided in [`ffi::ANativeActivity_onCreate()`], or accessible through
/// `ndk_glue::native_activity()`.
///
/// [`ANativeActivity *`]: https://developer.android.com/ndk/reference/struct/a-native-activity
#[derive(Debug)]
pub struct NativeActivity {
    ptr: NonNull<ffi::ANativeActivity>,
}

// It gets shared between threads in `ndk-glue`
unsafe impl Send for NativeActivity {}
unsafe impl Sync for NativeActivity {}

impl NativeActivity {
    /// Create a [`NativeActivity`] from a pointer
    ///
    /// # Safety
    /// By calling this function, you assert that it is a valid pointer to a native
    /// [`ffi::ANativeActivity`].
    pub unsafe fn from_ptr(ptr: NonNull<ffi::ANativeActivity>) -> Self {
        Self { ptr }
    }

    /// The pointer to the native `ANativeActivity`
    pub fn ptr(&self) -> NonNull<ffi::ANativeActivity> {
        self.ptr
    }
}

/// Methods that relate to fields of the struct itself
///
/// The relevant NDK docs can be found
/// [here](https://developer.android.com/ndk/reference/struct/a-native-activity).
impl NativeActivity {
    /// The platform's SDK version code
    pub fn sdk_version(&self) -> i32 {
        unsafe { self.ptr.as_ref().sdkVersion }
    }

    /// Path to this application's internal data directory
    pub fn internal_data_path(&self) -> &Path {
        OsStr::from_bytes(unsafe { CStr::from_ptr(self.ptr.as_ref().internalDataPath) }.to_bytes())
            .as_ref()
    }

    /// Path to this application's external (removable, mountable) data directory
    pub fn external_data_path(&self) -> &Path {
        OsStr::from_bytes(unsafe { CStr::from_ptr(self.ptr.as_ref().externalDataPath) }.to_bytes())
            .as_ref()
    }

    /// This app's asset manager, which can be used to access assets from the `.apk` file.
    pub fn asset_manager(&self) -> crate::asset::AssetManager {
        unsafe {
            crate::asset::AssetManager::from_ptr(
                NonNull::new(self.ptr.as_ref().assetManager).unwrap(),
            )
        }
    }

    /// Instance data associated with the activity
    pub fn instance(&self) -> *mut c_void {
        unsafe { self.ptr.as_ref().instance }
    }

    /// Set the instance data associated with the activity
    ///
    /// # Safety
    /// This can invalidate assumptions held by `ndk-glue`, as well as cause data
    /// races with concurrent access to the instance data.
    pub unsafe fn set_instance(&mut self, data: *mut c_void) {
        // FIXME Does this create undefined behavior by creating a mutable reference to what could
        // also be accessed immutably at the same time?
        //
        // I think that as long as we warn the users to avoid concurrent access, and we pass along
        // the `unsafe` burden, it's OK.
        self.ptr.as_mut().instance = data;
    }

    /// This process's `JavaVM` object.
    ///
    /// Usage with [__jni__](https://crates.io/crates/jni) crate:
    /// ```no_run
    /// # use ndk::native_activity::NativeActivity;
    /// # let native_activity: NativeActivity = unimplemented!();
    /// let vm_ptr = native_activity.vm();
    /// let vm = unsafe { jni::JavaVM::from_raw(vm_ptr) }.unwrap();
    /// let env = vm.attach_current_thread();
    /// // Do JNI with env ...
    /// ```
    pub fn vm(&self) -> *mut jni_sys::JavaVM {
        unsafe { self.ptr.as_ref() }.vm
    }

    /// The [`android.app.NativeActivity`] instance
    ///
    /// In the JNI, this is named `clazz`; however, as the docs say, "it should really be named
    /// 'activity' instead of 'clazz', since it's a reference to the NativeActivity instance".
    ///
    /// [`android.app.NativeActivity`]: https://developer.android.com/reference/android/app/NativeActivity
    pub fn activity(&self) -> jni_sys::jobject {
        unsafe { self.ptr.as_ref() }.clazz
    }

    /// Path to the directory with the application's OBB files.
    ///
    /// # Safety
    /// Only available as of Honeycomb (Android 3.0+, API level 11+)
    pub unsafe fn obb_path(&self) -> &Path {
        OsStr::from_bytes(CStr::from_ptr(self.ptr.as_ref().obbPath).to_bytes()).as_ref()
    }
}

/// Methods that relate to `ANativeActivity_*` functions.
///
/// The relevant NDK docs can be found
/// [here](https://developer.android.com/ndk/reference/group/native-activity).
impl NativeActivity {
    /// Sends a destroy event to the activity and stops it.
    pub fn finish(&self) {
        unsafe { ffi::ANativeActivity_finish(self.ptr.as_ptr()) }
    }

    /// Shows the IME (the on-screen keyboard).
    ///
    /// If `force` is true, the `SHOW_FORCED` flag is used; otherwise, the `SHOW_IMPLICIT` flag is
    /// used.  Depending on the value of this flag, the `hide_soft_input` method with behave
    /// differently.  See [the relevant
    /// javadoc](https://developer.android.com/reference/android/view/inputmethod/InputMethodManager#constants_2)
    /// for more information.
    pub fn show_soft_input(&self, force: bool) {
        let flag = if force {
            ffi::ANATIVEACTIVITY_SHOW_SOFT_INPUT_FORCED
        } else {
            ffi::ANATIVEACTIVITY_SHOW_SOFT_INPUT_IMPLICIT
        };
        unsafe { ffi::ANativeActivity_showSoftInput(self.ptr.as_ptr(), flag) }
    }

    /// Hides the IME (the on-screen keyboard).
    ///
    /// If `not_always` is true, the `HIDE_NOT_ALWAYS` flag is used; otherwise, the
    /// `HIDE_IMPLICIT_ONLY` flag is used.  Depending on the value of this flag and the way the IME
    /// was shown, it may or may not be hidden.  See [the relevant
    /// javadoc](https://developer.android.com/reference/android/view/inputmethod/InputMethodManager#constants_2)
    /// for more information.
    pub fn hide_soft_input(&self, not_always: bool) {
        let flag = if not_always {
            ffi::ANATIVEACTIVITY_HIDE_SOFT_INPUT_NOT_ALWAYS
        } else {
            ffi::ANATIVEACTIVITY_HIDE_SOFT_INPUT_IMPLICIT_ONLY
        };
        unsafe { ffi::ANativeActivity_hideSoftInput(self.ptr.as_ptr(), flag) }
    }

    /// Change the window format of the given activity.
    ///
    /// Calls [`getWindow().setFormat()`] of the given activity. Note that this method can be
    /// called from any thread; it will send a message to the main thread of the process where the
    /// Java finish call will take place.
    ///
    /// [`getWindow().setFormat()`]: https://developer.android.com/reference/android/view/Window#setFormat(int)
    pub fn set_window_format(&self, format: HardwareBufferFormat) {
        unsafe { ffi::ANativeActivity_setWindowFormat(self.ptr.as_ptr(), format.into()) }
    }

    /// Change the window flags of the given activity.
    ///
    /// Calls [`getWindow().setFlags()`] of the given activity.
    ///
    /// Note that this method can be called from any thread; it will send a message to the main
    /// thread of the process where the Java finish call will take place.
    ///
    /// [`getWindow().setFlags()`]: https://developer.android.com/reference/android/view/Window#setFlags(int,%20int)
    pub fn set_window_flags(&self, add_flags: WindowFlags, remove_flags: WindowFlags) {
        unsafe {
            ffi::ANativeActivity_setWindowFlags(
                self.ptr.as_ptr(),
                add_flags.bits(),
                remove_flags.bits(),
            )
        }
    }
}
