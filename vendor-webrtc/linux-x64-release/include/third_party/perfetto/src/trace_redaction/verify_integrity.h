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

#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

#ifndef SRC_TRACE_REDACTION_VERIFY_INTEGRITY_H_
#define SRC_TRACE_REDACTION_VERIFY_INTEGRITY_H_

namespace perfetto::trace_redaction {

// This breaks the normal collect primitive pattern. Rather than collecting
// information, it looks at packets and returns an error if the packet violates
// any requirements.
class VerifyIntegrity : public CollectPrimitive {
 public:
  base::Status Collect(const protos::pbzero::TracePacket::Decoder& packet,
                       Context* context) const override;

 private:
  base::Status OnFtraceEvents(const protozero::ConstBytes bytes) const;

  base::Status OnFtraceEvent(const protozero::ConstBytes bytes) const;

  base::Status OnTraceStats(const protozero::ConstBytes bytes) const;

  base::Status OnBufferStats(const protozero::ConstBytes bytes) const;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_VERIFY_INTEGRITY_H_
