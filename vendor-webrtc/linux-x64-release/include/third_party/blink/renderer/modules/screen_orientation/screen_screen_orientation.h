// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_SCREEN_ORIENTATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_SCREEN_ORIENTATION_H_

#include "third_party/blink/renderer/core/frame/screen.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ScreenOrientation;
class Screen;

class ScreenScreenOrientation final
    : public GarbageCollected<ScreenScreenOrientation>,
      public Supplement<Screen> {
 public:
  static const char kSupplementName[];

  static ScreenScreenOrientation& From(Screen&);

  static ScreenOrientation* orientation(Screen&);

  explicit ScreenScreenOrientation(Screen& screen);

  void Trace(Visitor*) const override;

 private:
  Member<ScreenOrientation> orientation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_SCREEN_ORIENTATION_H_
