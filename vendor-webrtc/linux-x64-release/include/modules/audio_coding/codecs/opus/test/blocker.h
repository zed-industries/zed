/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_OPUS_TEST_BLOCKER_H_
#define MODULES_AUDIO_CODING_CODECS_OPUS_TEST_BLOCKER_H_

#include <memory>

#include "common_audio/channel_buffer.h"
#include "modules/audio_coding/codecs/opus/test/audio_ring_buffer.h"

namespace webrtc {

// The callback function to process audio in the time domain. Input has already
// been windowed, and output will be windowed. The number of input channels
// must be >= the number of output channels.
class BlockerCallback {
 public:
  virtual ~BlockerCallback() {}

  virtual void ProcessBlock(const float* const* input,
                            size_t num_frames,
                            size_t num_input_channels,
                            size_t num_output_channels,
                            float* const* output) = 0;
};

// The main purpose of Blocker is to abstract away the fact that often we
// receive a different number of audio frames than our transform takes. For
// example, most FFTs work best when the fft-size is a power of 2, but suppose
// we receive 20ms of audio at a sample rate of 48000. That comes to 960 frames
// of audio, which is not a power of 2. Blocker allows us to specify the
// transform and all other necessary processing via the Process() callback
// function without any constraints on the transform-size
// (read: `block_size_`) or received-audio-size (read: `chunk_size_`).
// We handle this for the multichannel audio case, allowing for different
// numbers of input and output channels (for example, beamforming takes 2 or
// more input channels and returns 1 output channel). Audio signals are
// represented as deinterleaved floats in the range [-1, 1].
//
// Blocker is responsible for:
// - blocking audio while handling potential discontinuities on the edges
//   of chunks
// - windowing blocks before sending them to Process()
// - windowing processed blocks, and overlap-adding them together before
//   sending back a processed chunk
//
// To use blocker:
// 1. Impelment a BlockerCallback object `bc`.
// 2. Instantiate a Blocker object `b`, passing in `bc`.
// 3. As you receive audio, call b.ProcessChunk() to get processed audio.
//
// A small amount of delay is added to the first received chunk to deal with
// the difference in chunk/block sizes. This delay is <= chunk_size.
//
// Ownership of window is retained by the caller.  That is, Blocker makes a
// copy of window and does not attempt to delete it.
class Blocker {
 public:
  Blocker(size_t chunk_size,
          size_t block_size,
          size_t num_input_channels,
          size_t num_output_channels,
          const float* window,
          size_t shift_amount,
          BlockerCallback* callback);
  ~Blocker();

  void ProcessChunk(const float* const* input,
                    size_t chunk_size,
                    size_t num_input_channels,
                    size_t num_output_channels,
                    float* const* output);

  size_t initial_delay() const { return initial_delay_; }

 private:
  const size_t chunk_size_;
  const size_t block_size_;
  const size_t num_input_channels_;
  const size_t num_output_channels_;

  // The number of frames of delay to add at the beginning of the first chunk.
  const size_t initial_delay_;

  // The frame index into the input buffer where the first block should be read
  // from. This is necessary because shift_amount_ is not necessarily a
  // multiple of chunk_size_, so blocks won't line up at the start of the
  // buffer.
  size_t frame_offset_;

  // Since blocks nearly always overlap, there are certain blocks that require
  // frames from the end of one chunk and the beginning of the next chunk. The
  // input and output buffers are responsible for saving those frames between
  // calls to ProcessChunk().
  //
  // Both contain |initial delay| + `chunk_size` frames. The input is a fairly
  // standard FIFO, but due to the overlap-add it's harder to use an
  // AudioRingBuffer for the output.
  AudioRingBuffer input_buffer_;
  ChannelBuffer<float> output_buffer_;

  // Space for the input block (can't wrap because of windowing).
  ChannelBuffer<float> input_block_;

  // Space for the output block (can't wrap because of overlap/add).
  ChannelBuffer<float> output_block_;

  std::unique_ptr<float[]> window_;

  // The amount of frames between the start of contiguous blocks. For example,
  // `shift_amount_` = `block_size_` / 2 for a Hann window.
  size_t shift_amount_;

  BlockerCallback* callback_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_CODECS_OPUS_TEST_BLOCKER_H_
