// Trivial GameActivity subclass that loads the Rust .so and lets
// android-activity's glue spawn the `android_main(AndroidApp)` thread.
//
// We deliberately keep this Java-side surface minimal — every platform-level
// integration (clipboard, intents, IME, etc.) goes through JNI from
// `gpui_android` instead of being duplicated here.
package dev.zed.gpui.gallery;

import com.google.androidgamesdk.GameActivity;

public class GalleryActivity extends GameActivity {
    static {
        System.loadLibrary("hello_android");
    }
}
