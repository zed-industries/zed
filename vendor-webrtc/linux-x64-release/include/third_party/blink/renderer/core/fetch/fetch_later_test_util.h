// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_TEST_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_TEST_UTIL_H_

#include "base/memory/scoped_refptr.h"
#include "base/test/mock_callback.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/public/mojom/loader/fetch_later.mojom.h"
#include "third_party/blink/public/platform/child_url_loader_factory_bundle.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_testing.h"
#include "third_party/blink/renderer/core/frame/frame_test_helpers.h"
#include "third_party/blink/renderer/core/loader/empty_clients.h"
#include "third_party/blink/renderer/core/testing/dummy_page_holder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// FetchLaterTestingScope supports providing a custom implementation of
// `FetchLaterLoaderFactory` to allow `FetchLaterManager::GetFactory()` to work
// properly in unit tests.
//
// Example usage:
//
// MockFetchLaterLoaderFactory factory;
// auto* client = MakeGarbageCollected<FakeLocalFrameClient>();
// client->GetLoaderFactoryBundle()->SetFetchLaterLoaderFactory(
//     factory.BindNewEndpointAndPassDedicatedRemote());
// FetchLaterTestingScope scope(client);
// ...
class FetchLaterTestingScope : public V8TestingScope {
  STACK_ALLOCATED();

 public:
  explicit FetchLaterTestingScope(
      LocalFrameClient* frame_client,
      const String& source_page_url = "https://example.com");
};

class MockFetchLaterLoaderFactory
    : public blink::mojom::FetchLaterLoaderFactory {
 public:
  MockFetchLaterLoaderFactory() = default;

  mojo::PendingAssociatedRemote<blink::mojom::FetchLaterLoaderFactory>
  BindNewEndpointAndPassDedicatedRemote() {
    return receiver_.BindNewEndpointAndPassDedicatedRemote();
  }

  void FlushForTesting() { receiver_.FlushForTesting(); }

  // blink::mojom::FetchLaterLoaderFactory overrides:
  MOCK_METHOD(void,
              CreateLoader,
              (mojo::PendingAssociatedReceiver<blink::mojom::FetchLaterLoader>,
               int32_t,
               uint32_t,
               const network::ResourceRequest&,
               const net::MutableNetworkTrafficAnnotationTag&),
              (override));
  MOCK_METHOD(
      void,
      Clone,
      (mojo::PendingAssociatedReceiver<blink::mojom::FetchLaterLoaderFactory>),
      (override));

 private:
  mojo::AssociatedReceiver<blink::mojom::FetchLaterLoaderFactory> receiver_{
      this};
};

// A fake LocalFrameClient providing non-null ChildURLLoaderFactoryBundle.
class FakeLocalFrameClient : public EmptyLocalFrameClient {
 public:
  FakeLocalFrameClient()
      : loader_factory_bundle_(
            base::MakeRefCounted<blink::ChildURLLoaderFactoryBundle>()) {}

  // EmptyLocalFrameClient overrides:
  blink::ChildURLLoaderFactoryBundle* GetLoaderFactoryBundle() override {
    return loader_factory_bundle_.get();
  }

 private:
  scoped_refptr<blink::ChildURLLoaderFactoryBundle> loader_factory_bundle_;
};

// A fake WebLocalFrameClient providing non-null ChildURLLoaderFactoryBundle.
class FakeWebFrameClient : public frame_test_helpers::TestWebFrameClient {
 public:
  FakeWebFrameClient()
      : loader_factory_bundle_(
            base::MakeRefCounted<blink::ChildURLLoaderFactoryBundle>()) {}

  // TestWebFrameClient overrides:
  blink::ChildURLLoaderFactoryBundle* GetLoaderFactoryBundle() override {
    return loader_factory_bundle_.get();
  }

 private:
  scoped_refptr<blink::ChildURLLoaderFactoryBundle> loader_factory_bundle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FETCH_LATER_TEST_UTIL_H_
