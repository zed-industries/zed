// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_PROVIDER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_identity_provider_config.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"

namespace blink {
class IdentityUserInfo;
class IdentityResolveOptions;
class V8UnionIdentityProviderTokenOrUSVString;

class MODULES_EXPORT IdentityProvider : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ScriptPromise<IDLSequence<IdentityUserInfo>> getUserInfo(
      ScriptState*,
      const blink::IdentityProviderConfig*,
      ExceptionState&);

  static void close(ScriptState*);
  static ScriptPromise<IDLBoolean> registerIdentityProvider(ScriptState*,
                                                            const String&);
  static ScriptPromise<IDLUndefined> unregisterIdentityProvider(ScriptState*,
                                                                const String&);
  static ScriptPromise<IDLUndefined> resolve(
      ScriptState*,
      const V8UnionIdentityProviderTokenOrUSVString* token,
      const IdentityResolveOptions* options = nullptr);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CREDENTIALMANAGEMENT_IDENTITY_PROVIDER_H_
