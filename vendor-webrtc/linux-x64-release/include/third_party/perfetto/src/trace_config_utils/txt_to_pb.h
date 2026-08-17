/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACE_CONFIG_UTILS_TXT_TO_PB_H_
#define SRC_TRACE_CONFIG_UTILS_TXT_TO_PB_H_

#include "perfetto/ext/base/status_or.h"

#include <stdint.h>

#include <string>
#include <vector>

namespace perfetto {

base::StatusOr<std::vector<uint8_t>> TraceConfigTxtToPb(
    const std::string& input,
    const std::string& file_name = "-");

}  // namespace perfetto

#endif  // SRC_TRACE_CONFIG_UTILS_TXT_TO_PB_H_
