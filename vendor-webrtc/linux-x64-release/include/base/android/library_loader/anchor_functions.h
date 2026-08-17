// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_LIBRARY_LOADER_ANCHOR_FUNCTIONS_H_
#define BASE_ANDROID_LIBRARY_LOADER_ANCHOR_FUNCTIONS_H_

#include <cstdint>

#include "base/android/library_loader/anchor_functions_buildflags.h"
#include "base/base_export.h"

#if BUILDFLAG(SUPPORTS_CODE_ORDERING)

namespace base {
namespace android {

// Start and end of .text, respectively.
BASE_EXPORT extern const size_t kStartOfText;
BASE_EXPORT extern const size_t kEndOfText;
// Start and end of the ordered part of .text, respectively.
BASE_EXPORT extern const size_t kStartOfOrderedText;
BASE_EXPORT extern const size_t kEndOfOrderedText;

// Returns true if anchors are sane.
BASE_EXPORT bool AreAnchorsSane();

// Returns true if the ordering looks sane.
BASE_EXPORT bool IsOrderingSane();

}  // namespace android
}  // namespace base
#endif  // BUILDFLAG(SUPPORTS_CODE_ORDERING)

#endif  // BASE_ANDROID_LIBRARY_LOADER_ANCHOR_FUNCTIONS_H_
