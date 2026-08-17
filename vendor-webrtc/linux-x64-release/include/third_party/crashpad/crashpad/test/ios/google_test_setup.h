// Copyright 2019 The Crashpad Authors
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

#ifndef CRASHPAD_TEST_IOS_GOOGLE_TEST_SETUP_
#define CRASHPAD_TEST_IOS_GOOGLE_TEST_SETUP_

namespace crashpad {
namespace test {

//! \brief Runs all registered tests in the context of a UIKit application.
//!
//! Invokes UIApplicationMain() to launch the iOS application and runs all
//! registered tests after the application finishes
//! launching. UIApplicationMain() brings up the main runloop and never returns,
//! so therefore this function never returns either.  It invokes _exit() to
//! terminate the application after tests have completed.
void IOSLaunchApplicationAndRunTests(int argc, char* argv[]);

}  // namespace test
}  // namespace crashpad

#endif  // CRASHPAD_TEST_IOS_GOOGLE_TEST_SETUP_
