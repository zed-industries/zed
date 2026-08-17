// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_ADD_FEATURE_FLAGS_H_
#define BASE_SCOPED_ADD_FEATURE_FLAGS_H_

#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/feature_list.h"
#include "base/memory/raw_ptr.h"

namespace base {

class CommandLine;

// Helper class to enable and disable features if they are not already set in
// the command line. It reads the command line on construction, allows user to
// enable and disable features during its lifetime, and writes the modified
// --enable-features=... and --disable-features=... flags back to the command
// line on destruction.
class BASE_EXPORT ScopedAddFeatureFlags {
 public:
  explicit ScopedAddFeatureFlags(CommandLine* command_line);

  ScopedAddFeatureFlags(const ScopedAddFeatureFlags&) = delete;
  ScopedAddFeatureFlags& operator=(const ScopedAddFeatureFlags&) = delete;

  ~ScopedAddFeatureFlags();

  // Any existing (user set) enable/disable takes precedence.
  void EnableIfNotSet(const Feature& feature);
  void DisableIfNotSet(const Feature& feature);
  void EnableIfNotSetWithParameter(const Feature& feature,
                                   std::string_view name,
                                   std::string_view value);

  // Check if the feature is enabled from command line or functions above
  bool IsEnabled(const Feature& feature);

  // Check if the feature with the given parameter name and value is enabled
  // from command line or functions above. An empty parameter name means that we
  // are checking if the feature is enabled without any parameter.
  bool IsEnabledWithParameter(const Feature& feature,
                              std::string_view parameter_name,
                              std::string_view parameter_value);

 private:
  void AddFeatureIfNotSet(const Feature& feature,
                          std::string_view suffix,
                          bool enable);

  const raw_ptr<CommandLine> command_line_;
  std::vector<std::string> enabled_features_;
  std::vector<std::string> disabled_features_;
};

}  // namespace base

#endif  // BASE_SCOPED_ADD_FEATURE_FLAGS_H_
