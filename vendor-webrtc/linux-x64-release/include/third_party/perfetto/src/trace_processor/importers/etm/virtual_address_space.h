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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_VIRTUAL_ADDRESS_SPACE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_VIRTUAL_ADDRESS_SPACE_H_

#include <cstdint>
#include <optional>
#include <vector>

#include "src/trace_processor/importers/etm/mapping_version.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/perf_tables_py.h"

namespace perfetto::trace_processor {
class TraceProcessorContext;
namespace etm {

// Represents the virtual address space for a process.
// This class is used to answer queries in the form: At timestamp t, what was
// the mapping at address x for the thread tid.  We want these lookups to be as
// fast as possible, as we will be doing a lot of these during ETM parsing.
//
// Basically this boils down to the "point location" problem in a 2D rectilinear
// space where one dimension is time and the other is the address space.
//
// Eg:
//  T  ↑
//  i  │
//  m  │ ↑           ↑         ↑            ↑
//  e  │ │           │         │ Mapping 4  │
//     │ │ Mapping 3 │         └──────┬─────┘
//     │ │           │                │
//     │ └──┬───┬────┘                │
//     │    │   │      Mapping 2      │
//     │    │   └────────────┬────────┘
//     │    │   Mapping 1    │
//     │    └────────────────┘
//     └──────────────────────────────────────────────────────→ address
//
// There are many studied solutions to this problem or increased complexity and
// better performance. This class implements a "slab decomposition" approach as
// described by "Dobkin and Lipton"
// (https://en.wikipedia.org/wiki/Point_location).
//
// This is a very simple approach that just partitions the space using vertical
// lines that pass through each vertex, creating so called slabs. This
// partitions the address space in on overlapping regions, and for each region
// you can see that mappings will be ordered by time. This gives us O(log N)
// lookup but O(N^2) space, which is fine in our case as the ^2 comes from
// mapping overlaps, which we expect to rarely happen, so in practice space
// usage will be more like O(N).
//
// So the above example would look like:
//
//   T  ↑
//   i  │
//   m  │ ↑  ↑   ↑    ↑       ↑ ↑      ↑     ↑
//   e  │ │  │   │    │       │ │  4   │  4  │
//      │ │  │   │    │       │ ├──────┼─────┘
//      │ │3 │ 3 │ 3  │   2   │2│  2   │     ┊
//      │ └──┼───┼────┤       │ │      │     ┊
//      │ ┊  │   │ 2  │       │ │      │     ┊
//      │ ┊  │   ├────┼───────┼─┴──────┘     ┊
//      │ ┊  │ 1 │ 1  │   1   │ ┊      ┊     ┊
//      │ ┊  └───┴────┴───────┘ ┊      ┊     ┊
//      │ ┊  ┊   ┊    ┊       ┊ ┊      ┊     ┊
//      └─┴──┴───┴────┴───────┴─┴──────┴─────┴──────────────→ address
// Slabs    A  B   C      D    E    F     G
//
// Instead of keeping two separate structures (one to store the non overlapping
// ranges and one to store the mappings timestamp order), we have one array of
// `MappingVersion` objects (one for each of the boxes above) ordered by
// increasing address range and decreasing creation time. This allows us to do
// one lower_bound search to find the desired mapping. So the ordering keep in
// this class would look like:
//
// A3, B3, B1, C3, C2, C1, D2, D1, E2, F4, F2, G4

class VirtualAddressSpace {
 public:
  VirtualAddressSpace() {}
  class Builder {
   public:
    explicit Builder(TraceProcessorContext* context) : context_(context) {}
    void AddMapping(tables::MmapRecordTable::ConstRowReference mmap);
    VirtualAddressSpace Build() &&;

   private:
    // Mappings ordered by ascending address and descending creation time. We
    // resolve collisions by additionally ordering by mapping_id. Note that if
    // two mappings overlap, and they are created at the same time, only the one
    // with the higher mapping_id will be used. (Although iun practice this
    // should never happen, TM)
    struct FullSort {
      bool operator()(const MappingVersion& lhs,
                      const MappingVersion& rhs) const {
        if (lhs.start() < rhs.start()) {
          return true;
        }
        if (rhs.start() < lhs.start()) {
          return false;
        }
        if (lhs.create_ts() > rhs.create_ts()) {
          return true;
        }
        if (rhs.create_ts() > lhs.create_ts()) {
          return false;
        }
        return lhs.id() < rhs.id();
      }
    };

    using SortedMappingsWithOverlaps = std::set<MappingVersion, FullSort>;
    using Vertices = std::set<uint64_t>;

    TraceProcessorContext* context_;
    SortedMappingsWithOverlaps mappings_;
    Vertices vertices_;
  };
  const MappingVersion* FindMapping(int64_t ts, uint64_t address) const;

  template <typename Callback>
  void ForEach(Callback cb) const {
    for (const auto& m : mappings_) {
      cb(m);
    }
  }

 private:
  struct LookupKey {
    uint64_t address;
    int64_t ts;
  };

  // Mappings ordered by ascending address and descending creation time. So
  // point lookups can be answered with one lower_bound lookup.
  struct Lookup {
    bool operator()(const MappingVersion& lhs, const LookupKey& rhs) const {
      if (lhs.end() <= rhs.address) {
        return true;
      }
      if (rhs.address < lhs.start()) {
        return false;
      }
      return lhs.create_ts() > rhs.ts;
    }
  };

 private:
  explicit VirtualAddressSpace(std::vector<MappingVersion> mappings)
      : mappings_(std::move(mappings)) {}
  std::vector<MappingVersion> mappings_;
};

}  // namespace etm
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_VIRTUAL_ADDRESS_SPACE_H_
