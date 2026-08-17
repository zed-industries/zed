// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_MIME_SNIFFING_URL_LOADER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_MIME_SNIFFING_URL_LOADER_H_

#include <tuple>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "services/network/public/mojom/url_loader.mojom.h"
#include "services/network/public/mojom/url_loader_factory.mojom-forward.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/loader/url_loader_throttle.h"

namespace blink {

class MimeSniffingThrottle;

// Reads the response body and determines its mime type. This url loader buffers
// the response body until the mime type is decided. MimeSniffingURLLoader
// is expected to be created just after receiving OnReceiveResponse(), so this
// handles only OnComplete() as a network::mojom::URLLoaderClient.
//
// This loader has five states:
// kWaitForBody: The initial state until the body is received (=
//               OnReceiveResponse() is called) or the response is
//               finished (= OnComplete() is called). When body is provided, the
//               state is changed to kSniffing. Otherwise the state goes to
//               kCompleted.
// kSniffing: Receives the body from the source loader and estimate the mime
//            type. The received body is kept in this loader until the mime type
//            is decided. When the mime type is decided or all body has been
//            received, this loader will dispatch queued messages to the
//            destination loader client, and then the state is changed to
//            kSending.
// kSending: Receives the body and sends it to the destination loader client.
//           The state changes to kCompleted after all data is sent.
// kCompleted: All data has been sent to the destination loader.
// kAborted: Unexpected behavior happens. Watchers, pipes and the binding from
//           the source loader to |this| are stopped. All incoming messages from
//           the destination (through network::mojom::URLLoader) are ignored in
//           this state.
class BLINK_COMMON_EXPORT MimeSniffingURLLoader
    : public network::mojom::URLLoaderClient,
      public network::mojom::URLLoader {
 public:
  MimeSniffingURLLoader(const MimeSniffingURLLoader&) = delete;
  MimeSniffingURLLoader& operator=(const MimeSniffingURLLoader&) = delete;
  ~MimeSniffingURLLoader() override;

  // Start waiting for the body.
  void Start(
      mojo::PendingRemote<network::mojom::URLLoader> source_url_loader_remote,
      mojo::PendingReceiver<network::mojom::URLLoaderClient>
          source_url_client_receiver,
      mojo::ScopedDataPipeConsumerHandle body);

  // mojo::PendingRemote<network::mojom::URLLoader> controls the lifetime of the
  // loader.
  static std::tuple<mojo::PendingRemote<network::mojom::URLLoader>,
                    mojo::PendingReceiver<network::mojom::URLLoaderClient>,
                    MimeSniffingURLLoader*>
  CreateLoader(base::WeakPtr<MimeSniffingThrottle> throttle,
               const GURL& response_url,
               network::mojom::URLResponseHeadPtr response_head,
               scoped_refptr<base::SequencedTaskRunner> task_runner);

 private:
  MimeSniffingURLLoader(base::WeakPtr<MimeSniffingThrottle> throttle,
                        const GURL& response_url,
                        network::mojom::URLResponseHeadPtr response_head,
                        mojo::PendingRemote<network::mojom::URLLoaderClient>
                            destination_url_loader_client,
                        scoped_refptr<base::SequencedTaskRunner> task_runner);

  // network::mojom::URLLoaderClient implementation (called from the source of
  // the response):
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
                        OnUploadProgressCallback ack_callback) override;
  void OnTransferSizeUpdated(int32_t transfer_size_diff) override;
  void OnComplete(const network::URLLoaderCompletionStatus& status) override;

  // network::mojom::URLLoader implementation (called from the destination of
  // the response):
  void FollowRedirect(
      const std::vector<std::string>& removed_headers,
      const net::HttpRequestHeaders& modified_headers,
      const net::HttpRequestHeaders& modified_cors_exempt_headers,
      const std::optional<GURL>& new_url) override;
  void SetPriority(net::RequestPriority priority,
                   int32_t intra_priority_value) override;

  void OnBodyReadable(MojoResult);
  void OnBodyWritable(MojoResult);
  void CompleteSniffing();
  void CompleteSending();
  void SendReceivedBodyToClient();
  void ForwardBodyToClient();

  void Abort();

  static const char kDefaultMimeType[];

  base::WeakPtr<MimeSniffingThrottle> throttle_;

  mojo::Receiver<network::mojom::URLLoaderClient> source_url_client_receiver_{
      this};
  mojo::Remote<network::mojom::URLLoader> source_url_loader_;
  mojo::Remote<network::mojom::URLLoaderClient> destination_url_loader_client_;

  GURL response_url_;

  // Capture the response head to defer to send it to the destination until the
  // mime type is decided.
  network::mojom::URLResponseHeadPtr response_head_;

  scoped_refptr<base::SequencedTaskRunner> task_runner_;

  enum class State { kWaitForBody, kSniffing, kSending, kCompleted, kAborted };
  State state_ = State::kWaitForBody;

  // Set if OnComplete() is called during sniffing.
  std::optional<network::URLLoaderCompletionStatus> complete_status_;

  std::vector<char> buffered_body_;
  size_t bytes_remaining_in_buffer_;

  mojo::ScopedDataPipeConsumerHandle body_consumer_handle_;
  mojo::ScopedDataPipeProducerHandle body_producer_handle_;
  mojo::SimpleWatcher body_consumer_watcher_;
  mojo::SimpleWatcher body_producer_watcher_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_MIME_SNIFFING_URL_LOADER_H_
