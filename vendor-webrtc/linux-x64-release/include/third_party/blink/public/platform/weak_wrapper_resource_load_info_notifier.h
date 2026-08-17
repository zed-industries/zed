// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEAK_WRAPPER_RESOURCE_LOAD_INFO_NOTIFIER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEAK_WRAPPER_RESOURCE_LOAD_INFO_NOTIFIER_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "build/build_config.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info_notifier.mojom.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// A ResourceLoadInfoNotifier implementation that wraps a raw
// blink::mojom::ResourceLoadInfoNotifier pointer.
class BLINK_PLATFORM_EXPORT WeakWrapperResourceLoadInfoNotifier
    : public blink::mojom::ResourceLoadInfoNotifier {
 public:
  explicit WeakWrapperResourceLoadInfoNotifier(
      blink::mojom::ResourceLoadInfoNotifier* resource_load_info_notifier);
  ~WeakWrapperResourceLoadInfoNotifier() override = default;

  // blink::mojom::ResourceLoadInfoNotifier overrides, these methods should be
  // called from the same thread.
#if BUILDFLAG(IS_ANDROID)
  void NotifyUpdateUserGestureCarryoverInfo() override;
#endif
  void NotifyResourceRedirectReceived(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr redirect_response) override;
  void NotifyResourceResponseReceived(
      int64_t request_id,
      const url::SchemeHostPort& final_response_url,
      network::mojom::URLResponseHeadPtr response_head,
      network::mojom::RequestDestination request_destination,
      bool is_ad_resource) override;
  void NotifyResourceTransferSizeUpdated(int64_t request_id,
                                         int32_t transfer_size_diff) override;
  void NotifyResourceLoadCompleted(
      blink::mojom::ResourceLoadInfoPtr resource_load_info,
      const network::URLLoaderCompletionStatus& status) override;
  void NotifyResourceLoadCanceled(int64_t request_id) override;
  void Clone(mojo::PendingReceiver<blink::mojom::ResourceLoadInfoNotifier>
                 pending_resource_load_info_notifier) override;

  base::WeakPtr<WeakWrapperResourceLoadInfoNotifier> AsWeakPtr();

 private:
  THREAD_CHECKER(thread_checker_);

  // content::WebWorkerFetchContextImpl or content::RenderFrameImpl own
  // `resource_load_info_notifier_` and `this`, which ensure that
  // `resource_load_info_notifier_` outlives `this`.
  raw_ptr<mojom::ResourceLoadInfoNotifier> resource_load_info_notifier_;

  base::WeakPtrFactory<WeakWrapperResourceLoadInfoNotifier> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEAK_WRAPPER_RESOURCE_LOAD_INFO_NOTIFIER_H_
