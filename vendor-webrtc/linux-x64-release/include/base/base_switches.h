// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Defines all the "base" command-line switches.

#ifndef BASE_BASE_SWITCHES_H_
#define BASE_BASE_SWITCHES_H_

#include "build/build_config.h"

namespace switches {

extern const char kDisableBestEffortTasks[];
extern const char kDisableBreakpad[];
extern const char kDisableFeatures[];
extern const char kDisableLowEndDeviceMode[];
extern const char kEnableCrashReporter[];
extern const char kEnableFeatures[];
extern const char kEnableLowEndDeviceMode[];
extern const char kEnableBackgroundThreadPool[];
extern const char kFieldTrialHandle[];
extern const char kForceFieldTrials[];
extern const char kFullMemoryCrashReport[];
extern const char kLogBestEffortTasks[];
extern const char kMetricsSharedMemoryHandle[];
extern const char kNoErrorDialogs[];
extern const char kProfilingAtStart[];
extern const char kProfilingFile[];
extern const char kProfilingFlush[];
extern const char kTestChildProcess[];
extern const char kTraceToFile[];
extern const char kTraceToFileName[];
extern const char kV[];
extern const char kVModule[];
extern const char kWaitForDebugger[];

#if BUILDFLAG(IS_WIN)
extern const char kDisableHighResTimer[];
extern const char kDisableUsbKeyboardDetect[];
#endif

#if BUILDFLAG(IS_LINUX)
extern const char kDisableDevShmUsage[];
#endif

#if BUILDFLAG(IS_POSIX)
extern const char kEnableCrashReporterForTesting[];
#endif

#if BUILDFLAG(IS_ANDROID)
extern const char kAndroidSkipChildServiceInitForTesting[];
extern const char kDefaultCountryCodeAtInstall[];
extern const char kEnableIdleTracing[];
extern const char kHostPackageName[];
extern const char kHostPackageLabel[];
extern const char kHostVersionCode[];
extern const char kPackageName[];
extern const char kPackageVersionName[];
#endif

#if BUILDFLAG(IS_CHROMEOS)
extern const char kSchedulerBoostUrgent[];
#endif

}  // namespace switches

#endif  // BASE_BASE_SWITCHES_H_
