// Copyright 2020 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_MODULE_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_MODULE_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_

#include <mach-o/dyld_images.h>
#include <stdint.h>
#include <sys/types.h>

#include <map>
#include <string>
#include <vector>

#include "snapshot/ios/memory_snapshot_ios_intermediate_dump.h"
#include "snapshot/module_snapshot.h"
#include "util/ios/ios_intermediate_dump_map.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief A ModuleSnapshot of a code module (binary image) loaded into a
//!     running (or crashed) process on an iOS system.
class ModuleSnapshotIOSIntermediateDump final : public ModuleSnapshot {
 public:
  ModuleSnapshotIOSIntermediateDump();

  ModuleSnapshotIOSIntermediateDump(const ModuleSnapshotIOSIntermediateDump&) =
      delete;
  ModuleSnapshotIOSIntermediateDump& operator=(
      const ModuleSnapshotIOSIntermediateDump&) = delete;

  ~ModuleSnapshotIOSIntermediateDump() override;

  //! \brief Initialize the snapshot
  //!
  //! \param[in] image_data The intermediate dump map used to initialize this
  //!     object.
  //!
  //! \return `true` if the snapshot could be created.
  bool Initialize(const IOSIntermediateDumpMap* image_data);

  // ModuleSnapshot:
  std::string Name() const override;
  uint64_t Address() const override;
  uint64_t Size() const override;
  time_t Timestamp() const override;
  void FileVersion(uint16_t* version_0,
                   uint16_t* version_1,
                   uint16_t* version_2,
                   uint16_t* version_3) const override;
  void SourceVersion(uint16_t* version_0,
                     uint16_t* version_1,
                     uint16_t* version_2,
                     uint16_t* version_3) const override;
  ModuleType GetModuleType() const override;
  void UUIDAndAge(UUID* uuid, uint32_t* age) const override;
  std::string DebugFileName() const override;
  std::vector<uint8_t> BuildID() const override;
  std::vector<std::string> AnnotationsVector() const override;
  std::map<std::string, std::string> AnnotationsSimpleMap() const override;
  std::vector<AnnotationSnapshot> AnnotationObjects() const override;
  std::set<CheckedRange<uint64_t>> ExtraMemoryRanges() const override;
  std::vector<const UserMinidumpStream*> CustomMinidumpStreams() const override;

  // Used by ProcessSnapshot
  std::vector<const MemorySnapshot*> ExtraMemory() const;
  std::vector<const MemorySnapshot*> IntermediateDumpExtraMemory() const;

 private:
  std::string name_;
  uint64_t address_;
  uint64_t size_;
  time_t timestamp_;
  uint32_t dylib_version_;
  uint64_t source_version_;
  uint32_t filetype_;
  UUID uuid_;
  std::vector<std::string> annotations_vector_;
  std::map<std::string, std::string> annotations_simple_map_;
  std::vector<AnnotationSnapshot> annotation_objects_;
  std::vector<std::unique_ptr<internal::MemorySnapshotIOSIntermediateDump>>
      extra_memory_;
  std::vector<std::unique_ptr<internal::MemorySnapshotIOSIntermediateDump>>
      intermediate_dump_extra_memory_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_MODULE_SNAPSHOT_IOS_INTERMEDIATEDUMP_H_
