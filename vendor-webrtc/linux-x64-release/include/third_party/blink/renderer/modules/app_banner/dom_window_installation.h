// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_DOM_WINDOW_INSTALLATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_DOM_WINDOW_INSTALLATION_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/event_type_names.h"

namespace blink {

class DOMWindowInstallation {
 public:
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(appinstalled, kAppinstalled)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(beforeinstallprompt,
                                         kBeforeinstallprompt)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_APP_BANNER_DOM_WINDOW_INSTALLATION_H_
