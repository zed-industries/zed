// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Utility functions for MediaPipe time series streams.

#ifndef MEDIAPIPE_UTIL_TIME_SERIES_UTIL_H_
#define MEDIAPIPE_UTIL_TIME_SERIES_UTIL_H_

#include <cstdint>
#include <string>
#include <typeinfo>

#include "absl/strings/str_cat.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/formats/time_series_header.pb.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace time_series_util {

// Logs a warning and returns false if the current_timestamp is
// inconsistent with the cumulative_samples that have been processed
// so far, assuming a constant sample_rate and an offset of
// initial_timestamp.
//
// "Special" timestamps are not considered consistent by this
// function.
bool LogWarningIfTimestampIsInconsistent(const Timestamp& current_timestamp,
                                         const Timestamp& initial_timestamp,
                                         int64_t cumulative_samples,
                                         double sample_rate);

// Returns absl::Status::OK if the header is valid. Otherwise, returns a
// Status object with an error message.
absl::Status IsTimeSeriesHeaderValid(const TimeSeriesHeader& header);

// Fills header and returns absl::Status::OK if the header is non-empty and
// valid. Otherwise, returns a Status object with an error message.
absl::Status FillTimeSeriesHeaderIfValid(const Packet& header_packet,
                                         TimeSeriesHeader* header);

// Fills header and returns absl::Status::OK if the header contains a
// non-empty and valid TimeSeriesHeader. Otherwise, returns a Status object with
// an error message.
absl::Status FillMultiStreamTimeSeriesHeaderIfValid(
    const Packet& header_packet, MultiStreamTimeSeriesHeader* header);

// Returns absl::Status::OK iff options contains an extension of type
// OptionsClass.
template <typename OptionsClass>
absl::Status HasOptionsExtension(const CalculatorOptions& options) {
  if (options.HasExtension(OptionsClass::ext)) {
    return absl::OkStatus();
  }
  std::string error_message = "Options proto does not contain extension ";
  absl::StrAppend(&error_message,
                  MediaPipeTypeStringOrDemangled<OptionsClass>());
#ifndef MEDIAPIPE_MOBILE
  // Avoid lite proto APIs on mobile targets.
  absl::StrAppend(&error_message, " : ", options.DebugString());
#endif
  return absl::InvalidArgumentError(error_message);
}

// Returns absl::Status::OK if the shape of 'matrix' is consistent
// with the num_samples and num_channels fields present in 'header'.
// The corresponding matrix dimensions of unset header fields are
// ignored, so e.g. an empty header (which is not valid according to
// FillTimeSeriesHeaderIfValid) is considered consistent with any matrix.
absl::Status IsMatrixShapeConsistentWithHeader(const Matrix& matrix,
                                               const TimeSeriesHeader& header);

template <typename OptionsClass>
void FillOptionsExtensionOrDie(const CalculatorOptions& options,
                               OptionsClass* extension) {
  MEDIAPIPE_CHECK_OK(HasOptionsExtension<OptionsClass>(options));
  extension->CopyFrom(options.GetExtension(OptionsClass::ext));
}

template <typename TimeSeriesHeaderExtensionClass>
bool FillExtensionFromHeader(const TimeSeriesHeader& header,
                             TimeSeriesHeaderExtensionClass* extension) {
  if (header.HasExtension(TimeSeriesHeaderExtensionClass::time_series_ext)) {
    extension->CopyFrom(
        header.GetExtension(TimeSeriesHeaderExtensionClass::time_series_ext));
    return true;
  } else {
    return false;
  }
}

template <typename TimeSeriesHeaderExtensionClass>
void SetExtensionInHeader(const TimeSeriesHeaderExtensionClass& extension,
                          TimeSeriesHeader* header) {
  header->MutableExtension(TimeSeriesHeaderExtensionClass::time_series_ext)
      ->CopyFrom(extension);
}

// Converts from a time_in_seconds to an integer number of samples.
int64_t SecondsToSamples(double time_in_seconds, double sample_rate);

// Converts from an integer number of samples to a time duration in seconds
// spanned by the samples.
double SamplesToSeconds(int64_t num_samples, double sample_rate);

}  // namespace time_series_util
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TIME_SERIES_UTIL_H_
