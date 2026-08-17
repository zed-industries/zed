// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SUBRESOURCE_WEB_BUNDLE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SUBRESOURCE_WEB_BUNDLE_LIST_H_

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class KURL;
class SubresourceWebBundle;

// SubresourceWebBundleList is attached to ResourceFetcher and used to set
// WebBundleToken to subresource requests which should be served from a
// WebBundle. This is used for Subresource loading with Web Bundles
// (https://github.com/WICG/webpackage/blob/main/explainers/subresource-loading.md).
class PLATFORM_EXPORT SubresourceWebBundleList
    : public GarbageCollected<SubresourceWebBundleList> {
 public:
  void Trace(Visitor* visitor) const;

  void Add(SubresourceWebBundle& bundle);
  void Remove(SubresourceWebBundle& bundle);
  SubresourceWebBundle* GetMatchingBundle(const KURL& url) const;
  SubresourceWebBundle* FindSubresourceWebBundleWhichWillBeReleased(
      const KURL& bundle_url,
      network::mojom::CredentialsMode credentials_mode) const;

 private:
  HeapVector<Member<SubresourceWebBundle>> subresource_web_bundles_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_SUBRESOURCE_WEB_BUNDLE_LIST_H_
