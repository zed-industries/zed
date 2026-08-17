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

#ifndef CRASHPAD_UTIL_IOS_SCOPED_VM_READ_H_
#define CRASHPAD_UTIL_IOS_SCOPED_VM_READ_H_

#include <mach/mach.h>


namespace crashpad {
namespace internal {

//! \brief Non-templated internal class to be used by ScopedVMRead.
//!
//! Note: RUNS-DURING-CRASH.
class ScopedVMReadInternal {
 public:
  ScopedVMReadInternal();

  ScopedVMReadInternal(const ScopedVMReadInternal&) = delete;
  ScopedVMReadInternal& operator=(const ScopedVMReadInternal&) = delete;

  ~ScopedVMReadInternal();

  //! \brief Releases any previously read data and vm_reads \a data. Logs an
  //!     error on failure.
  //!
  //! \param[in] data Memory to be read by vm_read.
  //! \param[in] data_length Length of \a data.
  //!
  //! \return `true` if all the data was read. Logs an error and returns false
  //!   on failure
  bool Read(const void* data, size_t data_length);

  vm_address_t data() const { return data_; }

 private:
  //! \brief Deallocates any resources allocated by this object and resets it
  //!     to its original state.
  void Reset();

  // The address within region_start_ at which the the data is available.
  vm_address_t data_;

  // The region returned by vm_read().
  vm_address_t region_start_;

  // The size of the region returned by vm_read().
  mach_msg_type_number_t region_size_;
};

//! \brief A scoped wrapper for calls to `vm_read` and `vm_deallocate`.  Allows
//!     in-process handler to safely read memory for the intermediate dump.
//!
//! Note: RUNS-DURING-CRASH.
template <typename T>
class ScopedVMRead {
 public:
  ScopedVMRead() : internal_() {}

  ScopedVMRead(const ScopedVMRead&) = delete;
  ScopedVMRead& operator=(const ScopedVMRead&) = delete;

  ~ScopedVMRead() {}

  //! \brief Releases any previously read data and vm_reads data.
  //!
  //! \param[in] data Memory to be read by vm_read.
  //! \param[in] count Length of \a data.
  //!
  //! \return `true` if all \a data was read. Returns false on failure.
  bool Read(const void* data, size_t count = 1) {
    size_t data_length = count * sizeof(T);
    return internal_.Read(data, data_length);
  }

  //! \brief Releases any previously read data and vm_reads address.
  //!
  //! \param[in] address Address of memory to be read by vm_read.
  //! \param[in] count Length of \a data.
  //!
  //! \return `true` if all of \a address was read. Returns false on failure.
  bool Read(vm_address_t address, size_t count = 1) {
    return Read(reinterpret_cast<T*>(address), count);
  }

  //! \brief Returns the pointer to memory safe to read during the in-process
  //!   crash handler.
  T* operator->() const { return get(); }

  //! \brief Returns the pointer to memory safe to read during the in-process
  //!   crash handler.
  T* get() const { return reinterpret_cast<T*>(internal_.data()); }

 private:
  ScopedVMReadInternal internal_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_SCOPED_VM_READ_H_
