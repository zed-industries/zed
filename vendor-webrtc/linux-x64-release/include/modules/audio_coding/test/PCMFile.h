/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_TEST_PCMFILE_H_
#define MODULES_AUDIO_CODING_TEST_PCMFILE_H_

#include <stdio.h>
#include <stdlib.h>

#include <optional>
#include <string>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"

namespace webrtc {

class PCMFile {
 public:
  PCMFile();
  PCMFile(uint32_t timestamp);
  ~PCMFile();

  void Open(absl::string_view filename,
            uint16_t frequency,
            absl::string_view mode,
            bool auto_rewind = false);

  int32_t Read10MsData(AudioFrame& audio_frame);

  void Write10MsData(const int16_t* playout_buffer, size_t length_smpls);
  void Write10MsData(const AudioFrame& audio_frame);

  uint16_t PayloadLength10Ms() const;
  int32_t SamplingFrequency() const;
  void Close();
  bool EndOfFile() const { return end_of_file_; }
  // Moves forward the specified number of 10 ms blocks. If a limit has been set
  // with SetNum10MsBlocksToRead, fast-forwarding does not count towards this
  // limit.
  void FastForward(int num_10ms_blocks);
  void Rewind();
  static int16_t ChooseFile(std::string* file_name,
                            int16_t max_len,
                            uint16_t* frequency_hz);
  bool Rewinded();
  void SaveStereo(bool is_stereo = true);
  void ReadStereo(bool is_stereo = true);
  // If set, the reading will stop after the specified number of blocks have
  // been read. When that has happened, EndOfFile() will return true. Calling
  // Rewind() will reset the counter and start over.
  void SetNum10MsBlocksToRead(int value);

 private:
  FILE* pcm_file_;
  uint16_t samples_10ms_;
  int32_t frequency_;
  bool end_of_file_;
  bool auto_rewind_;
  bool rewinded_;
  uint32_t timestamp_;
  bool read_stereo_;
  bool save_stereo_;
  std::optional<int> num_10ms_blocks_to_read_;
  int blocks_read_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_TEST_PCMFILE_H_
