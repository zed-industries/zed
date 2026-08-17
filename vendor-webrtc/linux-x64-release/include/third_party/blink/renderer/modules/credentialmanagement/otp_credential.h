// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_OTP_CREDENTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_OTP_CREDENTIAL_H_

#include "third_party/blink/renderer/modules/credentialmanagement/credential.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// OtpCredentials represents credentials for when the otp is requested
// through the credential manager. It stores the one-time-passcode (otp) that
// is retrieved from SMS by the browser.
class MODULES_EXPORT OTPCredential final : public Credential {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit OTPCredential(const String& code);

  // Credential:
  bool IsOTPCredential() const override;

  // OTPCredential.idl
  const String& code() const { return code_; }

 private:
  String code_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_OTP_CREDENTIAL_H_
