// Copyright 2022 The Crashpad Authors
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

#ifndef CRASHPAD_TEST_SCOPED_SET_THREAD_NAME_H_
#define CRASHPAD_TEST_SCOPED_SET_THREAD_NAME_H_

#include <string>

#include "build/build_config.h"

namespace crashpad {
namespace test {

//! Sets the name of the current thread for the lifetime of this object.
class ScopedSetThreadName final {
 public:
  explicit ScopedSetThreadName(const std::string& new_thread_name);

  ScopedSetThreadName(const ScopedSetThreadName&) = delete;
  ScopedSetThreadName& operator=(const ScopedSetThreadName&) = delete;

  ~ScopedSetThreadName();

#if BUILDFLAG(IS_WIN) || DOXYGEN
  //! \brief Returns `true` if Windows supports setting and getting thread name.
  static bool IsSupported();
#endif

 private:
#if BUILDFLAG(IS_WIN)
  std::wstring original_name_;
#else
  const std::string original_name_;
#endif
};

}  // namespace test
}  // namespace crashpad

#endif  // CRASHPAD_TEST_SCOPED_SET_THREAD_NAME_H_
