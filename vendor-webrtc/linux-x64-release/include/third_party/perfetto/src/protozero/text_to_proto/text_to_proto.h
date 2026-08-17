/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_PROTOZERO_TEXT_TO_PROTO_TEXT_TO_PROTO_H_
#define SRC_PROTOZERO_TEXT_TO_PROTO_TEXT_TO_PROTO_H_

#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "perfetto/ext/base/status_or.h"

namespace protozero {

// Given a FileDescriptorSet in `descriptor_set_ptr` and `descriptor_set_size`
// converts `input` from binary proto to textproto by interpreting it as a proto
// of type `root_type`.
//
// `file_name` is an opaque string used to print good error messages: it is not
// used otherwise.
perfetto::base::StatusOr<std::vector<uint8_t>> TextToProto(
    const uint8_t* descriptor_set_ptr,
    size_t descriptor_set_size,
    const std::string& root_type,
    const std::string& file_name,
    std::string_view input);

}  // namespace protozero

#endif  // SRC_PROTOZERO_TEXT_TO_PROTO_TEXT_TO_PROTO_H_
