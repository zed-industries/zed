// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_UTIL_H_

#include <stdint.h>

#include "base/gtest_prod_util.h"
#include "third_party/blink/public/mojom/frame/deferred_fetch_policy.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_priority.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {
class FetchRequestData;
class Frame;
class FrameOwner;
class CalculateRequestSizeTestBase;
class CountDescendantsWithReservedMinimalQuotaTest;

// The ResourceType of FetchLater requests.
inline constexpr ResourceType kFetchLaterResourceType = ResourceType::kRaw;

// The minimal quota is 8 kibibytes, one of the possible values for
// "reserved deferred-fetch quota".
// https://whatpr.org/fetch/1647.html#reserved-deferred-fetch-quota-minimal-quota
inline constexpr uint32_t kMinimalReservedDeferredFetchQuota = 8 * 1024;

// The normal quota is 64 kibibytes, one of the possible values for
// "reserved deferred-fetch quota".
// https://whatpr.org/fetch/1647.html#reserved-deferred-fetch-quota-normal-quota
inline constexpr uint32_t kNormalReservedDeferredFetchQuota = 64 * 1024;

// The quota reserved for deferred-fetch-minimal is 128 kibibytes.
// https://whatpr.org/fetch/1647.html#quota-reserved-for-deferred-fetch-minimal
inline constexpr uint32_t kQuotaReservedForDeferredFetchMinimal = 128 * 1024;

// 64 kibibytes.
inline constexpr uint32_t kMaxPerRequestOriginScheduledDeferredBytes =
    64 * 1024;

// 640 kibibytes.
inline constexpr uint32_t kMaxScheduledDeferredBytes = 640 * 1024;

// Tells whether the FetchLater API should use subframe deferred fetch
// policy to decide whether a frame show allow using the API.
// Related discussions:
// https://github.com/WICG/pending-beacon/issues/87#issuecomment-2315624105
bool CORE_EXPORT IsFetchLaterUseDeferredFetchPolicyEnabled();

// Computes resource loader priority for a FetchLater request.
ResourceLoadPriority CORE_EXPORT
ComputeFetchLaterLoadPriority(const FetchParameters& params);

class CORE_EXPORT FetchLaterUtil {
 public:
  // Determines the deferred fetch policy of a navigable container
  // `container_frame`, e.g. iframe, when it begins to navigate its content to a
  // target URL `to_url` by the following algorithm:
  // https://whatpr.org/fetch/1647.html#reserve-deferred-fetch-quota
  //
  // `container_frame` is a FrameOwner, i.e. an iframe, on navigation.
  // `to_url` is the target URL the iframe is navigating to.
  //
  // Returns an enum that can be mapped to a "reserved deferred-fetch quota" by
  // calling `ToReservedDeferredFetchQuota()`.
  //
  // This must be called during "Beginning navigation" as described in
  // https://whatpr.org/html/10903/d1c086a...0e0afb3/browsing-the-web.html#beginning-navigation
  static mojom::blink::DeferredFetchPolicy
  GetContainerDeferredFetchPolicyOnNavigation(FrameOwner* container_frame,
                                              const KURL& to_url);

  // Returns the "deferred-fetch control document" for the given `frame`.
  // https://whatpr.org/fetch/1647.html#deferred-fetch-control-document
  //
  // `frame` is the node navigable of a Document to query for.
  // The returned is also a node navigable of the control document for `frame`.
  static Frame* GetDeferredFetchControlFrame(Frame* frame);

  // Tells if `frame` should clear its deferred-fetch policy.
  // https://whatpr.org/fetch/1647.html#potentially-free-deferred-fetch-quota
  //
  // Note that policy is stored in `frame`'s container if exists.
  //
  // This must be called during "document creation" flow as described in
  // https://whatpr.org/html/10903/d1c086a...0e0afb3/document-lifecycle.html
  static bool ShouldClearDeferredFetchPolicy(Frame* frame);

  // Performs the calculation of "available deferred fetch quota" algorithm
  // that requires a document and a request URL.
  //
  // `frame` is the source frame of the request `url`.
  // `url` is the request URL to calculate the deferred fetch quota for.
  //
  // https://whatpr.org/fetch/1647.html#available-deferred-fetch-quota
  static uint64_t GetAvailableDeferredFetchQuota(Frame* frame, const KURL& url);

  // Calculates the total size of a given request using
  // https://whatpr.org/fetch/1647.html#total-request-length
  static uint64_t CalculateRequestSize(const FetchRequestData& request);

 private:
  friend class CalculateRequestSizeTestBase;
  friend class CountDescendantsWithReservedMinimalQuotaTest;
  FRIEND_TEST_ALL_PREFIXES(AreSameOriginTest,
                           MultipleDifferentOriginSiblingFrames);
  FRIEND_TEST_ALL_PREFIXES(AreSameOriginTest, MultipleLevelFrames);

  FRIEND_TEST_ALL_PREFIXES(CountDescendantsWithReservedMinimalQuotaTest,
                           SingleCrossOriginFrame);
  FRIEND_TEST_ALL_PREFIXES(CountDescendantsWithReservedMinimalQuotaTest,
                           MultipleDifferentOriginSiblingFrames);
  FRIEND_TEST_ALL_PREFIXES(CountDescendantsWithReservedMinimalQuotaTest,
                           MultipleLevelFrames);

  FRIEND_TEST_ALL_PREFIXES(ToReservedDeferredFetchQuotaTest, PolicyDisabled);
  FRIEND_TEST_ALL_PREFIXES(ToReservedDeferredFetchQuotaTest,
                           PolicyDeferredFetch);
  FRIEND_TEST_ALL_PREFIXES(ToReservedDeferredFetchQuotaTest,
                           PolicyDeferredFetchMinimal);
  FRIEND_TEST_ALL_PREFIXES(GetReservedDeferredFetchQuotaTest,
                           IsTopLevelControlFrame);
  FRIEND_TEST_ALL_PREFIXES(
      GetReservedDeferredFetchQuotaTest,
      IsNonTopLevelFrameWithOwnerDeferredFetchPolicyDisabled);
  FRIEND_TEST_ALL_PREFIXES(
      GetReservedDeferredFetchQuotaTest,
      IsNonTopLevelCrossOriginFrameWithOwnerDeferredFetchPolicyMinimal);

  // Returns the length of `url` without any fragment parts.
  static uint64_t GetUrlLengthWithoutFragment(const KURL& url);

  // Converts `policy` to one of possible values of reserved deferred-fetch
  // quota.
  // https://whatpr.org/fetch/1647.html#reserved-deferred-fetch-quota
  static uint32_t ToReservedDeferredFetchQuota(
      mojom::blink::DeferredFetchPolicy policy);

  // Tells if the given two frames shares the same origin.
  // https://html.spec.whatwg.org/multipage/browsers.html#same-origin
  static bool AreSameOrigin(const Frame* frame_a, const Frame* frame_b);

  // Calculates the total number of descendant frames according to Step 4 of
  // https://whatpr.org/fetch/1647.html#reserve-deferred-fetch-quota
  //
  // `control_frame` is a frame to count the result for. It should be
  // obtained by calling `GetDeferredFetchControlFrame()`. Note that it must
  // be an iframe with `PermissionsPolicyFeature::kDeferredFetchMinimal` enabled
  // before calling this function.
  //
  // Example (with default Permissions Policy on every origin):
  //    root (a.com) -> frame-1 (a.com)
  //                 -> frame-2 (a.com)
  //                 -> frame-3 (b.com)
  //                 -> frame-4 (b.com)
  // * `control_frame` cannot be frame-1, frame-2.
  // * When `control_frame` = frame-3 or frame-4, the result is 2.
  static uint32_t CountDescendantsWithReservedMinimalQuota(
      const Frame* control_frame);

  // Returns corresponding reserved deferred-fetch quota on a control frame
  // `frame`.
  //
  // This method implements Step 3~6 of the following algorithm:
  // https://whatpr.org/fetch/1647.html#available-deferred-fetch-quota
  static uint32_t GetReservedDeferredFetchQuota(const Frame* frame);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_UTIL_H_
