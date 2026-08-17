// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_GLOBAL_EVENT_HANDLERS_XR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_GLOBAL_EVENT_HANDLERS_XR_H_

#include "third_party/blink/renderer/modules/event_target_modules.h"

namespace blink {

// TODO(https://crbug.com/1109272): This should be changed to a partial
// interface mixin once that's supported by the IDL generator. Currently,
// GlobalEventHandlers is an interface mixin with five separate implementing
// interfaces.
class GlobalEventHandlersXR {
  STATIC_ONLY(GlobalEventHandlersXR);

 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(beforexrselect, kBeforexrselect)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_GLOBAL_EVENT_HANDLERS_XR_H_
