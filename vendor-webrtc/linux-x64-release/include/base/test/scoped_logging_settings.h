// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_LOGGING_SETTINGS_H_
#define BASE_TEST_SCOPED_LOGGING_SETTINGS_H_

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "base/logging.h"
#include "base/memory/raw_ptr.h"
#include "build/build_config.h"

namespace logging {

class VlogInfo;

// Saves the current logging settings and restores them when destroyed.
// This is used by logging tests to avoid affecting later tests that
// may run afterward, in the same process.
// Note that this helper cannot be used when an un-named log-file is configured
// via |LoggingSettings::log_file|.
class BASE_EXPORT ScopedLoggingSettings {
 public:
  ScopedLoggingSettings();
  ~ScopedLoggingSettings();

  ScopedLoggingSettings(const ScopedLoggingSettings&) = delete;
  ScopedLoggingSettings& operator=(const ScopedLoggingSettings&) = delete;

#if BUILDFLAG(IS_CHROMEOS)
  void SetLogFormat(LogFormat) const;
#endif

 private:
  // Please keep the following fields in the same order as the corresponding
  // globals in //base/logging.cc

  const int min_log_level_;
  const uint32_t logging_destination_;

#if BUILDFLAG(IS_CHROMEOS)
  const LogFormat log_format_;
#endif  // BUILDFLAG(IS_CHROMEOS)

  base::FilePath::StringType log_file_name_;

  const bool enable_process_id_;
  const bool enable_thread_id_;
  const bool enable_timestamp_;
  const bool enable_tickcount_;
  const char* const log_prefix_;

  const LogMessageHandlerFunction message_handler_;
};

// Replaces the existing VLOG config with a new one based on it
// but with extra modules enabled.
//
// *** Using this leaks memory ***
//
// For thread safety, we cannot delete the VlogInfo object created by this.
//
// This is intended for use in testing only, e.g. in the setup of a test, enable
// vlogging for modules that are of interest. This can help debug a flaky test
// which cannot be reproduced locally while avoiding log-spam from the rest of
// the code.
//
// This follows the same pattern as ScopedFeatureList, with init separate from
// construction to allow easy use in test classes.
//
// Using this on multiple threads requires coordination, ScopedVmoduleSwitches
// must be destroyed in reverse creation order.
class BASE_EXPORT ScopedVmoduleSwitches {
 public:
  explicit ScopedVmoduleSwitches();
  // Specify which modules and levels to enable. This uses the same syntax as
  // the commandline flag, e.g. "file=1,dir/other_file=2".
  void InitWithSwitches(const std::string& vmodule_switch);
  ~ScopedVmoduleSwitches();

 private:
  // Creates a new instance of |VlogInfo| adding |vmodule_switch|.
  VlogInfo* CreateVlogInfoWithSwitches(const std::string& vmodule_switch);
  raw_ptr<VlogInfo> scoped_vlog_info_ = nullptr;
  raw_ptr<VlogInfo> previous_vlog_info_ = nullptr;
};
}  // namespace logging

#endif  // BASE_TEST_SCOPED_LOGGING_SETTINGS_H_
