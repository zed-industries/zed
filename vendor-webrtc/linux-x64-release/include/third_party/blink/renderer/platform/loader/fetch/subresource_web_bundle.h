// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SUBRESOURCE_WEB_BUNDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SUBRESOURCE_WEB_BUNDLE_H_

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class UnguessableToken;
}

namespace blink {

class KURL;

// SubresourceWebBundle is used for Subresource loading with Web Bundles.
// (https://github.com/WICG/webpackage/blob/main/explainers/subresource-loading.md).
class PLATFORM_EXPORT SubresourceWebBundle : public GarbageCollectedMixin {
 public:
  void Trace(Visitor* visitor) const override {}
  virtual bool CanHandleRequest(const KURL& url) const = 0;
  virtual const KURL& GetBundleUrl() const = 0;
  virtual const base::UnguessableToken& WebBundleToken() const = 0;
  virtual String GetCacheIdentifier() const = 0;
  virtual void OnWebBundleError(const String& message) const = 0;
  virtual void NotifyLoadingFinished() = 0;
  virtual bool IsScriptWebBundle() const = 0;
  virtual bool WillBeReleased() const = 0;
  virtual network::mojom::CredentialsMode GetCredentialsMode() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SUBRESOURCE_WEB_BUNDLE_H_
