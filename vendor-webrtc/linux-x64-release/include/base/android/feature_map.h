// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_FEATURE_MAP_H_
#define BASE_ANDROID_FEATURE_MAP_H_

#include <string>

#include "base/base_export.h"
#include "base/containers/flat_map.h"
#include "base/containers/span.h"
#include "base/feature_list.h"
#include "base/memory/raw_ptr.h"

namespace base::android {

// A FeatureMap is a mapping from base:Feature's names to a pointer to the
// base:Feature.
// This is necessary because in Java, features flags are identified by the
// feature name, a string, so calls from Java to (for example) check the state
// of a feature flag need to convert the string to a non-owning Feature*.
// Each component should have its own FeatureMap.
class BASE_EXPORT FeatureMap {
 public:
  explicit FeatureMap(
      base::span<const Feature* const> features_exposed_to_java);
  ~FeatureMap();

  // Map a |feature_name| to a Feature*.
  const Feature* FindFeatureExposedToJava(const std::string& feature_name);

 private:
  flat_map<std::string_view, raw_ptr<const Feature, CtnExperimental>> mapping_;
};

}  // namespace base::android

#endif  // BASE_ANDROID_FEATURE_MAP_H_
