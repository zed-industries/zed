// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_FUZZER_SUPPORT_IOS_FUZZER_SUPPORT_H_
#define TESTING_LIBFUZZER_FUZZER_SUPPORT_IOS_FUZZER_SUPPORT_H_

namespace ios_fuzzer {

// Launches an iOS app that runs libFuzzer with given args.
void RunFuzzerFromIOSApp(int argc, char* argv[]);

}  // namespace ios_fuzzer

#endif  // TESTING_LIBFUZZER_FUZZER_SUPPORT_IOS_FUZZER_SUPPORT_H_
