// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_BACKGROUND_RESOURCE_FETCH_ASSETS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_BACKGROUND_RESOURCE_FETCH_ASSETS_H_

#include "base/memory/scoped_refptr.h"
#include "services/network/public/cpp/shared_url_loader_factory.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_background_resource_fetch_assets.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace blink {

// This class can be used for testing the behaviour of a fetch request with a
// BackgroundURLLoader.
class FakeBackgroundResourceFetchAssets
    : public WebBackgroundResourceFetchAssets {
 public:
  using LoadStartCallback = base::OnceCallback<void(
      mojo::PendingReceiver<network::mojom::URLLoader>,
      mojo::PendingRemote<network::mojom::URLLoaderClient>)>;

  // Constructs a FakeBackgroundResourceFetchAssets object.
  // `load_start_callback` will be called on the `background_task_runner` when
  // a fetch request is started.
  FakeBackgroundResourceFetchAssets(
      scoped_refptr<base::SequencedTaskRunner> background_task_runner,
      LoadStartCallback load_start_callback);

  FakeBackgroundResourceFetchAssets(const FakeBackgroundResourceFetchAssets&) =
      delete;
  FakeBackgroundResourceFetchAssets& operator=(
      const FakeBackgroundResourceFetchAssets&) = delete;
  ~FakeBackgroundResourceFetchAssets() override;

  // WebBackgroundResourceFetchAssets implementation:
  const scoped_refptr<base::SequencedTaskRunner>& GetTaskRunner() override;
  scoped_refptr<network::SharedURLLoaderFactory> GetLoaderFactory() override;
  URLLoaderThrottleProvider* GetThrottleProvider() override { return nullptr; }
  const blink::LocalFrameToken& GetLocalFrameToken() override;

 private:
  scoped_refptr<base::SequencedTaskRunner> background_task_runner_;
  std::unique_ptr<network::PendingSharedURLLoaderFactory>
      pending_loader_factory_;
  scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory_;
  const blink::LocalFrameToken local_frame_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FAKE_BACKGROUND_RESOURCE_FETCH_ASSETS_H_
