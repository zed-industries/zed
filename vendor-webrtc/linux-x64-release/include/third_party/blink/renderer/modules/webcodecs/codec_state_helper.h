// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_STATE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_STATE_HELPER_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_codec_state.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Returns true and sets the exception state if the passed CodecState is
// kClosed. The exception message is built from the |operation| name.
bool ThrowIfCodecStateClosed(V8CodecState, String operation, ExceptionState&);

// Returns true and sets the exception state if the passed CodecState is
// kUnconfigured. The exception message is built from the |operation| name.
bool ThrowIfCodecStateUnconfigured(V8CodecState,
                                   String operation,
                                   ExceptionState&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_STATE_HELPER_H_
