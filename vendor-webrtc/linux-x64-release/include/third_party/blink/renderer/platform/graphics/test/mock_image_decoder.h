/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_IMAGE_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_IMAGE_DECODER_H_

#include <memory>
#include <utility>

#include "base/memory/ptr_util.h"
#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/graphics/image_frame_generator.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/skia/include/core/SkColorSpace.h"

namespace blink {

class MockImageDecoderClient {
 public:
  MockImageDecoderClient() : first_frame_forced_to_be_empty_(false) {}

  virtual void DecoderBeingDestroyed() = 0;
  virtual void DecodeRequested() = 0;
  virtual ImageFrame::Status GetStatus(wtf_size_t index) = 0;
  virtual wtf_size_t FrameCount() = 0;
  virtual int RepetitionCount() const = 0;
  virtual base::TimeDelta FrameDuration() const = 0;
  virtual void ClearCacheExceptFrameRequested(wtf_size_t) {}
  virtual void MemoryAllocatorSet() {}

  // Clients can control the behavior of MockImageDecoder::decodedSize() by
  // overriding this method. The default implementation causes
  // MockImageDecoder::decodedSize() to return the same thing as
  // MockImageDecoder::size(). See the precise implementation of
  // MockImageDecoder::decodedSize() below.
  virtual gfx::Size DecodedSize() const { return gfx::Size(); }

  void ForceFirstFrameToBeEmpty() { first_frame_forced_to_be_empty_ = true; }

  bool FirstFrameForcedToBeEmpty() const {
    return first_frame_forced_to_be_empty_;
  }

 private:
  bool first_frame_forced_to_be_empty_;
};

class MockImageDecoder : public ImageDecoder {
 public:
  MockImageDecoder(MockImageDecoderClient* client)
      : ImageDecoder(kAlphaPremultiplied,
                     ImageDecoder::kDefaultBitDepth,
                     ColorBehavior::kTransformToSRGB,
                     cc::AuxImage::kDefault,
                     ImageDecoder::kNoDecodedImageByteLimit),
        client_(client) {}

  ~MockImageDecoder() override { client_->DecoderBeingDestroyed(); }

  gfx::Size DecodedSize() const override {
    return client_->DecodedSize().IsEmpty() ? Size() : client_->DecodedSize();
  }

  String FilenameExtension() const override { return "mock"; }

  const AtomicString& MimeType() const override {
    DEFINE_STATIC_LOCAL(const AtomicString, mock_mime_type, ("image/x-mock"));
    return mock_mime_type;
  }

  int RepetitionCount() const override { return client_->RepetitionCount(); }

  bool FrameIsReceivedAtIndex(wtf_size_t index) const override {
    return client_->GetStatus(index) == ImageFrame::kFrameComplete;
  }

  base::TimeDelta FrameDurationAtIndex(wtf_size_t) const override {
    return client_->FrameDuration();
  }

  wtf_size_t ClearCacheExceptFrame(wtf_size_t clear_except_frame) override {
    client_->ClearCacheExceptFrameRequested(clear_except_frame);
    return 0;
  }

  wtf_size_t FrameBytesAtIndex(wtf_size_t index) const override {
    if (client_->FirstFrameForcedToBeEmpty() && index == 0)
      return 0;
    return ImageDecoder::FrameBytesAtIndex(index);
  }

  void SetMemoryAllocator(SkBitmap::Allocator* allocator) override {
    if (frame_buffer_cache_.empty()) {
      // Ensure that InitializeNewFrame is called, after parsing if
      // necessary.
      if (!FrameCount())
        return;
    }

    frame_buffer_cache_[0].SetMemoryAllocator(allocator);
    client_->MemoryAllocatorSet();
  }

 private:
  void DecodeSize() override {}

  wtf_size_t DecodeFrameCount() override { return client_->FrameCount(); }

  void Decode(wtf_size_t index) override {
    client_->DecodeRequested();
    frame_buffer_cache_[index].ClearPixelData();
    InitializeNewFrame(index);
    frame_buffer_cache_[index].SetStatus(client_->GetStatus(index));
  }

  void InitializeNewFrame(wtf_size_t index) override {
    if (frame_buffer_cache_[index].AllocatePixelData(
            Size().width(), Size().height(), ColorSpaceForSkImages()))
      frame_buffer_cache_[index].ZeroFillPixelData();
    frame_buffer_cache_[index].SetHasAlpha(false);
  }

  raw_ptr<MockImageDecoderClient> client_;
};

class MockImageDecoderFactory : public ImageDecoderFactory {
 public:
  static std::unique_ptr<MockImageDecoderFactory> Create(
      MockImageDecoderClient* client,
      const SkISize& decoded_size) {
    return base::WrapUnique(new MockImageDecoderFactory(
        client, gfx::Size(decoded_size.width(), decoded_size.height())));
  }

  static std::unique_ptr<MockImageDecoderFactory> Create(
      MockImageDecoderClient* client,
      const gfx::Size& decoded_size) {
    return base::WrapUnique(new MockImageDecoderFactory(client, decoded_size));
  }

  std::unique_ptr<ImageDecoder> Create() override {
    auto decoder = std::make_unique<MockImageDecoder>(client_);
    decoder->SetSize(static_cast<unsigned>(decoded_size_.width()),
                     static_cast<unsigned>(decoded_size_.height()));
    return std::move(decoder);
  }

 private:
  MockImageDecoderFactory(MockImageDecoderClient* client,
                          const gfx::Size& decoded_size)
      : client_(client), decoded_size_(decoded_size) {}

  raw_ptr<MockImageDecoderClient> client_;
  gfx::Size decoded_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_MOCK_IMAGE_DECODER_H_
