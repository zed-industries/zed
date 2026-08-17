// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_SCRIPTING_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_SCRIPTING_H_

namespace blink {

enum class Scripting {
  // Scripting is not supported or not enabled.
  kNone,
  // Scripting is supported and enabled, but only for initial page load
  // We will never match this value as it is intended for non-browser user
  // agents, but it is part of the spec so we should still parse it.
  // See https://github.com/w3c/csswg-drafts/issues/8621
  kInitialOnly,
  // Scripting is supported and enabled.
  kEnabled,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CSS_SCRIPTING_H_
