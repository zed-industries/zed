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

#ifndef SRC_TRACE_REDACTION_BROADPHASE_PACKET_FILTER_H_
#define SRC_TRACE_REDACTION_BROADPHASE_PACKET_FILTER_H_

#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/ftrace/ftrace_event.pbzero.h"
#include "protos/perfetto/trace/ftrace/ftrace_event_bundle.pbzero.h"

namespace perfetto::trace_redaction {

// Execute a broad-phase filter here and defer a narrow-phase filter via other
// primitives.
//
// The concepts of broad-phase and narrow-phase are borrowed from the graphics
// space where a cheap operations removes large chunks of information
// (broad-phase) so that less information goes through the more operations
// (narrow-phase).
//
// Here, the broad-phase operation is a filter that removes high-level fields
// from trace packets so that other primitives (narrow-phase operations) have
// fewer fields to read and write.
class BroadphasePacketFilter : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

 private:
  void OnFtraceEvents(const Context& context,
                      protozero::ConstBytes bytes,
                      protos::pbzero::FtraceEventBundle* message) const;

  void OnFtraceEvent(const Context& context,
                     protozero::ConstBytes bytes,
                     protos::pbzero::FtraceEvent* message) const;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_BROADPHASE_PACKET_FILTER_H_
