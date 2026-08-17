// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_MEDIA_INTEGRITY_MEDIA_INTEGRITY_TOKEN_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_MEDIA_INTEGRITY_MEDIA_INTEGRITY_TOKEN_PROVIDER_H_

#include "third_party/blink/public/mojom/webview/webview_media_integrity.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/extensions/webview/extensions_webview_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class EXTENSIONS_WEBVIEW_EXPORT MediaIntegrityTokenProvider
    : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  MediaIntegrityTokenProvider(
      ExecutionContext* execution_context,
      mojo::PendingRemote<mojom::blink::WebViewMediaIntegrityProvider>
          provider_pending_remote,
      uint64_t cloud_project_number);

  uint64_t cloudProjectNumber() { return cloud_project_number_; }

  ScriptPromise<IDLString> requestToken(ScriptState* script_state,
                                        const String& opt_content_binding,
                                        ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  void OnProviderConnectionError();
  void OnRequestTokenResponse(
      ScriptState* script_state,
      ScriptPromiseResolver<IDLString>* resolver,
      mojom::blink::WebViewMediaIntegrityTokenResponsePtr response);

  HeapHashSet<Member<ScriptPromiseResolver<IDLString>>> token_resolvers_;
  HeapMojoRemote<mojom::blink::WebViewMediaIntegrityProvider> provider_remote_;
  const uint64_t cloud_project_number_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_MEDIA_INTEGRITY_MEDIA_INTEGRITY_TOKEN_PROVIDER_H_
