// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAILBOX_REF_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAILBOX_REF_H_

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "components/viz/common/resources/release_callback.h"
#include "gpu/command_buffer/common/sync_token.h"
#include "third_party/blink/renderer/platform/scheduler/public/thread.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {
class WebGraphicsContext3DProviderWrapper;

class MailboxRef : public ThreadSafeRefCounted<MailboxRef> {
 public:
  MailboxRef(const gpu::SyncToken& sync_token,
             base::PlatformThreadRef context_thread_ref,
             scoped_refptr<base::SingleThreadTaskRunner> context_task_runner,
             viz::ReleaseCallback release_callback);
  ~MailboxRef();

  bool is_cross_thread() const {
    return base::PlatformThread::CurrentRef() != context_thread_ref_;
  }
  void set_sync_token(gpu::SyncToken token) { sync_token_ = token; }
  const gpu::SyncToken& sync_token() const { return sync_token_; }
  bool verified_flush() { return sync_token_.verified_flush(); }

 private:
  gpu::SyncToken sync_token_;
  const base::PlatformThreadRef context_thread_ref_;
  const scoped_refptr<base::SingleThreadTaskRunner> context_task_runner_;
  viz::ReleaseCallback release_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAILBOX_REF_H_
