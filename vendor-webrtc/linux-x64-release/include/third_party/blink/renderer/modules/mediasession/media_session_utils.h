// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_SESSION_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_SESSION_UTILS_H_

#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_media_image.h"

namespace blink::media_session_utils {

// Processes the `MediaImage` with a parsed url in the artwork list. Returns an
// empty list when any exception happens.
HeapVector<Member<MediaImage>> ProcessArtworkVector(
    ScriptState* script_state,
    const HeapVector<Member<MediaImage>>& artwork,
    ExceptionState& exception_state);

}  // namespace blink::media_session_utils

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASESSION_MEDIA_SESSION_UTILS_H_
