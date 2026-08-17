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

#ifndef CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_READER_H_
#define CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_READER_H_

#include "util/ios/ios_intermediate_dump_interface.h"
#include "util/ios/ios_intermediate_dump_map.h"
#include "util/misc/initialization_state_dcheck.h"

namespace crashpad {
namespace internal {

//! \brief The return value for IOSIntermediateDumpReader::Initialize.
enum class IOSIntermediateDumpReaderInitializeResult : int {
  //! \brief The intermediate dump was read successfully, initialization
  //!     succeeded.
  kSuccess,

  //! \brief The intermediate dump could be loaded, but parsing was incomplete.
  //!     An attempt to parse the RootMap should still be made, as there may
  //!     still be valuable information to put into a minidump.
  kIncomplete,

  //! \brief The intermediate dump could not be loaded, initialization failed.
  kFailure,
};

//! \brief Open and parse iOS intermediate dumps.
class IOSIntermediateDumpReader {
 public:
  IOSIntermediateDumpReader() {}

  IOSIntermediateDumpReader(const IOSIntermediateDumpReader&) = delete;
  IOSIntermediateDumpReader& operator=(const IOSIntermediateDumpReader&) =
      delete;

  //! \brief Open and parses \a dump_interface.
  //!
  //! Will attempt to parse the binary file, similar to a JSON file, using the
  //! same format used by IOSIntermediateDumpWriter, resulting in an
  //! IOSIntermediateDumpMap
  //!
  //! \param[in] dump_interface An interface corresponding to an intermediate
  //!     dump file.
  //!
  //! \return On success, returns `true`, otherwise returns `false`. Clients may
  //!     still attempt to parse RootMap, as partial minidumps may still be
  //!     usable.
  IOSIntermediateDumpReaderInitializeResult Initialize(
      const IOSIntermediateDumpInterface& dump_interface);

  //! \brief Returns an IOSIntermediateDumpMap corresponding to the root of the
  //!     intermediate dump.
  const IOSIntermediateDumpMap* RootMap();

 private:
  bool Parse(FileReaderInterface* reader, FileOffset file_size);
  IOSIntermediateDumpMap intermediate_dump_;
  InitializationStateDcheck initialized_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_READER_H_
