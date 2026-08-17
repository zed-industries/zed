/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ERROR_LOGGER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ERROR_LOGGER_H_

#include <optional>
#include <string>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/importers/etm/opencsd.h"

namespace perfetto::trace_processor::etm {
class ErrorLogger : public ITraceErrorLog {
 public:
#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wignored-qualifiers"
#elif defined(__GNUC__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wignored-qualifiers"
#endif
  const ocsd_hndl_err_log_t RegisterErrorSource(
      const std::string& component_name) override;
  const ocsd_err_severity_t GetErrorLogVerbosity() const override;
#if defined(__clang__)
#pragma clang diagnostic pop
#elif defined(__GNUC__)
#pragma GCC diagnostic pop
#endif

  void LogError(const ocsd_hndl_err_log_t handle,
                const ocsdError* error) override;
  void LogMessage(const ocsd_hndl_err_log_t handle,
                  const ocsd_err_severity_t filter_level,
                  const std::string& msg) override;

  ocsdError* GetLastError() override;
  ocsdError* GetLastIDError(const uint8_t chan_id) override;
  ocsdMsgLogger* getOutputLogger() override;
  void setOutputLogger(ocsdMsgLogger* logger) override;

  base::Status ToStatus(ocsd_err_t rc) {
    if (PERFETTO_LIKELY(rc == OCSD_OK)) {
      return base::OkStatus();
    }
    return ToError(rc);
  }

  base::StatusOr<bool> ToErrorOrKeepGoing(ocsd_datapath_resp_t resp);

 private:
  base::Status ToError(ocsd_err_t rc);
  base::Status ToError(ocsd_datapath_resp_t resp);

  std::vector<std::string> components_;
  std::optional<ocsdError> last_error_;
  base::FlatHashMap<uint8_t, ocsdError> last_error_by_channel_id_;
};
}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_ERROR_LOGGER_H_
