// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_APK_INFO_H_
#define BASE_ANDROID_APK_INFO_H_

namespace base::android::apk_info {
// The package name of the host app which has loaded WebView, retrieved from
// the application context. In the context of the SDK Runtime, the package
// name of the app that owns this particular instance of the SDK Runtime will
// also be included. e.g.
// com.google.android.sdksandbox:com:com.example.myappwithads
const char* host_package_name();

// The application name (e.g. "Chrome"). For WebView, this is name of the
// embedding app. In the context of the SDK Runtime, this is the name of the
// app that owns this particular instance of the SDK Runtime.
const char* host_version_code();

// By default: same as versionCode. For WebView: versionCode of the embedding
// app. In the context of the SDK Runtime, this is the versionCode of the app
// that owns this particular instance of the SDK Runtime.
const char* host_package_label();

const char* package_version_code();

const char* package_version_name();

const char* package_name();

const char* resources_version();

const char* installer_package_name();

bool is_debug_app();

int target_sdk_version();

bool targets_at_least_u();

}  // namespace base::android::apk_info
#endif  // BASE_ANDROID_APK_INFO_H_
