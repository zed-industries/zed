/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACING_TEST_API_TEST_SUPPORT_H_
#define SRC_TRACING_TEST_API_TEST_SUPPORT_H_

// This header is intended to be used only for api_integrationtest.cc and solves
// the following problem: api_integrationtest.cc doesn't pull any non-public
// perfetto header, to spot accidental public->non-public dependencies.
// Sometimes, however, we need to use some internal perfetto code for the test
// itself. This header exposes wrapper functions to achieve that without leaking
// internal headers.

//  IMPORTANT: This header must not pull any non-public perfetto header.

#include <stdint.h>

#include <functional>
#include <string>

#include "perfetto/tracing.h"

namespace perfetto {
namespace test {

int32_t GetCurrentProcessId();

// RAII wrapper to start and stop an in process system service. Only one at a
// time can be started.
class SystemService {
 public:
  static SystemService Start();
  SystemService() = default;
  SystemService(SystemService&& other) noexcept { *this = std::move(other); }
  SystemService& operator=(SystemService&&) noexcept;

  ~SystemService() { Clean(); }

  // Returns true if this SystemService has been started successfully and can be
  // used.
  bool valid() const { return valid_; }

  void Clean();

  // Restarts this SystemService. Producer and consumers will be disconnected.
  void Restart();

 private:
  SystemService(const SystemService&) = delete;
  SystemService& operator=(const SystemService&) = delete;
  bool valid_ = false;
};

void SyncProducers();

void SetBatchCommitsDuration(uint32_t batch_commits_duration_ms,
                             BackendType backend_type);

bool EnableDirectSMBPatching(BackendType backend_type);

void DisableReconnectLimit();

struct TestTempFile {
  int fd;
  std::string path;
};

// The caller must close(2) the returned TempFile.fd.
TestTempFile CreateTempFile();

class DataSourceInternalForTest {
 public:
  template <typename DerivedDataSource>
  static void ClearTlsState() {
    internal::DataSourceThreadLocalState*& tls_state =
        DerivedDataSource::tls_state_;
    if (tls_state) {
      tls_state = nullptr;
    }
  }
};

class TracingMuxerImplInternalsForTest {
 public:
  static bool DoesSystemBackendHaveSMB();
  static void ClearIncrementalState();

  template <typename DerivedDataSource>
  static void ClearDataSourceTlsStateOnReset() {
    AppendResetForTestingCallback(
        [] { DataSourceInternalForTest::ClearTlsState<DerivedDataSource>(); });
  }

 private:
  static void AppendResetForTestingCallback(std::function<void()>);
};

}  // namespace test
}  // namespace perfetto

#endif  // SRC_TRACING_TEST_API_TEST_SUPPORT_H_
