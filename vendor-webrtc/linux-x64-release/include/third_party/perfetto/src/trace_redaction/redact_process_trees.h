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

#ifndef SRC_TRACE_REDACTION_REDACT_PROCESS_TREES_H_
#define SRC_TRACE_REDACTION_REDACT_PROCESS_TREES_H_

#include "perfetto/base/status.h"
#include "perfetto/protozero/field.h"
#include "src/trace_redaction/redact_sched_events.h"
#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/ps/process_tree.pbzero.h"

namespace perfetto::trace_redaction {

class ProcessTreeModifier {
 public:
  virtual ~ProcessTreeModifier();
  virtual base::Status Modify(const Context& context,
                              protos::pbzero::ProcessTree* message) const = 0;
};

class ProcessTreeDoNothing : public ProcessTreeModifier {
 public:
  base::Status Modify(const Context& context,
                      protos::pbzero::ProcessTree* message) const override;
};

class ProcessTreeCreateSynthThreads : public ProcessTreeModifier {
 public:
  base::Status Modify(const Context& context,
                      protos::pbzero::ProcessTree* message) const override;
};

// Removes threads and processes from the process tree based on whether or not
// they are connected to the target package.
class RedactProcessTrees : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

  template <class Filter>
  void emplace_filter() {
    filter_ = std::make_unique<Filter>();
  }

  template <class Builder>
  void emplace_modifier() {
    modifier_ = std::make_unique<Builder>();
  }

 private:
  base::Status OnProcessTree(const Context& context,
                             uint64_t ts,
                             protozero::ConstBytes bytes,
                             protos::pbzero::ProcessTree* message) const;

  base::Status OnProcess(const Context& context,
                         uint64_t ts,
                         protozero::Field field,
                         protos::pbzero::ProcessTree* message) const;

  base::Status OnThread(const Context& context,
                        uint64_t ts,
                        protozero::Field field,
                        protos::pbzero::ProcessTree* message) const;

  base::Status AppendSynthThreads(const Context& context,
                                  protos::pbzero::ProcessTree* message) const;

  std::unique_ptr<PidFilter> filter_;
  std::unique_ptr<ProcessTreeModifier> modifier_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_REDACT_PROCESS_TREES_H_
