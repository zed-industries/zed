/*
 * Copyright (C) 2006, 2007, 2008, 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2009 Adam Barth. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_HTTP_REFRESH_SCHEDULER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_HTTP_REFRESH_SCHEDULER_H_

#include <memory>

#include "base/time/time.h"
#include "third_party/blink/public/web/web_frame_load_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class CORE_EXPORT HttpRefreshScheduler final
    : public GarbageCollected<HttpRefreshScheduler> {
 public:
  explicit HttpRefreshScheduler(Document*);
  HttpRefreshScheduler(const HttpRefreshScheduler&) = delete;
  HttpRefreshScheduler& operator=(const HttpRefreshScheduler&) = delete;
  ~HttpRefreshScheduler() = default;

  bool IsScheduledWithin(base::TimeDelta interval) const;
  void Schedule(base::TimeDelta delay, const KURL&, Document::HttpRefreshType);
  void MaybeStartTimer();
  void Cancel();

  void Trace(Visitor*) const;

 private:
  void NavigateTask();

  Member<Document> document_;
  TaskHandle navigate_task_handle_;

  struct ScheduledHttpRefresh {
   public:
    ScheduledHttpRefresh(base::TimeDelta delay,
                         const KURL& url,
                         ClientNavigationReason reason,
                         base::TimeTicks input_timestamp)
        : delay(delay),
          url(url),
          reason(reason),
          input_timestamp(input_timestamp) {}

    base::TimeDelta delay;
    KURL url;
    ClientNavigationReason reason;
    base::TimeTicks input_timestamp;
  };
  std::unique_ptr<ScheduledHttpRefresh> refresh_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_HTTP_REFRESH_SCHEDULER_H_
