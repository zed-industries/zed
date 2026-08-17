// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_URL_LOADER_FACTORY_FOR_BACKGROUND_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_URL_LOADER_FACTORY_FOR_BACKGROUND_THREAD_H_

#include "base/functional/callback.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver_set.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "services/network/public/mojom/url_loader.mojom-forward.h"

namespace blink {

// A fake SharedURLLoaderFactory that can handle a fetch request on the
// background thread. This SharedURLLoaderFactory is cloned and passed to a
// background thread via PendingFactory.
// This class can be used for testing the behaviour of a fetch request with a
// BackgroundURLLoader, or a synchronous fetch request.
class FakeURLLoaderFactoryForBackgroundThread
    : public network::SharedURLLoaderFactory {
 public:
  using LoadStartCallback = base::OnceCallback<void(
      mojo::PendingReceiver<network::mojom::URLLoader>,
      mojo::PendingRemote<network::mojom::URLLoaderClient>)>;

  // `load_start_callback` will be called in a background thread when a fetch
  // request is started.
  explicit FakeURLLoaderFactoryForBackgroundThread(
      LoadStartCallback load_start_callback);
  FakeURLLoaderFactoryForBackgroundThread(
      const FakeURLLoaderFactoryForBackgroundThread&) = delete;
  FakeURLLoaderFactoryForBackgroundThread& operator=(
      const FakeURLLoaderFactoryForBackgroundThread&) = delete;
  ~FakeURLLoaderFactoryForBackgroundThread() override;

  // network::SharedURLLoaderFactory:
  void CreateLoaderAndStart(
      mojo::PendingReceiver<network::mojom::URLLoader> loader,
      int32_t request_id,
      uint32_t options,
      const network::ResourceRequest& request,
      mojo::PendingRemote<network::mojom::URLLoaderClient> client,
      const net::MutableNetworkTrafficAnnotationTag& traffic_annotation)
      override;
  void Clone(mojo::PendingReceiver<network::mojom::URLLoaderFactory> receiver)
      override;
  std::unique_ptr<network::PendingSharedURLLoaderFactory> Clone() override;

 private:
  class PendingFactory;

  mojo::ReceiverSet<network::mojom::URLLoaderFactory,
                    scoped_refptr<FakeURLLoaderFactoryForBackgroundThread>>
      receivers_;
  LoadStartCallback load_start_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_URL_LOADER_FACTORY_FOR_BACKGROUND_THREAD_H_
