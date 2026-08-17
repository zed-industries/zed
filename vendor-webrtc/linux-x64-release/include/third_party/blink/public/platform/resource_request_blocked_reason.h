// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_RESOURCE_REQUEST_BLOCKED_REASON_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_RESOURCE_REQUEST_BLOCKED_REASON_H_

namespace blink {
// If updating this enum, also update DevTools protocol usages.
// Contact devtools owners for help.
enum class ResourceRequestBlockedReason {
  kOther = 0,
  kCSP,
  kMixedContent,
  kOrigin,
  kInspector,
  kSubresourceFilter,
  kContentType,
  kCoepFrameResourceNeedsCoepHeader,
  kCoopSandboxedIFrameCannotNavigateToCoopPage,
  kCorpNotSameOrigin,
  kCorpNotSameOriginAfterDefaultedToSameOriginByCoep,
  kCorpNotSameOriginAfterDefaultedToSameOriginByDip,
  kCorpNotSameOriginAfterDefaultedToSameOriginByCoepAndDip,
  kCorpNotSameSite,
  kConversionRequest,
  kSRIMessageSignatureMismatch,
  kMax = kConversionRequest,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_RESOURCE_REQUEST_BLOCKED_REASON_H_
