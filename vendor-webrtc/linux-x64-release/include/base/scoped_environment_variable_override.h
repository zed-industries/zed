// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_ENVIRONMENT_VARIABLE_OVERRIDE_H_
#define BASE_SCOPED_ENVIRONMENT_VARIABLE_OVERRIDE_H_

#include <memory>
#include <optional>
#include <string>

#include "base/base_export.h"

namespace base {

class Environment;

// Helper class to override |variable_name| environment variable to |value| for
// the lifetime of this class. Upon destruction, the previous value is restored.
class BASE_EXPORT ScopedEnvironmentVariableOverride final {
 public:
  ScopedEnvironmentVariableOverride(const std::string& variable_name,
                                    const std::string& value);
  // Unset the variable.
  explicit ScopedEnvironmentVariableOverride(const std::string& variable_name);
  ScopedEnvironmentVariableOverride(ScopedEnvironmentVariableOverride&&);
  ScopedEnvironmentVariableOverride& operator=(
      ScopedEnvironmentVariableOverride&&);
  ~ScopedEnvironmentVariableOverride();

  base::Environment* GetEnv() { return environment_.get(); }
  bool IsOverridden() { return overridden_; }

 private:
  ScopedEnvironmentVariableOverride(const std::string& variable_name,
                                    const std::string& value,
                                    bool unset_var);
  std::unique_ptr<Environment> environment_;
  std::string variable_name_;
  bool overridden_;
  std::optional<std::string> old_value_;
};

}  // namespace base

#endif  // BASE_SCOPED_ENVIRONMENT_VARIABLE_OVERRIDE_H_
