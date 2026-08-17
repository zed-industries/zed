use glib::translate::from_glib_full;
use gtk4::{gio, prelude::*};

use crate::ActivationToken;

impl ActivationToken {
    /// Gets an activation token from a window.
    ///
    /// Support for the XDG Activation Protocol was added in GLib 2.76, this
    /// method will return `None` on older versions.
    #[cfg_attr(docsrs, doc(cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))))]
    pub fn from_window(widget: &impl IsA<::gtk4::Widget>) -> Option<Self> {
        if glib::check_version(2, 76, 0).is_some() {
            #[cfg(feature = "tracing")]
            tracing::info!("Need glib 2.76 for XDG Activation protocol support");

            return None;
        }

        let display = widget.as_ref().display();
        let context = display.app_launch_context();

        // g_app_launch_context_get_startup_notify_id only accepts nullable
        // parameters since 2.82. On older versions we use the vfunc.
        if glib::check_version(2, 82, 0).is_some() {
            unsafe {
                let klass: *mut gtk4::gio::ffi::GAppLaunchContextClass =
                    &context.class().parent()? as *const _ as *mut _;
                let get_startup_notify_id = (*klass).get_startup_notify_id.as_ref()?;
                from_glib_full::<_, Option<String>>(get_startup_notify_id(
                    context.as_ptr().cast(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ))
            }
        } else {
            context
                .startup_notify_id(gio::AppInfo::NONE, &[])
                .map(String::from)
        }
        .map(Self::from)
    }
}
