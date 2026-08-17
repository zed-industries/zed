// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_OWNER_ELEMENT_TYPE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_OWNER_ELEMENT_TYPE_H_

namespace blink {

enum class FrameOwnerElementType {
  // For a main frame.
  kNone = 0,
  kIframe,
  kObject,
  kEmbed,
  kFrame,
  kFencedframe,
  kMaxValue = kFencedframe,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FRAME_OWNER_ELEMENT_TYPE_H_
