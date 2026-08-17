// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_FUZZER_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_FUZZER_UTILS_H_

#include <fuzzer/FuzzedDataProvider.h>

#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"

namespace blink {

enum class DecoderType {
  kBmpDecoder,
  kJpegDecoder,
  kPngDecoder,
  kCrabbyAvifDecoder,
};

std::unique_ptr<ImageDecoder> CreateImageDecoder(DecoderType decoder_type,
                                                 FuzzedDataProvider& fdp);

void FuzzDecoder(DecoderType decoder_type, FuzzedDataProvider& fdp);

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_FUZZER_UTILS_H_
