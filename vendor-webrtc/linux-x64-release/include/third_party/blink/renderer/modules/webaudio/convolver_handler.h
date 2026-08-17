// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONVOLVER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONVOLVER_HANDLER_H_

#include <memory>

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class AudioBuffer;
class ConvolverOptions;
class ExceptionState;
class Reverb;
class SharedAudioBuffer;

class MODULES_EXPORT ConvolverHandler final : public AudioHandler {
 public:
  static scoped_refptr<ConvolverHandler> Create(AudioNode&, float sample_rate);
  ~ConvolverHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  // Called in the main thread when the number of channels for the input may
  // have changed.
  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;

  // Impulse responses
  void SetBuffer(AudioBuffer*, ExceptionState&);

  bool Normalize() const { return normalize_; }
  void SetNormalize(bool normalize) { normalize_ = normalize; }
  void SetChannelCount(unsigned, ExceptionState&) final;
  void SetChannelCountMode(V8ChannelCountMode::Enum, ExceptionState&) final;

 private:
  ConvolverHandler(AudioNode&, float sample_rate);

  double TailTime() const override;
  double LatencyTime() const override;
  bool RequiresTailProcessing() const final;

  // Determine how many output channels to use from the number of
  // input channels and the number of channels in the impulse response
  // buffer.
  unsigned ComputeNumberOfOutputChannels(unsigned input_channels,
                                         unsigned response_channels) const;

  std::unique_ptr<Reverb> reverb_ GUARDED_BY(process_lock_);
  std::unique_ptr<SharedAudioBuffer> shared_buffer_ GUARDED_BY(process_lock_);

  // This synchronizes dynamic changes to the convolution impulse response with
  // process().
  mutable base::Lock process_lock_;

  // Normalize the impulse response or not. Must default to true.
  bool normalize_ = true;

  FRIEND_TEST_ALL_PREFIXES(ConvolverNodeTest, ReverbLifetime);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONVOLVER_HANDLER_H_
