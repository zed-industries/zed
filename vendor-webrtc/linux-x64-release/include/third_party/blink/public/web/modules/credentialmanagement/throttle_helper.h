// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_CREDENTIALMANAGEMENT_THROTTLE_HELPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_CREDENTIALMANAGEMENT_THROTTLE_HELPER_H_

#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"

namespace url {
class Origin;
}  // namespace url

namespace blink {

namespace mojom {
enum class IdpSigninStatus;
}  // namespace mojom

// Sets the identity provider (IDP) signin state of the given |origin| to
// |status|. This is meant for use with IdentityUrlLoaderThrottle.
// This method must be called on the main thread.
BLINK_MODULES_EXPORT void SetIdpSigninStatus(
    const blink::LocalFrameToken& local_frame_token,
    const url::Origin& origin,
    mojom::IdpSigninStatus status);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_CREDENTIALMANAGEMENT_THROTTLE_HELPER_H_
