// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_PNG_SKIA_PNG_RUST_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_PNG_SKIA_PNG_RUST_IMAGE_DECODER_H_

#include <memory>

#include "third_party/blink/renderer/platform/image-decoders/skia/skia_image_decoder_base.h"

namespace blink {

// This class decodes the PNG image format using `SkPngRustCodec`.
class PLATFORM_EXPORT SkiaPngRustImageDecoder final
    : public SkiaImageDecoderBase {
 public:
  // Exposing the same constructor as the base class:
  using SkiaImageDecoderBase::SkiaImageDecoderBase;

  SkiaPngRustImageDecoder(const SkiaPngRustImageDecoder&) = delete;
  SkiaPngRustImageDecoder& operator=(const SkiaPngRustImageDecoder&) = delete;
  ~SkiaPngRustImageDecoder() override;

  // ImageDecoder:
  String FilenameExtension() const override;
  const AtomicString& MimeType() const override;

 protected:
  // SkiaImageDecoderBase:
  std::unique_ptr<SkCodec> OnCreateSkCodec(std::unique_ptr<SkStream>,
                                           SkCodec::Result* result) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_PNG_SKIA_PNG_RUST_IMAGE_DECODER_H_
