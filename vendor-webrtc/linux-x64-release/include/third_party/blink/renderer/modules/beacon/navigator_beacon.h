// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BEACON_NAVIGATOR_BEACON_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BEACON_NAVIGATOR_BEACON_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ScriptState;
class ExceptionState;
class KURL;

class NavigatorBeacon final : public GarbageCollected<NavigatorBeacon>,
                              public Supplement<Navigator> {
 public:
  static const char kSupplementName[];

  static NavigatorBeacon& From(Navigator&);

  explicit NavigatorBeacon(Navigator&);
  ~NavigatorBeacon();

  static bool sendBeacon(
      ScriptState* script_state,
      Navigator& navigator,
      const String& url_string,
      const V8UnionReadableStreamOrXMLHttpRequestBodyInit* data,
      ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  bool SendBeaconImpl(ScriptState* script_state,
                      const String& url_string,
                      const V8UnionReadableStreamOrXMLHttpRequestBodyInit* data,
                      ExceptionState& exception_state);
  bool CanSendBeacon(ExecutionContext*, const KURL&, ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BEACON_NAVIGATOR_BEACON_H_
