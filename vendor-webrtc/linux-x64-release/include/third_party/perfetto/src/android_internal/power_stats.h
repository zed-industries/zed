/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_ANDROID_INTERNAL_POWER_STATS_H_
#define SRC_ANDROID_INTERNAL_POWER_STATS_H_

#include <stddef.h>
#include <stdint.h>

// This header declares proxy functions defined in
// libperfetto_android_internal.so that allow traced_probes to access internal
// android functions (e.g., hwbinder).
// Do not add any include to either perfetto headers or android headers. See
// README.md for more.

namespace perfetto {
namespace android_internal {

const int32_t ALL_UIDS_FOR_CONSUMER = -1;

struct RailDescriptor {
  // Index corresponding to the rail
  uint32_t index;
  // Name of the rail
  char rail_name[64];
  // Name of the subsystem to which this rail belongs
  char subsys_name[64];
  // Hardware sampling rate
  uint32_t sampling_rate;
};

struct RailEnergyData {
  // Index corresponding to RailDescriptor.index
  uint32_t index;
  // Time since device boot(CLOCK_BOOTTIME) in milli-seconds
  uint64_t timestamp;
  // Accumulated energy since device boot in microwatt-seconds (uWs)
  uint64_t energy;
};

struct EnergyConsumerInfo {
  // Unique ID of this energy consumer.  Matches the ID in a
  // EnergyEstimationBreakdown.
  int32_t energy_consumer_id;

  // For a group of energy consumers of the same logical type, sorting by
  // ordinal gives their physical order. Ordinals must be consecutive integers
  // starting from 0.
  int32_t ordinal;

  // Type of this energy consumer.
  char type[64];

  // Unique name of this energy consumer. Vendor/device specific. Opaque to
  // framework.
  char name[64];
};

struct EnergyEstimationBreakdown {
  // Energy consumer ID.
  int32_t energy_consumer_id;

  // Process uid.  ALL_UIDS_FOR_CONSUMER represents energy for all processes
  // for the energy_consumer_id.
  int32_t uid;

  // Energy usage in microwatts-second(µWs).
  int64_t energy_uws;
};

struct PowerEntityState {
  int32_t entity_id;
  int32_t state_id;
  char entity_name[64];
  char state_name[64];
};

struct PowerEntityStateResidency {
  int32_t entity_id;
  int32_t state_id;
  uint64_t total_time_in_state_ms;
  uint64_t total_state_entry_count;
  uint64_t last_entry_timestamp_ms;
};

extern "C" {

// These functions are not thread safe unless specified otherwise.

bool __attribute__((visibility("default"))) GetAvailableRails(
    RailDescriptor*,
    size_t* size_of_arr);

bool __attribute__((visibility("default"))) GetRailEnergyData(
    RailEnergyData*,
    size_t* size_of_arr);

bool __attribute__((visibility("default"))) GetEnergyConsumerInfo(
    EnergyConsumerInfo* consumers,
    size_t* size_of_arr);

// Retrieve the energy estimation breakdown for all energy consumer.  For each
// consumer, there will be an entry with a uid of ALL_UIDS_FOR_CONSUMER,
// followed by the energy breakdown for each process contributing to that
// consumer.
bool __attribute__((visibility("default"))) GetEnergyConsumed(
    EnergyEstimationBreakdown* breakdown,
    size_t* size_of_arr);

bool __attribute__((visibility("default"))) GetPowerEntityStates(
    PowerEntityState* state,
    size_t* size_of_arr);

bool __attribute__((visibility("default"))) GetPowerEntityStateResidency(
    PowerEntityStateResidency* residency,
    size_t* size_of_arr);

}  // extern "C"

}  // namespace android_internal
}  // namespace perfetto

#endif  // SRC_ANDROID_INTERNAL_POWER_STATS_H_
