// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SMS_WEBOTP_CONSTANTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SMS_WEBOTP_CONSTANTS_H_

#include "base/time/time.h"

namespace blink {

static constexpr int kMaxUniqueOriginInAncestorChainForWebOTP = 2;
// This is Blink.Sms.Receive.TimeSuccess at > 99.7 percentile.
static constexpr base::TimeDelta kWebOTPRequestTimeout = base::Minutes(4);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SMS_WEBOTP_CONSTANTS_H_
