// `:app` builds the actual APK.
//
// The Rust `.so` is produced separately by `script/build-gpui-android
// build-release` (or `build`), then copied into the right `jniLibs/<abi>/`
// directory before `./gradlew :app:assembleRelease` packages it.
//
// We deliberately *don't* shell out to cargo from Gradle here — keeping the
// Rust build in `cargo` keeps incremental compiles fast and lets developers
// iterate on Rust code without a Gradle context.

plugins {
    id("com.android.application")
}

android {
    namespace = "dev.zed.gpui.example"
    compileSdk = 35
    ndkVersion = "29.0.14206865"

    defaultConfig {
        applicationId = "dev.zed.gpui.example"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
        ndk {
            // ABI list to bundle. The `.so` files must already exist in
            // `app/src/main/jniLibs/<abi>/` before the APK is assembled.
            abiFilters += listOf("arm64-v8a", "x86_64")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = signingConfigs.getByName("debug") // dev signing only; replace for Play
        }
    }

    packaging {
        // Our jniLibs already contain only the `.so`s we want; let the build
        // tool include them as-is (no second-pass stripping that breaks Rust
        // debug info).
        jniLibs {
            useLegacyPackaging = false
            keepDebugSymbols += listOf("**/libgpui_android_example.so")
        }
    }
}

dependencies {
    // GameActivity ships in the games-activity AAR. android-activity expects
    // this to be on the classpath when the `game-activity` feature is on.
    implementation("androidx.games:games-activity:3.0.5")
    // androidx.core for splash-screen / window-insets compatibility shims.
    implementation("androidx.core:core:1.13.1")
}
