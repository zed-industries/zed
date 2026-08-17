// Copyright (c) 2017 Project Vogue. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FONT_TEST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FONT_TEST_BASE_H_

#include "base/test/task_environment.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace blink {

class FontTestBase : public ::testing::Test {
 protected:
  FontTestBase();
  ~FontTestBase() override;

  base::test::TaskEnvironment task_environment_{
      base::test::TaskEnvironment::TimeSource::MOCK_TIME};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FONT_TEST_BASE_H_
