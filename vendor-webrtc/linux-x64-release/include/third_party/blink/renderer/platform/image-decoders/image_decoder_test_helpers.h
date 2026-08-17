// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_TEST_HELPERS_H_

#include <memory>

#include "base/containers/span.h"
#include "base/metrics/histogram_base.h"
#include "third_party/blink/renderer/platform/image-decoders/image_decoder.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

class SkBitmap;

namespace blink {
class ImageDecoder;

const char kDecodersTestingDir[] = "renderer/platform/image-decoders/testing";
const unsigned kDefaultSegmentTestSize = 0x1000;
const unsigned kDefaultTestSize = 4 * kDefaultSegmentTestSize;

using DecoderCreator = std::unique_ptr<ImageDecoder> (*)();
using DecoderCreatorWithAlpha =
    std::unique_ptr<ImageDecoder> (*)(ImageDecoder::AlphaOption);

inline void PrepareReferenceData(base::span<char> buffer) {
  for (size_t i = 0; i < buffer.size(); ++i) {
    buffer[i] = static_cast<char>(i);
  }
}

inline void PrepareReferenceData(base::span<uint8_t> buffer) {
  PrepareReferenceData(base::as_writable_chars(buffer));
}

Vector<char> ReadFile(StringView file_name);
Vector<char> ReadFile(const char* dir, const char* file_name);

scoped_refptr<SharedBuffer> ReadFileToSharedBuffer(StringView file_name);
scoped_refptr<SharedBuffer> ReadFileToSharedBuffer(const char* dir,
                                                   const char* file_name);

unsigned HashBitmap(const SkBitmap&);
void CreateDecodingBaseline(DecoderCreator,
                            SharedBuffer*,
                            Vector<unsigned>* baseline_hashes);

void TestByteByByteDecode(DecoderCreator create_decoder,
                          SharedBuffer* data,
                          size_t expected_frame_count,
                          int expected_repetition_count);
void TestByteByByteDecode(DecoderCreator create_decoder,
                          const char* file,
                          size_t expected_frame_count,
                          int expected_repetition_count);
void TestByteByByteDecode(DecoderCreator create_decoder,
                          const char* dir,
                          const char* file,
                          size_t expected_frame_count,
                          int expected_repetition_count);

// |skipping_step| is used to randomize the decoding order. For images with
// a small number of frames (e.g. < 10), this value should be smaller, on the
// order of (number of frames) / 2.
void TestRandomFrameDecode(DecoderCreator,
                           const char* file,
                           size_t skipping_step = 5);
void TestRandomFrameDecode(DecoderCreator,
                           const char* dir,
                           const char* file,
                           size_t skipping_step = 5);

// |skipping_step| is used to randomize the decoding order. For images with
// a small number of frames (e.g. < 10), this value should be smaller, on the
// order of (number of frames) / 2.
void TestRandomDecodeAfterClearFrameBufferCache(DecoderCreator,
                                                const char* file,
                                                size_t skipping_step = 5);
void TestRandomDecodeAfterClearFrameBufferCache(DecoderCreator,
                                                const char* dir,
                                                const char* file,
                                                size_t skipping_step = 5);

void TestDecodeAfterReallocatingData(DecoderCreator, const char* file);
void TestDecodeAfterReallocatingData(DecoderCreator,
                                     const char* dir,
                                     const char* file);
void TestByteByByteSizeAvailable(DecoderCreator,
                                 const char* file,
                                 size_t frame_offset,
                                 bool has_color_space,
                                 int expected_repetition_count);
void TestByteByByteSizeAvailable(DecoderCreator,
                                 const char* dir,
                                 const char* file,
                                 size_t frame_offset,
                                 bool has_color_space,
                                 int expected_repetition_count);

// Data is provided in chunks of length |increment| to the decoder. This value
// can be increased to reduce processing time.
void TestProgressiveDecoding(DecoderCreator,
                             const char* file,
                             size_t increment = 1);
void TestProgressiveDecoding(DecoderCreator,
                             const char* dir,
                             const char* file,
                             size_t increment = 1);

void TestUpdateRequiredPreviousFrameAfterFirstDecode(DecoderCreator,
                                                     const char* dir,
                                                     const char* file);
void TestUpdateRequiredPreviousFrameAfterFirstDecode(DecoderCreator,
                                                     const char* file);

// Verifies that result of alpha blending is similar for AlphaPremultiplied and
// AlphaNotPremultiplied cases.
void TestAlphaBlending(DecoderCreatorWithAlpha, const char*);

// image_type must be "Avif", "Jpeg", or "WebP". If histogram_name is a null
// pointer, the test should not emit to any histogram and `sample` is ignored.
void TestBppHistogram(DecoderCreator create_decoder,
                      const char* image_type,
                      const char* image_name,
                      const char* histogram_name,
                      base::HistogramBase::Sample32 sample);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_IMAGE_DECODER_TEST_HELPERS_H_
