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

#ifndef SRC_TRACE_REDACTION_COLLECT_SYSTEM_INFO_H_
#define SRC_TRACE_REDACTION_COLLECT_SYSTEM_INFO_H_

#include "src/trace_redaction/trace_redaction_framework.h"

namespace perfetto::trace_redaction {

// Collects system info (e.g. tids and cpu info). These will provide the raw
// material needed by BuildThreadMap.
class CollectSystemInfo : public CollectPrimitive {
 public:
  base::Status Begin(Context*) const override;

  base::Status Collect(const protos::pbzero::TracePacket::Decoder&,
                       Context*) const override;

 private:
  base::Status OnFtraceEvents(protozero::ConstBytes bytes,
                              Context* context) const;
};

// Condenses system info into a query-focuesed structure, making it possible to
// replace a thread with a synthetic thread.
//
// This is done here, and not in CollectSystemInfo::End, so that other collect
// primitives can report additional system information.
class BuildSyntheticThreads : public BuildPrimitive {
 public:
  base::Status Build(Context* context) const override;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_COLLECT_SYSTEM_INFO_H_
