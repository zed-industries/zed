/*
 * Copyright (C) 2006 Apple Computer, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_PNG_PNG_IMAGE_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_PNG_PNG_IMAGE_READER_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/image-decoders/image_frame.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/rect.h"

#define PNG_SET_OPTION_SUPPORTED
#include "png.h"

#if !defined(PNG_LIBPNG_VER_MAJOR) || !defined(PNG_LIBPNG_VER_MINOR)
#error version error: compile against a versioned libpng.
#endif

#if PNG_LIBPNG_VER_MAJOR > 1 || \
    (PNG_LIBPNG_VER_MAJOR == 1 && PNG_LIBPNG_VER_MINOR >= 4)
#define JMPBUF(png_ptr) png_jmpbuf(png_ptr)
#else
#define JMPBUF(png_ptr) png_ptr->jmpbuf
#endif

namespace blink {

class FastSharedBufferReader;
class PNGImageDecoder;
class SegmentReader;

class PLATFORM_EXPORT PNGImageReader final {
  USING_FAST_MALLOC(PNGImageReader);

 public:
  PNGImageReader(PNGImageDecoder*, wtf_size_t initial_offset);
  PNGImageReader(const PNGImageReader&) = delete;
  PNGImageReader& operator=(const PNGImageReader&) = delete;
  ~PNGImageReader();

  struct FrameInfo {
    // The offset where the frame data of this frame starts.
    wtf_size_t start_offset;
    // The number of bytes that contain frame data, starting at start_offset.
    wtf_size_t byte_length;
    wtf_size_t duration;
    gfx::Rect frame_rect;
    ImageFrame::DisposalMethod disposal_method;
    ImageFrame::AlphaBlendSource alpha_blend;
  };

  enum class ParseQuery { kSize, kMetaData };

  bool Parse(SegmentReader&, ParseQuery);

  // Returns false on a fatal error.
  bool Decode(SegmentReader&, wtf_size_t);
  const FrameInfo& GetFrameInfo(wtf_size_t) const;

  // Number of complete frames parsed so far; includes frame 0 even if partial.
  wtf_size_t FrameCount() const { return frame_info_.size(); }

  bool ParseCompleted() const { return parse_completed_; }

  bool FrameIsReceivedAtIndex(wtf_size_t index) const {
    if (!index) {
      return FirstFrameFullyReceived();
    }
    return index < FrameCount();
  }

  void ClearDecodeState(wtf_size_t);

  png_structp PngPtr() const { return png_; }
  png_infop InfoPtr() const { return info_; }

  png_bytep InterlaceBuffer() const { return interlace_buffer_.get(); }
  void CreateInterlaceBuffer(int size) {
    interlace_buffer_ = std::make_unique<png_byte[]>(size);
  }
  void ClearInterlaceBuffer() { interlace_buffer_.reset(); }

 private:
  png_structp png_;
  png_infop info_;
  png_uint_32 width_;
  png_uint_32 height_;

  raw_ptr<PNGImageDecoder> decoder_;

  // The offset in the stream where the PNG image starts.
  const wtf_size_t initial_offset_;
  // How many bytes have been read during parsing.
  wtf_size_t read_offset_;
  wtf_size_t progressive_decode_offset_;
  wtf_size_t ihdr_offset_;
  wtf_size_t idat_offset_;

  bool idat_is_part_of_animation_;
  // All IDAT chunks must precede the first fdAT chunk, and all fdAT chunks
  // should be separated from the IDAT chunks by an fcTL chunk. So this is true
  // until the first fcTL chunk after an IDAT chunk. After that, only fdAT
  // chunks are expected.
  bool expect_idats_;
  bool is_animated_;
  bool parsed_signature_;
  bool parsed_ihdr_;
  bool parse_completed_;
  uint32_t reported_frame_count_;
  uint32_t next_sequence_number_;
  // True when an fcTL has been parsed but not its corresponding fdAT or IDAT
  // chunk. Consecutive fcTLs is an error.
  bool fctl_needs_dat_chunk_;
  bool ignore_animation_;

  std::unique_ptr<png_byte[]> interlace_buffer_;

  // Value used for the byte_length of a FrameInfo struct to indicate that it is
  // the first frame and its byte_length is not yet known. 1 is a safe value
  // since the byte_length field of a frame is at least 12.
  static constexpr wtf_size_t kFirstFrameIndicator = 1;

  // Stores information about a frame until it can be pushed to |frame_info|
  // once all the frame data has been read from the stream.
  FrameInfo new_frame_;
  Vector<FrameInfo, 1> frame_info_;

  wtf_size_t ProcessData(const FastSharedBufferReader&,
                         wtf_size_t offset,
                         wtf_size_t length);
  // Returns false on a fatal error.
  bool ParseSize(const FastSharedBufferReader&);
  // Returns false on an error.
  bool ParseFrameInfo(const png_byte* data);
  bool ShouldDecodeWithNewPNG(wtf_size_t) const;
  void StartFrameDecoding(const FastSharedBufferReader&, wtf_size_t);
  // Returns whether the frame was completely decoded.
  bool ProgressivelyDecodeFirstFrame(const FastSharedBufferReader&);
  void DecodeFrame(const FastSharedBufferReader&, wtf_size_t);
  void ProcessFdatChunkAsIdat(png_uint_32 fdat_length);
  // Returns false on a fatal error.
  bool CheckSequenceNumber(const png_byte* position);
  bool FirstFrameFullyReceived() const {
    return !frame_info_.empty() &&
           frame_info_[0].byte_length != kFirstFrameIndicator;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_IMAGE_DECODERS_PNG_PNG_IMAGE_READER_H_
