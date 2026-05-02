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
    namespace = "dev.zed.gpui.gallery"
    // We compile against the highest SDK installed locally (34) — Play
    // ultimately requires targetSdk = 35, which can be set independently of
    // the platform JAR we link against.
    compileSdk = 34
    ndkVersion = "29.0.14206865"

    defaultConfig {
        applicationId = "dev.zed.gpui.gallery"
        minSdk = 26
        targetSdk = 34
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
            keepDebugSymbols += listOf("**/libhello_android.so")
        }
    }
}

dependencies {
    // GameActivity ships in the games-activity AAR. android-activity expects
    // this to be on the classpath when the `game-activity` feature is on.
    implementation("androidx.games:games-activity:3.0.5")
    // GameActivity transitively extends AppCompatActivity, so we need
    // appcompat on the classpath even though we don't use any of its
    // widgets directly.
    implementation("androidx.appcompat:appcompat:1.7.0")
    // androidx.core for splash-screen / window-insets compatibility shims.
    implementation("androidx.core:core:1.13.1")

    // Force the unified Kotlin stdlib so `kotlin-stdlib-jdk7` and `jdk8`
    // (pulled in transitively by games-activity 3.0.5 at 1.6.21) don't
    // collide with the newer `kotlin-stdlib` 1.8.22 that brings their
    // classes in directly. Recommended fix from the official Kotlin docs.
    constraints {
        implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7:1.8.22") {
            because("kotlin-stdlib bundles its jdk7+ classes from 1.8.0+")
        }
        implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.8.22") {
            because("kotlin-stdlib bundles its jdk7+ classes from 1.8.0+")
        }
    }
}
