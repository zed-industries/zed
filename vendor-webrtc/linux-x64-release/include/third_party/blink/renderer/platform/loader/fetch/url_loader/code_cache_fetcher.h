// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CODE_CACHE_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CODE_CACHE_FETCHER_H_

#include "mojo/public/cpp/base/big_buffer.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/loader/fetch/code_cache_host.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace network {
struct ResourceRequest;
}  // namespace network

namespace blink {

class CodeCacheHost;

// Handles the fetching and validation of code cache entries.
class BLINK_PLATFORM_EXPORT CodeCacheFetcher
    : public WTF::RefCounted<CodeCacheFetcher> {
 public:
  CodeCacheFetcher() = default;
  CodeCacheFetcher(const CodeCacheFetcher&) = delete;
  CodeCacheFetcher& operator=(const CodeCacheFetcher&) = delete;

  // Creates a new fetcher for the given `request` and issues the async request
  // to fetch the associated code cache metadata.
  static scoped_refptr<CodeCacheFetcher> TryCreateAndStart(
      const network::ResourceRequest& request,
      CodeCacheHost* code_cache_host,
      scoped_refptr<base::SequencedTaskRunner> host_task_runner,
      base::OnceClosure done_closure);

  // Called when the request associated with the fetcher responds with code
  // cache metadata.
  virtual void DidReceiveCachedMetadataFromUrlLoader() = 0;

  // Called during resource loading to take the fetched code cache metadata, if
  // it exists.
  virtual std::optional<mojo_base::BigBuffer> TakeCodeCacheForResponse(
      const network::mojom::URLResponseHead& response_head) = 0;

  // Called when the request associated with the fetcher has been redirected to
  // `new_url`.
  virtual void OnReceivedRedirect(const KURL& new_url) = 0;

  // Indicates whether the fetcher has made a request to fetch the code cache
  // metadata and is waiting on the response.
  virtual bool IsWaiting() const = 0;

  // Starts the async request for the associated code cache, called after the
  // fetcher is constructed. Note this is not inlined into the constructor as
  // the async fetch task needs to take a ref-counted reference to this, and a
  // ref-counted reference cannot be taken until this has first been
  // constructed.
  virtual void Start() = 0;

 protected:
  friend class WTF::RefCounted<CodeCacheFetcher>;
  virtual ~CodeCacheFetcher() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_CODE_CACHE_FETCHER_H_
