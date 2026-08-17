// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_DEPRECATION_LABEL_COOKIE_DEPRECATION_LABEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_DEPRECATION_LABEL_COOKIE_DEPRECATION_LABEL_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Navigator;
class ScriptState;

class CookieDeprecationLabel : public ScriptWrappable,
                               public Supplement<Navigator> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  // Web exposed as navigator.cookieDeprecationLabel
  static CookieDeprecationLabel* cookieDeprecationLabel(Navigator& navigator);

  explicit CookieDeprecationLabel(Navigator& navigator);
  ~CookieDeprecationLabel() override;

  // Web exposed function defined in the IDL file.
  ScriptPromise<IDLString> getValue(ScriptState* script_state);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COOKIE_DEPRECATION_LABEL_COOKIE_DEPRECATION_LABEL_H_
