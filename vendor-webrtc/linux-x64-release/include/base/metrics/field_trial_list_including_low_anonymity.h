// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_FIELD_TRIAL_LIST_INCLUDING_LOW_ANONYMITY_H_
#define BASE_METRICS_FIELD_TRIAL_LIST_INCLUDING_LOW_ANONYMITY_H_

#include "base/gtest_prod_util.h"
#include "base/metrics/field_trial.h"
#include "base/values.h"

class AndroidFieldTrialListLogActiveTrialsFriendHelper;

namespace content {
class FieldTrialSynchronizer;
}

namespace variations {
class ChildProcessFieldTrialSyncer;
class EntropyProviders;
class ProcessedStudy;
struct SeedSimulationResult;
class VariationsCrashKeys;
class VariationsLayers;
SeedSimulationResult ComputeDifferences(
    const std::vector<ProcessedStudy>& processed_studies,
    const VariationsLayers& layers,
    const EntropyProviders& entropy_providers);
}  // namespace variations

namespace version_ui {
base::Value::List GetVariationsList();
}

namespace base {

// Provides a way to restrict access to the full set of field trials, including
// trials with low anonymity, to explicitly allowed callers.
//
// See |FieldTrialList::FactoryGetFieldTrial()| for background.
class BASE_EXPORT FieldTrialListIncludingLowAnonymity {
 public:
  // Exposed publicly, to avoid test code needing to be explicitly friended.
  static void GetActiveFieldTrialGroupsForTesting(
      FieldTrial::ActiveGroups* active_groups) {
    return GetActiveFieldTrialGroups(active_groups);
  }

  // Classes / functions which are allowed full access to all field trials
  // should be listed as friends here, with a comment explaining why this does
  // not risk revealing identifiable information externally.

  // This is used only for local logging on Android.
  friend class ::AndroidFieldTrialListLogActiveTrialsFriendHelper;

  // Used to synchronize field trial status between the browser and child
  // processes.
  // Access to these trials within each of these is then allowed only to the
  // other friend classes / methods listed here.
  friend class content::FieldTrialSynchronizer;
  friend class variations::ChildProcessFieldTrialSyncer;

  // This is only used to simulate seed changes, not sent to Google servers.
  friend variations::SeedSimulationResult variations::ComputeDifferences(
      const std::vector<variations::ProcessedStudy>& processed_studies,
      const variations::VariationsLayers& layers,
      const variations::EntropyProviders& entropy_providers);

  // Include all active field trials in crash reports, so that crashes are
  // reproducible: https://www.google.com/intl/en/chrome/privacy/.
  friend class variations::VariationsCrashKeys;

  // This usage is to display field trials in chrome://version and other local
  // internal UIs.
  friend base::Value::List version_ui::GetVariationsList();

  // Required for tests.
  friend class TestFieldTrialObserverIncludingLowAnonymity;
  FRIEND_TEST_ALL_PREFIXES(FieldTrialTest, ObserveIncludingLowAnonymity);

 private:
  // The same as |FieldTrialList::GetActiveFieldTrialGroups| but gives access to
  // low anonymity field trials too.
  static void GetActiveFieldTrialGroups(
      FieldTrial::ActiveGroups* active_groups);

  // Identical to |FieldTrialList::AddObserver| but also notifies of low
  // anonymity trials.
  static bool AddObserver(FieldTrialList::Observer* observer);

  // Identical to |FieldTrialList::RemoveObserver| but for observers registered
  // through the AddObserver() function of this class.
  static void RemoveObserver(FieldTrialList::Observer* observer);
};

}  // namespace base

#endif  // BASE_METRICS_FIELD_TRIAL_LIST_INCLUDING_LOW_ANONYMITY_H_
