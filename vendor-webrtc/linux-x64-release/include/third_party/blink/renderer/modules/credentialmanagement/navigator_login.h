// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_NAVIGATOR_LOGIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_NAVIGATOR_LOGIN_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Navigator;
class V8LoginStatus;
class LoginStatusOptions;

// Methods to let websites tell the browser about their login status.
class MODULES_EXPORT NavigatorLogin : public ScriptWrappable,
                                      public Supplement<Navigator> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static NavigatorLogin* login(Navigator&);
  explicit NavigatorLogin(Navigator&);

  ScriptPromise<IDLUndefined> setStatus(ScriptState* script_state,
                                        const V8LoginStatus& status);

  ScriptPromise<IDLUndefined> setStatus(ScriptState* script_state,
                                        const V8LoginStatus& status,
                                        const LoginStatusOptions* options);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_NAVIGATOR_LOGIN_H_
