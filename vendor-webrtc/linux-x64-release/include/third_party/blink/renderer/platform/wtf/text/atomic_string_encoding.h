// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ATOMIC_STRING_ENCODING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ATOMIC_STRING_ENCODING_H_

namespace WTF {

enum class AtomicStringUCharEncoding {
  kUnknown,

  // The string contains only 8-bit characters.
  kIs8Bit,

  // The string contains at least one 16-bit character.
  kIs16Bit,
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ATOMIC_STRING_ENCODING_H_
