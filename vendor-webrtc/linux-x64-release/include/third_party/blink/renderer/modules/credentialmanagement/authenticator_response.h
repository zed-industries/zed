// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_RESPONSE_H_

#include <variant>

#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class AuthenticatorAssertionResponseJSON;
class AuthenticatorAttestationResponseJSON;

class MODULES_EXPORT AuthenticatorResponse : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit AuthenticatorResponse(DOMArrayBuffer* client_data_json);
  ~AuthenticatorResponse() override;

  DOMArrayBuffer* clientDataJSON() const { return client_data_json_.Get(); }

  void Trace(Visitor*) const override;

  virtual std::variant<AuthenticatorAssertionResponseJSON*,
                       AuthenticatorAttestationResponseJSON*>
  toJSON() const;

 private:
  const Member<DOMArrayBuffer> client_data_json_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_AUTHENTICATOR_RESPONSE_H_
