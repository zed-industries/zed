/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_PROFILING_COMMON_PRODUCER_SUPPORT_H_
#define SRC_PROFILING_COMMON_PRODUCER_SUPPORT_H_

#include <cinttypes>
#include <string>
#include <vector>

#include "perfetto/tracing/core/forward_decls.h"

namespace perfetto {
namespace profiling {

bool CanProfile(const DataSourceConfig& ds_config,
                uint64_t uid,
                const std::vector<std::string>& installed_by);
bool CanProfileAndroid(const DataSourceConfig& ds_config,
                       uint64_t uid,
                       const std::vector<std::string>& installed_by,
                       const std::string& build_type,
                       const std::string& packages_list_path);

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_COMMON_PRODUCER_SUPPORT_H_
