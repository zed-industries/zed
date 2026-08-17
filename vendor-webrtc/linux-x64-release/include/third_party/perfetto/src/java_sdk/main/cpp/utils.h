/*
 * Copyright (C) 2025 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_JAVA_SDK_MAIN_CPP_UTILS_H_
#define SRC_JAVA_SDK_MAIN_CPP_UTILS_H_

#include <string>

#include <condition_variable>
#include <cstdint>
#include <memory>
#include <mutex>
#include <vector>

#include "perfetto/public/abi/tracing_session_abi.h"

// Copied from src/shared_lib/test/utils.h

namespace perfetto {
namespace java_sdk {
namespace utils {
class WaitableEvent {
 public:
  WaitableEvent() = default;
  void Notify() {
    std::unique_lock<std::mutex> lock(m_);
    notified_ = true;
    cv_.notify_one();
  }
  bool WaitForNotification() {
    std::unique_lock<std::mutex> lock(m_);
    cv_.wait(lock, [this] { return notified_; });
    return notified_;
  }
  bool IsNotified() {
    std::unique_lock<std::mutex> lock(m_);
    return notified_;
  }

 private:
  std::mutex m_;
  std::condition_variable cv_;
  bool notified_ = false;
};

class TracingSession {
 public:
  class Builder {
   public:
    Builder() = default;
    Builder& set_data_source_name(std::string data_source_name) {
      data_source_name_ = std::move(data_source_name);
      return *this;
    }
    Builder& add_enabled_category(std::string category) {
      enabled_categories_.push_back(std::move(category));
      return *this;
    }
    Builder& add_disabled_category(std::string category) {
      disabled_categories_.push_back(std::move(category));
      return *this;
    }
    TracingSession Build();

   private:
    std::string data_source_name_;
    std::vector<std::string> enabled_categories_;
    std::vector<std::string> disabled_categories_;
  };

  static TracingSession Adopt(struct PerfettoTracingSessionImpl*);

  TracingSession(TracingSession&&) noexcept;

  ~TracingSession();

  struct PerfettoTracingSessionImpl* session() const { return session_; }

  bool FlushBlocking(uint32_t timeout_ms);
  // Waits for the tracing session to be stopped.
  void WaitForStopped();
  // Asks the tracing session to stop. Doesn't wait for it to be stopped.
  void StopAsync();
  // Equivalent to StopAsync() + WaitForStopped().
  void StopBlocking();
  std::vector<uint8_t> ReadBlocking();

 private:
  TracingSession() = default;
  struct PerfettoTracingSessionImpl* session_;
  std::unique_ptr<WaitableEvent> stopped_;
};
}  // namespace utils
}  // namespace java_sdk
}  // namespace perfetto

#endif  // SRC_JAVA_SDK_MAIN_CPP_UTILS_H_
