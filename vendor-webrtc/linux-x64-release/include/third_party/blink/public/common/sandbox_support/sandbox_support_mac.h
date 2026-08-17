// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SANDBOX_SUPPORT_SANDBOX_SUPPORT_MAC_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SANDBOX_SUPPORT_SANDBOX_SUPPORT_MAC_H_

namespace blink {

// Named Mac system colors.
enum class MacSystemColorID {
  kControlAccentBlueColor,
  kControlAccentColor,
  kKeyboardFocusIndicator,
  kSecondarySelectedControl,
  kSelectedTextBackground,
  kCount,
};

constexpr size_t kMacSystemColorIDCount =
    static_cast<size_t>(MacSystemColorID::kCount);

constexpr size_t kMacSystemColorSchemeCount = 2;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SANDBOX_SUPPORT_SANDBOX_SUPPORT_MAC_H_
