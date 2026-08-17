// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_TRANSFERRING_OPTIMIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_TRANSFERRING_OPTIMIZER_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/core/streams/readable_stream_transferring_optimizer.h"
#include "third_party/blink/renderer/modules/breakout_box/frame_queue_underlying_source.h"
#include "third_party/blink/renderer/modules/breakout_box/transferred_frame_queue_underlying_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

template <typename NativeFrameType>
class FrameQueueTransferringOptimizer final
    : public ReadableStreamTransferringOptimizer {
 public:
  using FrameQueueHost = FrameQueueUnderlyingSource<NativeFrameType>;

  using ConnectHostCallback = CrossThreadOnceFunction<void(
      scoped_refptr<base::SequencedTaskRunner>,
      CrossThreadPersistent<
          TransferredFrameQueueUnderlyingSource<NativeFrameType>>)>;

  FrameQueueTransferringOptimizer(
      FrameQueueHost*,
      scoped_refptr<base::SequencedTaskRunner> host_runner,
      wtf_size_t max_queue_size,
      ConnectHostCallback connect_host_callback,
      CrossThreadOnceFunction<void()> transferred_source_destroyed_callback);
  ~FrameQueueTransferringOptimizer() override = default;

  UnderlyingSourceBase* PerformInProcessOptimization(
      ScriptState* script_state) override;

 private:
  CrossThreadWeakPersistent<FrameQueueHost> host_;
  scoped_refptr<base::SequencedTaskRunner> host_runner_;
  ConnectHostCallback connect_host_callback_;
  CrossThreadOnceFunction<void()> transferred_source_destroyed_callback_;
  wtf_size_t max_queue_size_;
};

extern template class MODULES_EXTERN_TEMPLATE_EXPORT
    FrameQueueTransferringOptimizer<scoped_refptr<media::VideoFrame>>;
extern template class MODULES_EXTERN_TEMPLATE_EXPORT
    FrameQueueTransferringOptimizer<scoped_refptr<media::AudioBuffer>>;

using VideoFrameQueueTransferOptimizer =
    FrameQueueTransferringOptimizer<scoped_refptr<media::VideoFrame>>;
using AudioDataQueueTransferOptimizer =
    FrameQueueTransferringOptimizer<scoped_refptr<media::AudioBuffer>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_TRANSFERRING_OPTIMIZER_H_
