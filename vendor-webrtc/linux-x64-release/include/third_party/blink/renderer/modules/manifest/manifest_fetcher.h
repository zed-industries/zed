// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_FETCHER_H_

#include <memory>

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/loader/threadable_loader.h"
#include "third_party/blink/renderer/core/loader/threadable_loader_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_response.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class KURL;
class LocalDOMWindow;
class TextResourceDecoder;

// Helper class to download a Web Manifest. When an instance is created, the
// caller need to call Start() and wait for the passed callback to be executed.
// If the fetch fails, the callback will be called with two empty objects.
class ManifestFetcher final : public GarbageCollected<ManifestFetcher>,
                              public ThreadableLoaderClient {
  // This will be called asynchronously after the URL has been fetched,
  // successfully or not.  If there is a failure, response and data will both be
  // empty.  |response| and |data| are both valid until the ManifestFetcher
  // instance is destroyed.
  using Callback =
      base::OnceCallback<void(const ResourceResponse&, const String&)>;

 public:
  explicit ManifestFetcher(const KURL& url);

  ManifestFetcher(const ManifestFetcher&) = delete;
  ManifestFetcher& operator=(const ManifestFetcher&) = delete;

  ~ManifestFetcher() override;

  void Start(LocalDOMWindow& window,
             bool use_credentials,
             ResourceFetcher* resource_fetcher,
             ManifestFetcher::Callback callback);
  void Cancel();

  // ThreadableLoaderClient
  void DidReceiveResponse(uint64_t, const ResourceResponse&) override;
  void DidReceiveData(base::span<const char>) override;
  void DidFinishLoading(uint64_t) override;
  void DidFail(uint64_t, const ResourceError&) override;
  void DidFailRedirectCheck(uint64_t) override;

  void Trace(Visitor* visitor) const override;

 private:
  KURL url_;
  bool completed_;
  ManifestFetcher::Callback callback_;
  ResourceResponse response_;
  std::unique_ptr<TextResourceDecoder> decoder_;
  StringBuilder data_;
  Member<ThreadableLoader> loader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_FETCHER_H_
