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

#ifndef SRC_TRACE_REDACTION_SCRUB_PROCESS_STATS_H_
#define SRC_TRACE_REDACTION_SCRUB_PROCESS_STATS_H_

#include <memory>

#include "src/trace_redaction/redact_sched_events.h"
#include "src/trace_redaction/trace_redaction_framework.h"

namespace perfetto::trace_redaction {

class ScrubProcessStats : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

  template <class Filter>
  void emplace_filter() {
    filter_ = std::make_unique<Filter>();
  }

 private:
  base::Status OnProcessStats(const Context& context,
                              uint64_t ts,
                              protozero::ConstBytes bytes,
                              protos::pbzero::ProcessStats* message) const;

  base::Status OnProcess(const Context& context,
                         uint64_t ts,
                         protozero::Field field,
                         protos::pbzero::ProcessStats* message) const;

  std::unique_ptr<PidFilter> filter_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_SCRUB_PROCESS_STATS_H_
