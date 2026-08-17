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

#ifndef CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_DATA_H_
#define CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_DATA_H_

#include <string>
#include <vector>

#include "util/ios/ios_intermediate_dump_object.h"

namespace crashpad {
namespace internal {

//! \brief A data object, consisting of a std::vector<uint8_t>.
class IOSIntermediateDumpData : public IOSIntermediateDumpObject {
 public:
  IOSIntermediateDumpData();

  IOSIntermediateDumpData(const IOSIntermediateDumpData&) = delete;
  IOSIntermediateDumpData& operator=(const IOSIntermediateDumpData&) = delete;

  ~IOSIntermediateDumpData() override;

  //! \brief Constructs a new data object which owns a std::vector<uint8_t>.
  //!
  //! \param[in] data A vector of uint8_t.
  IOSIntermediateDumpData(std::vector<uint8_t> data) : data_(std::move(data)) {}

  // IOSIntermediateDumpObject:
  Type GetType() const override;

  //! \brief Returns data as a string.
  std::string GetString() const;

  //! \brief Copies the data into \a value if sizeof(T) matches data_.size().
  //!
  //! \param[out] value The data to populate.
  //!
  //! \return On success, returns `true`, otherwise returns `false`.
  template <typename T>
  bool GetValue(T* value) const {
    return GetValueInternal(reinterpret_cast<void*>(value), sizeof(*value));
  }

  const std::vector<uint8_t>& bytes() const { return data_; }

 private:
  bool GetValueInternal(void* value, size_t value_size) const;

  std::vector<uint8_t> data_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_DATA_H_
