// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_TESTING_INTERNALS_FED_CM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_TESTING_INTERNALS_FED_CM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ExceptionState;
class Internals;
class ScriptState;
class V8DialogButton;

class InternalsFedCm {
  STATIC_ONLY(InternalsFedCm);

 public:
  static ScriptPromise<IDLString> getFedCmDialogType(ScriptState*, Internals&);
  static ScriptPromise<IDLString> getFedCmTitle(ScriptState*, Internals&);
  static ScriptPromise<IDLUndefined> selectFedCmAccount(ScriptState*,
                                                        Internals&,
                                                        int account_index,
                                                        ExceptionState&);
  static ScriptPromise<IDLUndefined> dismissFedCmDialog(ScriptState*,
                                                        Internals&);
  static ScriptPromise<IDLUndefined> clickFedCmDialogButton(
      ScriptState*,
      Internals&,
      const V8DialogButton& button);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_TESTING_INTERNALS_FED_CM_H_
