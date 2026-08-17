// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_DIGITAL_CREDENTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_DIGITAL_CREDENTIAL_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/credentialmanagement/credential.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class MODULES_EXPORT DigitalCredential final : public Credential {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static DigitalCredential* Create(const String& protocol, ScriptObject data);

  explicit DigitalCredential(const String& protocol, ScriptObject data);

  // Credential:
  bool IsDigitalCredential() const override;
  void Trace(Visitor* visitor) const override;

  // DigitalCredential.idl
  const String& protocol() const { return protocol_; }
  const ScriptObject& data() const { return data_; }

 private:
  const String protocol_;
  ScriptObject data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_DIGITAL_CREDENTIAL_H_
