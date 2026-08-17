// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_GWP_ASAN_SUPPORT_H_
#define PARTITION_ALLOC_GWP_ASAN_SUPPORT_H_

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(ENABLE_GWP_ASAN_SUPPORT)

#include <cstddef>
#include <cstdint>
#include <vector>

namespace partition_alloc {

// This class allows GWP-ASan allocations to be backed by PartitionAlloc and,
// consequently, protected by MiraclePtr.
//
// GWP-ASan mainly operates at the system memory page granularity. During
// process startup, it reserves a certain number of consecutive system pages.
//
// The standard layout is as follows:
//
//   +-------------------+--------
//   |                   | ▲   ▲
//   |   system page 0   |(a) (c)
//   |                   | ▼   ▼
//   +-------------------+--------
//   |                   | ▲   ▲
//   |   system page 1   |(b)  |
//   |                   | ▼   |
//   +-------------------+--- (d)    (a) inaccessible
//   |                   | ▲   |     (b) accessible
//   |   system page 2   |(a)  |     (c) initial guard page
//   |                   | ▼   ▼     (d) allocation slot
//   +-------------------+--------
//   |                   | ▲   ▲
//   |   system page 3   |(b)  |
//   |                   | ▼   |
//   +-------------------+--- (d)
//   |                   | ▲   |
//   |   system page 4   |(a)  |
//   |                   | ▼   ▼
//   |-------------------|--------
//   |                   | ▲   ▲
//   |        ...        |(a) (d)
//
// Unfortunately, PartitionAlloc can't provide GWP-ASan an arbitrary number of
// consecutive allocation slots. Allocations need to be grouped into 2MB super
// pages so that the allocation metadata can be easily located.
//
// Below is the new layout:
//
//   +-----------------------------------
//   |                   |         ▲   ▲
//   |   system page 0   |         |   |
//   |                   |         |   |
//   +-------------------+         |   |
//   |                   |         |   |
//   |        ...        |        (e)  |
//   |                   |         |   |
//   +-------------------+-------  |   |
//   |                   | ▲   ▲   |   |
//   |  system page k-1  |(a) (c)  |   |
//   |                   | ▼   ▼   ▼   |
//   +-------------------+----------- (f)
//   |                   | ▲   ▲       |
//   |   system page k   |(b)  |       |
//   |                   | ▼   |       |
//   +-------------------+--- (d)      |
//   |                   | ▲   |       |
//   |  system page k+1  |(a)  |       |
//   |                   | ▼   ▼       |
//   +-------------------+-----------  |
//   |                   |             |    (a) inaccessible
//   |        ...        |             |    (b) accessible
//   |                   |             ▼    (c) initial guard page
//   +-----------------------------------   (d) allocation slot
//   |                   |         ▲   ▲    (e) super page metadata
//   |   system page m   |         |   |    (f) super page
//   |                   |         |   |    (g) pseudo allocation slot
//   +-------------------+-------  |   |
//   |                   |     ▲   |   |
//   |        ...        |     |  (e)  |
//   |                   |     |   |   |
//   +-------------------+--- (g)  |   |
//   |                   | ▲   |   |   |
//   | system page m+k-1 |(a)  |   |   |
//   |                   | ▼   ▼   ▼   |
//   +-------------------+----------- (f)
//   |                   | ▲   ▲       |
//   |  system page m+k  |(b)  |       |
//   |                   | ▼   |       |
//   +-------------------+--- (d)      |
//   |                   | ▲   |       |
//   | system page m+k+1 |(a)  |       |
//   |                   | ▼   ▼       |
//   +-------------------+-----------  |
//   |                   |             |
//   |        ...        |             |
//   |                   |             ▼
//   +-------------------+---------------
//
// This means some allocation slots will be reserved to hold PA
// metadata. We exclude these pseudo slots from the GWP-ASan free list so that
// they are never used for anything other that storing the metadata.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) GwpAsanSupport {
 public:
  static void* MapRegion(size_t slot_count, std::vector<uint16_t>& free_list);
  static bool CanReuse(uintptr_t slot_start);
  static void DestructForTesting();
};

}  // namespace partition_alloc

#endif  // PA_BUILDFLAG(ENABLE_GWP_ASAN_SUPPORT)

#endif  // PARTITION_ALLOC_GWP_ASAN_SUPPORT_H_
