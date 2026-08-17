// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_PUBLIC_KEY_CREDENTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_PUBLIC_KEY_CREDENTIAL_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_authentication_extensions_client_outputs.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/credentialmanagement/authenticator_response.h"
#include "third_party/blink/renderer/modules/credentialmanagement/credential.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-local-handle.h"
#include "v8/include/v8-value.h"

namespace blink {

namespace mojom {
enum class AuthenticatorAttachment;
}

class AllAcceptedCredentialsOptions;
class AuthenticatorResponse;
class CurrentUserDetailsOptions;
class PublicKeyCredentialCreationOptions;
class PublicKeyCredentialCreationOptionsJSON;
class PublicKeyCredentialRequestOptions;
class PublicKeyCredentialRequestOptionsJSON;
class ScriptState;
class UnknownCredentialOptions;

class MODULES_EXPORT PublicKeyCredential : public Credential {
  DEFINE_WRAPPERTYPEINFO();

 public:
  PublicKeyCredential(
      const String& id,
      DOMArrayBuffer* raw_id,
      AuthenticatorResponse*,
      mojom::AuthenticatorAttachment authenticator_attachment,
      const AuthenticationExtensionsClientOutputs* extension_outputs,
      const String& type = g_empty_string);

  DOMArrayBuffer* rawId() const { return raw_id_.Get(); }

  AuthenticatorResponse* response() const { return response_.Get(); }

  const String& authenticatorAttachment() const {
    return authenticator_attachment_;
  }

  static ScriptPromise<IDLBoolean>
  isUserVerifyingPlatformAuthenticatorAvailable(ScriptState*);

  static ScriptPromise<IDLRecord<IDLString, IDLBoolean>> getClientCapabilities(
      ScriptState*);

  AuthenticationExtensionsClientOutputs* getClientExtensionResults() const;

  static ScriptPromise<IDLBoolean> isConditionalMediationAvailable(
      ScriptState*);

  static const PublicKeyCredentialCreationOptions* parseCreationOptionsFromJSON(
      ScriptState*,
      const PublicKeyCredentialCreationOptionsJSON*,
      ExceptionState&);

  static const PublicKeyCredentialRequestOptions* parseRequestOptionsFromJSON(
      ScriptState*,
      const PublicKeyCredentialRequestOptionsJSON*,
      ExceptionState&);

  v8::Local<v8::Object> toJSON(ScriptState*) const;

  static ScriptPromise<IDLUndefined> signalUnknownCredential(
      ScriptState*,
      const UnknownCredentialOptions*,
      ExceptionState&);

  static ScriptPromise<IDLUndefined> signalAllAcceptedCredentials(
      ScriptState*,
      const AllAcceptedCredentialsOptions*,
      ExceptionState&);

  static ScriptPromise<IDLUndefined> signalCurrentUserDetails(
      ScriptState*,
      const CurrentUserDetailsOptions*,
      ExceptionState&);

  // Credential:
  void Trace(Visitor*) const override;
  bool IsPublicKeyCredential() const override;

 private:
  const Member<DOMArrayBuffer> raw_id_;
  const Member<AuthenticatorResponse> response_;
  const String authenticator_attachment_;
  Member<const AuthenticationExtensionsClientOutputs> extension_outputs_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_PUBLIC_KEY_CREDENTIAL_H_
