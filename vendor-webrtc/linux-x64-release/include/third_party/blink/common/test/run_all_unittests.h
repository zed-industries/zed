// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_TEST_RUN_ALL_UNITTESTS_H_
#define THIRD_PARTY_BLINK_COMMON_TEST_RUN_ALL_UNITTESTS_H_

#include "base/test/launcher/unit_test_launcher.h"

base::RunTestSuiteCallback GetLaunchCallback(int argc, char** argv);

#endif  // THIRD_PARTY_BLINK_COMMON_TEST_RUN_ALL_UNITTESTS_H_
