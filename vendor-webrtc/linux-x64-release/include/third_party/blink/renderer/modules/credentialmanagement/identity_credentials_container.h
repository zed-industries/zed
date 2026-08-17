// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIALS_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIALS_CONTAINER_H_

#include "third_party/blink/renderer/modules/credentialmanagement/credentials_container.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {
class Credential;
class CredentialCreationOptions;
class CredentialRequestOptions;
class ExceptionState;
class Navigator;
class ScriptState;

class MODULES_EXPORT IdentityCredentialsContainer final
    : public CredentialsContainer,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];
  static CredentialsContainer* identity(Navigator&);
  explicit IdentityCredentialsContainer(Navigator&);

  // CredentialsContainer.idl
  ScriptPromise<IDLNullable<Credential>> get(ScriptState*,
                                             const CredentialRequestOptions*,
                                             ExceptionState&) override;
  ScriptPromise<Credential> store(ScriptState*,
                                  Credential*,
                                  ExceptionState&) override;
  ScriptPromise<IDLNullable<Credential>> create(
      ScriptState*,
      const CredentialCreationOptions*,
      ExceptionState&) override;
  ScriptPromise<IDLUndefined> preventSilentAccess(ScriptState*) override;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIALS_CONTAINER_H_
