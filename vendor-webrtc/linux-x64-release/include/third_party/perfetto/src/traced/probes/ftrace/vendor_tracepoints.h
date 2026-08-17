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

#ifndef SRC_TRACED_PROBES_FTRACE_VENDOR_TRACEPOINTS_H_
#define SRC_TRACED_PROBES_FTRACE_VENDOR_TRACEPOINTS_H_

#include <map>
#include <string>
#include <vector>

#include "perfetto/base/status.h"
#include "src/traced/probes/ftrace/atrace_hal_wrapper.h"
#include "src/traced/probes/ftrace/ftrace_procfs.h"
#include "src/traced/probes/ftrace/proto_translation_table.h"

namespace perfetto {
namespace vendor_tracepoints {

// Path to the vendor categories file in Android (since Android 14).
constexpr const char* kCategoriesFile =
    "/vendor/etc/atrace/atrace_categories.txt";

// Returns a map from vendor category to events we should enable. Queries the
// atrace HAL.
std::map<std::string, std::vector<GroupAndName>>
DiscoverVendorTracepointsWithHal(AtraceHalWrapper* hal, FtraceProcfs* ftrace);

// Fills `*categories_map` with a map from vendor category to events we should
// enable. Queries the vendor categories file at
// `vendor_atrace_categories_path` (which should always be `kCategoriesFile`
// except in tests).
base::Status DiscoverVendorTracepointsWithFile(
    const std::string& vendor_atrace_categories_path,
    std::map<std::string, std::vector<GroupAndName>>* categories_map);

// Like `DiscoverVendorTracepointsWithFile`, but does not return events that are
// not accessible or do not actually exist on the tracing file system.
base::Status DiscoverAccessibleVendorTracepointsWithFile(
    const std::string& vendor_atrace_categories_path,
    std::map<std::string, std::vector<GroupAndName>>* categories_map,
    FtraceProcfs* ftrace);

}  // namespace vendor_tracepoints
}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_FTRACE_VENDOR_TRACEPOINTS_H_
