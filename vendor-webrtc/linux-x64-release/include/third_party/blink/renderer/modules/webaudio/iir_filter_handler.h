// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_IIR_FILTER_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_IIR_FILTER_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/webaudio/audio_handler.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AudioNode;
class IIRFilter;

class IIRFilterHandler final : public AudioHandler {
 public:
  static scoped_refptr<IIRFilterHandler> Create(
      AudioNode&,
      float sample_rate,
      const Vector<double>& feedforward_coef,
      const Vector<double>& feedback_coef,
      bool is_filter_stable);
  ~IIRFilterHandler() override;

  // Get the magnitude and phase response of the filter at the given
  // set of frequencies (in Hz). The phase response is in radians.
  void GetFrequencyResponse(int n_frequencies,
                            const float* frequency_hz,
                            float* mag_response,
                            float* phase_response) const;

 private:
  IIRFilterHandler(AudioNode&,
                   float sample_rate,
                   const Vector<double>& feedforward_coef,
                   const Vector<double>& feedback_coef,
                   bool is_filter_stable);

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void ProcessOnlyAudioParams(uint32_t frames_to_process) override {}
  void Initialize() override;
  void Uninitialize() override;
  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;
  bool RequiresTailProcessing() const override;
  double TailTime() const override;
  double LatencyTime() const override;
  void PullInputs(uint32_t frames_to_process) override;

  // Returns true if the first output sample of any channel is non-finite.  This
  // is a proxy for determining if the filter state is bad.  For IIRFilterNodes,
  // if the internal state has non-finite values, the non-finite value
  // propagates pretty much forever in the output.  This is because infinities
  // and NaNs are sticky.
  bool HasNonFiniteOutput() const;

  void NotifyBadState() const;

  // Since `Process()` is called on a different thread than `Initialize()` and
  // `Uninitialize()`, guard access to the processing kernels with a lock.
  mutable base::Lock process_lock_;
  Vector<std::unique_ptr<IIRFilter>> kernels_ GUARDED_BY(process_lock_);

  // The feedback and feedforward filter coefficients for the IIR filter.
  AudioDoubleArray feedback_;
  AudioDoubleArray feedforward_;

  // The Nyquist frequency (half the sampling rate) is used in
  // `GetFrequencyResponse()`.
  const float nyquist_frequency_;

  // Tail time is expensive to calculate for IIR filters.  Since the filter
  // parameters do not change in this class, cache this value during
  // construction.
  double tail_time_;

  // The IIR kernel for computing the frequency response and tail time.
  std::unique_ptr<IIRFilter> response_kernel_;

  // Only notify the user once.  No need to spam the console with messages,
  // because once we're in a bad state, it usually stays that way forever.  Only
  // accessed from audio thread.
  bool did_warn_bad_filter_state_ = false;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  base::WeakPtrFactory<IIRFilterHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_IIR_FILTER_HANDLER_H_
