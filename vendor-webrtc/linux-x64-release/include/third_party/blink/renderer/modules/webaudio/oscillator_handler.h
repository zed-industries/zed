// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OSCILLATOR_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OSCILLATOR_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_oscillator_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_oscillator_type.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/audio_scheduled_source_node.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

namespace blink {

class BaseAudioContext;
class ExceptionState;
class OscillatorOptions;
class PeriodicWave;
class PeriodicWaveImpl;

class OscillatorHandler final : public AudioScheduledSourceHandler {
 public:
  // The waveform type.
  // These must be defined as in the .idl file.
  enum : uint8_t {
    SINE = 0,
    SQUARE = 1,
    SAWTOOTH = 2,
    TRIANGLE = 3,
    CUSTOM = 4
  };

  // Breakpoints where we decide to do linear interpolation, 3-point
  // interpolation or 5-point interpolation.  See DoInterpolation().
  static constexpr float kInterpolate2Point = 0.3f;
  static constexpr float kInterpolate3Point = 0.16f;

  static scoped_refptr<OscillatorHandler> Create(AudioNode&,
                                                 float sample_rate,
                                                 const String& oscillator_type,
                                                 PeriodicWaveImpl* wave_table,
                                                 AudioParamHandler& frequency,
                                                 AudioParamHandler& detune);
  ~OscillatorHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;

  V8OscillatorType::Enum GetType() const;
  void SetType(V8OscillatorType::Enum, ExceptionState&);

  void SetPeriodicWave(PeriodicWaveImpl*);

  void HandleStoppableSourceNode() override;

 private:
  OscillatorHandler(AudioNode&,
                    float sample_rate,
                    const String& oscillator_type,
                    PeriodicWaveImpl* wave_table,
                    AudioParamHandler& frequency,
                    AudioParamHandler& detune);

  bool SetType(uint8_t);  // Returns true on success.

  // Returns true if there are sample-accurate timeline parameter changes.
  bool CalculateSampleAccuratePhaseIncrements(uint32_t frames_to_process);

  bool PropagatesSilence() const override;

  base::WeakPtr<AudioScheduledSourceHandler> AsWeakPtr() override;

  // Compute the output for k-rate AudioParams
  double ProcessKRate(int n, float* dest_p, double virtual_read_index) const;

  // Scalar version for the main loop in ProcessKRate().  Returns the updated
  // virtual_read_index.
  double ProcessKRateScalar(int start_index,
                            int n,
                            float* dest_p,
                            double virtual_read_index,
                            float frequency,
                            float rate_scale) const;

  // Vectorized version (if available) for the main loop in ProcessKRate().
  // Returns the number of elements processed and the updated
  // virtual_read_index.
  std::tuple<int, double> ProcessKRateVector(int n,
                                             float* dest_p,
                                             double virtual_read_index,
                                             float frequency,
                                             float rate_scale) const;

  // Compute the output for a-rate AudioParams
  double ProcessARate(int n,
                      float* dest_p,
                      double virtual_read_index,
                      float* phase_increments) const;

  // Scalar version of ProcessARate().  Also handles any values not handled by
  // the vector version.
  //
  //   k
  //     start index for where to write the result (and read phase_increments)
  //   n
  //     total number of frames to process
  //   destination
  //     Array where the samples values are written
  //   virtual_read_index
  //     index into the wave data tables containing the waveform
  //   phase_increments
  //     phase change to use for each frame of output
  //
  // Returns the updated virtual_read_index.
  double ProcessARateScalar(int k,
                            int n,
                            float* destination,
                            double virtual_read_index,
                            const float* phase_increments) const;

  // Vector version of ProcessARate().  Returns the number of frames processed
  // and the update virtual_read_index.
  std::tuple<int, double> ProcessARateVector(
      int n,
      float* destination,
      double virtual_read_index,
      const float* phase_increments) const;

  // Handles the linear interpolation in ProcessARateVector().
  //
  //   destination
  //     Where the interpolated values are written.
  //   virtual_read_index
  //     index into the wave table data
  //   phase_increments
  //     phase increments array
  //   periodic_wave_size
  //     Length of the periodic wave stored in the wave tables
  //   lower_wave_data
  //     Array of the 4 lower wave table arrays
  //   higher_wave_data
  //     Array of the 4 higher wave table arrays
  //   table_interpolation_factor
  //     Array of linear interpolation factors to use between the lower and
  //     higher wave tables.
  //
  // Returns the updated virtual_read_index
  double ProcessARateVectorKernel(
      float* destination,
      double virtual_read_index,
      const float* phase_increments,
      unsigned periodic_wave_size,
      const float* const lower_wave_data[4],
      const float* const higher_wave_data[4],
      const float table_interpolation_factor[4]) const;

  // One of the waveform types defined in the enum.
  uint8_t type_;

  // Frequency value in Hertz.
  scoped_refptr<AudioParamHandler> frequency_;

  // Detune value (deviating from the frequency) in Cents.
  scoped_refptr<AudioParamHandler> detune_;

  bool first_render_ = true;

  // m_virtualReadIndex is a sample-frame index into our buffer representing the
  // current playback position.  Since it's floating-point, it has sub-sample
  // accuracy.
  double virtual_read_index_ = 0;

  // Stores sample-accurate values calculated according to frequency and detune.
  AudioFloatArray phase_increments_;
  AudioFloatArray detune_values_;

  // PeriodicWaveImpl cannot cause cycles with OscillatorNode as it is not
  // scriptable.
  CrossThreadPersistent<PeriodicWaveImpl> periodic_wave_;

  base::WeakPtrFactory<AudioScheduledSourceHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OSCILLATOR_HANDLER_H_
