// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_RECORD_H_

#include <memory>

#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class IDBKey;
class IDBValue;

// A result from `IDBObjectStore::getAllRecords()` or
// `IDBIndex::getAllRecords()`.
class MODULES_EXPORT IDBRecord final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  IDBRecord(std::unique_ptr<IDBKey> primary_key,
            std::unique_ptr<IDBValue> value,
            std::unique_ptr<IDBKey> index_key);
  ~IDBRecord() final;

  // Disallow copy and assign.
  IDBRecord(const IDBRecord&) = delete;
  IDBRecord& operator=(const IDBRecord&) = delete;

  // For `IDBIndex` records, `key()` returns the index key.  For
  // `IDBObjectStore` records, `key()` returns the primary key, which is the the
  // same value as `primaryKey()`.
  ScriptValue key(ScriptState*);

  ScriptValue primaryKey(ScriptState*);
  ScriptValue value(ScriptState*);

  bool isKeyDirty() const;
  bool isPrimaryKeyDirty() const;
  bool isValueDirty() const;

 private:
  std::unique_ptr<IDBKey> primary_key_;
  std::unique_ptr<IDBValue> value_;

  // Optional.  `nullptr` for `IDBObjectStore::getAllRecords()`.
  std::unique_ptr<IDBKey> index_key_;

  bool key_dirty_ = true;
  bool primary_key_dirty_ = true;
  bool value_dirty_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_RECORD_H_
