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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_UTIL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_UTIL_H_

#include <optional>

#include "src/trace_processor/importers/etm/opencsd.h"

namespace perfetto::trace_processor::etm {

const char* ToString(ocsd_gen_trc_elem_t type);
std::optional<ocsd_gen_trc_elem_t> FromString(const char* type_str);
const char* ToString(ocsd_isa isa);
const char* ToString(ocsd_instr_type type);
const char* ToString(ocsd_instr_subtype sub_type);
const char* ToString(ocsd_core_profile_t profile);
const char* ToString(ocsd_arch_version_t ver);

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_UTIL_H_
