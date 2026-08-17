// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_CURRENT_PROCESS_TEST_H_
#define BASE_PROCESS_CURRENT_PROCESS_TEST_H_

#include <string>

#include "base/process/current_process.h"

namespace base::test {

// This class is used for getting current process type and name for testing
// without any access controls.
class CurrentProcessForTest {
 public:
  static CurrentProcessType GetType() {
    return CurrentProcess::GetInstance().GetType(CurrentProcess::TypeKey());
  }

  static std::string GetName() {
    return CurrentProcess::GetInstance().GetName(CurrentProcess::NameKey());
  }
};

}  // namespace base::test

#endif  // BASE_PROCESS_CURRENT_PROCESS_TEST_H_
