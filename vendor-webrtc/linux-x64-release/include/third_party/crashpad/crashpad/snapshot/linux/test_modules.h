// Copyright 2020 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_LINUX_TEST_MODULES_H_
#define CRASHPAD_SNAPSHOT_LINUX_TEST_MODULES_H_

#include <string>

#include "test/scoped_module_handle.h"

namespace crashpad {
namespace test {

//! \brief Constructs and loads a test module.
//!
//! \param module_name The filename of the mdoule.
//! \param module_soname The SONAME for the module.
//! \return a handle to the loaded module on success. On failure, the handle
//!     will be invalid and a message will be logged.
ScopedModuleHandle LoadTestModule(const std::string& module_name,
                                  const std::string& module_soname);

}  // namespace test
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_LINUX_DEBUG_RENDEZVOUS_H_
