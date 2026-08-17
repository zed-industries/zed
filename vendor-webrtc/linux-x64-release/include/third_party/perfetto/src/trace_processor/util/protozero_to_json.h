/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_UTIL_PROTOZERO_TO_JSON_H_
#define SRC_TRACE_PROCESSOR_UTIL_PROTOZERO_TO_JSON_H_

#include <cstdint>
#include <string>
#include <vector>

#include "perfetto/protozero/field.h"

namespace perfetto::trace_processor {

class DescriptorPool;

namespace protozero_to_json {

enum Flags {
  kNone = 0,

  // Produce nice json (newlines, 1 space post :, 2 space indents)
  kPretty = 1 << 0,

  // Report errors as an extra key on the root json object. For example
  // the output with this flag might look like:
  // {
  //    "foo": { ... },
  //    "baz": { ... },
  //    "__error": "Failed to decode key 'bar' due to <some error>"
  // }
  kInlineErrors = 1 << 1,

  // Report annotations as an extra key on the root json object. For example
  // the output with this flag might look like:
  // {
  //    "foo": { ... },
  //    "baz": { ... },
  //    "__annotations": {
  //      "foo": {
  //        "__field_options": { "unit": "ms_smallerIsBetter" }
  //      }
  //    }
  // }
  kInlineAnnotations = 1 << 2,
};

// Given a protozero message |protobytes| which is of fully qualified name
// |type|, convert this into a text proto format string. All types used in
// message definition of |type| must be available in |pool|.
std::string ProtozeroToJson(const DescriptorPool& pool,
                            const std::string& type,
                            protozero::ConstBytes protobytes,
                            int flags);

std::string ProtozeroToJson(const DescriptorPool& pool,
                            const std::string& type,
                            const std::vector<uint8_t>& protobytes,
                            int flags);

}  // namespace protozero_to_json
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_UTIL_PROTOZERO_TO_JSON_H_
