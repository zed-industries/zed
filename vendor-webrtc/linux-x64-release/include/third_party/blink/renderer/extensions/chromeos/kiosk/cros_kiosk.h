// Copyright 2024 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_CHROMEOS_KIOSK_CROS_KIOSK_H_
#define THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_CHROMEOS_KIOSK_CROS_KIOSK_H_

#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class CrosKiosk : public ScriptWrappable, public Supplement<ExecutionContext> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  static CrosKiosk& From(ExecutionContext&);

  explicit CrosKiosk(ExecutionContext&);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_CHROMEOS_KIOSK_CROS_KIOSK_H_
