// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_RESOURCE_LOAD_INFO_NOTIFIER_WRAPPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_RESOURCE_LOAD_INFO_NOTIFIER_WRAPPER_H_

#include "base/sequence_checker.h"
#include "base/task/single_thread_task_runner.h"
#include "build/build_config.h"
#include "net/base/request_priority.h"
#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom.h"
#include "third_party/blink/public/platform/web_common.h"

class GURL;

namespace net {
struct RedirectInfo;
}  // namespace net

namespace network {
struct URLLoaderCompletionStatus;
}  // namespace network

namespace blink {

class WeakWrapperResourceLoadInfoNotifier;

// A wrapper over a weak pointer of WeakWrapperResourceLoadInfoNotifier used to
// notify the loading stats. Also, it collects histograms related to resource
// load. Callers should start with NotifyResourceLoadInitiated.
class BLINK_PLATFORM_EXPORT ResourceLoadInfoNotifierWrapper {
 public:
  explicit ResourceLoadInfoNotifierWrapper(
      base::WeakPtr<WeakWrapperResourceLoadInfoNotifier>
          weak_wrapper_resource_load_info_notifier);
  ResourceLoadInfoNotifierWrapper(
      base::WeakPtr<WeakWrapperResourceLoadInfoNotifier>
          weak_wrapper_resource_load_info_notifier,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  ~ResourceLoadInfoNotifierWrapper();

#if BUILDFLAG(IS_ANDROID)
  void NotifyUpdateUserGestureCarryoverInfo();
#endif
  void NotifyResourceLoadInitiated(
      int64_t request_id,
      const GURL& request_url,
      const std::string& http_method,
      const GURL& referrer,
      network::mojom::RequestDestination request_destination,
      net::RequestPriority request_priority,
      bool is_ad_resource);
  void NotifyResourceRedirectReceived(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr redirect_response);
  void NotifyResourceResponseReceived(
      network::mojom::URLResponseHeadPtr response_head);
  void NotifyResourceTransferSizeUpdated(int32_t transfer_size_diff);
  void NotifyResourceLoadCompleted(
      const network::URLLoaderCompletionStatus& status);
  void NotifyResourceLoadCanceled(int net_error);

 private:
  SEQUENCE_CHECKER(sequence_checker_);

  // |weak_wrapper_resource_load_info_notifier_| should only be dereferenced on
  // the same thread as |task_runner_| runs on.
  base::WeakPtr<WeakWrapperResourceLoadInfoNotifier>
      weak_wrapper_resource_load_info_notifier_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // This struct holds the loading stats passed to
  // |weak_wrapper_resource_load_info_notifier_|.
  mojom::ResourceLoadInfoPtr resource_load_info_;

  bool is_ad_resource_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_RESOURCE_LOAD_INFO_NOTIFIER_WRAPPER_H_
