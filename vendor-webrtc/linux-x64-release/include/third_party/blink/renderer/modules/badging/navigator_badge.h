// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BADGING_NAVIGATOR_BADGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BADGING_NAVIGATOR_BADGE_H_

#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/badging/badging.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class Navigator;
class WorkerNavigator;

class NavigatorBadge final : public GarbageCollected<NavigatorBadge>,
                             public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  static NavigatorBadge& From(ScriptState*);
  explicit NavigatorBadge(ExecutionContext*);

  // Badge IDL interface.
  static ScriptPromise<IDLUndefined> setAppBadge(ScriptState*,
                                                 Navigator&,
                                                 ExceptionState&);
  static ScriptPromise<IDLUndefined> setAppBadge(ScriptState*,
                                                 WorkerNavigator&,
                                                 ExceptionState&);

  static ScriptPromise<IDLUndefined> setAppBadge(ScriptState*,
                                                 Navigator&,
                                                 uint64_t content,
                                                 ExceptionState&);
  static ScriptPromise<IDLUndefined> setAppBadge(ScriptState*,
                                                 WorkerNavigator&,
                                                 uint64_t content,
                                                 ExceptionState&);

  static ScriptPromise<IDLUndefined> clearAppBadge(ScriptState*,
                                                   Navigator&,
                                                   ExceptionState&);
  static ScriptPromise<IDLUndefined> clearAppBadge(ScriptState*,
                                                   WorkerNavigator&,
                                                   ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  static ScriptPromise<IDLUndefined> SetAppBadgeHelper(
      ScriptState* script_state,
      mojom::blink::BadgeValuePtr badge_value,
      ExceptionState& exception_state);
  static ScriptPromise<IDLUndefined> ClearAppBadgeHelper(
      ScriptState* script_state,
      ExceptionState& exception_state);
  // Returns true if using the Badging API is allowed in this context.
  static bool IsAllowed(ScriptState* script_state);

  mojo::Remote<mojom::blink::BadgeService> badge_service();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BADGING_NAVIGATOR_BADGE_H_
