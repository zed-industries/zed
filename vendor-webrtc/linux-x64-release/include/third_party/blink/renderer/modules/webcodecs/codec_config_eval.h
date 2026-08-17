// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_CONFIG_EVAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_CONFIG_EVAL_H_

namespace blink {

// Possible outcomes of evaluating the user provided codec configurations (
// VideoDecoderConfig, VideoEncoderConfig, ...).
enum class CodecConfigEval {
  // The codec config is not valid (e.g. bad codec string).
  kInvalid,
  // The codec config is valid, but unsupported.
  kUnsupported,
  // The codec config is valid and supported.
  kSupported
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_CONFIG_EVAL_H_
