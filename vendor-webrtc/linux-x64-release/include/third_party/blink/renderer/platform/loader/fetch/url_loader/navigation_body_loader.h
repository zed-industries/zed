// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_NAVIGATION_BODY_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_NAVIGATION_BODY_LOADER_H_

#include <stddef.h>
#include <stdint.h>

#include <utility>
#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "services/network/public/mojom/url_loader.mojom.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/mojom/loader/resource_load_info.mojom.h"
#include "third_party/blink/public/mojom/navigation/navigation_params.mojom-forward.h"
#include "third_party/blink/public/platform/web_loader_freeze_mode.h"
#include "third_party/blink/public/platform/web_navigation_body_loader.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace network {
struct URLLoaderCompletionStatus;
}  // namespace network

namespace blink {

class BodyTextDecoder;

// Navigation request is started in the browser process, and all redirects
// and final response are received there. Then we pass URLLoader and
// URLLoaderClient bindings to the renderer process, and create an instance
// of this class. It receives the response body, completion status and cached
// metadata, and dispatches them to Blink. It also ensures that completion
// status comes to Blink after the whole body was read and cached code metadata
// was received.
class PLATFORM_EXPORT NavigationBodyLoader
    : public WebNavigationBodyLoader,
      public network::mojom::URLLoaderClient {
 public:
  NavigationBodyLoader(
      const KURL& original_url,
      network::mojom::URLResponseHeadPtr response_head,
      mojo::ScopedDataPipeConsumerHandle response_body,
      network::mojom::URLLoaderClientEndpointsPtr url_loader_client_endpoints,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper);
  ~NavigationBodyLoader() override;

  // Starts reading and decoding the body on a background thread. Client
  // callbacks will not be called until StartLoadingBody() is called. If
  // |should_keep_encoded_data| is true, the original data will be copied from
  // the background thread and passed to DecodedBodyDataReceived().
  //
  // If the preload scanner is being used in the parser, the flow of data to the
  // scanner will go like this:
  //   1. NavigationBodyLoader calls DecodedBodyDataReceived(), which will
  //      end up getting forwarded to HTMLDocumentParser::Append().
  //   2. HTMLDocumentParser::Append() will cause the background preload
  //      scanner to be created and scan the initial data passed to Append().
  //   3. NavigationBodyLoader calls TakeProcessBackgroundDataCallback()
  //      which tells HTMLDocumentParser to stop sending data to the
  //      preload scanner in Append().
  //   4. NavigationBodyLoader will pass data directly to the callback
  //      taken from TakeProcessBackgroundDataCallback(), which avoids
  //      hitting the main thread at all. HTMLDocumentParser will still
  //      receive data through Append() calls.
  void StartLoadingBodyInBackground(std::unique_ptr<BodyTextDecoder> decoder,
                                    bool should_keep_encoded_data);

  void FlushOffThreadBodyReaderForTesting();

 private:
  // The loading flow is outlined below. NavigationBodyLoader can be safely
  // deleted at any moment, and it will record cancelation stats, but will not
  // notify the client.
  //
  // StartLoadingBody
  //   request code cache
  //   Note: If the kEarlyBodyLoad feature is enabled, BindURLLoaderAndContinue
  //   can run in parallel with requesting the code cache. The client will get a
  //   completion signal for both these events.
  // ContinueWithCodeCache
  //   notify client about cache
  // BindURLLoaderAndContinue
  // OnReceiveResponse
  //   start reading from the pipe
  // OnReadable (zero or more times)
  //   notify client about data
  // OnComplete (this might come before the whole body data is read,
  //             for example due to different mojo pipes being used
  //             without a relative order guarantee)
  //   save status for later use
  // OnReadable (zero or more times)
  //   notify client about data
  // NotifyCompletionIfAppropriate
  //   notify client about completion

  // WebNavigationBodyLoader implementation.
  void SetDefersLoading(WebLoaderFreezeMode mode) override;
  void StartLoadingBody(WebNavigationBodyLoader::Client* client) override;
  BodyLoaderType GetType() const override { return BodyLoaderType::kNetwork; }

  // network::mojom::URLLoaderClient implementation.
  void OnReceiveEarlyHints(network::mojom::EarlyHintsPtr early_hints) override;
  void OnReceiveResponse(
      network::mojom::URLResponseHeadPtr response_head,
      mojo::ScopedDataPipeConsumerHandle body,
      std::optional<mojo_base::BigBuffer> cached_metadata) override;
  void OnReceiveRedirect(
      const net::RedirectInfo& redirect_info,
      network::mojom::URLResponseHeadPtr response_head) override;
  void OnUploadProgress(int64_t current_position,
                        int64_t total_size,
                        OnUploadProgressCallback callback) override;
  void OnTransferSizeUpdated(int32_t transfer_size_diff) override;
  void OnComplete(const network::URLLoaderCompletionStatus& status) override;

  void BindURLLoaderAndContinue();
  void OnConnectionClosed();
  void OnReadable(MojoResult unused);
  // This method reads data from the pipe in a cycle and dispatches
  // BodyDataReceived synchronously.
  void ReadFromDataPipe();
  void NotifyCompletionIfAppropriate();
  void BindURLLoaderAndStartLoadingResponseBodyIfPossible();

  // Takes and processes data loaded by |off_thread_body_reader_|.
  void ProcessOffThreadData();

  NavigationBodyLoader& operator=(const NavigationBodyLoader&) = delete;
  NavigationBodyLoader(const NavigationBodyLoader&) = delete;

  // Navigation parameters.
  network::mojom::URLResponseHeadPtr response_head_;
  mojo::ScopedDataPipeConsumerHandle response_body_;
  network::mojom::URLLoaderClientEndpointsPtr endpoints_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // These bindings are live while loading the response.
  mojo::Remote<network::mojom::URLLoader> url_loader_;
  mojo::Receiver<network::mojom::URLLoaderClient> url_loader_client_receiver_{
      this};
  raw_ptr<WebNavigationBodyLoader::Client> client_ = nullptr;

  // The handle and watcher are live while loading the body.
  mojo::ScopedDataPipeConsumerHandle handle_;
  mojo::SimpleWatcher handle_watcher_;

  // Used to notify the navigation loading stats.
  std::unique_ptr<ResourceLoadInfoNotifierWrapper>
      resource_load_info_notifier_wrapper_;

  // The final status received from network or cancelation status if aborted.
  network::URLLoaderCompletionStatus status_;

  // Whether we got the body handle to read data from.
  bool has_received_body_handle_ = false;
  // Whether we got the final status.
  bool has_received_completion_ = false;
  // Whether we got all the body data.
  bool has_seen_end_of_data_ = false;

  // Frozen body loader does not send any notifications to the client
  // and tries not to read from the body pipe.
  WebLoaderFreezeMode freeze_mode_ = WebLoaderFreezeMode::kNone;

  // This protects against reentrancy into OnReadable,
  // which can happen due to nested message loop triggered
  // from iniside BodyDataReceived client notification.
  bool is_in_on_readable_ = false;

  // The original navigation url to start with.
  const KURL original_url_;

  class MainThreadBodyReader;
  class OffThreadBodyReader;
  struct OffThreadBodyReaderDeleter {
    void operator()(const OffThreadBodyReader* ptr);
  };
  using OffThreadBodyReaderPtr =
      std::unique_ptr<OffThreadBodyReader, OffThreadBodyReaderDeleter>;
  OffThreadBodyReaderPtr off_thread_body_reader_;
  bool should_send_directly_to_preload_scanner_ = false;
  size_t max_data_to_process_per_task_ = 0;

  base::WeakPtrFactory<NavigationBodyLoader> weak_factory_{this};
};

template <>
struct DowncastTraits<NavigationBodyLoader> {
  static bool AllowFrom(const WebNavigationBodyLoader& body_loader) {
    return body_loader.GetType() ==
           NavigationBodyLoader::BodyLoaderType::kNetwork;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_NAVIGATION_BODY_LOADER_H_
