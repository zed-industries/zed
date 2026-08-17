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

#include <cstdint>

#include "protos/perfetto/trace/ftrace/ftrace_event_bundle.pbzero.h"
#include "src/trace_redaction/trace_redaction_framework.h"

#ifndef SRC_TRACE_REDACTION_MODIFY_H_
#define SRC_TRACE_REDACTION_MODIFY_H_

namespace perfetto::trace_redaction {

class PidCommModifier {
 public:
  virtual ~PidCommModifier();
  virtual void Modify(const Context& context,
                      uint64_t ts,
                      int32_t cpu,
                      int32_t* pid,
                      std::string* comm) const = 0;
};

class FtraceEventModifier {
 public:
  virtual ~FtraceEventModifier();
  virtual void Modify(const Context& context,
                      const protos::pbzero::FtraceEventBundle::Decoder& bundle,
                      protozero::Field event,
                      protos::pbzero::FtraceEventBundle* message) const = 0;
};

class ClearComms : public PidCommModifier {
 public:
  void Modify(const Context& context,
              uint64_t ts,
              int32_t cpu,
              int32_t* pid,
              std::string* comm) const override;
};

// Implementation of every type of modifier, allow any modifier to be assigned
// "Do Nothing" as if it was nullptr.
class DoNothing : public PidCommModifier, public FtraceEventModifier {
 public:
  void Modify(const Context& context,
              uint64_t ts,
              int32_t cpu,
              int32_t* pid,
              std::string* comm) const override;

  void Modify(const Context& context,
              const protos::pbzero::FtraceEventBundle::Decoder& bundle,
              protozero::Field event,
              protos::pbzero::FtraceEventBundle* message) const override;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_MODIFY_H_
