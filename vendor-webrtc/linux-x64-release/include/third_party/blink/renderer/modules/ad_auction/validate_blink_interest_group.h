// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_VALIDATE_BLINK_INTEREST_GROUP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_VALIDATE_BLINK_INTEREST_GROUP_H_

#include "third_party/blink/public/mojom/interest_group/interest_group_types.mojom-blink-forward.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Checks that the specified mojom::blink::InterestGroup is valid. Invalid
// interest groups contain auction or update URLs cross-origin to the owner, or
// URLs that contain disallowed components (e.g., user/pass). When it returns
// false, writes information about the error to `error_field_name`,
// `error_field_value`, and `error`.
//
// Checks all provided URLs. Does no validation of expiration time. Does no
// validation of values expected to be in JSON, since
// blink::InterestGroup::IsValid() does not validate JSON. Must be kept in sync
// with blink::InterestGroup::IsValid(), which performs the exact same logic,
// except on blink::InterestGroups.
MODULES_EXPORT bool ValidateBlinkInterestGroup(
    const mojom::blink::InterestGroup& group,
    String& error_field_name,
    String& error_field_value,
    String& error);

// Additional checks required only at join and update time. Specifically, this
// validates that the selectedBuyerAndSellerReportingIds field on each ad object
// does not exceed a Finch-controlled "soft limit".
//
// Note that this function is only called at join time. The reference to
// update and the separation from ValidateBlinkInterestGroup above (which is
// also called only at join time) are useful only in maintaining parity with
// blink::InterestGroup::IsValidForJoinAndUpdate(), with which this function
// must remain in sync. blink::InterestGroup::IsValidForJoinAndUpdate() performs
// the exact same logic, except on blink::InterestGroups.
MODULES_EXPORT bool PerformAdditionalJoinAndUpdateTimeValidations(
    const mojom::blink::InterestGroup& group,
    String& error_field_name,
    String& error_field_value,
    String& error);

MODULES_EXPORT size_t
EstimateBlinkInterestGroupSize(const mojom::blink::InterestGroup& group);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AD_AUCTION_VALIDATE_BLINK_INTEREST_GROUP_H_
