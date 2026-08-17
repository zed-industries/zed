// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_WEB_VIEW_ANDROID_H_
#define THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_WEB_VIEW_ANDROID_H_

#include "third_party/blink/public/mojom/webview/webview_media_integrity.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/extensions_webview/v8/v8_get_media_integrity_token_provider_params.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/extensions/webview/extensions_webview_export.h"
#include "third_party/blink/renderer/extensions/webview/media_integrity/media_integrity_token_provider.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class EXTENSIONS_WEBVIEW_EXPORT WebViewAndroid
    : public ScriptWrappable,
      public Supplement<ExecutionContext>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  static WebViewAndroid& From(ExecutionContext&);

  explicit WebViewAndroid(ExecutionContext&);

  ScriptPromise<MediaIntegrityTokenProvider>
  getExperimentalMediaIntegrityTokenProvider(
      ScriptState* script_state,
      GetMediaIntegrityTokenProviderParams* params,
      ExceptionState& exception_state);

  void Trace(Visitor*) const override;

 private:
  void EnsureServiceConnection(ExecutionContext* execution_context);
  void OnServiceConnectionError();
  void OnGetIntegrityProviderResponse(
      ScriptState* script_state,
      mojo::PendingRemote<mojom::blink::WebViewMediaIntegrityProvider>
          provider_pending_remote,
      uint64_t cloud_project_number,
      ScriptPromiseResolver<MediaIntegrityTokenProvider>* resolver,
      std::optional<mojom::blink::WebViewMediaIntegrityErrorCode> error);

  HeapHashSet<Member<ScriptPromiseResolver<MediaIntegrityTokenProvider>>>
      provider_resolvers_;
  HeapMojoRemote<mojom::blink::WebViewMediaIntegrityService>
      media_integrity_service_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_WEB_VIEW_ANDROID_H_
