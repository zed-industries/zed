// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class KURL;

class MODULES_EXPORT Credential : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~Credential() override;
  void Trace(Visitor*) const override;

  virtual bool IsPasswordCredential() const { return false; }
  virtual bool IsFederatedCredential() const { return false; }
  virtual bool IsDigitalCredential() const { return false; }
  virtual bool IsPublicKeyCredential() const { return false; }
  virtual bool IsOTPCredential() const { return false; }
  virtual bool IsIdentityCredential() const { return false; }

  // Credential.idl
  const String& id() const { return id_; }
  const String& type() const { return type_; }

 protected:
  Credential(const String& id, const String& type);

  // Parses a String into a KURL that is potentially empty or null. Throws an
  // exception via |exceptionState| if an invalid URL is produced.
  static KURL ParseStringAsURLOrThrow(const String&, ExceptionState&);

 private:
  String id_;
  String type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_CREDENTIAL_H_
