// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_PROCESS_INFO_H_
#define BASE_PROCESS_PROCESS_INFO_H_

#include "base/base_export.h"
#include "base/process/process_handle.h"
#include "build/build_config.h"

namespace base {

#if BUILDFLAG(IS_WIN)
enum IntegrityLevel {
  INTEGRITY_UNKNOWN,
  UNTRUSTED_INTEGRITY,
  LOW_INTEGRITY,
  MEDIUM_INTEGRITY,
  HIGH_INTEGRITY,
};

// Returns the integrity level of the process with PID `process_id`. Returns
// INTEGRITY_UNKNOWN in the case of an underlying system failure.
BASE_EXPORT IntegrityLevel GetProcessIntegrityLevel(ProcessId process_id);

// Returns the integrity level of the process. Returns INTEGRITY_UNKNOWN in the
// case of an underlying system failure.
BASE_EXPORT IntegrityLevel GetCurrentProcessIntegrityLevel();

// Determines whether the current process is elevated. Note: in some
// configurations this may be true for processes launched without using
// LaunchOptions::elevated.
BASE_EXPORT bool IsCurrentProcessElevated();

// Determines whether the current process is running within an App Container.
BASE_EXPORT bool IsCurrentProcessInAppContainer();

#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_MAC)
// Checks if the responsible process has Bluetooth metadata in its Info.plist
// file. See https://bugs.chromium.org/p/chromium/issues/detail?id=945969 and
// https://bugs.chromium.org/p/chromium/issues/detail?id=996993.
BASE_EXPORT bool DoesResponsibleProcessHaveBluetoothMetadata();
#endif

}  // namespace base

#endif  // BASE_PROCESS_PROCESS_INFO_H_
