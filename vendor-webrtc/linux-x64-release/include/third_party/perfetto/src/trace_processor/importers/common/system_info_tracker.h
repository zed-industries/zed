/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SYSTEM_INFO_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SYSTEM_INFO_TRACKER_H_

#include <optional>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/types/version_number.h"

namespace perfetto {
namespace trace_processor {

class SystemInfoTracker : public Destructible {
 public:
  ~SystemInfoTracker() override;

  SystemInfoTracker(const SystemInfoTracker&) = delete;
  SystemInfoTracker& operator=(const SystemInfoTracker&) = delete;

  static SystemInfoTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->system_info_tracker) {
      context->system_info_tracker.reset(new SystemInfoTracker());
    }
    return static_cast<SystemInfoTracker*>(context->system_info_tracker.get());
  }

  void SetKernelVersion(base::StringView name, base::StringView release);
  void SetNumCpus(uint32_t num_cpus) { num_cpus_ = num_cpus; }

  std::optional<VersionNumber> GetKernelVersion() const { return version_; }
  std::optional<uint32_t> GetNumCpus() const { return num_cpus_; }

 private:
  explicit SystemInfoTracker();

  std::optional<VersionNumber> version_;
  std::optional<uint32_t> num_cpus_;
};
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SYSTEM_INFO_TRACKER_H_
