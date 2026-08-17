// Copyright 2018 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_SNAPSHOT_FUCHSIA_MEMORY_MAP_REGION_SNAPSHOT_FUCHSIA_H_
#define CRASHPAD_SNAPSHOT_FUCHSIA_MEMORY_MAP_REGION_SNAPSHOT_FUCHSIA_H_

#include "snapshot/memory_map_region_snapshot.h"

#include <zircon/syscalls/object.h>

namespace crashpad {
namespace internal {

class MemoryMapRegionSnapshotFuchsia : public MemoryMapRegionSnapshot {
 public:
  explicit MemoryMapRegionSnapshotFuchsia(const zx_info_maps_t& info_map);
  ~MemoryMapRegionSnapshotFuchsia() override;

  virtual const MINIDUMP_MEMORY_INFO& AsMinidumpMemoryInfo() const override;

 private:
  MINIDUMP_MEMORY_INFO memory_info_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_FUCHSIA_MEMORY_MAP_REGION_SNAPSHOT_FUCHSIA_H_
