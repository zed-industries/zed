// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_UTIL_H_

#include "services/device/public/mojom/smart_card.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class AbortSignal;
class ScriptPromiseResolverBase;
class V8SmartCardAccessMode;
class V8SmartCardProtocol;

device::mojom::blink::SmartCardShareMode ToMojoSmartCardShareMode(
    V8SmartCardAccessMode access_mode);

device::mojom::blink::SmartCardProtocolsPtr ToMojoSmartCardProtocols(
    const Vector<V8SmartCardProtocol>& preferred_protocols);

void RejectWithAbortionReason(ScriptPromiseResolverBase* resolver,
                              AbortSignal* signal);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SMART_CARD_SMART_CARD_UTIL_H_
