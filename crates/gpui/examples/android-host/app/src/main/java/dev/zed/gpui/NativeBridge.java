// Java-side counterpart to the JNI symbols in `gpui_android::android::pickers`.
//
// `gpui_android` calls `Activity.startActivityForResult` from Rust to launch
// SAF / PhotoPicker dialogs, and the result comes back through the host
// Activity's `onActivityResult` override. Forwarding that callback into
// native code requires a JNI symbol whose name is fixed at link time
// (`Java_dev_zed_gpui_NativeBridge_onActivityResult`). Concentrating that
// fixed name here lets every host project use whatever package + Activity
// name it likes, while still binding to the same `.so` symbol.
//
// Host integration: in your Activity's onActivityResult,
//
//     dev.zed.gpui.NativeBridge.onActivityResult(requestCode, resultCode, data);
//
// after `super.onActivityResult(...)`. Nothing else is required.

package dev.zed.gpui;

import android.content.Intent;

public final class NativeBridge {
    private NativeBridge() {}

    /**
     * Forwarded from the host Activity's onActivityResult override. The
     * native implementation lives in the `gpui_android` Rust crate and
     * dispatches the result to whichever Rust caller launched the
     * picker (SAF file/folder picker, PhotoPicker, etc.).
     */
    public static native void onActivityResult(int requestCode, int resultCode, Intent data);
}
