// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BUFFERING_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BUFFERING_BYTES_CONSUMER_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// BufferingBytesConsumer is a BytesConsumer. It takes a BytesConsumer
// ("the original BytesConsumer") as a constructor parameter, and results read
// from the BufferingBytesConsumer are as same as results which would be read
// from the original BytesConsumer.
// BufferingBytesConsumer buffers reads chunks from the original BytesConsumer
// and store it until they are read, before read requests are issued from the
// client.
class PLATFORM_EXPORT BufferingBytesConsumer final
    : public BytesConsumer,
      private BytesConsumer::Client {
 public:
  // Once `total_buffer_size_` reaches this, we will stop reading until it drops
  // below it again. For most users the simple cache won't store any file larger
  // than 55MB, so there's no point in buffering more than that. This limit is
  // only applied when the BufferedBytesConsumerLimitSize feature is enabled.
  static constexpr size_t kMaxBufferSize = 55 * 1024 * 1024;

  // Creates a BufferingBytesConsumer that waits some delay before beginning
  // to buffer data from the underlying consumer. This delay provides an
  // opportunity for the data to be drained before buffering begins. The
  // |bytes_consumer| is the original BytesConsumer. |bytes_consumer| must
  // not have a client.
  static BufferingBytesConsumer* CreateWithDelay(
      BytesConsumer* bytes_consumer,
      scoped_refptr<base::SingleThreadTaskRunner> timer_task_runner);

  // Creates a BufferingBytesConsumer that buffers immediately without any
  // delay. |bytes_consumer| is the original BytesConsumer. |bytes_consumer|
  // must not have a client.
  static BufferingBytesConsumer* Create(BytesConsumer* bytes_consumer);

  // Use the Create*() factory methods instead of direct instantiation.
  BufferingBytesConsumer(
      base::PassKey<BufferingBytesConsumer> key,
      BytesConsumer* bytes_consumer,
      scoped_refptr<base::SingleThreadTaskRunner> timer_task_runner,
      base::TimeDelta buffering_start_delay);
  ~BufferingBytesConsumer() override;

  // Attempt to start buffering data from the underlying consumer.  This will
  // only have an effect if we're currently in the kDelayed state.  If
  // buffering has already started or been explicitly stopped then this method
  // has no effect.
  void MaybeStartBuffering();

  // After this function is called, |this| will not do buffering. Already
  // buffered data still waits to be consumed, but after all the buffered data
  // is consumed, BeginRead and EndRead will result in BeginRead and EndRead
  // calls to the original BytesConsumer.
  void StopBuffering();

  // BufferingBytesConsumer
  Result BeginRead(base::span<const char>& buffer) override;
  Result EndRead(size_t read_size) override;
  scoped_refptr<BlobDataHandle> DrainAsBlobDataHandle(BlobSizePolicy) override;
  scoped_refptr<EncodedFormData> DrainAsFormData() override;
  mojo::ScopedDataPipeConsumerHandle DrainAsDataPipe() override;
  void SetClient(BytesConsumer::Client*) override;
  void ClearClient() override;
  void Cancel() override;
  PublicState GetPublicState() const override;
  Error GetError() const override;
  String DebugName() const override;

  void Trace(Visitor*) const override;

 private:
  void OnTimerFired(TimerBase*);

  // BufferingBytesConsumer::Client
  void OnStateChange() override;
  void BufferData();

  const Member<BytesConsumer> bytes_consumer_;
  HeapTaskRunnerTimer<BufferingBytesConsumer> timer_;
  HeapDeque<Member<GCedHeapVector<char>>> buffer_;
  size_t offset_for_first_chunk_ = 0;

  // The sum of the sizes of all Vectors in `buffer_`.
  size_t total_buffer_size_ = 0;

  enum class BufferingState {
    kDelayed,
    kStarted,
    kStopped,
  };
  BufferingState buffering_state_ = BufferingState::kDelayed;

  bool has_seen_end_of_data_ = false;
  bool has_seen_error_ = false;
  bool is_limiting_total_buffer_size_;
  Member<BytesConsumer::Client> client_;
  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_BUFFERING_BYTES_CONSUMER_H_
