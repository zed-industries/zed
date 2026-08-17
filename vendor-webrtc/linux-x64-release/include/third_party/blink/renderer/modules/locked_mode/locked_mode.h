// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_LOCKED_MODE_LOCKED_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_LOCKED_MODE_LOCKED_MODE_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class NavigatorBase;

class MODULES_EXPORT LockedMode final : public ScriptWrappable,
                                        public Supplement<NavigatorBase> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Web-exposed getter for `navigator.lockedMode`.
  static LockedMode* lockedMode(NavigatorBase&);

  explicit LockedMode(NavigatorBase&);
  ~LockedMode() override;

  // ScriptWrappable
  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_LOCKED_MODE_LOCKED_MODE_H_
