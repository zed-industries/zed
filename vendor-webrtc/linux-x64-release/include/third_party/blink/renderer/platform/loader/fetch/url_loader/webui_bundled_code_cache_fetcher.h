// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_WEBUI_BUNDLED_CODE_CACHE_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_WEBUI_BUNDLED_CODE_CACHE_FETCHER_H_

#include "base/functional/callback_forward.h"
#include "base/functional/callback_helpers.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/code_cache_fetcher.h"

namespace blink {

// Fetches webui code cache metadata from the application's resource bundle. See
// crbug.com/378504631 for design details.
class BLINK_PLATFORM_EXPORT WebUIBundledCodeCacheFetcher final
    : public CodeCacheFetcher {
 public:
  WebUIBundledCodeCacheFetcher(
      scoped_refptr<base::SequencedTaskRunner> host_task_runner,
      int resource_id,
      base::OnceClosure done_closure);
  WebUIBundledCodeCacheFetcher(const WebUIBundledCodeCacheFetcher&) = delete;
  WebUIBundledCodeCacheFetcher& operator=(const WebUIBundledCodeCacheFetcher&) =
      delete;
  ~WebUIBundledCodeCacheFetcher() override;

  // CodeCacheFetcher:
  void DidReceiveCachedMetadataFromUrlLoader() override;
  std::optional<mojo_base::BigBuffer> TakeCodeCacheForResponse(
      const network::mojom::URLResponseHead& response_head) override;
  void OnReceivedRedirect(const KURL& new_url) override;
  bool IsWaiting() const override;
  void Start() override;

  // Called on the `host_task_runner_` after the code cache data has been read
  // from the resource bundle on the worker thread.
  void DidReceiveCachedCode(mojo_base::BigBuffer data);

 private:
  // The host task runner of the fetcher's owner.
  scoped_refptr<base::SequencedTaskRunner> host_task_runner_;

  // The resource id used for WebUI code cache fetching. Note that redirects are
  // not supported and CHECK-ed in `OnReceivedRedirect()`.
  const int resource_id_;

  // Called once the metadata request has finished and `code_cache_data_` is
  // set.
  base::OnceClosure done_closure_;

  // Tracks whether the fetcher has made a request to fetch the code cache
  // metadata and is waiting on the response.
  bool is_waiting_ = true;

  // Stores the metadata read from the resource bundle.
  std::optional<mojo_base::BigBuffer> code_cache_data_;

  base::WeakPtrFactory<WebUIBundledCodeCacheFetcher> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_WEBUI_BUNDLED_CODE_CACHE_FETCHER_H_
