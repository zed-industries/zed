// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_METRICS_ACTION_SUFFIX_READER_H_
#define BASE_TEST_METRICS_ACTION_SUFFIX_READER_H_

#include <map>
#include <string>
#include <variant>
#include <vector>

namespace base {

using ActionSuffixEntryMap = std::map<std::string, std::string>;

// Find and read the suffixes list(s) which apply to the given `affected_action`
// in actions.xml.
//
// Useful for when you want to verify that the set of suffixes associated with
// a particular action contains expected values. For example,
// BrowserUserEducationServiceTest.CheckFeaturePromoActions verifies that for
// every registered Chrome Desktop in-product-help experience, there is a
// corresponding suffix for recording UserEducation.MessageAction* actions. This
// prevents someone from adding an IPH experience without adding the
// corresponding action entry.
//
// Returns a list of maps, each of which corresponds to one list of suffixes
// associated with the action (an `<affected-action>` could theoretically show
// up in more than one `<action-suffix>` block.)
//
// If no suffix list is found, returns an empty list. May generate test errors
// on malformed/duplicate entries even if valid suffixes are found.
extern std::vector<ActionSuffixEntryMap> ReadActionSuffixesForAction(
    const std::string& affected_action);

}  // namespace base

#endif  // BASE_TEST_METRICS_ACTION_SUFFIX_READER_H_
