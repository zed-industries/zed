// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRCAT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRCAT_H_

#include <initializer_list>

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// StrCat is a function to perform concatenation on a sequence of strings.
// It is preferable to a sequence of "a + b + c" because it is both faster and
// generates less code.
//
//   String result = WTF::StrCat({"foo ", result, "\nfoo ", bar});
//
// It's a Blink-variant of base::StrCat() and absl::StrCat().
//
// StrCat is generally faster than operator+ and String::Format.
[[nodiscard]] WTF_EXPORT String StrCat(base::span<const StringView> pieces);
[[nodiscard]] inline String StrCat(std::initializer_list<StringView> pieces) {
  return StrCat(base::span(pieces));
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRCAT_H_
