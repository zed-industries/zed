// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ML_NAVIGATOR_ML_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ML_NAVIGATOR_ML_H_

#include "third_party/blink/renderer/core/execution_context/navigator_base.h"
#include "third_party/blink/renderer/modules/ml/ml.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class NavigatorML : public GarbageCollected<NavigatorML>,
                    public Supplement<NavigatorBase> {
 public:
  static const char kSupplementName[];
  static ML* ml(NavigatorBase& navigator);
  explicit NavigatorML(NavigatorBase& navigator);

  NavigatorML(const NavigatorML&) = delete;
  NavigatorML& operator=(const NavigatorML&) = delete;

  void Trace(blink::Visitor*) const override;

 private:
  Member<ML> ml_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ML_NAVIGATOR_ML_H_
