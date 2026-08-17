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

#ifndef CRASHPAD_TEST_SCOPED_GUARDED_PAGE_
#define CRASHPAD_TEST_SCOPED_GUARDED_PAGE_


namespace crashpad {
namespace test {

//! \brief A RAII object that allocates a read-write page with an inacessible
//!     page following it.
//!
//! Upon construction, a mapping will be created. Failure to create the mapping
//! is fatal. On destruction, the mapping is freed.
//!
//! This object should not be used in multi-threded contexts, the POSIX
//! implementation can not be made thread-safe.
class ScopedGuardedPage {
 public:
  ScopedGuardedPage();

  ScopedGuardedPage(const ScopedGuardedPage&) = delete;
  ScopedGuardedPage& operator=(const ScopedGuardedPage&) = delete;

  ~ScopedGuardedPage();

  //! \brief Returns the address of the read-write page.
  //!
  //! \return The address of the read-write page.
  void* Pointer() const { return ptr_; }

 private:
  void* ptr_;
};

}  // namespace test
}  // namespace crashpad

#endif  // CRASHPAD_TEST_SCOPED_GUARDED_PAGE_
