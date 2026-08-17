// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MAC_LOGIN_UTIL_H_
#define BASE_MAC_LOGIN_UTIL_H_

#include <MacTypes.h>

#include <optional>

#include "base/base_export.h"

namespace base::mac {

// Various useful functions from the private login.framework. Because these are
// SPI, all return values are optional; `std::nullopt` is returned if the SPI is
// not available.

// Returns whether the screen lock on this Mac is enabled for the user.
BASE_EXPORT std::optional<bool> IsScreenLockEnabled();

// Moves the current user session to the background and goes to the
// fast-user-switching screen.
BASE_EXPORT std::optional<OSStatus> SwitchToLoginWindow();

}  // namespace base::mac

#endif  // BASE_MAC_LOGIN_UTIL_H_
