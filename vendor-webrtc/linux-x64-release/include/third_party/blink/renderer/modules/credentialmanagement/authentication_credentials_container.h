// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATION_CREDENTIALS_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATION_CREDENTIALS_CONTAINER_H_

#include <optional>

#include "third_party/blink/public/mojom/webauthn/authenticator.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/credentialmanagement/credential_manager_type_converters.h"  // IWYU pragma: keep
#include "third_party/blink/renderer/modules/credentialmanagement/credentials_container.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Credential;
class CredentialCreationOptions;
class CredentialRequestOptions;
class IdentityCredentialRequestOptions;
class ExceptionState;
class Navigator;
class ScriptState;

DOMException* AuthenticatorStatusToDOMException(
    mojom::blink::AuthenticatorStatus status,
    const mojom::blink::WebAuthnDOMExceptionDetailsPtr& dom_exception_details);

class MODULES_EXPORT AuthenticationCredentialsContainer final
    : public CredentialsContainer,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];
  static CredentialsContainer* credentials(Navigator&);
  explicit AuthenticationCredentialsContainer(Navigator&);

  // CredentialsContainer:
  ScriptPromise<IDLNullable<Credential>> get(ScriptState*,
                                             const CredentialRequestOptions*,
                                             ExceptionState&) override;
  ScriptPromise<Credential> store(ScriptState*,
                                  Credential*,
                                  ExceptionState& exception_state) override;
  ScriptPromise<IDLNullable<Credential>> create(
      ScriptState*,
      const CredentialCreationOptions*,
      ExceptionState&) override;
  ScriptPromise<IDLUndefined> preventSilentAccess(ScriptState*) override;
  void Trace(Visitor*) const override;

 private:
  // get() implementation for FedCM and WebIdentityDigitalCredential.
  void GetForIdentity(ScriptState*,
                      ScriptPromiseResolver<IDLNullable<Credential>>*,
                      const CredentialRequestOptions&,
                      const IdentityCredentialRequestOptions&);

  class OtpRequestAbortAlgorithm;
  class PublicKeyRequestAbortAlgorithm;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATION_CREDENTIALS_CONTAINER_H_
