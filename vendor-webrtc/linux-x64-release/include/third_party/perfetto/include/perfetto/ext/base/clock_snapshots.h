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

#ifndef INCLUDE_PERFETTO_EXT_BASE_CLOCK_SNAPSHOTS_H_
#define INCLUDE_PERFETTO_EXT_BASE_CLOCK_SNAPSHOTS_H_

#include <cstdint>
#include <vector>

namespace perfetto::base {

struct ClockReading {
  ClockReading(uint32_t _clock_id, uint64_t _timestamp)
      : clock_id(_clock_id), timestamp(_timestamp) {}
  ClockReading() = default;

  // Identifier of the clock domain (of type protos::pbzero::BuiltinClock).
  uint32_t clock_id = 0;
  // Clock reading as uint64_t.
  uint64_t timestamp = 0;
};

using ClockSnapshotVector = std::vector<ClockReading>;

// Takes snapshots of clock readings of all supported built-in clocks.
ClockSnapshotVector CaptureClockSnapshots();

}  // namespace perfetto::base

#endif  // INCLUDE_PERFETTO_EXT_BASE_CLOCK_SNAPSHOTS_H_
