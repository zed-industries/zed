// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_NAVIGATION_BODY_LOADER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_NAVIGATION_BODY_LOADER_H_

#include <optional>
#include <variant>

#include "base/containers/span.h"
#include "base/containers/span_or_size.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "services/network/public/mojom/url_loader.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_loader_freeze_mode.h"
#include "third_party/blink/public/platform/web_url_error.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class ResourceLoadInfoNotifierWrapper;
struct WebEncodingData;
struct WebNavigationParams;

// This class is used to load the body of main resource during navigation.
// It is provided by the client which commits a navigation.
// See WebNavigationParams for more details.
class BLINK_EXPORT WebNavigationBodyLoader {
 public:
  class Client {
   public:
    virtual ~Client() {}

    // Notifies about more data available. Called multiple times.
    // If main resource is empty, can be not called at all. This will not be
    // called if DecodedBodyDataReceived() has been called.
    virtual void BodyDataReceived(base::span<const char> data) = 0;

    // Called with decoded data. This will be called instead of
    // BodyDataReceived() if the data is able to be decoded off thread.
    // |encoded_data| will contain the original data if
    // |should_keep_encoded_data| was passed to StartLoadingBodyInBackground().
    virtual void DecodedBodyDataReceived(
        const WebString& data,
        const WebEncodingData& encoding_data,
        base::SpanOrSize<const char> encoded_data) {
      NOTREACHED();
    }

    // Called once at the end. If something went wrong, |error| will be set.
    // No more calls are issued after this one.
    virtual void BodyLoadingFinished(
        base::TimeTicks completion_time,
        int64_t total_encoded_data_length,
        int64_t total_encoded_body_length,
        int64_t total_decoded_body_length,
        const std::optional<WebURLError>& error) = 0;

    // The client can return a ProcessBackgroundDataCallback which will be
    // called on a background thread with the decoded data. The returned
    // callback will be called on a background thread with the same decoded data
    // which will be given to DecodedBodyDataReceived().
    using ProcessBackgroundDataCallback =
        WTF::CrossThreadRepeatingFunction<void(const WebString&)>;
    virtual ProcessBackgroundDataCallback TakeProcessBackgroundDataCallback() {
      return ProcessBackgroundDataCallback();
    }
  };

  // This method fills navigation params related to the navigation request,
  // redirects and response, and also creates a body loader if needed.
  static void FillNavigationParamsResponseAndBodyLoader(
      mojom::CommonNavigationParamsPtr common_params,
      mojom::CommitNavigationParamsPtr commit_params,
      int request_id,
      network::mojom::URLResponseHeadPtr response_head,
      mojo::ScopedDataPipeConsumerHandle response_body,
      network::mojom::URLLoaderClientEndpointsPtr url_loader_client_endpoints,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper,
      bool is_main_frame,
      WebNavigationParams* navigation_params,
      bool is_ad_frame);

  // It should be safe to destroy WebNavigationBodyLoader at any moment,
  // including from inside any client notification.
  virtual ~WebNavigationBodyLoader() {}

  // While frozen, data will be read on the renderer side but will not invoke
  // any web-exposed behavior such as dispatching messages or handling
  // redirects. This method can be called multiple times at any moment.
  virtual void SetDefersLoading(WebLoaderFreezeMode mode) = 0;

  // Starts loading the body. Client must be non-null, and will receive the
  // body, and final result.
  virtual void StartLoadingBody(Client*) = 0;

  enum class BodyLoaderType {
    kStatic,
    kNetwork,
  };
  virtual BodyLoaderType GetType() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_NAVIGATION_BODY_LOADER_H_
