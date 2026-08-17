// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_WEBCODECS_LOGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_WEBCODECS_LOGGER_H_

#include "base/memory/scoped_refptr.h"
#include "base/time/time.h"
#include "media/base/video_frame.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

// This class is used to distribute a VideoFrameCloseAuditor flag to
// VideoFrameHandles. If a handle's destructor is run without having received a
// call to Invalidate(), it will set |close_auditor_|. The WebCodecsLogger
// periodically checks whether or not the flag is set, and outputs an error
// message to the JS console, reminding developers to call close() on their
// VideoFrames.
//
// This class lets us avoid making VideoFrames ExecutionLifeCycleObservers,
// which could add 1000s of observers per second. It also avoids the use of
// a pre-finzalizer on VideoFrames, which could have a GC performance impact.
class MODULES_EXPORT WebCodecsLogger : public GarbageCollected<WebCodecsLogger>,
                                       public Supplement<ExecutionContext> {
 public:
  // Class that reports when blink::VideoFrames have been garbage collected
  // without having close() called on them. This is a web page application
  // error which can cause a web page to stall.
  class VideoFrameCloseAuditor
      : public WTF::ThreadSafeRefCounted<VideoFrameCloseAuditor> {
   public:
    void ReportUnclosedFrame();
    void Clear();

    bool were_frames_not_closed() { return were_frames_not_closed_; }

   private:
    friend class WTF::ThreadSafeRefCounted<VideoFrameCloseAuditor>;
    ~VideoFrameCloseAuditor() = default;

    bool were_frames_not_closed_ = false;
  };

  static const char kSupplementName[];

  static WebCodecsLogger& From(ExecutionContext&);

  explicit WebCodecsLogger(ExecutionContext&);
  virtual ~WebCodecsLogger() = default;

  // Disallow copy and assign.
  WebCodecsLogger& operator=(const WebCodecsLogger&) = delete;
  WebCodecsLogger(const WebCodecsLogger&) = delete;

  // Returns |close_auditor_| and starts |timer_| if needed.
  scoped_refptr<VideoFrameCloseAuditor> GetCloseAuditor();

  void Trace(Visitor*) const override;

 private:
  void LogCloseErrors(TimerBase*);

  base::TimeTicks last_auditor_access_;
  scoped_refptr<VideoFrameCloseAuditor> close_auditor_;
  HeapTaskRunnerTimer<WebCodecsLogger> timer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_WEBCODECS_LOGGER_H_
