// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_INFO_PLIST_DATA_H_
#define BASE_MAC_INFO_PLIST_DATA_H_

#include <stdint.h>

#include <vector>

#include "base/base_export.h"

namespace base::mac {

// Returns the outer bundle's Info.plist data.
//
// This data is derived from NSBundle's cached copy of the Info.plist
// rather than being read from disk. This ensures that it matches the running
// application, even if the Info.plist in the outer bundle has been modified on
// disk due to a pending update.
//
// This is intended to be used for dynamic-only code signature validation.
// See ProcessRequirement::Builder::CheckDynamicValidityOnly.
BASE_EXPORT
std::vector<uint8_t> OuterBundleCachedInfoPlistData();
}  // namespace base::mac

#endif  // BASE_MAC_INFO_PLIST_DATA_H_
