// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_RECORD_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_RECORD_ARRAY_H_

#include <memory>

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8-local-handle.h"
#include "v8/include/v8-value.h"

namespace blink {
class IDBKey;
class IDBValue;
class ScriptState;

// Stores results from `getAll()`, `getAllKeys()` or `getAllRecords()` for later
// serialization to JavaScript.  Uses 3 parallel vectors to store a record:
// `primary_keys`, `values` and `index_keys`. This allows `IDBRecordArray` to
// transfer ownership of one of its members to another class.  For example,
// `IDBKey` can take ownership of `primary_keys` for `getAllKeys()`.
// `IDBRequestLoader` can temporarily take ownership of `values` while loading
// values.
//
// Depending on the type of get all request, some members may remain empty.
// For example, `getAll()` populates `values` only.  `getAllKeys()` populates
// `primary_keys` only.  `getAllRecords()` populates all 3 vectors for
// `IDBIndex`.  For `IDBObjectStore::getAllRecords()`, populates `primary_keys`
// and `values` only.
struct MODULES_EXPORT IDBRecordArray {
  IDBRecordArray();
  ~IDBRecordArray();

  IDBRecordArray(IDBRecordArray&& source);
  IDBRecordArray& operator=(IDBRecordArray&& source);

  IDBRecordArray(const IDBRecordArray&) = delete;
  IDBRecordArray& operator=(const IDBRecordArray&) = delete;

  // Converts `IDBRecordArray` to a V8 array of `IDBRecord` objects.  Extracts
  // the `primary_keys`, `values` and `index_keys` from `source_records`, moving
  // them into the V8 array of `IDBRecord` objects.
  static v8::Local<v8::Value> ToV8(ScriptState* script_state,
                                   IDBRecordArray source_records);

  // Clears all vector members.
  void clear();

  WTF::Vector<std::unique_ptr<IDBKey>> primary_keys;
  WTF::Vector<std::unique_ptr<IDBValue>> values;
  WTF::Vector<std::unique_ptr<IDBKey>> index_keys;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_RECORD_ARRAY_H_
