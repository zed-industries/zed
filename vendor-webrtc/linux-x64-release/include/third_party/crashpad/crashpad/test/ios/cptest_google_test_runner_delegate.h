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

#ifndef CRASHPAD_TEST_IOS_CPTEST_GOOGLE_TEST_RUNNER_DELEGATE_
#define CRASHPAD_TEST_IOS_CPTEST_GOOGLE_TEST_RUNNER_DELEGATE_

@protocol CPTestGoogleTestRunnerDelegate

// Returns YES if this delegate supports running Google Test tests via a call to
// |runGoogleTests|.
@property(nonatomic, readonly, assign)
    BOOL supportsRunningGoogleTestsWithXCTest;

// Runs Google Test tests and returns the final exit code.
- (int)runGoogleTests;

@end

#endif  // CRASHPAD_TEST_IOS_CPTEST_GOOGLE_TEST_RUNNER_DELEGATE_H_
