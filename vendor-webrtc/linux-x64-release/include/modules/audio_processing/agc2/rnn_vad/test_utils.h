/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_TEST_UTILS_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_TEST_UTILS_H_

#include <array>
#include <fstream>
#include <memory>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "modules/audio_processing/agc2/rnn_vad/common.h"
#include "rtc_base/checks.h"
#include "rtc_base/numerics/safe_compare.h"

namespace webrtc {
namespace rnn_vad {

constexpr float kFloatMin = std::numeric_limits<float>::min();

// Fails for every pair from two equally sized webrtc::ArrayView<float> views
// such that the values in the pair do not match.
void ExpectEqualFloatArray(ArrayView<const float> expected,
                           ArrayView<const float> computed);

// Fails for every pair from two equally sized webrtc::ArrayView<float> views
// such that their absolute error is above a given threshold.
void ExpectNearAbsolute(ArrayView<const float> expected,
                        ArrayView<const float> computed,
                        float tolerance);

// File reader interface.
class FileReader {
 public:
  virtual ~FileReader() = default;
  // Number of values in the file.
  virtual int size() const = 0;
  // Reads `dst.size()` float values into `dst`, advances the internal file
  // position according to the number of read bytes and returns true if the
  // values are correctly read. If the number of remaining bytes in the file is
  // not sufficient to read `dst.size()` float values, `dst` is partially
  // modified and false is returned.
  virtual bool ReadChunk(ArrayView<float> dst) = 0;
  // Reads a single float value, advances the internal file position according
  // to the number of read bytes and returns true if the value is correctly
  // read. If the number of remaining bytes in the file is not sufficient to
  // read one float, `dst` is not modified and false is returned.
  virtual bool ReadValue(float& dst) = 0;
  // Advances the internal file position by `hop` float values.
  virtual void SeekForward(int hop) = 0;
  // Resets the internal file position to BOF.
  virtual void SeekBeginning() = 0;
};

// File reader for files that contain `num_chunks` chunks with size equal to
// `chunk_size`.
struct ChunksFileReader {
  const int chunk_size;
  const int num_chunks;
  std::unique_ptr<FileReader> reader;
};

// Creates a reader for the PCM S16 samples file.
std::unique_ptr<FileReader> CreatePcmSamplesReader();

// Creates a reader for the 24 kHz pitch buffer test data.
ChunksFileReader CreatePitchBuffer24kHzReader();

// Creates a reader for the LP residual and pitch information test data.
ChunksFileReader CreateLpResidualAndPitchInfoReader();

// Creates a reader for the sequence of GRU input vectors.
std::unique_ptr<FileReader> CreateGruInputReader();

// Creates a reader for the VAD probabilities test data.
std::unique_ptr<FileReader> CreateVadProbsReader();

// Class to retrieve a test pitch buffer content and the expected output for the
// analysis steps.
class PitchTestData {
 public:
  PitchTestData();
  ~PitchTestData();
  ArrayView<const float, kBufSize24kHz> PitchBuffer24kHzView() const {
    return pitch_buffer_24k_;
  }
  ArrayView<const float, kRefineNumLags24kHz> SquareEnergies24kHzView() const {
    return square_energies_24k_;
  }
  ArrayView<const float, kNumLags12kHz> AutoCorrelation12kHzView() const {
    return auto_correlation_12k_;
  }

 private:
  std::array<float, kBufSize24kHz> pitch_buffer_24k_;
  std::array<float, kRefineNumLags24kHz> square_energies_24k_;
  std::array<float, kNumLags12kHz> auto_correlation_12k_;
};

// Writer for binary files.
class FileWriter {
 public:
  explicit FileWriter(absl::string_view file_path)
      : os_(std::string(file_path), std::ios::binary) {}
  FileWriter(const FileWriter&) = delete;
  FileWriter& operator=(const FileWriter&) = delete;
  ~FileWriter() = default;
  void WriteChunk(ArrayView<const float> value) {
    const std::streamsize bytes_to_write = value.size() * sizeof(float);
    os_.write(reinterpret_cast<const char*>(value.data()), bytes_to_write);
  }

 private:
  std::ofstream os_;
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_TEST_UTILS_H_
