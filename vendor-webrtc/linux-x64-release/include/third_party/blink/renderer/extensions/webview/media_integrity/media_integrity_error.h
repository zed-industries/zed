// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_MEDIA_INTEGRITY_MEDIA_INTEGRITY_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_MEDIA_INTEGRITY_MEDIA_INTEGRITY_ERROR_H_

#include "third_party/blink/public/mojom/webview/webview_media_integrity.mojom-blink.h"
#include "third_party/blink/renderer/bindings/extensions_webview/v8/v8_media_integrity_error_name.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/extensions/webview/extensions_webview_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class MediaIntegrityErrorOptions;

class EXTENSIONS_WEBVIEW_EXPORT MediaIntegrityError : public DOMException {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Constructor exposed to script. Called by the V8 bindings.
  static MediaIntegrityError* Create(String message,
                                     const MediaIntegrityErrorOptions* options);

  // Constructor for C++ code, which attaches stack trace.
  static v8::Local<v8::Value> CreateForName(
      v8::Isolate* isolate,
      V8MediaIntegrityErrorName::Enum name);

  // Constructor for C++ code, which attaches stack trace.
  static v8::Local<v8::Value> CreateFromMojomEnum(
      v8::Isolate* isolate,
      mojom::blink::WebViewMediaIntegrityErrorCode error_code);

  // Use one of the Create() methods instead. This constructor has to be public
  // so that it can be used with MakeGarbageCollected<> inside the Create
  // methods.
  MediaIntegrityError(String message, V8MediaIntegrityErrorName name);
  ~MediaIntegrityError() override;

  // Web-facing implementation, returns an integer type.
  V8MediaIntegrityErrorName mediaIntegrityErrorName() const {
    return media_integrity_error_name_;
  }

 private:
  const V8MediaIntegrityErrorName media_integrity_error_name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_MEDIA_INTEGRITY_MEDIA_INTEGRITY_ERROR_H_
