// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUS_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUS_PARAMS_H_

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_focus_options.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Focus changes that cross a fenced frame boundary are observable by both
// frames involved in the focus change. Because of that, focus calls that
// originate from a JavaScript call can be used as a communication channel
// between a fenced frame and its embedder. For those focus calls, we gate focus
// on user activation to ensure that a user has recently interacted with a frame
// before allowing focus to happen. By default, we assume focus can be used as a
// communication channel. However, if a focus call can only be triggered
// directly through user interaction and can't be triggered via script, it is
// safe to not gate the focus call on user activation.
enum class FocusTrigger {
  // e.g. by calling `window.focus()`
  kScript,
  // e.g. by the user using tab to advance focus
  kUserGesture,
};

struct FocusParams {
  STACK_ALLOCATED();

 public:
  FocusParams() : options(FocusOptions::Create()) {}
  explicit FocusParams(FocusTrigger focus_trigger)
      : options(FocusOptions::Create()), focus_trigger(focus_trigger) {}
  FocusParams(SelectionBehaviorOnFocus selection,
              mojom::blink::FocusType focus_type,
              InputDeviceCapabilities* capabilities,
              const FocusOptions* focus_options = FocusOptions::Create(),
              FocusTrigger focus_trigger = FocusTrigger::kScript)
      : selection_behavior(selection),
        type(focus_type),
        source_capabilities(capabilities),
        options(focus_options),
        focus_trigger(focus_trigger) {}

  SelectionBehaviorOnFocus selection_behavior =
      SelectionBehaviorOnFocus::kRestore;
  mojom::blink::FocusType type = mojom::blink::FocusType::kNone;
  InputDeviceCapabilities* source_capabilities = nullptr;
  const FocusOptions* options = nullptr;
  bool omit_blur_events = false;
  FocusTrigger focus_trigger = FocusTrigger::kScript;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FOCUS_PARAMS_H_
