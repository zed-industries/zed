// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_HTTPS_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_HTTPS_STATE_H_

#include <optional>

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class SecurityOrigin;

// https://fetch.spec.whatwg.org/#concept-https-state-value
enum class HttpsState {
  kNone,
  // kDeprecated is not used.
  kModern
};

// According to the Fetch spec, HTTPS state is set during fetch, e.g. to
// modern for https: or to request's client's HTTPS state for data:.
// In the Blink implementation however, HTTPS state is calculated from
// response URL's SecurityOrigin and optional |parent_https_state| (that
// represents request's client's HTTPS state) to emulate this behavior.
// TODO(https://crbug.com/880986): Implement HTTPS state in more
// spec-conformant way.
PLATFORM_EXPORT HttpsState CalculateHttpsState(
    const SecurityOrigin*,
    std::optional<HttpsState> parent_https_state = std::nullopt);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_HTTPS_STATE_H_
