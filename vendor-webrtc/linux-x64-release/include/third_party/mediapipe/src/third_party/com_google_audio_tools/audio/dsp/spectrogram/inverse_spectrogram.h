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

// A simple inverse spectrogram which can be used in conjunction with
// the class Spectrogram.

// Initialize() should be called before calls to other functions.
// Once Initialize() has been called and returned true, Process() can
// be called repeatedly with sequential input data (ie. the first
// element of the next input vector directly follows the last element
// of the previous input vector). The output vector is populated with
// as many samples as it is possible to reconstruct from the input
// spectrogram slices. This class is thread-unsafe, and should only be
// called from one thread at a time.

#ifndef AUDIO_DSP_SPECTROGRAM_INVERSE_SPECTROGRAM_H_
#define AUDIO_DSP_SPECTROGRAM_INVERSE_SPECTROGRAM_H_

#include <complex>
#include <deque>
#include <vector>

namespace audio_dsp {

class InverseSpectrogram {
 public:
  // Computes a window that can be used in an InverseSpectrogram.
  // Constructs a window that is equal to the forward window with a further
  // pointwise amplitude correction.
  // Args:
  //  - forward_window: should be the same window used for the forward transform
  //  (Spectrogram).
  //  - step_length: should be the same step length used for the forward
  //  transform (Spectrogram).
  // Returns a window suitable for reconstruction original waveform, assuming it
  // was supplied to this class's Initialize method.
  // Should be similar to TensorFlow's tf.signal.inverse_stft_window_fn.
  static std::vector<double> GenerateSynthesisWindow(
      const std::vector<double>& analysis_window, int step_length);

  InverseSpectrogram() : initialized_(false),
                         at_least_one_frame_processed_(false) {}
  ~InverseSpectrogram() {}

  // Initializes the class to expect input spectrogram data generated
  // with a given FFT length and step length (both in samples).
  // Returns true on success, after which calls to Process() are
  // possible.
  bool Initialize(int fft_length, int step_length);

  // Initialize with an explicit synthesis window function instead of a
  // rectangular window. The FFT length is inferred as the next
  // power of 2 of the window's length - i.e. the input spectrograms should be
  // the output of a FFT with that FFT-length and step-length (both in samples).
  // For perfect reconstruction the synthesis window should be the normalized
  // analysis window which was used to create the spectrogram. It can be created
  // with the GenerateSynthesisWindow function.
  bool Initialize(const std::vector<double>& window, int step_length);

  // Processes an arbitrary number of spectrogram frames from input,
  // placing the results in output. Process() may be called repeatedly
  // with new input frames each time. output will be cleared on each
  // call to Process() before being populated with new data.
  // The template is instantiated for the 4 combinations of float and
  // double input and output samples.  It always goes from complex spectrum
  // input to real waveform output.
  template <class InputSample, class OutputSample>
  bool Process(const std::vector<std::vector<std::complex<InputSample>>>& input,
               std::vector<OutputSample>* output);

  // After the end of a sequence of frames has been processed by
  // Process(), any remaining audio data buffered internally by the
  // class may be obtained by calling Flush(). After a call to
  // Flush(), the internal buffers are reset and Process() may be
  // called again as if Initialize() had just been called.
  // With NULL arg, just re-initializes.
  template <class OutputSample>
  bool Flush(std::vector<OutputSample>* output);

 protected:
  // Initializes the class assuming that the fft_length_, frame_size_ and
  // step_length_ have been initialized and are valid values.
  bool InternalInitialize();

  // Returns the value of the window function for a given sample number.
  // For initialization without window a rect window of ones will be used. The
  // window is padded with zeros to fill the whole FFT length.
  double WindowValue(int sample_number) const;

 private:
  void ProcessCoreFFT();

  int fft_length_;
  int frame_size_;   // In samples.
  int step_length_;  // In samples.
  int overlap_;      // In samples.
  bool initialized_;
  bool at_least_one_frame_processed_;

  bool with_window_function_;
  // Used as a synthesis window (after the inverse-FFT). For weighted overlap
  // add (WOLA).
  std::vector<double> window_function_;
  std::vector<double> fft_input_output_;
  std::deque<double>
      working_output_;  // A queue to do waveform overlap-add into.

  // Working data areas for the FFT routines.
  std::vector<int> fft_integer_working_area_;
  std::vector<double> fft_double_working_area_;

  InverseSpectrogram(const InverseSpectrogram&) = delete;
  InverseSpectrogram& operator=(const InverseSpectrogram&) = delete;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_SPECTROGRAM_INVERSE_SPECTROGRAM_H_
