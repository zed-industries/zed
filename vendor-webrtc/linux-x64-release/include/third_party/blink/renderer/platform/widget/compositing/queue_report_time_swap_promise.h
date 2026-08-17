// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_QUEUE_REPORT_TIME_SWAP_PROMISE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_QUEUE_REPORT_TIME_SWAP_PROMISE_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "cc/trees/swap_promise.h"

namespace blink {

// This class invokes DrainCallback to drain queued callbacks for frame numbers
// lower or equal to |source_frame_number| when the commit results in a
// successful activation of the pending layer tree in swap promise.
//
// This class doesn't have the reporting callback of the swap time.
class QueueReportTimeSwapPromise : public cc::SwapPromise {
 public:
  using DrainCallback = base::OnceCallback<void(int)>;
  QueueReportTimeSwapPromise(
      int source_frame_number,
      DrainCallback drain_callback,
      base::OnceClosure swap_callback,
      scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner);
  ~QueueReportTimeSwapPromise() override;
  QueueReportTimeSwapPromise(const QueueReportTimeSwapPromise&) = delete;
  QueueReportTimeSwapPromise& operator=(const QueueReportTimeSwapPromise&) =
      delete;

  void WillSwap(viz::CompositorFrameMetadata* metadata) override;
  void DidSwap() override;
  cc::SwapPromise::DidNotSwapAction DidNotSwap(DidNotSwapReason reason,
                                               base::TimeTicks now) override;
  void DidActivate() override;
  int64_t GetTraceId() const override { return 0; }

 private:
  int source_frame_number_;
  DrainCallback drain_callback_;
  base::OnceClosure swap_callback_;
#if BUILDFLAG(IS_ANDROID)
  const bool call_swap_on_activate_;
#endif
  scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_QUEUE_REPORT_TIME_SWAP_PROMISE_H_
