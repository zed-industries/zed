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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PERIODIC_WAVE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PERIODIC_WAVE_H_

#include <memory>

#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class BaseAudioContext;
class ExceptionState;
class PeriodicWaveImpl;
class PeriodicWaveOptions;

class PeriodicWave final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static PeriodicWave* CreateSine(float sample_rate);
  static PeriodicWave* CreateSquare(float sample_rate);
  static PeriodicWave* CreateSawtooth(float sample_rate);
  static PeriodicWave* CreateTriangle(float sample_rate);

  // Creates an arbitrary periodic wave given the frequency components (Fourier
  // coefficients).
  static PeriodicWave* Create(BaseAudioContext&,
                              const Vector<float>& real,
                              const Vector<float>& imag,
                              bool normalize,
                              ExceptionState&);

  static PeriodicWave* Create(BaseAudioContext*,
                              const PeriodicWaveOptions*,
                              ExceptionState&);

  explicit PeriodicWave(float sample_rate);
  ~PeriodicWave() final = default;

  void Trace(Visitor*) const final;

  PeriodicWaveImpl* impl() { return periodic_wave_impl_.Get(); }

 private:
  const Member<PeriodicWaveImpl> periodic_wave_impl_;
};

// PeriodicWaveImpl is not scriptable and thus can never have back references
// to an AudioNode. This allows it to be held strongly from the audio thread
// which avoids converting weak to strong references which is prone to
// GC interference.
class PeriodicWaveImpl final : public GarbageCollected<PeriodicWaveImpl> {
 public:
  explicit PeriodicWaveImpl(float sample_rate);
  ~PeriodicWaveImpl();

  void Trace(Visitor*) const {}

  // Returns pointers to the lower and higher wave data for the pitch range
  // containing the given fundamental frequency. These two tables are in
  // adjacent "pitch" ranges where the higher table will have the maximum number
  // of partials which won't alias when played back at this fundamental
  // frequency. The lower wave is the next range containing fewer partials than
  // the higher wave.  Interpolation between these two tables can be made
  // according to tableInterpolationFactor.
  // Where values from 0 -> 1 interpolate between lower -> higher.
  void WaveDataForFundamentalFrequency(float,
                                       float*& lower_wave_data,
                                       float*& higher_wave_data,
                                       float& table_interpolation_factor);

  // Like the above, except we compute accept 4 frequencies at a time and return
  // 4 lower/higher wave data tables and the 4 corresponding table interpolation
  // factors.  Intended for use with the OscillatorNode for faster a-rate
  // processing.
  void WaveDataForFundamentalFrequency(const float fundamental_frequency[4],
                                       float* lower_wave_data[4],
                                       float* higher_wave_data[4],
                                       float table_interpolation_factor[4]);

  // Returns the scalar multiplier to the oscillator frequency to calculate wave
  // buffer phase increment.
  float RateScale() const { return rate_scale_; }

  // The size of the FFT to use based on the sampling rate.
  unsigned PeriodicWaveSize() const;

  // The number of ranges needed for the given sampling rate and FFT size.
  unsigned NumberOfRanges() const { return number_of_ranges_; }

 private:
  void GenerateBasicWaveform(int);

  float sample_rate_;
  unsigned number_of_ranges_;
  float cents_per_range_;

  // The lowest frequency (in Hertz) where playback will include all of the
  // partials.  Playing back lower than this frequency will gradually lose more
  // high-frequency information.  This frequency is quite low (~10Hz @ 44.1KHz)
  float lowest_fundamental_frequency_;

  float rate_scale_;

  // Maximum possible number of partials (before culling).
  unsigned MaxNumberOfPartials() const;

  unsigned NumberOfPartialsForRange(unsigned range_index) const;

  // Creates tables based on numberOfComponents Fourier coefficients.
  void CreateBandLimitedTables(const float* real,
                               const float* imag,
                               unsigned number_of_components,
                               bool disable_normalization);
  Vector<std::unique_ptr<AudioFloatArray>> band_limited_tables_;

  friend class PeriodicWave;

  V8ExternalMemoryAccounter external_memory_accounter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_PERIODIC_WAVE_H_
