// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_SWITCHES_H_
#define BASE_TEST_TEST_SWITCHES_H_

#include "build/build_config.h"

namespace switches {

// All switches in alphabetical order. The switches should be documented
// alongside the definition of their values in the .cc file.
extern const char kEnforceExactPositiveFilter[];
extern const char kHelpFlag[];
extern const char kIsolatedScriptTestLauncherRetryLimit[];
extern const char kRebaselinePixelTests[];
extern const char kSingleProcessTests[];
extern const char kTestLauncherBatchLimit[];
extern const char kTestLauncherBotMode[];
extern const char kTestLauncherDebugLauncher[];
extern const char kTestLauncherFilterFile[];
extern const char kTestLauncherForceRunBrokenTests[];
extern const char kTestLauncherInteractive[];
extern const char kTestLauncherJobs[];
extern const char kTestLauncherListTests[];
extern const char kTestLauncherOutput[];
extern const char kTestLauncherOutputBytesLimit[];
extern const char kTestLauncherPrintTempLeaks[];
extern const char kTestLauncherPrintTestStdio[];
extern const char kTestLauncherPrintTimestamps[];
extern const char kTestLauncherPrintWritablePath[];
extern const char kTestLauncherRetriesLeft[];
extern const char kTestLauncherRetryLimit[];
extern const char kTestLauncherShardIndex[];
extern const char kTestLauncherSummaryOutput[];
extern const char kTestLauncherTestPartResultsLimit[];
extern const char kTestLauncherTotalShards[];
extern const char kTestLauncherTimeout[];
extern const char kTestLauncherTrace[];
extern const char kTestTinyTimeout[];
extern const char kUiTestActionMaxTimeout[];
extern const char kUiTestActionTimeout[];
extern const char kWithDeathTestStackTraces[];
extern const char kFuzz[];
extern const char kFuzzFor[];
extern const char kListFuzzTests[];

#if BUILDFLAG(IS_IOS)
extern const char kEnableRunIOSUnittestsWithXCTest[];
extern const char kWriteCompiledTestsJsonToWritablePath[];
#endif

}  // namespace switches

#endif  // BASE_TEST_TEST_SWITCHES_H_
