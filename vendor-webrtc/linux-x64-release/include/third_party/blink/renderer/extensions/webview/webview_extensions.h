// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_WEBVIEW_EXTENSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_WEBVIEW_EXTENSIONS_H_

#include "third_party/blink/renderer/extensions/webview/extensions_webview_export.h"

namespace blink {

class EXTENSIONS_WEBVIEW_EXPORT WebViewExtensions {
 public:
  // Should be called during Blink initialization.
  static void Initialize();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_EXTENSIONS_WEBVIEW_WEBVIEW_EXTENSIONS_H_
