// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_TEST_TEST_H_
#define IPCZ_SRC_TEST_TEST_H_

#include "test/test_base.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace ipcz::test {

// Base class for ipcz unit tests.
class Test : public internal::TestBase, public ::testing::Test {};

}  // namespace ipcz::test

#endif  // IPCZ_SRC_TEST_TEST_H_
