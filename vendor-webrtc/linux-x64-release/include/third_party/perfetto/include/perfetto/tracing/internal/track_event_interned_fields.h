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

#include "perfetto/base/export.h"
#include "perfetto/tracing/track_event_interned_data_index.h"

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_INTERNED_FIELDS_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_INTERNED_FIELDS_H_

namespace perfetto {
namespace internal {

// These helpers are exposed here to allow Chromium-without-client library
// to share the interning buffers with Perfetto internals (e.g.
// perfetto::TracedValue implementation).

struct PERFETTO_EXPORT_COMPONENT InternedEventCategory
    : public TrackEventInternedDataIndex<
          InternedEventCategory,
          perfetto::protos::pbzero::InternedData::kEventCategoriesFieldNumber,
          const char*,
          SmallInternedDataTraits> {
  ~InternedEventCategory() override;

  static void Add(protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const char* value,
                  size_t length);
};

struct PERFETTO_EXPORT_COMPONENT InternedEventName
    : public TrackEventInternedDataIndex<
          InternedEventName,
          perfetto::protos::pbzero::InternedData::kEventNamesFieldNumber,
          const char*,
          SmallInternedDataTraits> {
  ~InternedEventName() override;

  static void Add(protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const char* value);
};

struct PERFETTO_EXPORT_COMPONENT InternedDebugAnnotationName
    : public TrackEventInternedDataIndex<
          InternedDebugAnnotationName,
          perfetto::protos::pbzero::InternedData::
              kDebugAnnotationNamesFieldNumber,
          const char*,
          SmallInternedDataTraits> {
  ~InternedDebugAnnotationName() override;

  static void Add(protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const char* value);
};

struct PERFETTO_EXPORT_COMPONENT InternedDebugAnnotationValueTypeName
    : public TrackEventInternedDataIndex<
          InternedDebugAnnotationValueTypeName,
          perfetto::protos::pbzero::InternedData::
              kDebugAnnotationValueTypeNamesFieldNumber,
          const char*,
          SmallInternedDataTraits> {
  ~InternedDebugAnnotationValueTypeName() override;

  static void Add(protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const char* value);
};

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_TRACK_EVENT_INTERNED_FIELDS_H_
