/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_STORAGE_METADATA_H_
#define SRC_TRACE_PROCESSOR_STORAGE_METADATA_H_

#include <stddef.h>

#include "src/trace_processor/types/variadic.h"

namespace perfetto {
namespace trace_processor {
namespace metadata {

// Compile time list of metadata items.
// clang-format off
#define PERFETTO_TP_METADATA(F)                                               \
  F(all_data_source_flushed_ns,        KeyType::kMulti,   Variadic::kInt),    \
  F(all_data_source_started_ns,        KeyType::kSingle,  Variadic::kInt),    \
  F(android_build_fingerprint,         KeyType::kSingle,  Variadic::kString), \
  F(android_device_manufacturer,       KeyType::kSingle,  Variadic::kString), \
  F(android_sdk_version,               KeyType::kSingle,  Variadic::kInt),    \
  F(android_soc_model,                 KeyType::kSingle,  Variadic::kString), \
  F(android_guest_soc_model,           KeyType::kSingle,  Variadic::kString), \
  F(android_hardware_revision,         KeyType::kSingle,  Variadic::kString), \
  F(android_storage_model,             KeyType::kSingle,  Variadic::kString), \
  F(android_ram_model,                 KeyType::kSingle,  Variadic::kString), \
  F(android_serial_console,            KeyType::kSingle,  Variadic::kString), \
  F(android_profile_boot_classpath,    KeyType::kSingle,  Variadic::kInt),    \
  F(android_profile_system_server,     KeyType::kSingle,  Variadic::kInt),    \
  F(benchmark_description,             KeyType::kSingle,  Variadic::kString), \
  F(benchmark_had_failures,            KeyType::kSingle,  Variadic::kInt),    \
  F(benchmark_label,                   KeyType::kSingle,  Variadic::kString), \
  F(benchmark_name,                    KeyType::kSingle,  Variadic::kString), \
  F(benchmark_start_time_us,           KeyType::kSingle,  Variadic::kInt),    \
  F(benchmark_story_name,              KeyType::kSingle,  Variadic::kString), \
  F(benchmark_story_run_index,         KeyType::kSingle,  Variadic::kInt),    \
  F(benchmark_story_run_time_us,       KeyType::kSingle,  Variadic::kInt),    \
  F(benchmark_story_tags,              KeyType::kMulti,   Variadic::kString), \
  F(ftrace_setup_errors,               KeyType::kMulti,   Variadic::kString), \
  F(ftrace_latest_data_start_ns,       KeyType::kSingle,  Variadic::kInt),    \
  F(range_of_interest_start_us,        KeyType::kSingle,  Variadic::kInt),    \
  F(slow_start_data_source,            KeyType::kMulti,   Variadic::kString), \
  F(statsd_triggering_subscription_id, KeyType::kSingle,  Variadic::kInt),    \
  F(system_machine,                    KeyType::kSingle,  Variadic::kString), \
  F(system_name,                       KeyType::kSingle,  Variadic::kString), \
  F(system_release,                    KeyType::kSingle,  Variadic::kString), \
  F(system_version,                    KeyType::kSingle,  Variadic::kString), \
  F(timezone_off_mins,                 KeyType::kSingle,  Variadic::kInt),    \
  F(trace_config_pbtxt,                KeyType::kSingle,  Variadic::kString), \
  F(trace_size_bytes,                  KeyType::kSingle,  Variadic::kInt),    \
  F(trace_time_clock_id,               KeyType::kSingle,  Variadic::kInt),    \
  F(trace_type,                        KeyType::kSingle,  Variadic::kString), \
  F(trace_uuid,                        KeyType::kSingle,  Variadic::kString), \
  F(tracing_disabled_ns,               KeyType::kSingle,  Variadic::kInt),    \
  F(tracing_started_ns,                KeyType::kSingle,  Variadic::kInt),    \
  F(ui_state,                          KeyType::kSingle,  Variadic::kString), \
  F(unique_session_name,               KeyType::kSingle,  Variadic::kString), \
  F(trace_trigger,                     KeyType::kSingle,  Variadic::kString)
// clang-format on

// Compile time list of metadata items.
// clang-format off
#define PERFETTO_TP_METADATA_KEY_TYPES(F) \
  F(kSingle, "single"),                   \
  F(kMulti,  "multi")
// clang-format

#if defined(__GNUC__) || defined(__clang__)
#if defined(__clang__)
#pragma clang diagnostic push
// Fix 'error: #pragma system_header ignored in main file' for clang in Google3.
#pragma clang diagnostic ignored "-Wpragma-system-header-outside-header"
#endif

// Ignore GCC warning about a missing argument for a variadic macro parameter.
#pragma GCC system_header

#if defined(__clang__)
#pragma clang diagnostic pop
#endif
#endif

#define PERFETTO_TP_META_TYPE_ENUM(varname, ...) varname
enum class KeyType : size_t {
  PERFETTO_TP_METADATA_KEY_TYPES(PERFETTO_TP_META_TYPE_ENUM),
  kNumKeyTypes,
};

#define PERFETTO_TP_META_TYPE_NAME(_, name, ...) name
constexpr char const* kKeyTypeNames[] = {
  PERFETTO_TP_METADATA_KEY_TYPES(PERFETTO_TP_META_TYPE_NAME)
};

// Declares an enum of literals (one for each item). The enum values of each
// literal corresponds to the string index in the arrays below.
#define PERFETTO_TP_META_ENUM(name, ...) name
enum KeyId : size_t {
  PERFETTO_TP_METADATA(PERFETTO_TP_META_ENUM),
  kNumKeys
};

// The code below declares an array for each property:
// name, key type, value type.

#define PERFETTO_TP_META_NAME(name, ...) #name
constexpr char const* kNames[] = {
  PERFETTO_TP_METADATA(PERFETTO_TP_META_NAME)};

#define PERFETTO_TP_META_KEYTYPE(_, type, ...) type
constexpr KeyType kKeyTypes[] = {
    PERFETTO_TP_METADATA(PERFETTO_TP_META_KEYTYPE)};

#define PERFETTO_TP_META_VALUETYPE(_, __, type, ...) type
constexpr Variadic::Type kValueTypes[] = {
    PERFETTO_TP_METADATA(PERFETTO_TP_META_VALUETYPE)};

}  // namespace metadata
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_STORAGE_METADATA_H_
