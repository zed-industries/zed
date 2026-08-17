// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SHARED_STORAGE_MODULE_SCRIPT_DOWNLOADER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SHARED_STORAGE_MODULE_SCRIPT_DOWNLOADER_H_

#include "base/functional/callback.h"
#include "net/url_request/redirect_info.h"
#include "services/network/public/mojom/url_loader_factory.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom.h"
#include "third_party/blink/public/common/common_export.h"

namespace network {
class SimpleURLLoader;
}  // namespace network

namespace blink {

// Download utility for worklet module script. Creates requests and blocks
// responses.
class BLINK_COMMON_EXPORT ModuleScriptDownloader {
 public:
  // Passes in nullptr for `response_body` on failure. Always invoked
  // asynchronously.
  using ModuleScriptDownloaderCallback =
      base::OnceCallback<void(std::unique_ptr<std::string> response_body,
                              std::string error_message,
                              network::mojom::URLResponseHeadPtr)>;

  // Starts loading the worklet module script on construction. Callback will be
  // invoked asynchronously once the data has been fetched or an error has
  // occurred.
  ModuleScriptDownloader(
      network::mojom::URLLoaderFactory* url_loader_factory,
      const GURL& source_url,
      ModuleScriptDownloaderCallback module_script_downloader_callback);

  ModuleScriptDownloader(const ModuleScriptDownloader&) = delete;
  ModuleScriptDownloader& operator=(const ModuleScriptDownloader&) = delete;

  ~ModuleScriptDownloader();

 private:
  void OnBodyReceived(std::unique_ptr<std::string> body);

  void OnRedirect(const GURL& url_before_redirect,
                  const net::RedirectInfo& redirect_info,
                  const network::mojom::URLResponseHead& response_head,
                  std::vector<std::string>* removed_headers);

  const GURL source_url_;
  std::unique_ptr<network::SimpleURLLoader> simple_url_loader_;
  ModuleScriptDownloaderCallback module_script_downloader_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SHARED_STORAGE_MODULE_SCRIPT_DOWNLOADER_H_
