/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_TRACK_EVENT_ABI_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_TRACK_EVENT_ABI_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "perfetto/public/abi/atomic.h"
#include "perfetto/public/abi/data_source_abi.h"
#include "perfetto/public/abi/export.h"

#ifdef __cplusplus
extern "C" {
#endif

PERFETTO_SDK_EXPORT void PerfettoTeInit(void);

// The attributes of a single category.
struct PerfettoTeCategoryDescriptor {
  // The category name. Null terminated string.
  const char* name;
  // A human readable string shown by the tracing service when listing the data
  // sources. Null terminated string.
  const char* desc;
  // List of tags, can be null if num_tags is 0. Array of pointers to null
  // terminated strings.
  const char** tags;
  // Number of elements in the `tags` array.
  size_t num_tags;
};

// Opaque pointer to a registered category.
struct PerfettoTeCategoryImpl;

// An already registered category that's considered enabled if the track event
// data source is enabled. Useful for dynamic categories.
extern PERFETTO_SDK_EXPORT struct PerfettoTeCategoryImpl*
    perfetto_te_any_categories;

// Points to true if the track event data source is enabled.
extern PERFETTO_SDK_EXPORT PERFETTO_ATOMIC(bool) *
    perfetto_te_any_categories_enabled;

// Registers a category.
//
// `desc` (and all the objects pointed by it) need to be alive until
// PerfettoTeCategoryImplDestroy() is called.
PERFETTO_SDK_EXPORT struct PerfettoTeCategoryImpl* PerfettoTeCategoryImplCreate(
    struct PerfettoTeCategoryDescriptor* desc);

// Tells the tracing service about newly registered categories. Must be called
// after one or more call to PerfettoTeCategoryImplCreate() or
// PerfettoTeCategoryImplDestroy().
PERFETTO_SDK_EXPORT void PerfettoTePublishCategories(void);

// Returns a pointer to a boolean that tells if the category is enabled or not.
// The pointer is valid until the category is destroyed.
PERFETTO_SDK_EXPORT PERFETTO_ATOMIC(bool) *
    PerfettoTeCategoryImplGetEnabled(struct PerfettoTeCategoryImpl*);

// Called when a data source instance is created (if `created` is true) or
// destroyed (if `created` is false) with a registered category enabled.
// `global_state_changed` is true if this was the first instance created with
// the category enabled or the last instance destroyed with the category
// enabled.
typedef void (*PerfettoTeCategoryImplCallback)(struct PerfettoTeCategoryImpl*,
                                               PerfettoDsInstanceIndex inst_id,
                                               bool created,
                                               bool global_state_changed,
                                               void* user_arg);

// Registers `cb` to be called every time a data source instance with `cat`
// enabled is created or destroyed. `user_arg` will be passed unaltered to `cb`.
//
// `cb` can be NULL to disable the callback.
PERFETTO_SDK_EXPORT void PerfettoTeCategoryImplSetCallback(
    struct PerfettoTeCategoryImpl* cat,
    PerfettoTeCategoryImplCallback cb,
    void* user_arg);

// Returns the interning id (iid) associated with the registered category `cat`.
PERFETTO_SDK_EXPORT uint64_t
PerfettoTeCategoryImplGetIid(struct PerfettoTeCategoryImpl* cat);

// Destroys a previously registered category. The category cannot be used for
// tracing anymore after this.
PERFETTO_SDK_EXPORT void PerfettoTeCategoryImplDestroy(
    struct PerfettoTeCategoryImpl*);

enum PerfettoTeTimestampType {
  PERFETTO_TE_TIMESTAMP_TYPE_MONOTONIC = 3,
  PERFETTO_TE_TIMESTAMP_TYPE_BOOT = 6,
  PERFETTO_TE_TIMESTAMP_TYPE_INCREMENTAL = 64,
  PERFETTO_TE_TIMESTAMP_TYPE_ABSOLUTE = 65,
};

enum {
#ifdef __linux__
  PERFETTO_I_CLOCK_INCREMENTAL_UNDERNEATH = PERFETTO_TE_TIMESTAMP_TYPE_BOOT,
#else
  PERFETTO_I_CLOCK_INCREMENTAL_UNDERNEATH =
      PERFETTO_TE_TIMESTAMP_TYPE_MONOTONIC,
#endif
};

struct PerfettoTeTimestamp {
  // PerfettoTeTimestampType
  uint32_t clock_id;
  uint64_t value;
};

// Returns the current timestamp.
PERFETTO_SDK_EXPORT struct PerfettoTeTimestamp PerfettoTeGetTimestamp(void);

struct PerfettoTeRegisteredTrackImpl {
  void* descriptor;  // Owned (malloc).
  size_t descriptor_size;
  uint64_t uuid;
};

// The UUID of the process track for the current process.
extern PERFETTO_SDK_EXPORT uint64_t perfetto_te_process_track_uuid;

// The type of an event.
enum PerfettoTeType {
  PERFETTO_TE_TYPE_SLICE_BEGIN = 1,
  PERFETTO_TE_TYPE_SLICE_END = 2,
  PERFETTO_TE_TYPE_INSTANT = 3,
  PERFETTO_TE_TYPE_COUNTER = 4,
};

#ifdef __cplusplus
}
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_TRACK_EVENT_ABI_H_
