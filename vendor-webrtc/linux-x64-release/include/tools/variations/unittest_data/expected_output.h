// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// GENERATED FROM THE SCHEMA DEFINITION AND DESCRIPTION IN
//   field_trial_testing_config_schema.json
//   test_config.json
// DO NOT EDIT.

#ifndef TEST_OUTPUT_H_
#define TEST_OUTPUT_H_

#include <cstddef>

#include <optional>

#include "base/containers/span.h"
#include "components/variations/proto/study.pb.h"

struct OverrideUIString {
  const int name_hash;
  const char* const value;
};

struct FieldTrialTestingExperimentParams {
  const char* const key;
  const char* const value;
};

struct FieldTrialTestingExperiment {
  const char* const name;
  const base::span<const Study::Platform> platforms;
  const base::span<const Study::FormFactor> form_factors;
  const std::optional<bool> is_low_end_device;
  const std::optional<bool> disable_benchmarking;
  const char* const min_os_version;
  const base::span<const FieldTrialTestingExperimentParams> params;
  const base::span<const char* const> enable_features;
  const base::span<const char* const> disable_features;
  const char* const forcing_flag;
  const base::span<const OverrideUIString> override_ui_string;
  const base::span<const char* const> hardware_classes;
  const base::span<const char* const> exclude_hardware_classes;
};

struct FieldTrialTestingStudy {
  const char* const name;
  const base::span<const FieldTrialTestingExperiment> experiments;
};

struct FieldTrialTestingConfig {
  const base::span<const FieldTrialTestingStudy> studies;
};


extern const FieldTrialTestingConfig kFieldTrialConfig;

#endif  // TEST_OUTPUT_H_
