// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_TEST_HELPER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_value.h"
#include "v8/include/v8.h"

namespace blink {

std::unique_ptr<IDBValue> CreateNullIDBValueForTesting(v8::Isolate*);

// The created value is an array of true. If create_wrapped_value is true, the
// IDBValue's byte array will be wrapped in a Blob, otherwise it will not be.
std::unique_ptr<IDBValue> CreateIDBValueForTesting(v8::Isolate*,
                                                   bool create_wrapped_value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_TEST_HELPER_H_
