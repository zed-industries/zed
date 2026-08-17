// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIAL_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_identity_credential_disconnect_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_identity_credential_logout_r_ps_request.h"
#include "third_party/blink/renderer/modules/credentialmanagement/credential.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"

namespace blink {

class ContentSecurityPolicy;

class MODULES_EXPORT IdentityCredential final : public Credential {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static IdentityCredential* Create(
      const String& token,
      bool is_auto_selected = false,
      const String& config_url = WTF::g_empty_string);

  static bool IsRejectingPromiseDueToCSP(ContentSecurityPolicy* policy,
                                         ScriptPromiseResolverBase* resolver,
                                         const KURL& provider_url);

  explicit IdentityCredential(const String& token,
                              bool is_auto_selected,
                              const String& config_url);

  // Credential:
  bool IsIdentityCredential() const override;

  // IdentityCredential.idl
  const String& token() const { return token_; }
  const bool& isAutoSelected() const { return is_auto_selected_; }
  const String& configURL() const { return config_url_; }

  static ScriptPromise<IDLUndefined> disconnect(
      ScriptState*,
      const IdentityCredentialDisconnectOptions* options,
      ExceptionState&);

 private:
  const String token_;
  const bool is_auto_selected_{false};
  const String config_url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_CREDENTIAL_H_
