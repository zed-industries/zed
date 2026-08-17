// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_GLOBAL_INDEXED_DB_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_GLOBAL_INDEXED_DB_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class IDBFactory;
class LocalDOMWindow;
class WorkerGlobalScope;

class GlobalIndexedDB {
  STATIC_ONLY(GlobalIndexedDB);

 public:
  static IDBFactory* indexedDB(LocalDOMWindow&);
  static IDBFactory* indexedDB(WorkerGlobalScope&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_GLOBAL_INDEXED_DB_H_
