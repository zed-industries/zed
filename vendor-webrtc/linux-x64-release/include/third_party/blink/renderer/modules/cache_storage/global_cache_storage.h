// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_GLOBAL_CACHE_STORAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_GLOBAL_CACHE_STORAGE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CacheStorage;
class ExceptionState;
class ExecutionContext;
class LocalDOMWindow;
class WorkerGlobalScope;

class GlobalCacheStorage {
  STATIC_ONLY(GlobalCacheStorage);

 public:
  static bool CanCreateCacheStorage(ExecutionContext*, ExceptionState&);

  static CacheStorage* caches(LocalDOMWindow&, ExceptionState&);
  static CacheStorage* caches(WorkerGlobalScope&, ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_GLOBAL_CACHE_STORAGE_H_
