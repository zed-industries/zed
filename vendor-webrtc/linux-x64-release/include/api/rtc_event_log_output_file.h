/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_RTC_EVENT_LOG_OUTPUT_FILE_H_
#define API_RTC_EVENT_LOG_OUTPUT_FILE_H_

#include <stddef.h>
#include <stdio.h>

#include <string>

#include "absl/strings/string_view.h"
#include "api/rtc_event_log_output.h"
#include "rtc_base/system/file_wrapper.h"

namespace webrtc {

class RtcEventLogOutputFile final : public RtcEventLogOutput {
 public:
  static const size_t kMaxReasonableFileSize;  // Explanation at declaration.

  // Unlimited/limited-size output file (by filename).
  explicit RtcEventLogOutputFile(const std::string& file_name);
  RtcEventLogOutputFile(const std::string& file_name, size_t max_size_bytes);

  // Limited-size output file (by FILE*). This class takes ownership
  // of the FILE*, and closes it on destruction.
  RtcEventLogOutputFile(FILE* file, size_t max_size_bytes);

  ~RtcEventLogOutputFile() override = default;

  bool IsActive() const override;

  bool Write(absl::string_view output) override;

 private:
  RtcEventLogOutputFile(FileWrapper file, size_t max_size_bytes);

  // IsActive() can be called either from outside or from inside, but we don't
  // want to incur the overhead of a virtual function call if called from inside
  // some other function of this class.
  inline bool IsActiveInternal() const;

  // Maximum size, or zero for no limit.
  const size_t max_size_bytes_;
  size_t written_bytes_{0};
  FileWrapper file_;
};

}  // namespace webrtc

#endif  // API_RTC_EVENT_LOG_OUTPUT_FILE_H_
