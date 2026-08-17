/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_LOGGING_MEMORY_LOG_WRITER_H_
#define TEST_LOGGING_MEMORY_LOG_WRITER_H_

#include <map>
#include <memory>
#include <string>
#include <vector>

#include "test/logging/log_writer.h"

namespace webrtc {

// Allows creating log writer factories that creates log writers that saves
// their content to memory. When the log writers are destroyed, their content is
// saved to the logs_ member of this class. The intended usage is to keep this
// class alive after the created factories and writers have been destroyed and
// then use logs() to access all the saved logs.
class MemoryLogStorage {
 public:
  MemoryLogStorage();
  ~MemoryLogStorage();
  std::unique_ptr<LogWriterFactoryInterface> CreateFactory();
  const std::map<std::string, std::string>& logs() { return logs_; }

 private:
  std::map<std::string, std::string> logs_;
};

}  // namespace webrtc

#endif  // TEST_LOGGING_MEMORY_LOG_WRITER_H_
