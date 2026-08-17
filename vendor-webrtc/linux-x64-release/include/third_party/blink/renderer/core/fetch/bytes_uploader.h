// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_UPLOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_UPLOADER_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "services/network/public/mojom/chunked_data_pipe_getter.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

class CORE_EXPORT BytesUploader
    : public GarbageCollected<BytesUploader>,
      public BytesConsumer::Client,
      public network::mojom::blink::ChunkedDataPipeGetter,
      public ExecutionContextLifecycleObserver {
  USING_PRE_FINALIZER(BytesUploader, Dispose);

 public:
  class Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;
    virtual void OnComplete() = 0;
    virtual void OnError() = 0;
  };

  BytesUploader(
      ExecutionContext* execution_context,
      BytesConsumer* consumer,
      mojo::PendingReceiver<network::mojom::blink::ChunkedDataPipeGetter>
          pending_receiver,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      Client* client);
  ~BytesUploader() override;
  BytesUploader(const BytesUploader&) = delete;
  BytesUploader& operator=(const BytesUploader&) = delete;

  void Trace(blink::Visitor*) const override;

 private:
  // BytesConsumer::Client implementation
  void OnStateChange() override;
  String DebugName() const override { return "BytesUploader"; }

  // mojom::ChunkedDataPipeGetter implementation:
  void GetSize(GetSizeCallback get_size_callback) override;
  void StartReading(mojo::ScopedDataPipeProducerHandle upload_pipe) override;

  // ExecutionContextLifecycleObserver implementation:
  void ContextDestroyed() override;

  void OnPipeWriteable(MojoResult unused);
  void WriteDataOnPipe();

  void Close();
  // TODO(yoichio): Add a string parameter and show it on console.
  void CloseOnError();

  void Dispose();

  Member<BytesConsumer> consumer_;
  Member<Client> client_;
  HeapMojoReceiver<network::mojom::blink::ChunkedDataPipeGetter, BytesUploader>
      receiver_;
  mojo::ScopedDataPipeProducerHandle upload_pipe_;
  mojo::SimpleWatcher upload_pipe_watcher_;
  GetSizeCallback get_size_callback_;
  uint64_t total_size_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BYTES_UPLOADER_H_
