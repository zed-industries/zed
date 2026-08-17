// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_DATA_H_

#include "third_party/blink/public/mojom/notifications/notification.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace WTF {
class String;
}

namespace blink {

class ExceptionState;
class ExecutionContext;
class NotificationOptions;

// Creates a mojom::blink::NotificationData object based on the
// developer-provided notification options. An exception will be thrown on the
// ExceptionState when the given options do not match the constraints imposed by
// the specification.
MODULES_EXPORT mojom::blink::NotificationDataPtr CreateNotificationData(
    ExecutionContext* context,
    const String& title,
    const NotificationOptions* options,
    ExceptionState& exception_state);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NOTIFICATIONS_NOTIFICATION_DATA_H_
