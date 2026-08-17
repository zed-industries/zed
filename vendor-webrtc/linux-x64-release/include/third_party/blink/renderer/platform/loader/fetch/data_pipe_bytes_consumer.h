// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_DATA_PIPE_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_DATA_PIPE_BYTES_CONSUMER_H_

#include "base/memory/scoped_refptr.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

// An adapter for mojo::DataPipe. As mojo::DataPipe lacks signals completion and
// error signals, we define another interface, CompletionNotifier, for the
// signals.
class PLATFORM_EXPORT DataPipeBytesConsumer final : public BytesConsumer {
  USING_PRE_FINALIZER(DataPipeBytesConsumer, Dispose);

 public:
  class PLATFORM_EXPORT CompletionNotifier final
      : public GarbageCollected<CompletionNotifier> {
   public:
    explicit CompletionNotifier(DataPipeBytesConsumer* bytes_consumer)
        : bytes_consumer_(bytes_consumer) {}

    // One of these methods must be called to signal the end of the data
    // stream. (SignalSize notifies the total size. That information can
    // be used to detect the end-of-stream). We cannot assume that the end
    // of the pipe completes the stream successfully since errors can
    // occur after the last byte is written into the pipe.
    void SignalComplete();
    void SignalSize(uint64_t size);
    void SignalError(const BytesConsumer::Error& error);
    void Trace(Visitor*) const;

   private:
    const WeakMember<DataPipeBytesConsumer> bytes_consumer_;
  };

  DataPipeBytesConsumer(scoped_refptr<base::SingleThreadTaskRunner> task_runner,
                        mojo::ScopedDataPipeConsumerHandle,
                        CompletionNotifier** notifier);
  ~DataPipeBytesConsumer() override;

  Result BeginRead(base::span<const char>& buffer) override;
  Result EndRead(size_t read_size) override;
  mojo::ScopedDataPipeConsumerHandle DrainAsDataPipe() override;
  void SetClient(BytesConsumer::Client*) override;
  void ClearClient() override;

  void Cancel() override;
  PublicState GetPublicState() const override;
  Error GetError() const override {
    DCHECK_EQ(InternalState::kErrored, state_);
    return error_;
  }
  String DebugName() const override { return "DataPipeBytesConsumer"; }

  void Trace(Visitor*) const override;

 private:
  bool IsWaiting() const;
  void MaybeClose();
  void SetError(const Error& error);
  void Notify(MojoResult);
  void ClearDataPipe();
  void SignalComplete();
  void SignalSize(uint64_t);
  void SignalError(const Error& error);
  void Dispose();

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  mojo::ScopedDataPipeConsumerHandle data_pipe_;
  mojo::SimpleWatcher watcher_;
  Member<BytesConsumer::Client> client_;
  InternalState state_ = InternalState::kWaiting;
  Error error_;
  uint64_t num_read_bytes_ = 0;
  std::optional<uint64_t> total_size_;
  bool is_in_two_phase_read_ = false;
  bool has_pending_notification_ = false;
  bool has_pending_complete_ = false;
  bool has_pending_error_ = false;
  bool completion_signaled_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_DATA_PIPE_BYTES_CONSUMER_H_
