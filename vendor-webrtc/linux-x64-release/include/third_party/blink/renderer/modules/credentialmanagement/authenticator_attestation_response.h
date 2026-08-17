// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_ATTESTATION_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_ATTESTATION_RESPONSE_H_

#include <variant>

#include "third_party/blink/public/mojom/webauthn/authenticator.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/credentialmanagement/authenticator_response.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class MODULES_EXPORT AuthenticatorAttestationResponse final
    : public AuthenticatorResponse {
  DEFINE_WRAPPERTYPEINFO();

 public:
  AuthenticatorAttestationResponse(
      DOMArrayBuffer* client_data_json,
      DOMArrayBuffer* attestation_object,
      Vector<mojom::AuthenticatorTransport> transports,
      DOMArrayBuffer* authenticator_data,
      DOMArrayBuffer* public_key_der,
      int32_t public_key_algorithm);
  ~AuthenticatorAttestationResponse() override;

  DOMArrayBuffer* attestationObject() const {
    return attestation_object_.Get();
  }

  DOMArrayBuffer* getAuthenticatorData() const {
    return authenticator_data_.Get();
  }

  DOMArrayBuffer* getPublicKey() const { return public_key_der_.Get(); }

  int32_t getPublicKeyAlgorithm() const { return public_key_algo_; }

  Vector<String> getTransports() const;

  std::variant<AuthenticatorAssertionResponseJSON*,
               AuthenticatorAttestationResponseJSON*>
  toJSON() const override;

  void Trace(Visitor*) const override;

 private:
  const Member<DOMArrayBuffer> attestation_object_;
  const Vector<mojom::AuthenticatorTransport> transports_;
  const Member<DOMArrayBuffer> authenticator_data_;
  const Member<DOMArrayBuffer> public_key_der_;
  const int32_t public_key_algo_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_ATTESTATION_RESPONSE_H_
