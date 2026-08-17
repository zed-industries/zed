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

#ifndef CRASHPAD_UTIL_LINUX_SCOPED_PR_SET_DUMPABLE_H_
#define CRASHPAD_UTIL_LINUX_SCOPED_PR_SET_DUMPABLE_H_


namespace crashpad {

class ScopedPrSetDumpable {
 public:
  //! \brief Uses `PR_SET_DUMPABLE` to make the current process dumpable.
  //!
  //! Restores the dumpable flag to its original value on destruction. If the
  //! original value couldn't be determined, the destructor attempts to restore
  //! the flag to 0 (non-dumpable).
  //!
  //! \param[in] may_log `true` if this object may log error messages.
  explicit ScopedPrSetDumpable(bool may_log);

  ScopedPrSetDumpable(const ScopedPrSetDumpable&) = delete;
  ScopedPrSetDumpable& operator=(const ScopedPrSetDumpable&) = delete;

  ~ScopedPrSetDumpable();

 private:
  bool was_dumpable_;
  bool may_log_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_LINUX_SCOPED_PR_SET_DUMPABLE_H_
