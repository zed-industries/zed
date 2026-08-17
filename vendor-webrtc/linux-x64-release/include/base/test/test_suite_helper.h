// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_SUITE_HELPER_H_
#define BASE_TEST_TEST_SUITE_HELPER_H_

namespace base::test {

class ScopedFeatureList;

void InitScopedFeatureListForTesting(ScopedFeatureList& scoped_feature_list);

}  // namespace base::test

#endif  // BASE_TEST_TEST_SUITE_HELPER_H_
