// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_BLINK_CATEGORIZED_WORKER_POOL_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_BLINK_CATEGORIZED_WORKER_POOL_DELEGATE_H_

#include "base/functional/callback.h"
#include "cc/raster/categorized_worker_pool.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

class PLATFORM_EXPORT BlinkCategorizedWorkerPoolDelegate
    : public cc::CategorizedWorkerPool::Delegate {
 public:
  BlinkCategorizedWorkerPoolDelegate();
  ~BlinkCategorizedWorkerPoolDelegate() override;

  static BlinkCategorizedWorkerPoolDelegate& Get();

  // cc::CategorizedWorkerPool::Delegate:
  void NotifyThreadWillRun(base::PlatformThreadId tid) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_BLINK_CATEGORIZED_WORKER_POOL_DELEGATE_H_
