// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_ANDROID_H_
#define THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_ANDROID_H_

#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/extensions/webview/extensions_webview_export.h"
#include "third_party/blink/renderer/extensions/webview/web_view_android.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class EXTENSIONS_WEBVIEW_EXPORT Android : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Android();

  WebViewAndroid* webview(ExecutionContext*);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_ANDROID_H_
