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

#ifndef TEST_CTS_HEAPPROFD_TEST_HELPER_H_
#define TEST_CTS_HEAPPROFD_TEST_HELPER_H_

#include <cinttypes>
#include <string>
#include <vector>

#include "protos/perfetto/trace/trace_packet.gen.h"

namespace perfetto {

// Heapprofd CTS test utils shared by tests covering native malloc and ART's
// java allocator (which reports samples using the heapprofd NDK custom
// allocator API).

std::string RandomSessionName();

std::vector<protos::gen::TracePacket> ProfileRuntime(
    const std::string& app_name,
    const std::string& activity,
    uint64_t sampling_interval,
    const std::vector<std::string>& heap_names);

std::vector<protos::gen::TracePacket> ProfileStartup(
    const std::string& app_name,
    const std::string& activity,
    uint64_t sampling_interval,
    const std::vector<std::string>& heap_names,
    const bool enable_extra_guardrails = false);

void AssertExpectedMallocsPresent(
    uint64_t expected_individual_alloc_sz,
    const std::vector<protos::gen::TracePacket>& packets);

void AssertHasSampledAllocs(
    const std::vector<protos::gen::TracePacket>& packets);

void AssertNoProfileContents(
    const std::vector<protos::gen::TracePacket>& packets);

}  // namespace perfetto

#endif  // TEST_CTS_HEAPPROFD_TEST_HELPER_H_
