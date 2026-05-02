# Keep all native methods so JNI binding via dlsym/RegisterNatives still works.
# Without this rule R8 strips JNI entry-points in release builds and the app
# crashes the moment Rust calls back into Java.
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep our entry-point activity so the manifest's `android:name` resolves.
-keep class dev.zed.gpui.gallery.** { *; }

# `gpui_android`'s JNI bridge looks up `dev.zed.gpui.NativeBridge` by its
# fixed mangled symbol name (`Java_dev_zed_gpui_NativeBridge_*`). R8 must
# leave the class name and the native method declarations intact so the
# JVM's lazy native-method resolution can find them.
-keep class dev.zed.gpui.NativeBridge { *; }

# GameActivity's native_app_glue calls back into the activity at well-known
# signatures; preserve the whole class.
-keep class com.google.androidgamesdk.** { *; }
