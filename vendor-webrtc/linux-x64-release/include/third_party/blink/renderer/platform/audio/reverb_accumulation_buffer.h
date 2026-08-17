/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_ACCUMULATION_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_ACCUMULATION_BUFFER_H_

#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// ReverbAccumulationBuffer is a circular delay buffer with one client reading
// from it and multiple clients writing/accumulating to it at different delay
// offsets from the read position.  The read operation will zero the memory
// just read from the buffer, so it will be ready for accumulation the next
// time around.
class ReverbAccumulationBuffer final {
  DISALLOW_NEW();

 public:
  explicit ReverbAccumulationBuffer(uint32_t length);
  ReverbAccumulationBuffer(const ReverbAccumulationBuffer&) = delete;
  ReverbAccumulationBuffer& operator=(const ReverbAccumulationBuffer&) = delete;

  // This will read from, then clear-out numberOfFrames
  void ReadAndClear(float* destination, uint32_t number_of_frames);

  // Each ReverbConvolverStage will accumulate its output at the appropriate
  // delay from the read position.  We need to pass in and update readIndex
  // here, since each ReverbConvolverStage may be running in a different thread
  // than the realtime thread calling ReadAndClear() and maintaining
  // m_readIndex
  // Returns the writeIndex where the accumulation took place
  uint32_t Accumulate(float* source,
                      uint32_t number_of_frames,
                      uint32_t* read_index,
                      size_t delay_frames);

  uint32_t ReadIndex() const { return read_index_; }
  void UpdateReadIndex(uint32_t* read_index, uint32_t number_of_frames) const;

  uint32_t ReadTimeFrame() const { return read_time_frame_; }

  void Reset();

 private:
  AudioFloatArray buffer_;
  uint32_t read_index_;
  uint32_t read_time_frame_;  // for debugging (frame on continuous timeline)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_ACCUMULATION_BUFFER_H_
