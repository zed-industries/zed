// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CLIENT_HINTS_CLIENT_HINTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CLIENT_HINTS_CLIENT_HINTS_H_

#include <stddef.h>

#include <optional>
#include <set>
#include <string>

#include "base/containers/flat_map.h"
#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom.h"
#include "services/network/public/mojom/web_client_hints_types.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"

class GURL;

namespace network {
class PermissionsPolicy;
}  // namespace network

namespace blink {

// Indicates that a hint is sent by default, regardless of an opt-in.
BLINK_COMMON_EXPORT
bool IsClientHintSentByDefault(network::mojom::WebClientHintsType type);

// Add a list of Client Hints headers to be removed to the output vector, based
// on Permissions Policy and the url's origin.
BLINK_COMMON_EXPORT void FindClientHintsToRemove(
    const network::PermissionsPolicy* permissions_policy,
    const GURL& url,
    std::vector<std::string>* removed_headers);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CLIENT_HINTS_CLIENT_HINTS_H_
