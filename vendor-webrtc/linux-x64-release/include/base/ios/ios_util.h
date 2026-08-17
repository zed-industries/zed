// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_IOS_IOS_UTIL_H_
#define BASE_IOS_IOS_UTIL_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "build/ios_buildflags.h"

namespace base {
namespace ios {

// Returns whether the operating system is iOS 16 or later.
BASE_EXPORT bool IsRunningOnIOS16OrLater();

// Returns whether the operating system is iOS 17 or later.
BASE_EXPORT bool IsRunningOnIOS17OrLater();

// Returns whether the operating system is at the given version or later.
BASE_EXPORT bool IsRunningOnOrLater(int32_t major,
                                    int32_t minor,
                                    int32_t bug_fix);

// Returns whether iOS is signaling that an RTL text direction should be used
// regardless of the current locale. This should not return true if the current
// language is a "real" RTL language such as Arabic or Urdu; it should only
// return true in cases where the RTL text direction has been forced (for
// example by using the "RTL Pseudolanguage" option when launching from Xcode).
BASE_EXPORT bool IsInForcedRTL();

// Stores the |path| of the ICU dat file in a global to be referenced later by
// FilePathOfICUFile().  This should only be called once.
BASE_EXPORT void OverridePathOfEmbeddedICU(const char* path);

// Returns the overriden path set by OverridePathOfEmbeddedICU(), otherwise
// returns invalid FilePath.
BASE_EXPORT FilePath FilePathOfEmbeddedICU();

#if !BUILDFLAG(IS_IOS_APP_EXTENSION)
// Returns true iff multiple windows can be opened, i.e. when the multiwindow
// build flag is on, the device is running on iOS 13+ and it's a compatible
// iPad.
BASE_EXPORT bool IsMultipleScenesSupported();
#endif

// iOS 15 introduced pre-warming, which launches and then pauses the app, to
// speed up actual launch time.
BASE_EXPORT bool IsApplicationPreWarmed();

}  // namespace ios
}  // namespace base

#endif  // BASE_IOS_IOS_UTIL_H_
