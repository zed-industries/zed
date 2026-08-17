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

#ifndef SRC_TRACED_PROBES_SYSTEM_INFO_CPU_INFO_FEATURES_ALLOWLIST_H_
#define SRC_TRACED_PROBES_SYSTEM_INFO_CPU_INFO_FEATURES_ALLOWLIST_H_

namespace perfetto {

// APPEND ONLY. DO NOT EVER REMOVE ENTRIES FROM THIS ARRAY OR REORDER.
// This array is used both by traced_probes and trace_processor to index the
// cpuinfo flags. Changing the order will break trace_processor compatibility
// with old traces.
constexpr const char* kCpuInfoFeatures[] = {
    "mte",   // DO NOT REMOVE/REODER.
    "mte3",  // DO NOT REMOVE/REODER.
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_SYSTEM_INFO_CPU_INFO_FEATURES_ALLOWLIST_H_
