// Copyright 2023 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_IOS_SCOPED_VM_MAP_H_
#define CRASHPAD_UTIL_IOS_SCOPED_VM_MAP_H_

#include <mach/mach.h>

namespace crashpad {
namespace internal {

//! \brief Non-templated internal class to be used by ScopedVMMap.
//!
//! Note: RUNS-DURING-CRASH.
class ScopedVMMapInternal {
 public:
  ScopedVMMapInternal();

  ScopedVMMapInternal(const ScopedVMMapInternal&) = delete;
  ScopedVMMapInternal& operator=(const ScopedVMMapInternal&) = delete;

  ~ScopedVMMapInternal();

  //! \brief Releases any previously mapped data and vm_remaps \a data. Logs an
  //!     error on failure.
  //!
  //! \param[in] data Memory to be mapped by vm_remap.
  //! \param[in] data_length Length of \a data.
  //!
  //! \return `true` if all the data was mapped. Logs an error and returns false
  //!   on failure.
  bool Map(const void* data, size_t data_length);

  //! \brief Returns the current protection for the memory in the region.
  vm_prot_t CurrentProtection() const { return cur_protection_; }

  vm_address_t data() const { return data_; }

 private:
  //! \brief Deallocates any resources allocated by this object and resets it
  //!     to its original state.
  void Reset();

  // The address within region_start_ at which the mapped data is available.
  vm_address_t data_;

  // The region returned by vm_remap().
  vm_address_t region_start_;

  // The size of the region returned by vm_remap().
  vm_size_t region_size_;

  // The current protection for the memory region.
  vm_prot_t cur_protection_;
};

//! \brief A scoped wrapper for calls to `vm_remap` and `vm_deallocate`.  Allows
//!     in-process handler to safely read and write memory (modulo its
//!     protection level) for the intermediate dump.
//!
//! Note: RUNS-DURING-CRASH.
template <typename T>
class ScopedVMMap {
 public:
  ScopedVMMap() : internal_() {}

  ScopedVMMap(const ScopedVMMap&) = delete;
  ScopedVMMap& operator=(const ScopedVMMap&) = delete;

  ~ScopedVMMap() {}

  //! \brief Releases any previously mapped data and vm_remaps data.
  //!
  //! \param[in] data Memory to be mapped by vm_remap.
  //! \param[in] count Length of \a data.
  //!
  //! \return `true` if all \a data was mapped. Returns false on failure.
  bool Map(const void* data, size_t count = 1) {
    size_t data_length = count * sizeof(T);
    return internal_.Map(data, data_length);
  }

  //! \brief Releases any previously mapped data and vm_remaps address.
  //!
  //! Before reading or writing the memory, check `CurrentProtection()` to
  //! ensure the data is readable or writable.
  //!
  //! \param[in] address Address of memory to be mapped by vm_remap.
  //! \param[in] count Length of \a data.
  //!
  //! \return `true` if all of \a address was mapped. Returns false on failure.
  bool Map(vm_address_t address, size_t count = 1) {
    return Map(reinterpret_cast<T*>(address), count);
  }

  //! \brief Returns the pointer to memory safe to read and write (respecting
  //!   the CurrentProtection() level) during the in-process crash handler.
  T* operator->() const { return get(); }

  //! \brief Returns the pointer to memory safe to read and write (respecting
  //!   the CurrentProtection() level) during the in-process crash handler.
  T* get() const { return reinterpret_cast<T*>(internal_.data()); }

  //! \brief Returns the current protection level of the mapped memory.
  vm_prot_t CurrentProtection() const { return internal_.CurrentProtection(); }

 private:
  ScopedVMMapInternal internal_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_SCOPED_VM_MAP_H_
