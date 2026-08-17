// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCHEDULING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCHEDULING_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class IsInputPendingOptions;
class Navigator;

// Low-level scheduling primitives for JS scheduler implementations.
class CORE_EXPORT Scheduling : public ScriptWrappable,
                               public Supplement<Navigator> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static Scheduling* scheduling(Navigator&);
  explicit Scheduling(Navigator&);

  bool isInputPending(const IsInputPendingOptions* options) const;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SCHEDULING_H_
