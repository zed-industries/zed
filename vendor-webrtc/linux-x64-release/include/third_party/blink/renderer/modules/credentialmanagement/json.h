// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_JSON_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_JSON_H_

#include "third_party/blink/renderer/core/typed_arrays/dom_array_piece.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AuthenticationExtensionsClientOutputsJSON;
class AuthenticationExtensionsClientOutputs;
class PublicKeyCredentialCreationOptions;
class PublicKeyCredentialCreationOptionsJSON;
class PublicKeyCredentialRequestOptions;
class PublicKeyCredentialRequestOptionsJSON;
class ScriptState;

// WebAuthn JSON-encodes binary-valued fields as Base64URL without trailing '='
// padding characters.
WTF::String WebAuthnBase64UrlEncode(DOMArrayPiece buffer);

AuthenticationExtensionsClientOutputsJSON*
AuthenticationExtensionsClientOutputsToJSON(
    ScriptState* script_state,
    const AuthenticationExtensionsClientOutputs& extension_outputs);

// Implements `PublicKeyCredential.parseCredentialCreationOptions()` from the
// WebAuthn API.
PublicKeyCredentialCreationOptions* PublicKeyCredentialCreationOptionsFromJSON(
    const PublicKeyCredentialCreationOptionsJSON* json,
    ExceptionState& exception_state);

// Implements `PublicKeyCredential.parseCredentialRequestOptions()` from the
// WebAuthn API.
PublicKeyCredentialRequestOptions* PublicKeyCredentialRequestOptionsFromJSON(
    const PublicKeyCredentialRequestOptionsJSON* json,
    ExceptionState& exception_state);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_JSON_H_
