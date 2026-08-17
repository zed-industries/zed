/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_TRACING_POLICY_H_
#define INCLUDE_PERFETTO_TRACING_TRACING_POLICY_H_

#include <functional>

#include "perfetto/base/export.h"
#include "perfetto/tracing/backend_type.h"

namespace perfetto {

// Applies policy decisions, such as allowing or denying connections, when
// certain tracing SDK events occur. All methods are called on an internal
// perfetto thread.
class PERFETTO_EXPORT_COMPONENT TracingPolicy {
 public:
  virtual ~TracingPolicy();

  // Called when the current process attempts to connect a new consumer to the
  // backend of |backend_type| to check if the connection should be allowed. Its
  // implementation should execute |result_callback| with the result of the
  // check (synchronuosly or asynchronously on any thread). If the result is
  // false, the consumer connection is aborted. Chrome uses this to restrict
  // creating (system) tracing sessions based on an enterprise policy.
  struct ShouldAllowConsumerSessionArgs {
    BackendType backend_type;
    std::function<void(bool /*allow*/)> result_callback;
  };
  virtual void ShouldAllowConsumerSession(
      const ShouldAllowConsumerSessionArgs&) = 0;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_TRACING_POLICY_H_
