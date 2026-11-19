package dev.zed.gpui;

import android.app.NativeActivity;
import android.content.Intent;
import android.graphics.Insets;
import android.view.View;
import android.view.WindowInsets;

/**
 * Tells GPUI what NativeActivity keeps to itself: links that open the app
 * while it is running, and changes to the window's insets.
 */
public class GpuiActivity extends NativeActivity {
    /**
     * NativeActivity keeps reporting the intent it was launched with.
     * Recording each new one lets GPUI read its link when the activity
     * resumes, which Android guarantees follows onNewIntent.
     */
    @Override
    protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        setIntent(intent);
    }

    /**
     * Starts reporting the window's insets, now and whenever they change.
     * GPUI calls this once it has registered onInsetsChanged.
     */
    void watchInsets() {
        runOnUiThread(() -> {
            View decorView = getWindow().getDecorView();
            decorView.setOnApplyWindowInsetsListener((view, windowInsets) -> {
                Insets safeArea = windowInsets.getInsets(
                    WindowInsets.Type.systemBars() | WindowInsets.Type.displayCutout());
                Insets ime = windowInsets.getInsets(WindowInsets.Type.ime());
                onInsetsChanged(
                    safeArea.left, safeArea.top, safeArea.right, safeArea.bottom,
                    ime.left, ime.top, ime.right, ime.bottom);
                return view.onApplyWindowInsets(windowInsets);
            });
            decorView.requestApplyInsets();
        });
    }

    private native void onInsetsChanged(
        int safeAreaLeft, int safeAreaTop, int safeAreaRight, int safeAreaBottom,
        int imeLeft, int imeTop, int imeRight, int imeBottom);
}
