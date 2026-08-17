// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_FUZZER_IPCZ_FUZZER_TESTCASE_H_
#define IPCZ_FUZZER_IPCZ_FUZZER_TESTCASE_H_

namespace ipcz::fuzzer {

class Fuzzer;

// Helper class to execute a test scenario which exercises many interesting
// codepaths within ipcz, while driven by a Fuzzer. This can be used both to
// generate a seed corpus as well as to run new fuzzer inputs.
class IpczFuzzerTestcase {
 public:
  explicit IpczFuzzerTestcase(Fuzzer& fuzzer);

  // Runs the testcase to completion.
  void Run();

 private:
  Fuzzer& fuzzer_;
};

}  // namespace ipcz::fuzzer

#endif  // IPCZ_FUZZER_IPCZ_FUZZER_TESTCASE_H_
