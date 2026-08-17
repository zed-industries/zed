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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_CONVOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_CONVOLVER_H_

#include <memory>
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/audio/direct_convolver.h"
#include "third_party/blink/renderer/platform/audio/fft_convolver.h"
#include "third_party/blink/renderer/platform/audio/reverb_accumulation_buffer.h"
#include "third_party/blink/renderer/platform/audio/reverb_convolver_stage.h"
#include "third_party/blink/renderer/platform/audio/reverb_input_buffer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/non_main_thread.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioChannel;

class PLATFORM_EXPORT ReverbConvolver final {
  USING_FAST_MALLOC(ReverbConvolver);

 public:
  // maxFFTSize can be adjusted (from say 2048 to 32768) depending on how much
  // precision is necessary.  For certain tweaky de-convolving applications the
  // phase errors add up quickly and lead to non-sensical results with larger
  // FFT sizes and single-precision floats.  In these cases 2048 is a good
  // size.  If not doing multi-threaded convolution, then should not go > 8192.
  ReverbConvolver(AudioChannel* impulse_response,
                  unsigned render_slice_size,
                  unsigned max_fft_size,
                  size_t convolver_render_phase,
                  bool use_background_threads,
                  float scale);
  ReverbConvolver(const ReverbConvolver&) = delete;
  ReverbConvolver& operator=(const ReverbConvolver&) = delete;
  ~ReverbConvolver();

  void Process(const AudioChannel* source_channel,
               AudioChannel* destination_channel,
               uint32_t frames_to_process);
  void Reset();

  ReverbInputBuffer* InputBuffer() { return &input_buffer_; }

  size_t LatencyFrames() const;

 private:
  void ProcessInBackground();

  Vector<std::unique_ptr<ReverbConvolverStage>> stages_;
  Vector<std::unique_ptr<ReverbConvolverStage>> background_stages_;
  size_t impulse_response_length_;

  ReverbAccumulationBuffer accumulation_buffer_;

  // One or more background threads read from this input buffer which is fed
  // from the realtime thread.
  ReverbInputBuffer input_buffer_;

  // First stage will be of size m_minFFTSize.  Each next stage will be twice as
  // big until we hit m_maxFFTSize.
  unsigned min_fft_size_;
  unsigned max_fft_size_;

  // But don't exceed this size in the real-time thread (if we're doing
  // background processing).
  unsigned max_realtime_fft_size_;

  // Background thread and synchronization
  std::unique_ptr<NonMainThread> background_thread_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_REVERB_CONVOLVER_H_
