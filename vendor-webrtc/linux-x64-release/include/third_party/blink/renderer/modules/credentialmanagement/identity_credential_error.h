// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIAL_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIAL_ERROR_H_

#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class IdentityCredentialErrorInit;

// https://fedidcg.github.io/FedCM/#browser-api-identity-credential-error-interface
class MODULES_EXPORT IdentityCredentialError : public DOMException {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor exposed to script. Called by the V8 bindings.
  static IdentityCredentialError* Create(
      const String& message,
      const IdentityCredentialErrorInit* options);

  IdentityCredentialError(const String& message,
                          const IdentityCredentialErrorInit* options);
  IdentityCredentialError(const String& message,
                          const String& code,
                          const String& url);

  String code() const { return code_; }
  String url() const { return url_; }

 private:
  const String code_;
  const String url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIAL_ERROR_H_
