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

// (c) Google Inc. All Rights Reserved.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//
// Wrappers for using the Kiss FFT library for one-dimensional FFTs in single-
// precision arithmetic.

#ifndef AUDIO_DSP_KISS_FFT_H_
#define AUDIO_DSP_KISS_FFT_H_

#include <complex>
#include <vector>


#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

class ComplexFFTTransformerImpl;
class RealFFTTransformerImpl;

// Perform complex-to-complex FFT transforms.
class ComplexFFTTransformer {
 public:
  // Construct a transformer for a specified transform size. Kiss FFT is
  // optimized for sizes that only have factors 2, 3, 5, though any positive
  // integer size is supported. The normalization flag specifies scaling
  // on the inverse transform: if normalization is true, then forward followed
  // by inverse transformation yields the original array, see
  // InverseTransform() for details.
  ComplexFFTTransformer(int size, bool normalization);

  ~ComplexFFTTransformer();

  // Get the transform size.
  int GetSize() const;

  // Rounds up to the closest size that is efficient for Kiss FFT to transform,
  // that is, having only factors 2, 3, 5.
  static int GetNextFastSize(int size);

  // Performs the forward complex-to-complex FFT. The input must have size
  // GetSize(). The output has size GetSize() and has elements
  //
  //             size - 1
  // output[k] =   sum     input[n] exp(-i 2 pi k n / size)
  //              n = 0
  //
  // for k = 0, ..., size - 1.
  void ForwardTransform(const std::vector<std::complex<float>>& input,
                        std::vector<std::complex<float>>* output);

  // Raw pointer interface. It is the caller's responsibility to ensure that
  // input has size GetSize() and to allocate output with space for GetSize()
  // results.
  void ForwardTransform(const std::complex<float>* input,
                        std::complex<float>* output);

  // Performs the inverse complex-to-complex FFT. The input must have size
  // GetSize(). If normalization is enabled, the inverse transform is
  // normalized by factor 1/size such that the forward transform followed by
  // the inverse yields the original array,
  //
  //              1    size - 1
  // output[k] = ----    sum     input[n] exp(+i 2 pi k n / size)
  //             size   n = 0
  //
  // for k = 0, ..., size - 1.
  //
  // If normalization is disabled, then the result is
  //
  //             size - 1
  // output[k] =   sum     input[n] exp(+i 2 pi k n / size)
  //              n = 0
  //
  // and forward followed by inverse transformation yields the original array
  // scaled by GetSize(). The size factor can often be absorbed into
  // computations before or after the transforms to save a few multiplies.
  void InverseTransform(const std::vector<std::complex<float>>& input,
                        std::vector<std::complex<float>>* output);

  // Raw pointer interface. It is the caller's responsibility to ensure that
  // input has size GetSize() and to allocate output with space for GetSize()
  // results.
  void InverseTransform(const std::complex<float>* input,
                        std::complex<float>* output);

 private:
  ComplexFFTTransformerImpl* impl_;
  bool normalization_;

  ComplexFFTTransformer(const ComplexFFTTransformer&) = delete;
  ComplexFFTTransformer& operator=(const ComplexFFTTransformer&) = delete;
};

// Perform real-to-complex and complex-to-real FFT transforms.
class RealFFTTransformer {
 public:
  // Construct a transformer for a specified transform size. Any positive size
  // is allowed. Odd sizes are less efficient since Kiss FFT only supports even
  // sizes for real-to-complex and complex-to-real transforms, and
  // complex-to-complex transformation is used internally in this case. The
  // normalization flag determines scaling on the inverse transform, see
  // InverseTransform() for details.
  RealFFTTransformer(int size, bool normalization);

  ~RealFFTTransformer();

  // Get the size of the real data to transform.
  int GetSize() const;

  // Get the size of the transformed complex data, which is GetSize() / 2 + 1
  // where the division is rounded down.
  inline int GetTransformedSize() const {
    return GetSize() / 2 + 1;
  }

  // Rounds up to the closest size that is efficient for Kiss FFT to transform,
  // that is, even and having only factors 2, 3, 5.
  static int GetNextFastSize(int size);

  // Performs the forward real-to-complex FFT. The input must have size
  // GetSize(). The output has size GetTransformedSize() := size / 2 + 1 and
  //
  //             size - 1
  // output[k] =   sum     input[n] exp(-i 2 pi k n / size)
  //              n = 0
  //
  // for k = 0, ..., size / 2. (Only the nonnegative frequencies from DC to
  // Nyquist are computed.)
  void ForwardTransform(const std::vector<float>& input,
                        std::vector<std::complex<float>>* output);

  // Raw pointer interface. It is the caller's responsibility to ensure that
  // input has size GetSize() and to allocate output with space for
  // GetTransformedSize() results.
  void ForwardTransform(const float* input, std::complex<float>* output);

  // Performs the inverse complex-to-real FFT. The input must have size
  // GetTransformedSize(). The output has size GetSize().
  //
  // If normalization is enabled, the inverse transform is normalized by factor
  // 1/size such that the forward transform followed by the inverse yields the
  // original array.
  //
  // If normalization is disabled, then the result is not normalized and
  // forward followed by inverse transformation yields the original array
  // scaled by GetSize().
  void InverseTransform(const std::vector<std::complex<float>>& input,
                        std::vector<float>* output);

  // Raw pointer interface. It is the caller's responsibility to ensure that
  // input has size GetTransformedSize() and to allocate output with space for
  // GetSize() results.
  void InverseTransform(const std::complex<float>* input, float* output);

 private:
  RealFFTTransformerImpl* impl_;
  bool normalization_;

  RealFFTTransformer(const RealFFTTransformer&) = delete;
  RealFFTTransformer& operator=(const RealFFTTransformer&) = delete;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_KISS_FFT_H_
