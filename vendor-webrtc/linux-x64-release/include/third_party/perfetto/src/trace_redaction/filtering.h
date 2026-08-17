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

#include "src/trace_redaction/trace_redaction_framework.h"

#ifndef SRC_TRACE_REDACTION_FILTERING_H_
#define SRC_TRACE_REDACTION_FILTERING_H_

namespace perfetto::trace_redaction {

class PidFilter {
 public:
  virtual ~PidFilter();

  virtual bool Includes(const Context& context,
                        uint64_t ts,
                        int32_t pid) const = 0;
};

class FtraceEventFilter {
 public:
  virtual ~FtraceEventFilter();
  virtual bool Includes(const Context& context,
                        protozero::Field event) const = 0;
};

class ConnectedToPackage : public PidFilter {
 public:
  bool Includes(const Context& context,
                uint64_t ts,
                int32_t pid) const override;
};

class AllowAll : public PidFilter, public FtraceEventFilter {
 public:
  bool Includes(const Context&, uint64_t, int32_t) const override;

  bool Includes(const Context& context, protozero::Field event) const override;
};

class MatchesPid : public PidFilter {
 public:
  explicit MatchesPid(int32_t pid);
  bool Includes(const Context&, uint64_t, int32_t pid) const override;

 private:
  int32_t pid_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_FILTERING_H_
