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

#ifndef CRASHPAD_CLIENT_UPLOAD_BEHAVIOR_IOS_H_
#define CRASHPAD_CLIENT_UPLOAD_BEHAVIOR_IOS_H_

namespace crashpad {

//! \brief Enum to control upload behavior when processing pending reports.
enum class UploadBehavior {
  //! \brief Only upload reports while the application is active (e.g., in the
  //!     foreground).
  kUploadWhenAppIsActive = 1,

  //! \brief Upload reports immediately, regardless of whether or not the
  //!     application is active.
  kUploadImmediately = 2,
};

}  // namespace crashpad

#endif  // CRASHPAD_CLIENT_UPLOAD_BEHAVIOR_IOS_H_
