// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_UTILS_H_

#include "third_party/blink/public/mojom/push_messaging/push_messaging.mojom-blink.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

namespace mojom {
enum class PushErrorType;
enum class PushRegistrationStatus;
}  // namespace mojom

WTF::String PushRegistrationStatusToString(
    mojom::PushRegistrationStatus status);

mojom::PushErrorType PushRegistrationStatusToPushErrorType(
    mojom::PushRegistrationStatus status);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_UTILS_H_
