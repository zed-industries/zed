// Copyright 2021 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_IOS_PACK_IOS_MAP_H_
#define CRASHPAD_UTIL_IOS_PACK_IOS_MAP_H_

#include <map>
#include <memory>

#include "util/ios/ios_intermediate_dump_format.h"
#include "util/ios/ios_intermediate_dump_object.h"

namespace crashpad {
namespace internal {

class IOSIntermediateDumpList;
class IOSIntermediateDumpData;

//! \brief A map object containing a IntermediateDump Key-Object pair.
//!
//! Also provides an element access helper.
class IOSIntermediateDumpMap : public IOSIntermediateDumpObject {
 public:
  IOSIntermediateDumpMap();

  IOSIntermediateDumpMap(const IOSIntermediateDumpMap&) = delete;
  IOSIntermediateDumpMap& operator=(const IOSIntermediateDumpMap&) = delete;

  ~IOSIntermediateDumpMap() override;

  // IOSIntermediateDumpObject:
  Type GetType() const override;

  //! \brief Returns an IOSIntermediateDumpData. If the type is not kData,
  //!     returns nullptr
  const IOSIntermediateDumpData* GetAsData(
      const IntermediateDumpKey& key) const;

  //! \brief Returns an IOSIntermediateDumpList. If the type is not kList,
  //!     returns nullptr
  const IOSIntermediateDumpList* GetAsList(
      const IntermediateDumpKey& key) const;

  //! \brief Returns an IOSIntermediateDumpMap.  If the type is not kMap,
  //!     returns nullptr
  const IOSIntermediateDumpMap* GetAsMap(const IntermediateDumpKey& key) const;

  //! \brief Returns `true` if the map is empty.
  bool empty() const { return map_.empty(); }

 private:
  friend class IOSIntermediateDumpReader;
  std::map<IntermediateDumpKey, std::unique_ptr<IOSIntermediateDumpObject>>
      map_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_PACK_IOS_MAP_H_
