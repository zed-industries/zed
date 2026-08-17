//
// Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS.  All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
//

#ifndef SYSTEM_WRAPPERS_INCLUDE_FIELD_TRIAL_H_
#define SYSTEM_WRAPPERS_INCLUDE_FIELD_TRIAL_H_

#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/containers/flat_set.h"

// Field trials allow webrtc clients (such as Chrome) to turn on feature code
// in binaries out in the field and gather information with that.
//
// By default WebRTC provides an implementation of field trials that can be
// found in system_wrappers/source/field_trial.cc. If clients want to provide
// a custom version, they will have to:
//
// 1. Compile WebRTC defining the preprocessor macro
//    WEBRTC_EXCLUDE_FIELD_TRIAL_DEFAULT (if GN is used this can be achieved
//    by setting the GN arg rtc_exclude_field_trial_default to true).
// 2. Provide an implementation of:
//    std::string webrtc::field_trial::FindFullName(absl::string_view trial).
//
// They are designed to wire up directly to chrome field trials and to speed up
// developers by reducing the need to wire APIs to control whether a feature is
// on/off. E.g. to experiment with a new method that could lead to a different
// trade-off between CPU/bandwidth:
//
// 1 - Develop the feature with default behaviour off:
//
//   if (FieldTrial::FindFullName("WebRTCExperimentMethod2") == "Enabled")
//     method2();
//   else
//     method1();
//
// 2 - Once the changes are rolled to chrome, the new code path can be
//     controlled as normal chrome field trials.
//
// 3 - Evaluate the new feature and clean the code paths.
//
// Notes:
//   - NOT every feature is a candidate to be controlled by this mechanism as
//     it may require negotiation between involved parties (e.g. SDP).
//
// TODO(andresp): since chrome --force-fieldtrials does not marks the trial
//     as active it does not get propagated to the renderer process. For now one
//     needs to push a config with start_active:true or run a local finch
//     server.
//
// TODO(andresp): find out how to get bots to run tests with trials enabled.

namespace webrtc {
namespace field_trial {

// Returns the group name chosen for the named trial, or the empty string
// if the trial does not exists.
//
// Note: To keep things tidy append all the trial names with WebRTC.
std::string FindFullName(absl::string_view name);

// Convenience method, returns true iff FindFullName(name) return a string that
// starts with "Enabled".
// TODO(tommi): Make sure all implementations support this.
inline bool IsEnabled(absl::string_view name) {
  return FindFullName(name).find("Enabled") == 0;
}

// Convenience method, returns true iff FindFullName(name) return a string that
// starts with "Disabled".
inline bool IsDisabled(absl::string_view name) {
  return FindFullName(name).find("Disabled") == 0;
}

// Optionally initialize field trial from a string.
// This method can be called at most once before any other call into webrtc.
// E.g. before the peer connection factory is constructed.
// Note: trials_string must never be destroyed.
void InitFieldTrialsFromString(const char* trials_string);

const char* GetFieldTrialString();

// Validates the given field trial string.
bool FieldTrialsStringIsValid(absl::string_view trials_string);

// Merges two field trial strings.
//
// If a key (trial) exists twice with conflicting values (groups), the value
// in 'second' takes precedence.
// Shall only be called with valid FieldTrial strings.
std::string MergeFieldTrialsStrings(absl::string_view first,
                                    absl::string_view second);

// This helper allows to temporary "register" a field trial within the current
// scope. This is only useful for tests that use the global field trial string,
// otherwise you can use `webrtc::FieldTrialsRegistry`.
//
// If you want to isolate changes to the global field trial string itself within
// the current scope you should use `webrtc::test::ScopedFieldTrials`.
class FieldTrialsAllowedInScopeForTesting {
 public:
  explicit FieldTrialsAllowedInScopeForTesting(flat_set<std::string> keys);
  ~FieldTrialsAllowedInScopeForTesting();
};

}  // namespace field_trial
}  // namespace webrtc

#endif  // SYSTEM_WRAPPERS_INCLUDE_FIELD_TRIAL_H_
