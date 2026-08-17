/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_LOGGING_APM_DATA_DUMPER_H_
#define MODULES_AUDIO_PROCESSING_LOGGING_APM_DATA_DUMPER_H_

#include <stdint.h>
#include <stdio.h>

#if WEBRTC_APM_DEBUG_DUMP == 1
#include <memory>
#include <string>
#include <unordered_map>
#endif

#include <optional>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#if WEBRTC_APM_DEBUG_DUMP == 1
#include "common_audio/wav_file.h"
#include "rtc_base/checks.h"
#include "rtc_base/string_utils.h"
#endif

// Check to verify that the define is properly set.
#if !defined(WEBRTC_APM_DEBUG_DUMP) || \
    (WEBRTC_APM_DEBUG_DUMP != 0 && WEBRTC_APM_DEBUG_DUMP != 1)
#error "Set WEBRTC_APM_DEBUG_DUMP to either 0 or 1"
#endif

namespace webrtc {

#if WEBRTC_APM_DEBUG_DUMP == 1
// Functor used to use as a custom deleter in the map of file pointers to raw
// files.
struct RawFileCloseFunctor {
  void operator()(FILE* f) const { fclose(f); }
};
#endif

// Class that handles dumping of variables into files.
class ApmDataDumper {
 public:
  // Constructor that takes an instance index that may
  // be used to distinguish data dumped from different
  // instances of the code.
  explicit ApmDataDumper(int instance_index);

  ApmDataDumper() = delete;
  ApmDataDumper(const ApmDataDumper&) = delete;
  ApmDataDumper& operator=(const ApmDataDumper&) = delete;

  ~ApmDataDumper();

  // Activates or deactivate the dumping functionality.
  static void SetActivated([[maybe_unused]] bool activated) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    recording_activated_ = activated;
#endif
  }

  // Returns whether dumping functionality is enabled/available.
  static bool IsAvailable() {
#if WEBRTC_APM_DEBUG_DUMP == 1
    return true;
#else
    return false;
#endif
  }

  // Default dump set.
  static constexpr size_t kDefaultDumpSet = 0;

  // Specifies what dump set to use. All dump commands with a different dump set
  // than the one specified will be discarded. If not specificed, all dump sets
  // will be used.
  static void SetDumpSetToUse([[maybe_unused]] int dump_set_to_use) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    dump_set_to_use_ = dump_set_to_use;
#endif
  }

  // Set an optional output directory.
  static void SetOutputDirectory(
      [[maybe_unused]] absl::string_view output_dir) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    RTC_CHECK_LT(output_dir.size(), kOutputDirMaxLength);
    webrtc::strcpyn(output_dir_, kOutputDirMaxLength, output_dir);
#endif
  }

  // Reinitializes the data dumping such that new versions
  // of all files being dumped to are created.
  void InitiateNewSetOfRecordings() {
#if WEBRTC_APM_DEBUG_DUMP == 1
    ++recording_set_index_;
#endif
  }

  // Methods for performing dumping of data of various types into
  // various formats.
  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] double v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(&v, sizeof(v), 1, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const double* v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(v, sizeof(v[0]), v_length, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] ArrayView<const double> v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpRaw(name, v.size(), v.data());
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] float v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(&v, sizeof(v), 1, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const float* v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(v, sizeof(v[0]), v_length, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] ArrayView<const float> v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpRaw(name, v.size(), v.data());
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] bool v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpRaw(name, static_cast<int16_t>(v));
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const bool* v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      for (size_t k = 0; k < v_length; ++k) {
        int16_t value = static_cast<int16_t>(v[k]);
        fwrite(&value, sizeof(value), 1, file);
      }
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] ArrayView<const bool> v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpRaw(name, v.size(), v.data());
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] int16_t v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(&v, sizeof(v), 1, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const int16_t* v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(v, sizeof(v[0]), v_length, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] ArrayView<const int16_t> v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpRaw(name, v.size(), v.data());
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] int32_t v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(&v, sizeof(v), 1, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const int32_t* v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(v, sizeof(v[0]), v_length, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(&v, sizeof(v), 1, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const size_t* v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      FILE* file = GetRawFile(name);
      fwrite(v, sizeof(v[0]), v_length, file);
    }
#endif
  }

  void DumpRaw([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] ArrayView<const int32_t> v,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpRaw(name, v.size(), v.data());
    }
#endif
  }

  void DumpRaw(absl::string_view name,
               ArrayView<const size_t> v,
               int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    DumpRaw(name, v.size(), v.data());
#endif
  }

  void DumpWav([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] size_t v_length,
               [[maybe_unused]] const float* v,
               [[maybe_unused]] int sample_rate_hz,
               [[maybe_unused]] int num_channels,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      WavWriter* file = GetWavFile(name, sample_rate_hz, num_channels,
                                   WavFile::SampleFormat::kFloat);
      file->WriteSamples(v, v_length);
    }
#endif
  }

  void DumpWav([[maybe_unused]] absl::string_view name,
               [[maybe_unused]] ArrayView<const float> v,
               [[maybe_unused]] int sample_rate_hz,
               [[maybe_unused]] int num_channels,
               [[maybe_unused]] int dump_set = kDefaultDumpSet) {
#if WEBRTC_APM_DEBUG_DUMP == 1
    if (dump_set_to_use_ && *dump_set_to_use_ != dump_set)
      return;

    if (recording_activated_) {
      DumpWav(name, v.size(), v.data(), sample_rate_hz, num_channels);
    }
#endif
  }

 private:
#if WEBRTC_APM_DEBUG_DUMP == 1
  static bool recording_activated_;
  static std::optional<int> dump_set_to_use_;
  static constexpr size_t kOutputDirMaxLength = 1024;
  static char output_dir_[kOutputDirMaxLength];
  const int instance_index_;
  int recording_set_index_ = 0;
  std::unordered_map<std::string, std::unique_ptr<FILE, RawFileCloseFunctor>>
      raw_files_;
  std::unordered_map<std::string, std::unique_ptr<WavWriter>> wav_files_;

  FILE* GetRawFile(absl::string_view name);
  WavWriter* GetWavFile(absl::string_view name,
                        int sample_rate_hz,
                        int num_channels,
                        WavFile::SampleFormat format);
#endif
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_LOGGING_APM_DATA_DUMPER_H_
