/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_TEST_MAIN_LIB_H_
#define TEST_TEST_MAIN_LIB_H_

#include <memory>
#include <string>

namespace webrtc {

// Class to initialize test environment and run tests.
class TestMain {
 public:
  virtual ~TestMain() {}

  static std::unique_ptr<TestMain> Create();

  // Initializes test environment. Clients can add their own initialization
  // steps after call to this method and before running tests.
  // Returns 0 if initialization was successful and non 0 otherwise.
  virtual int Init() = 0;
  // Temporary for backward compatibility
  virtual int Init(int* argc, char* argv[]) = 0;

  // Runs test end return result error code. 0 - no errors.
  virtual int Run(int argc, char* argv[]) = 0;

 protected:
  TestMain() = default;

  std::string field_trials_;
};

}  // namespace webrtc

#endif  // TEST_TEST_MAIN_LIB_H_
