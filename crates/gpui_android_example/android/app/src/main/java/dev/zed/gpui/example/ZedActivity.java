// Trivial GameActivity subclass that just loads the Rust .so and lets
// android-activity's glue spawn the `android_main(AndroidApp)` thread.
//
// We deliberately keep this Java-side surface minimal (~10 lines) — every
// platform-level integration (clipboard, intents, IME, etc.) goes through
// JNI from `gpui_android` instead of being duplicated here.
package dev.zed.gpui.example;

import com.google.androidgamesdk.GameActivity;

public class ZedActivity extends GameActivity {
    static {
        System.loadLibrary("gpui_android_example");
    }
}
