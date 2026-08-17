/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_TEST_DEPENDENCY_FACTORY_H_
#define API_TEST_TEST_DEPENDENCY_FACTORY_H_

#include <memory>

#include "api/test/video_quality_test_fixture.h"

namespace webrtc {

// Override this class if to inject custom components into WebRTC tests.
// Not all WebRTC tests get their components from here, so you need to make
// sure the tests you want actually use this class.
//
// This class is not thread safe and you need to make call calls from the same
// (test main) thread.
class TestDependencyFactory {
 public:
  virtual ~TestDependencyFactory() = default;

  // The singleton MUST be stateless since tests execute in any order. It must
  // be set before tests start executing.
  static const TestDependencyFactory& GetInstance();
  static void SetInstance(std::unique_ptr<TestDependencyFactory> instance);

  // Returns the component a test should use. Returning nullptr means that the
  // test is free to use whatever defaults it wants. The injection components
  // themselves can be mutable, but we need to make new ones for every test that
  // executes so state doesn't spread between tests.
  virtual std::unique_ptr<VideoQualityTestFixtureInterface::InjectionComponents>
  CreateComponents() const;

 private:
  static std::unique_ptr<TestDependencyFactory> instance_;
};

}  // namespace webrtc

#endif  // API_TEST_TEST_DEPENDENCY_FACTORY_H_
