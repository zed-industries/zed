// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_ASSERTION_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_ASSERTION_RESPONSE_H_

#include <stdint.h>

#include <optional>
#include <variant>

#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/credentialmanagement/authenticator_response.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class MODULES_EXPORT AuthenticatorAssertionResponse final
    : public AuthenticatorResponse {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // It is recommended to use std::move() for the Vector parameters into this
  // constructor to avoid copying potentially large chunks of memory. Note this
  // constructor will std::move() the Vector from `optional_user_handle`, if
  // present.
  AuthenticatorAssertionResponse(
      const Vector<uint8_t> client_data_json,
      const Vector<uint8_t> authenticator_data,
      const Vector<uint8_t> signature,
      std::optional<Vector<uint8_t>> optional_user_handle);

  AuthenticatorAssertionResponse(DOMArrayBuffer* client_data_json,
                                 DOMArrayBuffer* authenticator_data,
                                 DOMArrayBuffer* signature,
                                 DOMArrayBuffer* user_handle);
  ~AuthenticatorAssertionResponse() override;

  DOMArrayBuffer* authenticatorData() const {
    return authenticator_data_.Get();
  }

  DOMArrayBuffer* signature() const { return signature_.Get(); }

  DOMArrayBuffer* userHandle() const { return user_handle_.Get(); }

  std::variant<AuthenticatorAssertionResponseJSON*,
               AuthenticatorAttestationResponseJSON*>
  toJSON() const override;

  void Trace(Visitor*) const override;

 private:
  const Member<DOMArrayBuffer> authenticator_data_;
  const Member<DOMArrayBuffer> signature_;
  const Member<DOMArrayBuffer> user_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_ASSERTION_RESPONSE_H_
