# Keep all native methods so JNI binding via dlsym/RegisterNatives still works.
# Without this rule R8 strips JNI entry-points in release builds and the app
# crashes the moment Rust calls back into Java.
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep our entry-point activity so the manifest's `android:name` resolves.
-keep class dev.zed.gpui.gallery.** { *; }

# GameActivity's native_app_glue calls back into the activity at well-known
# signatures; preserve the whole class.
-keep class com.google.androidgamesdk.** { *; }
