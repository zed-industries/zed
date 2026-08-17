// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_RESOURCE_LOAD_INFO_NOTIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_RESOURCE_LOAD_INFO_NOTIFIER_H_

#include "build/build_config.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info_notifier.mojom.h"

namespace blink {

// A fake implementation of blink::mojom::ResourceLoadInfoNotifier.
class FakeResourceLoadInfoNotifier final
    : public blink::mojom::ResourceLoadInfoNotifier {
 public:
  FakeResourceLoadInfoNotifier();
  ~FakeResourceLoadInfoNotifier() override;

  FakeResourceLoadInfoNotifier(const FakeResourceLoadInfoNotifier&) = delete;
  FakeResourceLoadInfoNotifier& operator=(const FakeResourceLoadInfoNotifier&) =
      delete;

  // blink::mojom::ResourceLoadInfoNotifier overrides.
#if BUILDFLAG(IS_ANDROID)
  void NotifyUpdateUserGestureCarryoverInfo() override {}
#endif
  void NotifyResourceRedirectReceived(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr redirect_response) override {}
  void NotifyResourceResponseReceived(
      int64_t request_id,
      const url::SchemeHostPort& final_url,
      network::mojom::URLResponseHeadPtr head,
      network::mojom::RequestDestination request_destination,
      bool is_ad_resource) override {}
  void NotifyResourceTransferSizeUpdated(int64_t request_id,
                                         int32_t transfer_size_diff) override {}
  void NotifyResourceLoadCompleted(
      blink::mojom::ResourceLoadInfoPtr resource_load_info,
      const ::network::URLLoaderCompletionStatus& status) override;
  void NotifyResourceLoadCanceled(int64_t request_id) override {}
  void Clone(mojo::PendingReceiver<blink::mojom::ResourceLoadInfoNotifier>
                 pending_resource_load_info_notifier) override {}

  std::string GetMimeType();

 private:
  blink::mojom::ResourceLoadInfoPtr resource_load_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_RESOURCE_LOAD_INFO_NOTIFIER_H_
