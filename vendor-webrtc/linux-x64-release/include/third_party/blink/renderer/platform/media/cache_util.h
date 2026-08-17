// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CACHE_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CACHE_UTIL_H_

#include <stdint.h>

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class WebURLResponse;

// Reasons that a cached WebURLResponse will *not* prevent a future request to
// the server.  Reported via UMA, so don't change/reuse previously-existing
// values.
enum UncacheableReason {
  kNoData = 1 << 0,  // Not 200 or 206.
  kPre11PartialResponse = 1 << 1,  // 206 but HTTP version < 1.1.
  kNoStrongValidatorOnPartialResponse = 1 << 2,  // 206, no strong validator.
  kShortMaxAge = 1 << 3,  // Max age less than 1h (arbitrary value).
  kExpiresTooSoon = 1 << 4,  // Expires in less than 1h (arbitrary value).
  kHasMustRevalidate = 1 << 5,  // Response asks for revalidation.
  kNoCache = 1 << 6,  // Response included a no-cache header.
  kNoStore = 1 << 7,  // Response included a no-store header.
  kMaxReason  // Needs to be one more than max legitimate reason.
};

// Return the logical OR of the reasons "response" cannot be used for a future
// request (using the disk cache), or 0 if it might be useful.
PLATFORM_EXPORT uint32_t
GetReasonsForUncacheability(const WebURLResponse& response);

// Returns when we should evict data from this response from our
// memory cache. Note that we may still cache data longer if
// a audio/video tag is currently using it. Returns a TimeDelta
// which is should be added to base::Time::Now() or base::TimeTicks::Now().
PLATFORM_EXPORT base::TimeDelta GetCacheValidUntil(
    const WebURLResponse& response);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_CACHE_UTIL_H_
