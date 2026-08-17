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

#ifndef CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_OBJECT_H_
#define CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_OBJECT_H_

#include "util/ios/ios_intermediate_dump_writer.h"

namespace crashpad {
namespace internal {

//! \brief Base class for intermediate dump object types.
class IOSIntermediateDumpObject {
 public:
  IOSIntermediateDumpObject();

  IOSIntermediateDumpObject(const IOSIntermediateDumpObject&) = delete;
  IOSIntermediateDumpObject& operator=(const IOSIntermediateDumpObject&) =
      delete;

  virtual ~IOSIntermediateDumpObject();

  //! \brief The type of object stored in the intermediate dump.  .
  enum class Type {
    //! \brief A data object, containing array of bytes.
    kData,

    //! \brief A map object, containing other lists, maps and data objects.
    kMap,

    //! \brief A list object, containing a list of map objects.
    kList,
  };

  //! \brief Returns a type.
  virtual Type GetType() const = 0;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_OBJECT_H_
