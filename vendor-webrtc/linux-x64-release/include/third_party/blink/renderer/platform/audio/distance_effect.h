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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DISTANCE_EFFECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DISTANCE_EFFECT_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Distance models are defined according to the OpenAL specification:
// http://connect.creativelabs.com/openal/Documentation/OpenAL%201.1%20Specification.htm.

class PLATFORM_EXPORT DistanceEffect final {
  DISALLOW_NEW();

 public:
  enum ModelType { kModelLinear = 0, kModelInverse = 1, kModelExponential = 2 };

  DistanceEffect();

  // Returns scalar gain for the given distance the current distance model is
  // used
  double Gain(double distance);

  ModelType Model() { return model_; }

  void SetModel(ModelType model) { model_ = model; }

  // Distance params
  void SetRefDistance(double ref_distance) { ref_distance_ = ref_distance; }
  void SetMaxDistance(double max_distance) { max_distance_ = max_distance; }
  void SetRolloffFactor(double rolloff_factor) {
    rolloff_factor_ = rolloff_factor;
  }

  double RefDistance() const { return ref_distance_; }
  double MaxDistance() const { return max_distance_; }
  double RolloffFactor() const { return rolloff_factor_; }

 protected:
  double LinearGain(double distance);
  double InverseGain(double distance);
  double ExponentialGain(double distance);

  ModelType model_;
  double ref_distance_;
  double max_distance_;
  double rolloff_factor_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_DISTANCE_EFFECT_H_
