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

#ifndef SRC_TRACE_REDACTION_COLLECT_FRAME_COOKIES_H_
#define SRC_TRACE_REDACTION_COLLECT_FRAME_COOKIES_H_

#include "perfetto/protozero/field.h"
#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto::trace_redaction {

// Populates Context::global_frame_cookies using FrameTimelineEvent messages.
class CollectFrameCookies : public CollectPrimitive {
 public:
  base::Status Begin(Context* context) const override;

  base::Status Collect(const protos::pbzero::TracePacket::Decoder& packet,
                       Context* context) const override;

 private:
  void OnTimelineEvent(const protos::pbzero::TracePacket::Decoder& packet,
                       protozero::ConstBytes bytes,
                       Context* context) const;
};

// Moves cookies from Context::global_frame_cookies to
// Context::package_frame_cookies using Cookies::timeline and
// Cookies::package_uid.
class ReduceFrameCookies : public BuildPrimitive {
 public:
  base::Status Build(Context* context) const override;
};

class FilterFrameEvents : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

 private:
  bool KeepField(const Context& context, const protozero::Field& field) const;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_COLLECT_FRAME_COOKIES_H_
