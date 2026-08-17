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

#include <ostream>

#include "util/ios/ios_intermediate_dump_data.h"
#include "util/ios/ios_intermediate_dump_map.h"

#ifndef CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_READER_UTILS_H_
#define CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_READER_UTILS_H_

namespace crashpad {

namespace internal {

//! \brief Overload the ostream output operator to make logged keys readable.
std::ostream& operator<<(std::ostream& os, const IntermediateDumpKey& t);

//! \brief Determine if GetDataFromMap will log and report missing keys.
enum class LogMissingDataValueFromMap : bool {
  //! \brief Do not log an error and report to UMA if a key is missing.
  kDontLogIfMissing,

  //! \brief Log an error and report to UMA if a key is missing.
  kLogIfMissing,
};

//! \brief Call GetAsData with error and UMA logging.
//!
//! \param[in] map The map to load from.
//! \param[in] key The key to load from \a map.
//! \param[in] logging This call will log missing keys unless \a logging is
//!     LogDataValueFromMap::kDontLogIfMissing
//!
//! \return The IOSIntermediateDumpData pointer or a nullptr;
const IOSIntermediateDumpData* GetDataFromMap(
    const IOSIntermediateDumpMap* map,
    const IntermediateDumpKey& key,
    LogMissingDataValueFromMap logging =
        LogMissingDataValueFromMap::kLogIfMissing);

//! \brief Call GetAsMap with error and UMA logging.
//!
//! \param[in] map The map to load from.
//! \param[in] key The key to load from \a map.
//!
//! \return The IOSIntermediateDumpMap pointer or a nullptr;
const IOSIntermediateDumpMap* GetMapFromMap(const IOSIntermediateDumpMap* map,
                                            const IntermediateDumpKey& key);

//! \brief Call GetAsList with error and UMA logging.
//!
//! \param[in] map The map to load from.
//! \param[in] key The key to load from \a map.
//!
//! \return The IOSIntermediateDumpList pointer or a nullptr;
const IOSIntermediateDumpList* GetListFromMap(const IOSIntermediateDumpMap* map,
                                              const IntermediateDumpKey& key);

//! \brief Call GetAsList with error and UMA logging.
//!
//! \param[in] map The map to load from.
//! \param[in] key The key to load from \a map.
//! \param[out] value The loaded string.
//!
//! \return Returns `true` with \a value set accordingly if the string could be
//!     loaded, otherwise returns `false` and logs an error.
bool GetDataStringFromMap(const IOSIntermediateDumpMap* map,
                          const IntermediateDumpKey& key,
                          std::string* value);

//! \brief Log key size error and record error with UMA.
void GetDataValueFromMapErrorInternal(const IntermediateDumpKey& key);

//! \brief Call GetAsData and GetValue with error and UMA logging.
//!
//! \param[in] map The map to load from.
//! \param[in] key The key to load from \a map.
//! \param[out] value The data to populate.
//! \param[in] logging This call will log missing keys unless \a logging is
//!     LogDataValueFromMap::kDontLogIfMissing. This call will always log
//!     keys with an invalid size.
//!
//! \return On success, returns `true`, otherwise returns `false`.
template <typename T>
bool GetDataValueFromMap(const IOSIntermediateDumpMap* map,
                         const IntermediateDumpKey& key,
                         T* value,
                         LogMissingDataValueFromMap logging =
                             LogMissingDataValueFromMap::kLogIfMissing) {
  const IOSIntermediateDumpData* data = GetDataFromMap(map, key, logging);
  if (!data) {
    return false;
  }
  if (!data->GetValue(value)) {
    GetDataValueFromMapErrorInternal(key);
    return false;
  }
  return true;
}

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_IOS_INTERMEDIATE_DUMP_READER_UTILS_H_
