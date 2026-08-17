// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIALS_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIALS_CONTAINER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class Credential;
class CredentialCreationOptions;
class CredentialRequestOptions;
class ExceptionState;
class ScriptState;

class MODULES_EXPORT CredentialsContainer : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~CredentialsContainer() override = default;

  // credentials_container.idl
  virtual ScriptPromise<IDLNullable<Credential>>
  get(ScriptState*, const CredentialRequestOptions*, ExceptionState&) = 0;
  virtual ScriptPromise<Credential> store(ScriptState*,
                                          Credential*,
                                          ExceptionState&) = 0;
  virtual ScriptPromise<IDLNullable<Credential>>
  create(ScriptState*, const CredentialCreationOptions*, ExceptionState&) = 0;
  virtual ScriptPromise<IDLUndefined> preventSilentAccess(ScriptState*) = 0;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIALS_CONTAINER_H_
