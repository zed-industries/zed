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

#ifndef SRC_TRACE_REDACTION_PROTO_UTIL_H_
#define SRC_TRACE_REDACTION_PROTO_UTIL_H_

#include "perfetto/protozero/field.h"
#include "perfetto/protozero/message.h"

namespace perfetto::trace_redaction {

// This is here, and not in protozero, because field and message are never found
// together. Because trace redaction is the only user of this function, it is
// here.
namespace proto_util {

void AppendField(const protozero::Field& field, protozero::Message* message);

}  // namespace proto_util

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_PROTO_UTIL_H_
