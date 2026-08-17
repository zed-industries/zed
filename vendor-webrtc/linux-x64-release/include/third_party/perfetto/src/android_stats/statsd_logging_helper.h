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

#ifndef SRC_ANDROID_STATS_STATSD_LOGGING_HELPER_H_
#define SRC_ANDROID_STATS_STATSD_LOGGING_HELPER_H_

#include <stdint.h>
#include <optional>
#include <string>
#include <vector>

#include "src/android_stats/perfetto_atoms.h"

namespace perfetto {
namespace android_stats {

// Functions in this file are only active on built in the Android
// tree. On other platforms (including Android standalone and Chromium
// on Android) these functions are a noop.

// Logs the upload event to statsd if built in the Android tree.
void MaybeLogUploadEvent(PerfettoStatsdAtom atom,
                         int64_t uuid_lsb,
                         int64_t uuid_msb,
                         const std::string& trigger_name = "");

// Logs the trigger events to statsd if built in the Android tree.
void MaybeLogTriggerEvent(PerfettoTriggerAtom atom, const std::string& trigger);

// Logs the trigger events to statsd if built in the Android tree.
void MaybeLogTriggerEvents(PerfettoTriggerAtom atom,
                           const std::vector<std::string>& triggers);

}  // namespace android_stats
}  // namespace perfetto

#endif  // SRC_ANDROID_STATS_STATSD_LOGGING_HELPER_H_
