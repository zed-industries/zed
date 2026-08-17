// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_BACKGROUND_RESPONSE_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_BACKGROUND_RESPONSE_PROCESSOR_H_

#include <memory>
#include <optional>
#include <variant>
#include <vector>

#include "mojo/public/cpp/base/big_buffer.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "services/network/public/mojom/url_response_head.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace base {
class SequencedTaskRunner;
}  // namespace base

namespace blink {

// BackgroundResponseProcessor is used for processing the response on the
// background thread of the BackgroundURLLoader. This class is created by
// a BackgroundResponseProcessorFactory on the background thread, and lives in
// the background thread.
class BLINK_PLATFORM_EXPORT BackgroundResponseProcessor {
 public:
  using BodyVariant =
      std::variant<mojo::ScopedDataPipeConsumerHandle, SegmentedBuffer>;
  class Client {
   public:
    virtual ~Client() = default;
    virtual void DidFinishBackgroundResponseProcessor(
        network::mojom::URLResponseHeadPtr head,
        BodyVariant body,
        std::optional<mojo_base::BigBuffer> cached_metadata) = 0;
    virtual void PostTaskToMainThread(CrossThreadOnceClosure task) = 0;
  };

  virtual ~BackgroundResponseProcessor() = default;

  // Called on the background thread to start processing the response. Returns
  // false if `this` can synchronously decide not to process the response.
  // Otherwise returns true, and `Client::DidFinishBackgroundResponseProcessor`
  // will be asynchronously called on the background thread, with the passed
  // `head` and `cached_metadata`. If `this` consumes the passed data pipe of
  // `body`, `Client::DidFinishBackgroundResponseProcessor` will be called with
  // a Deque<Vector<char>> `body`. Otherwise, it will be called with the passed
  // data pipe handle `body`.
  virtual bool MaybeStartProcessingResponse(
      network::mojom::URLResponseHeadPtr& head,
      mojo::ScopedDataPipeConsumerHandle& body,
      std::optional<mojo_base::BigBuffer>& cached_metadata,
      scoped_refptr<base::SequencedTaskRunner> background_task_runner,
      Client* client) = 0;
};

// A factory for BackgroundResponseProcessor. This is created in the main
// thread, and passed to the background thread.
class BLINK_PLATFORM_EXPORT BackgroundResponseProcessorFactory {
 public:
  virtual ~BackgroundResponseProcessorFactory() = default;
  // Creates a new BackgroundResponseProcessor. Called on the background
  // thread.
  virtual std::unique_ptr<BackgroundResponseProcessor> Create() && = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_BACKGROUND_RESPONSE_PROCESSOR_H_
