// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_NAVIGATOR_PREFERENCES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_NAVIGATOR_PREFERENCES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class PreferenceManager;

// Spec:
// https://wicg.github.io/web-preferences-api/#extensions-to-the-navigator-interface
class CORE_EXPORT NavigatorPreferences final
    : public GarbageCollected<NavigatorPreferences>,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];

  static PreferenceManager* preferences(Navigator& navigator);
  PreferenceManager* preferences();

  explicit NavigatorPreferences(Navigator&);

  void Trace(Visitor*) const override;

 private:
  static NavigatorPreferences& From(Navigator&);

  Member<PreferenceManager> preference_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PREFERENCES_NAVIGATOR_PREFERENCES_H_
