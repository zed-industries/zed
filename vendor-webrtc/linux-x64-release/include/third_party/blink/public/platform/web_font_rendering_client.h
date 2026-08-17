// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_RENDERING_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_RENDERING_CLIENT_H_

#include "third_party/blink/public/platform/web_font_prewarmer.h"

namespace blink {

class ThreadSafeBrowserInterfaceBrokerProxy;

// The interface accessing `DWriteFontCollectionProxy` functions from Blink.
class WebFontRenderingClient : public WebFontPrewarmer {
 public:
  // Bind `DWriteFontCollectionProxy` to browser for calling thread.
  virtual void BindFontProxyUsingBroker(
      ThreadSafeBrowserInterfaceBrokerProxy* interface_broker) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_RENDERING_CLIENT_H_
