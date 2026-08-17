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

#ifndef SRC_TRACE_REDACTION_POPULATE_ALLOW_LISTS_H_
#define SRC_TRACE_REDACTION_POPULATE_ALLOW_LISTS_H_

#include "perfetto/base/status.h"
#include "src/trace_redaction/trace_redaction_framework.h"

namespace perfetto::trace_redaction {

// Populates the different allow-lists needed to remove data from the trace.
// Configuration data in the context can be used to change the contents of the
// lists.
class PopulateAllowlists final : public BuildPrimitive {
 public:
  base::Status Build(Context* context) const override;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_POPULATE_ALLOW_LISTS_H_
