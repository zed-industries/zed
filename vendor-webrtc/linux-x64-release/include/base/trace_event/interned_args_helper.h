// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_INTERNED_ARGS_HELPER_H_
#define BASE_TRACE_EVENT_INTERNED_ARGS_HELPER_H_

#include <optional>
#include <string>

#include "base/base_export.h"
#include "base/hash/hash.h"
#include "base/location.h"
#include "base/profiler/module_cache.h"
#include "base/trace_event/trace_event.h"
#include "build/build_config.h"
#include "third_party/perfetto/include/perfetto/tracing/track_event_interned_data_index.h"
#include "third_party/perfetto/protos/perfetto/trace/interned_data/interned_data.pbzero.h"

namespace base {
namespace trace_event {

// TrackEventInternedDataIndex expects the same data structure to be used for
// all interned fields with the same field number. We can't use base::Location
// for log event's location since base::Location uses program counter based
// location.
struct BASE_EXPORT TraceSourceLocation {
  const char* function_name = nullptr;
  const char* file_name = nullptr;
  int line_number = 0;

  TraceSourceLocation() = default;
  TraceSourceLocation(const char* function_name,
                      const char* file_name,
                      int line_number)
      : function_name(function_name),
        file_name(file_name),
        line_number(line_number) {}
  // Construct a new source location from an existing base::Location, the only
  // attributes that are read are |function_name|, |file_name| and
  // |line_number|.
  explicit TraceSourceLocation(const base::Location& location)
      : function_name(location.function_name()),
        file_name(location.file_name()),
        line_number(location.line_number()) {}

  bool operator==(const TraceSourceLocation& other) const {
    return file_name == other.file_name &&
           function_name == other.function_name &&
           line_number == other.line_number;
  }
};

// Data structure for constructing an interned
// perfetto.protos.UnsymbolizedSourceLocation proto message.
struct BASE_EXPORT UnsymbolizedSourceLocation {
  uint64_t mapping_id = 0;
  uint64_t rel_pc = 0;

  UnsymbolizedSourceLocation() = default;
  UnsymbolizedSourceLocation(uint64_t mapping_id, uint64_t rel_pc)
      : mapping_id(mapping_id), rel_pc(rel_pc) {}

  bool operator==(const UnsymbolizedSourceLocation& other) const {
    return mapping_id == other.mapping_id && rel_pc == other.rel_pc;
  }
};

}  // namespace trace_event
}  // namespace base

namespace std {

template <>
struct hash<base::trace_event::TraceSourceLocation> {
  std::size_t operator()(
      const base::trace_event::TraceSourceLocation& loc) const {
    return base::HashInts(
        base::HashInts(reinterpret_cast<uintptr_t>(loc.file_name),
                       reinterpret_cast<uintptr_t>(loc.function_name)),
        static_cast<size_t>(loc.line_number));
  }
};

template <>
struct hash<base::trace_event::UnsymbolizedSourceLocation> {
  std::size_t operator()(
      const base::trace_event::UnsymbolizedSourceLocation& module) const {
    return base::HashInts(module.mapping_id, module.rel_pc);
  }
};

}  // namespace std

namespace base {
namespace trace_event {

struct BASE_EXPORT InternedSourceLocation
    : public perfetto::TrackEventInternedDataIndex<
          InternedSourceLocation,
          perfetto::protos::pbzero::InternedData::kSourceLocationsFieldNumber,
          TraceSourceLocation> {
  static void Add(perfetto::protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const TraceSourceLocation& location);
  using perfetto::TrackEventInternedDataIndex<
      InternedSourceLocation,
      perfetto::protos::pbzero::InternedData::kSourceLocationsFieldNumber,
      TraceSourceLocation>::Get;
  static size_t Get(perfetto::EventContext* ctx, const Location& location) {
    return perfetto::TrackEventInternedDataIndex<
        InternedSourceLocation,
        perfetto::protos::pbzero::InternedData::kSourceLocationsFieldNumber,
        TraceSourceLocation>::Get(ctx, TraceSourceLocation(location));
  }
};

struct BASE_EXPORT InternedLogMessage
    : public perfetto::TrackEventInternedDataIndex<
          InternedLogMessage,
          perfetto::protos::pbzero::InternedData::kLogMessageBodyFieldNumber,
          std::string> {
  static void Add(perfetto::protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const std::string& log_message);
};

struct BASE_EXPORT InternedBuildId
    : public perfetto::TrackEventInternedDataIndex<
          InternedBuildId,
          perfetto::protos::pbzero::InternedData::kBuildIdsFieldNumber,
          std::string> {
  static void Add(perfetto::protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const std::string& build_id);
};

struct BASE_EXPORT InternedMappingPath
    : public perfetto::TrackEventInternedDataIndex<
          InternedMappingPath,
          perfetto::protos::pbzero::InternedData::kMappingPathsFieldNumber,
          std::string> {
  static void Add(perfetto::protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const std::string& mapping_path);
};

struct BASE_EXPORT InternedMapping
    : public perfetto::TrackEventInternedDataIndex<
          InternedMapping,
          perfetto::protos::pbzero::InternedData::kMappingsFieldNumber,
          const base::ModuleCache::Module*> {
  // We need a custom implementation here to plumb EventContext to Add.
  static size_t Get(perfetto::EventContext* ctx,
                    const base::ModuleCache::Module* module);
  static void Add(perfetto::EventContext* ctx,
                  size_t iid,
                  const base::ModuleCache::Module* module);
};

// Interns an unsymbolized source code location + all it's "dependencies", i.e.
// module, strings used in the module definition, and so on.
struct BASE_EXPORT InternedUnsymbolizedSourceLocation
    : public perfetto::TrackEventInternedDataIndex<
          InternedUnsymbolizedSourceLocation,
          perfetto::protos::pbzero::InternedData::
              kUnsymbolizedSourceLocationsFieldNumber,
          uintptr_t> {
  // We need a custom Get implementation to use ModuleCache, and to return
  // a nullopt if a module for the given address cannot be found.
  static std::optional<size_t> Get(perfetto::EventContext* ctx,
                                   uintptr_t address);
  static void Add(perfetto::protos::pbzero::InternedData* interned_data,
                  size_t iid,
                  const UnsymbolizedSourceLocation& location);
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_INTERNED_ARGS_HELPER_H_
