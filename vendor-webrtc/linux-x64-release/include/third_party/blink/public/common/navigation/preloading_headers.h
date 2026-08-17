// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_PRELOADING_HEADERS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_PRELOADING_HEADERS_H_

namespace blink {

// The header used to indicate the purpose of a request. For more info see
// https://wicg.github.io/nav-speculation/prefetch.html#sec-purpose-header
inline constexpr char kSecPurposeHeaderName[] = "Sec-Purpose";

// This value indicates that the request is a prefetch request made directly to
// the server.
inline constexpr char kSecPurposePrefetchHeaderValue[] = "prefetch";

// This value indicates that the request is a prefetch request made via an
// anonymous client IP proxy.
inline constexpr char kSecPurposePrefetchAnonymousClientIpHeaderValue[] =
    "prefetch;anonymous-client-ip";

// This value indicates that the request is a prerender request.
inline constexpr char kSecPurposePrefetchPrerenderHeaderValue[] =
    "prefetch;prerender";

// This value indicates that the request is a preview request.
inline constexpr char kSecPurposePrefetchPrerenderPreviewHeaderValue[] =
    "prefetch;prerender;preview";

// The Chromium specific header equivalent for 'Sec-Purpose':
inline constexpr char kPurposeHeaderName[] = "Purpose";

// For more info see
// https://wicg.github.io/nav-speculation/prefetch.html#sec-speculation-tags-header
inline constexpr char kSecSpeculationTagsHeaderName[] = "Sec-Speculation-Tags";

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_PRELOADING_HEADERS_H_
