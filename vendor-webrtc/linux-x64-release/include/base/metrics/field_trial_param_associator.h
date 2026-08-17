// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_FIELD_TRIAL_PARAM_ASSOCIATOR_H_
#define BASE_METRICS_FIELD_TRIAL_PARAM_ASSOCIATOR_H_

#include <functional>
#include <map>
#include <string>
#include <utility>

#include "base/base_export.h"
#include "base/memory/singleton.h"
#include "base/metrics/field_trial.h"
#include "base/metrics/field_trial_params.h"
#include "base/synchronization/lock.h"
#include "base/types/pass_key.h"

class AppShimController;

namespace base {

// Keeps track of the parameters of all field trials and ensures access to them
// is thread-safe.
class BASE_EXPORT FieldTrialParamAssociator {
 public:
  FieldTrialParamAssociator();

  FieldTrialParamAssociator(const FieldTrialParamAssociator&) = delete;
  FieldTrialParamAssociator& operator=(const FieldTrialParamAssociator&) =
      delete;

  ~FieldTrialParamAssociator();

  // Retrieve the singleton.
  static FieldTrialParamAssociator* GetInstance();

  // Sets parameters for the given field trial name and group.
  bool AssociateFieldTrialParams(const std::string& trial_name,
                                 const std::string& group_name,
                                 const FieldTrialParams& params);

  // Gets the parameters for a field trial and its chosen group. If not found in
  // field_trial_params_, then tries to looks it up in shared memory. Returns
  // false if no params are available or the passed |field_trial| is null.
  bool GetFieldTrialParams(FieldTrial* field_trial, FieldTrialParams* params);

  // Gets the parameters for a field trial and its chosen group. Does not
  // fallback to looking it up in shared memory. This should only be used if you
  // know for sure the params are in the mapping, like if you're in the browser
  // process, and even then you should probably just use GetFieldTrialParams().
  bool GetFieldTrialParamsWithoutFallback(const std::string& trial_name,
                                          const std::string& group_name,
                                          FieldTrialParams* params);

  // Clears the internal field_trial_params_ mapping, plus removes all params in
  // shared memory.
  void ClearAllParamsForTesting();

  // Clears a single field trial param.
  // Note: this does NOT remove the param in shared memory.
  void ClearParamsForTesting(const std::string& trial_name,
                             const std::string& group_name);

  // Clears the internal field_trial_params_ mapping.
  void ClearAllCachedParamsForTesting();

  // Clears the internal field_trial_params_ mapping for use by
  // AppShimController when switching over from initial "early access" field
  // trial information to the real long-term field trial information.
  void ClearAllCachedParams(PassKey<AppShimController>);

 private:
  friend struct DefaultSingletonTraits<FieldTrialParamAssociator>;

  // (field_trial_name, field_trial_group)
  using FieldTrialKey = std::pair<std::string, std::string>;
  // The following type can be used for lookups without needing to copy strings.
  using FieldTrialRefKey = std::pair<const std::string&, const std::string&>;

  Lock lock_;
  std::map<FieldTrialKey, FieldTrialParams, std::less<>> field_trial_params_;
};

}  // namespace base

#endif  // BASE_METRICS_FIELD_TRIAL_PARAM_ASSOCIATOR_H_
