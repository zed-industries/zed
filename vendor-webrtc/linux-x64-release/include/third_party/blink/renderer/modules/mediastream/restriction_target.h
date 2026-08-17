// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_RESTRICTION_TARGET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_RESTRICTION_TARGET_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/modules/mediastream/sub_capture_target.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"

namespace blink {

class ScriptState;

class MODULES_EXPORT RestrictionTarget final : public SubCaptureTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ScriptPromise<RestrictionTarget> fromElement(
      ScriptState* script_state,
      Element* element,
      ExceptionState& exception_state);

  // Not Web-exposed.
  explicit RestrictionTarget(String id);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_RESTRICTION_TARGET_H_
