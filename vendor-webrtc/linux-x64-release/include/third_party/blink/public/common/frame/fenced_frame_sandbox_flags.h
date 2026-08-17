// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FENCED_FRAME_SANDBOX_FLAGS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FENCED_FRAME_SANDBOX_FLAGS_H_

#include "services/network/public/cpp/web_sandbox_flags.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-shared.h"

namespace blink {

// In fenced frame trees, the features of the following flags are restricted.
constexpr network::mojom::WebSandboxFlags kFencedFrameForcedSandboxFlags =
    network::mojom::WebSandboxFlags::kDocumentDomain |
    network::mojom::WebSandboxFlags::kDownloads |
    network::mojom::WebSandboxFlags::kModals |
    network::mojom::WebSandboxFlags::kNavigation |
    network::mojom::WebSandboxFlags::kOrientationLock |
    network::mojom::WebSandboxFlags::kPlugins |
    network::mojom::WebSandboxFlags::kPointerLock |
    network::mojom::WebSandboxFlags::kPresentationController |
    network::mojom::WebSandboxFlags::kStorageAccessByUserActivation |
    network::mojom::WebSandboxFlags::kTopNavigation |
    network::mojom::WebSandboxFlags::kAllowSameSiteNoneCookies;

// In fenced frame trees, the features of the following flags are allowed.
// Sandboxed frames that do not allow these features can't load fenced frames.
constexpr network::mojom::WebSandboxFlags
    kFencedFrameMandatoryUnsandboxedFlags =
        network::mojom::WebSandboxFlags::kAutomaticFeatures |
        network::mojom::WebSandboxFlags::kForms |
        network::mojom::WebSandboxFlags::kOrigin |
        network::mojom::WebSandboxFlags::kPopups |
        network::mojom::WebSandboxFlags::
            kPropagatesToAuxiliaryBrowsingContexts |
        network::mojom::WebSandboxFlags::kScripts |
        network::mojom::WebSandboxFlags::kTopNavigationByUserActivation |
        network::mojom::WebSandboxFlags::kTopNavigationToCustomProtocols;

static_assert((kFencedFrameForcedSandboxFlags &
               kFencedFrameMandatoryUnsandboxedFlags) ==
              network::mojom::WebSandboxFlags::kNone);

static_assert(static_cast<uint32_t>(kFencedFrameForcedSandboxFlags |
                                    kFencedFrameMandatoryUnsandboxedFlags) ==
              (static_cast<uint32_t>(network::mojom::WebSandboxFlags::kMaxValue)
               << 1) -
                  1);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FRAME_FENCED_FRAME_SANDBOX_FLAGS_H_
