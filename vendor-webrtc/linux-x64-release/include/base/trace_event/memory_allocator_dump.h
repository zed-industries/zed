// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_MEMORY_ALLOCATOR_DUMP_H_
#define BASE_TRACE_EVENT_MEMORY_ALLOCATOR_DUMP_H_

#include <stdint.h>

#include <iosfwd>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/trace_event/memory_allocator_dump_guid.h"
#include "base/trace_event/memory_dump_request_args.h"
#include "base/unguessable_token.h"

namespace perfetto {
namespace protos {
namespace pbzero {
class MemoryTrackerSnapshot_ProcessSnapshot_MemoryNode;
}
}  // namespace protos
}  // namespace perfetto

namespace base {
namespace trace_event {

class ProcessMemoryDump;
class TracedValue;

// Data model for user-land memory allocator dumps.
class BASE_EXPORT MemoryAllocatorDump {
 public:
  enum Flags {
    kDefault = 0,

    // A dump marked weak will be discarded by TraceViewer.
    kWeak = 1 << 0,
  };

  // In the TraceViewer UI table each MemoryAllocatorDump becomes
  // a row and each Entry generates a column (if it doesn't already
  // exist).
  struct BASE_EXPORT Entry {
    enum EntryType {
      kUint64,
      kString,
    };

    // By design name, units and value_string are  always coming from
    // indefinitely lived const char* strings, the only reason we copy
    // them into a std::string is to handle Mojo (de)serialization.
    // TODO(hjd): Investigate optimization (e.g. using std::string_view).
    Entry();  // Only for deserialization.
    Entry(std::string name, std::string units, uint64_t value);
    Entry(std::string name, std::string units, std::string value);
    Entry(Entry&& other) noexcept;
    Entry(const Entry&) = delete;
    Entry& operator=(const Entry&) = delete;
    Entry& operator=(Entry&& other);
    bool operator==(const Entry& rhs) const;

    std::string name;
    std::string units;

    EntryType entry_type;

    uint64_t value_uint64;
    std::string value_string;
  };

  MemoryAllocatorDump(const std::string& absolute_name,
                      MemoryDumpLevelOfDetail,
                      const MemoryAllocatorDumpGuid&);
  MemoryAllocatorDump(const MemoryAllocatorDump&) = delete;
  MemoryAllocatorDump& operator=(const MemoryAllocatorDump&) = delete;
  ~MemoryAllocatorDump();

  // Standard attribute |name|s for the AddScalar and AddString() methods.
  static const char kNameSize[];         // To represent allocated space.
  static const char kNameObjectCount[];  // To represent number of objects.

  // Standard attribute |unit|s for the AddScalar and AddString() methods.
  static const char kUnitsBytes[];    // Unit name to represent bytes.
  static const char kUnitsObjects[];  // Unit name to represent #objects.

  // Constants used only internally and by tests.
  static const char kTypeScalar[];  // Type name for scalar attributes.
  static const char kTypeString[];  // Type name for string attributes.

  // Setters for scalar attributes. Some examples:
  // - "size" column (all dumps are expected to have at least this one):
  //     AddScalar(kNameSize, kUnitsBytes, 1234);
  // - Some extra-column reporting internal details of the subsystem:
  //    AddScalar("number_of_freelist_entries", kUnitsObjects, 42)
  // - Other informational column:
  //    AddString("kitten", "name", "shadow");
  void AddScalar(const char* name, const char* units, uint64_t value);
  void AddString(const char* name, const char* units, const std::string& value);

  // Absolute name, unique within the scope of an entire ProcessMemoryDump.
  const std::string& absolute_name() const LIFETIME_BOUND {
    return absolute_name_;
  }

  // Called at trace generation time to populate the TracedValue.
  void AsValueInto(TracedValue* value) const;

  void AsProtoInto(
      perfetto::protos::pbzero::
          MemoryTrackerSnapshot_ProcessSnapshot_MemoryNode* memory_node) const;

  // Get the size for this dump.
  // The size is the value set with AddScalar(kNameSize, kUnitsBytes, size);
  // TODO(hjd): this should return an optional<uint64_t>.
  uint64_t GetSizeInternal() const;

  MemoryDumpLevelOfDetail level_of_detail() const { return level_of_detail_; }

  // Use enum Flags to set values.
  void set_flags(int flags) { flags_ |= flags; }
  void clear_flags(int flags) { flags_ &= ~flags; }
  int flags() const { return flags_; }

  // |guid| is an optional global dump identifier, unique across all processes
  // within the scope of a global dump. It is only required when using the
  // graph APIs (see TODO_method_name) to express retention / suballocation or
  // cross process sharing. See crbug.com/492102 for design docs.
  // Subsequent MemoryAllocatorDump(s) with the same |absolute_name| are
  // expected to have the same guid.
  const MemoryAllocatorDumpGuid& guid() const LIFETIME_BOUND { return guid_; }

  const std::vector<Entry>& entries() const LIFETIME_BOUND { return entries_; }

  // Only for mojo serialization, which can mutate the collection.
  std::vector<Entry>* mutable_entries_for_serialization() const {
    cached_size_.reset();  // The caller can mutate the collection.

    // Mojo takes a const input argument even for move-only types that can be
    // mutate while serializing (like this one). Hence the const_cast.
    return const_cast<std::vector<Entry>*>(&entries_);
  }

 private:
  const std::string absolute_name_;
  MemoryAllocatorDumpGuid guid_;
  MemoryDumpLevelOfDetail level_of_detail_;
  int flags_;                                    // See enum Flags.
  mutable std::optional<uint64_t> cached_size_;  // Lazy, for GetSizeInternal().
  std::vector<Entry> entries_;
};

// This is required by gtest to print a readable output on test failures.
void BASE_EXPORT PrintTo(const MemoryAllocatorDump::Entry&, std::ostream*);

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_MEMORY_ALLOCATOR_DUMP_H_
