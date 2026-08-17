/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_STATUS_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_STATUS_H_

#include "perfetto/base/status.h"

// Once upon a time Status used to live in perfetto::trace_processor. At some
// point it has been moved up to base. This forwarding header stayed here
// because of out-of-repo users.

namespace perfetto {
namespace trace_processor {
namespace util {

using Status = ::perfetto::base::Status;

constexpr auto OkStatus = ::perfetto::base::OkStatus;
constexpr auto ErrStatus = ::perfetto::base::ErrStatus;

}  // namespace util
}  // namespace trace_processor
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_STATUS_H_
