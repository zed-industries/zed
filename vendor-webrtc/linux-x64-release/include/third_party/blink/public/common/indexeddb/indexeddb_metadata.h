// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_METADATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_METADATA_H_

#include <stdint.h>

#include <map>
#include <string>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/indexeddb/indexeddb_key_path.h"

namespace blink {

struct BLINK_COMMON_EXPORT IndexedDBIndexMetadata {
  static const int64_t kInvalidId = -1;

  IndexedDBIndexMetadata();
  IndexedDBIndexMetadata(const std::u16string& name,
                         int64_t id,
                         const blink::IndexedDBKeyPath& key_path,
                         bool unique,
                         bool multi_entry);
  IndexedDBIndexMetadata(const IndexedDBIndexMetadata& other);
  IndexedDBIndexMetadata(IndexedDBIndexMetadata&& other);
  ~IndexedDBIndexMetadata();
  IndexedDBIndexMetadata& operator=(const IndexedDBIndexMetadata& other);
  IndexedDBIndexMetadata& operator=(IndexedDBIndexMetadata&& other);
  bool operator==(const IndexedDBIndexMetadata& other) const;

  std::u16string name;
  int64_t id;
  blink::IndexedDBKeyPath key_path;
  bool unique;
  bool multi_entry;
};

struct BLINK_COMMON_EXPORT IndexedDBObjectStoreMetadata {
  static const int64_t kInvalidId = -1;
  static const int64_t kMinimumIndexId = 30;

  IndexedDBObjectStoreMetadata();
  IndexedDBObjectStoreMetadata(const std::u16string& name,
                               int64_t id,
                               const blink::IndexedDBKeyPath& key_path,
                               bool auto_increment,
                               int64_t max_index_id);
  IndexedDBObjectStoreMetadata(const IndexedDBObjectStoreMetadata& other);
  IndexedDBObjectStoreMetadata(IndexedDBObjectStoreMetadata&& other);
  ~IndexedDBObjectStoreMetadata();
  IndexedDBObjectStoreMetadata& operator=(
      const IndexedDBObjectStoreMetadata& other);
  IndexedDBObjectStoreMetadata& operator=(IndexedDBObjectStoreMetadata&& other);
  bool operator==(const IndexedDBObjectStoreMetadata& other) const;

  std::u16string name;
  int64_t id;
  blink::IndexedDBKeyPath key_path;
  bool auto_increment;
  int64_t max_index_id;

  std::map<int64_t, IndexedDBIndexMetadata> indexes;
};

struct BLINK_COMMON_EXPORT IndexedDBDatabaseMetadata {
  // TODO(jsbell): These can probably be collapsed into 0.
  enum { NO_VERSION = -1, DEFAULT_VERSION = 0 };

  IndexedDBDatabaseMetadata();
  IndexedDBDatabaseMetadata(const std::u16string& name,
                            int64_t id,
                            int64_t version,
                            int64_t max_object_store_id);
  IndexedDBDatabaseMetadata(const IndexedDBDatabaseMetadata& other);
  IndexedDBDatabaseMetadata(IndexedDBDatabaseMetadata&& other);
  ~IndexedDBDatabaseMetadata();
  IndexedDBDatabaseMetadata& operator=(const IndexedDBDatabaseMetadata& other);
  IndexedDBDatabaseMetadata& operator=(IndexedDBDatabaseMetadata&& other);
  bool operator==(const IndexedDBDatabaseMetadata& other) const;

  std::u16string name;
  int64_t id;
  int64_t version;
  int64_t max_object_store_id;

  std::map<int64_t, IndexedDBObjectStoreMetadata> object_stores;

  bool was_cold_open;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_METADATA_H_
