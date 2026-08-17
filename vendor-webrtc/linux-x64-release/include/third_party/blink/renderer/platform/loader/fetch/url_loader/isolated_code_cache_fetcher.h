// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_ISOLATED_CODE_CACHE_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_ISOLATED_CODE_CACHE_FETCHER_H_

#include "base/functional/callback_forward.h"
#include "base/functional/callback_helpers.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/code_cache_fetcher.h"

namespace blink {

class CodeCacheHost;

// Fetches code cache metadata from the isolated code cache.
class BLINK_PLATFORM_EXPORT IsolatedCodeCacheFetcher final
    : public CodeCacheFetcher {
 public:
  IsolatedCodeCacheFetcher(CodeCacheHost& code_cache_host,
                           mojom::blink::CodeCacheType code_cache_type,
                           const KURL& url,
                           base::OnceClosure done_closure);
  IsolatedCodeCacheFetcher(const IsolatedCodeCacheFetcher&) = delete;
  IsolatedCodeCacheFetcher& operator=(const IsolatedCodeCacheFetcher&) = delete;
  ~IsolatedCodeCacheFetcher() override;

  // CodeCacheFetcher:
  void DidReceiveCachedMetadataFromUrlLoader() override;
  std::optional<mojo_base::BigBuffer> TakeCodeCacheForResponse(
      const network::mojom::URLResponseHead& response_head) override;
  void OnReceivedRedirect(const KURL& new_url) override;
  bool IsWaiting() const override;
  void Start() override;

 private:
  void DidReceiveCachedCode(base::Time response_time,
                            mojo_base::BigBuffer data);
  void ClearCodeCacheEntryIfPresent();

  base::WeakPtr<CodeCacheHost> code_cache_host_;
  mojom::blink::CodeCacheType code_cache_type_;
  base::TimeTicks time_of_last_fetch_start_;

  // The initial URL used for code cache fetching, prior to redirects. This
  // should match the initial URL for script fetching.
  const KURL initial_url_;

  // The current URL used for code cache fetching. This should match / will be
  // updated to the current url for script fetching (either initial or
  // redirected).
  KURL current_url_;

  base::OnceClosure done_closure_;

  bool is_waiting_ = true;
  bool did_receive_cached_metadata_from_url_loader_ = false;
  std::optional<mojo_base::BigBuffer> code_cache_data_;
  base::Time code_cache_response_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_ISOLATED_CODE_CACHE_FETCHER_H_
