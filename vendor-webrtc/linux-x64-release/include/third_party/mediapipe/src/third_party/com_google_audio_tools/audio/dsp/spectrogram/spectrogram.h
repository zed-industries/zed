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


// Class for generating spectrogram slices from a waveform.
// http://goto/waveform-erase Used in conjunction with the class
// InverseSpectrogram.  Initialize() should be called before calls to
// other functions.  Once Initialize() has been called and returned
// true, The Compute*() functions can be called repeatedly with
// sequential input data (ie. the first element of the next input
// vector directly follows the last element of the previous input
// vector). Whenever enough audio samples are buffered to produce a
// new frame, it will be placed in output. Output is cleared on each
// call to Compute*(). This class is thread-unsafe, and should only be
// called from one thread at a time.
// With the default parameters, the output of this class should be very
// close to the results of the following MATLAB code:
// overlap_samples = window_length_samples - step_samples;
// window = hann(window_length_samples, 'periodic');
// S = abs(spectrogram(audio, window, overlap_samples)).^2;

#ifndef AUDIO_DSP_SPECTROGRAM_SPECTROGRAM_H_
#define AUDIO_DSP_SPECTROGRAM_SPECTROGRAM_H_

#include <complex>
#include <optional>
#include <deque>
#include <vector>

#include "glog/logging.h"
#include "third_party/fft2d/fft.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {
using std::vector;

class Spectrogram {
 public:
  Spectrogram() : initialized_(false) {}
  ~Spectrogram() {}

  // Initializes the class with a given window length and step length
  // (both in samples). Internally a Hann window is used as the window
  // function. Returns true on success, after which calls to Process()
  // are possible. window_length must be greater than 1 and step
  // length must be greater than 0. fft_length defines the fft length which must
  // be greater than window_length and a power of 2.
  bool Initialize(int window_length, int step_length,
                  std::optional<int> fft_length = std::nullopt);

  // Initialize with an explicit window instead of a length.
  bool Initialize(const std::vector<double>& window, int step_length,
                  std::optional<int> fft_length = std::nullopt);

  // Re-initializes/resets the internal sample buffer to the state before any
  // samples have been passed to the Compute methods.
  bool ResetSampleBuffer();

  // Processes an arbitrary amount of audio data (contained in input)
  // to yield complex spectrogram frames. After a successful call to
  // Initialize(), Process() may be called repeatedly with new input data
  // each time.  The audio input is buffered internally, and the output
  // vector is populated with as many temporally-ordered spectral slices
  // as it is possible to generate from the input.  The output is cleared
  // on each call before the new frames (if any) are added.
  //
  // The template parameters can be float or double.
  template <class InputSample, class OutputSample>
  bool ComputeComplexSpectrogram(
      const vector<InputSample>& input,
      vector<vector<std::complex<OutputSample>>>* output);

  // This function works as the one above, but returns the power
  // (the L2 norm, or the squared magnitude) of each complex value.
  template <class InputSample, class OutputSample>
  bool ComputeSquaredMagnitudeSpectrogram(
      const vector<InputSample>& input,
      vector<vector<OutputSample>>* output);

  // Allow templating of a single function name that emits complex values or
  // magnitude depending on the type of the output pointer.  This allows
  // the caller to support both complex and real output types with templated
  // code.
  template <class InputSample, class OutputSample>
  bool ComputeSpectrogram(
      const vector<InputSample>& input,
      vector<vector<std::complex<OutputSample>>>* output);
  template <class InputSample, class OutputSample>
  bool ComputeSpectrogram(
      const vector<InputSample>& input,
      vector<vector<OutputSample>>* output);

  // Return reference to the window function used internally.
  const vector<double>& GetWindow() const { return window_; }

  // Return the number of frequency channels in the spectrogram.
  int output_frequency_channels() const { return output_frequency_channels_; }

 private:
  template <class InputSample>
  bool GetNextWindowOfSamples(const vector<InputSample>& input,
                              int* input_start);
  void ProcessCoreFFT();

  int fft_length_;
  int output_frequency_channels_;
  int window_length_;
  int step_length_;
  bool initialized_;
  int samples_to_next_step_;

  vector<double> window_;
  vector<double> fft_input_output_;
  std::deque<double> input_queue_;

  // Working data areas for the FFT routines.
  vector<int> fft_integer_working_area_;
  vector<double> fft_double_working_area_;

  Spectrogram(const Spectrogram&) = delete;
  Spectrogram& operator=(const Spectrogram&) = delete;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_SPECTROGRAM_SPECTROGRAM_H_
