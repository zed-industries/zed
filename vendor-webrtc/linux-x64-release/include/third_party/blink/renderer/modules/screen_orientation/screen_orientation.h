// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_ORIENTATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_ORIENTATION_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_orientation_type.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/display/mojom/screen_orientation.mojom-blink.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class LocalDOMWindow;
class ScriptState;
class ScreenOrientationController;
class V8OrientationLockType;

class MODULES_EXPORT ScreenOrientation final : public EventTarget,
                                               public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ScreenOrientation* Create(LocalDOMWindow*);

  explicit ScreenOrientation(LocalDOMWindow*);
  ~ScreenOrientation() override;

  // EventTarget implementation.
  const WTF::AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  V8OrientationType type() const;
  uint16_t angle() const;

  void SetType(display::mojom::blink::ScreenOrientation);
  void SetAngle(uint16_t);

  ScriptPromise<IDLUndefined> lock(ScriptState*,
                                   const V8OrientationLockType& orientation,
                                   ExceptionState&);
  void unlock();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  // Helper being used by this class and LockOrientationCallback.
  static V8OrientationType::Enum OrientationTypeToV8Enum(
      display::mojom::blink::ScreenOrientation);

  void Trace(Visitor*) const override;

 private:
  ScreenOrientationController* Controller();

  display::mojom::blink::ScreenOrientation type_;
  uint16_t angle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_ORIENTATION_SCREEN_ORIENTATION_H_
