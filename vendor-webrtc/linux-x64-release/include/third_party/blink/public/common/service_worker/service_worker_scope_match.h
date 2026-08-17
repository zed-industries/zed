// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_SCOPE_MATCH_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_SCOPE_MATCH_H_

#include "third_party/blink/public/common/common_export.h"
#include "url/gurl.h"

namespace blink {

// Returns true if `scope` or `script_url` contains a disallowed character.
bool BLINK_COMMON_EXPORT
ServiceWorkerScopeOrScriptUrlContainsDisallowedCharacter(
    const GURL& scope,
    const GURL& script_url,
    std::string* error_message);

// Returns true if `scope` matches `url`.
bool BLINK_COMMON_EXPORT ServiceWorkerScopeMatches(const GURL& scope,
                                                   const GURL& url);

// A helper class for finding the longest matching scope.
class BLINK_COMMON_EXPORT ServiceWorkerLongestScopeMatcher {
 public:
  explicit ServiceWorkerLongestScopeMatcher(const GURL& url);
  ~ServiceWorkerLongestScopeMatcher();

  ServiceWorkerLongestScopeMatcher(const ServiceWorkerLongestScopeMatcher&) =
      delete;
  ServiceWorkerLongestScopeMatcher& operator=(
      const ServiceWorkerLongestScopeMatcher&) = delete;

  // Returns true if `scope` matches `url_` longer than `match_`.
  bool MatchLongest(const GURL& scope);

 private:
  const GURL url_;
  GURL match_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_SCOPE_MATCH_H_
