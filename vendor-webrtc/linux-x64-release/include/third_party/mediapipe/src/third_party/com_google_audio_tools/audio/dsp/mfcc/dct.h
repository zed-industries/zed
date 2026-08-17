/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Basic minimal DCT class.
#ifndef AUDIO_DSP_MFCC_DCT_H_
#define AUDIO_DSP_MFCC_DCT_H_

#include <vector>


#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

class Dct {
 public:
  Dct();
  bool Initialize(int input_length, int coefficient_count);
  void Compute(const std::vector<double>& input,
               std::vector<double>* output) const;

 private:
  bool initialized_;
  int coefficient_count_;
  int input_length_;
  std::vector<std::vector<double> > cosines_;

  Dct(const Dct&) = delete;
  Dct& operator=(const Dct&) = delete;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_MFCC_DCT_H_
