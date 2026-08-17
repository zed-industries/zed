// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CONTENT_AS_TEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CONTENT_AS_TEXT_H_

#include <stdint.h>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

class LocalFrame;

// Recursively dumps the text inside |frame| and its local subtree to
// |output|, up to the length of |max_chars|.
CORE_EXPORT void FrameContentAsText(wtf_size_t max_chars,
                                    LocalFrame* frame,
                                    WTF::StringBuilder& output);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CONTENT_AS_TEXT_H_
