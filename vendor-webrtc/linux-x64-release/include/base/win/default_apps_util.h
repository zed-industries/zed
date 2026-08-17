// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_DEFAULT_APPS_UTIL_H_
#define BASE_WIN_DEFAULT_APPS_UTIL_H_

#include <string_view>

#include "base/base_export.h"
#include "base/strings/cstring_view.h"
#include "base/win/windows_types.h"

namespace base::win {

// Launches the Windows 'settings' modern app with the 'default apps' view
// focused. On Windows 10, if `protocol` is not empty, it also highlights
// the `protocol` in the dialog. Returns true if the default apps dialog was
// successfully opened, and the `protocol`, if not empty, was highlighted.
BASE_EXPORT bool LaunchDefaultAppsSettingsModernDialog(
    std::wstring_view protocol);

// Launches a Windows Settings app pop-up that reads:
//   * Windows 10: "How do you want to open `file_extension` files from now on?"
//   * Windows 11: "Select a default app for `file_extension` files"
// Returns true if the dialog was successfully opened.
// Returns false on failure or if `file_extension` is empty.
// `parent_hwnd` is used by Windows to position the pop-up logically based on
// the parent window's location.
BASE_EXPORT bool LaunchDefaultAppForFileExtensionSettings(
    base::wcstring_view file_extension,
    HWND parent_hwnd);

// Launches the Windows Settings app and navigates to the "Apps > Default apps"
// page for `app_name`.
BASE_EXPORT bool LaunchSettingsDefaultApps(std::wstring_view app_name,
                                           bool is_per_user_install);

// Launches the Windows Settings app and navigates to `uri`. Returns true if
// successful.
BASE_EXPORT bool LaunchSettingsUri(base::wcstring_view uri);

}  // namespace base::win

#endif  // BASE_WIN_DEFAULT_APPS_UTIL_H_
