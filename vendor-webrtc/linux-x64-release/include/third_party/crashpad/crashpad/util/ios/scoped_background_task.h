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

#ifndef CRASHPAD_UTIL_IOS_SCOPED_BACKGROUND_TASK_H_
#define CRASHPAD_UTIL_IOS_SCOPED_BACKGROUND_TASK_H_

#include <dispatch/dispatch.h>

namespace crashpad {
namespace internal {

//! \brief Marks the start of a task that should continue if the application
//!     enters the background.
class ScopedBackgroundTask {
 public:
  //! \param[in] task_name A string used in debugging to indicate the reason the
  //!     activity began. This parameter must not be nullptr or an empty string.
  ScopedBackgroundTask(const char* task_name);

  ScopedBackgroundTask(const ScopedBackgroundTask&) = delete;
  ScopedBackgroundTask& operator=(const ScopedBackgroundTask&) = delete;
  ~ScopedBackgroundTask();

 private:
  dispatch_semaphore_t task_complete_semaphore_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_SCOPED_BACKGROUND_TASK_H_
