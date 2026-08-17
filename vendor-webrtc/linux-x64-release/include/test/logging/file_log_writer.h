/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_LOGGING_FILE_LOG_WRITER_H_
#define TEST_LOGGING_FILE_LOG_WRITER_H_

#include <cstdio>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "test/logging/log_writer.h"

namespace webrtc {
namespace webrtc_impl {
class FileLogWriter final : public RtcEventLogOutput {
 public:
  explicit FileLogWriter(absl::string_view file_path);
  ~FileLogWriter() final;
  bool IsActive() const override;
  bool Write(absl::string_view value) override;
  void Flush() override;

 private:
  std::FILE* const out_;
};
}  // namespace webrtc_impl
class FileLogWriterFactory final : public LogWriterFactoryInterface {
 public:
  explicit FileLogWriterFactory(absl::string_view base_path);
  ~FileLogWriterFactory() final;

  std::unique_ptr<RtcEventLogOutput> Create(
      absl::string_view filename) override;

 private:
  const std::string base_path_;
  std::vector<std::unique_ptr<webrtc_impl::FileLogWriter>> writers_;
};

}  // namespace webrtc

#endif  // TEST_LOGGING_FILE_LOG_WRITER_H_
