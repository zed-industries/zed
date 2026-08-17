// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FEATURE_VISITOR_H_
#define BASE_FEATURE_VISITOR_H_

#include <map>
#include <string>

#include "base/feature_list.h"

namespace variations::cros_early_boot::evaluate_seed {
class EarlyBootFeatureVisitor;
}

namespace gin {
class V8FeatureVisitor;
}

namespace base {

class TestFeatureVisitor;

// An interface for FeatureList that provides a method to iterate over a
// feature's name, override state, parameters, and associated field trial.
//
// NOTE: This is intended only for the special case of needing to get all
// feature overrides. Most users should call FeatureList::IsEnabled() to query
// a feature's state.
class FeatureVisitor {
 public:
  FeatureVisitor(const FeatureVisitor&) = delete;
  FeatureVisitor& operator=(const FeatureVisitor&) = delete;

  virtual ~FeatureVisitor() = default;

  // Intended to be called in FeatureList::VisitFeaturesAndParams(). This method
  // is called once per feature.
  virtual void Visit(const std::string& feature_name,
                     FeatureList::OverrideState override_state,
                     const std::map<std::string, std::string>& params,
                     const std::string& trial_name,
                     const std::string& group_name) = 0;

 private:
  friend variations::cros_early_boot::evaluate_seed::EarlyBootFeatureVisitor;
  friend gin::V8FeatureVisitor;
  friend TestFeatureVisitor;

  // The constructor is private so only friend classes can inherit from this
  // class. This limits access to who can iterate over features in
  // FeatureList::VisitFeaturesAndParams().
  FeatureVisitor() = default;
};

}  // namespace base

#endif  // BASE_FEATURE_VISITOR_H_
