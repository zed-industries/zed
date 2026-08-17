// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXED_DB_DEFAULT_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXED_DB_DEFAULT_MOJOM_TRAITS_H_

#include "base/containers/span.h"
#include "mojo/public/cpp/bindings/array_traits_span.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/indexeddb/indexeddb_key.h"
#include "third_party/blink/public/common/indexeddb/indexeddb_key_range.h"
#include "third_party/blink/public/common/indexeddb/indexeddb_metadata.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-forward.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::IDBDatabaseMetadataDataView,
                 blink::IndexedDBDatabaseMetadata> {
  static int64_t id(const blink::IndexedDBDatabaseMetadata& metadata) {
    return metadata.id;
  }
  static const std::u16string& name(
      const blink::IndexedDBDatabaseMetadata& metadata) {
    return metadata.name;
  }
  static int64_t version(const blink::IndexedDBDatabaseMetadata& metadata) {
    return metadata.version;
  }
  static int64_t max_object_store_id(
      const blink::IndexedDBDatabaseMetadata& metadata) {
    return metadata.max_object_store_id;
  }
  static std::map<int64_t, blink::IndexedDBObjectStoreMetadata> object_stores(
      const blink::IndexedDBDatabaseMetadata& metadata) {
    return metadata.object_stores;
  }
  static bool was_cold_open(const blink::IndexedDBDatabaseMetadata& metadata) {
    return metadata.was_cold_open;
  }
  static bool Read(blink::mojom::IDBDatabaseMetadataDataView data,
                   blink::IndexedDBDatabaseMetadata* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::IDBIndexKeysDataView,
                                        blink::IndexedDBIndexKeys> {
  static int64_t index_id(const blink::IndexedDBIndexKeys& index_keys) {
    return index_keys.id;
  }
  static const std::vector<blink::IndexedDBKey>& index_keys(
      const blink::IndexedDBIndexKeys& index_keys) {
    return index_keys.keys;
  }
  static bool Read(blink::mojom::IDBIndexKeysDataView data,
                   blink::IndexedDBIndexKeys* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::IDBIndexMetadataDataView,
                                        blink::IndexedDBIndexMetadata> {
  static int64_t id(const blink::IndexedDBIndexMetadata& metadata) {
    return metadata.id;
  }
  static const std::u16string& name(
      const blink::IndexedDBIndexMetadata& metadata) {
    return metadata.name;
  }
  static const blink::IndexedDBKeyPath& key_path(
      const blink::IndexedDBIndexMetadata& metadata) {
    return metadata.key_path;
  }
  static bool unique(const blink::IndexedDBIndexMetadata& metadata) {
    return metadata.unique;
  }
  static bool multi_entry(const blink::IndexedDBIndexMetadata& metadata) {
    return metadata.multi_entry;
  }
  static bool Read(blink::mojom::IDBIndexMetadataDataView data,
                   blink::IndexedDBIndexMetadata* out);
};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::IDBKeyDataView, blink::IndexedDBKey> {
  static blink::mojom::IDBKeyDataView::Tag GetTag(
      const blink::IndexedDBKey& key);
  static bool Read(blink::mojom::IDBKeyDataView data, blink::IndexedDBKey* out);
  static const std::vector<blink::IndexedDBKey>& key_array(
      const blink::IndexedDBKey& key) {
    return key.array();
  }
  static base::span<const uint8_t> binary(const blink::IndexedDBKey& key) {
    return base::as_byte_span(key.binary());
  }
  static const std::u16string& string(const blink::IndexedDBKey& key) {
    return key.string();
  }
  static double date(const blink::IndexedDBKey& key) { return key.date(); }
  static double number(const blink::IndexedDBKey& key) { return key.number(); }
  static bool other_invalid(const blink::IndexedDBKey& key) {
    return key.type() == blink::mojom::IDBKeyType::Invalid;
  }
  static bool other_none(const blink::IndexedDBKey& key) {
    return key.type() == blink::mojom::IDBKeyType::None;
  }
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::IDBKeyPathDataView, blink::IndexedDBKeyPath> {
  static blink::mojom::IDBKeyPathDataPtr data(
      const blink::IndexedDBKeyPath& key_path);
  static bool Read(blink::mojom::IDBKeyPathDataView data,
                   blink::IndexedDBKeyPath* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::IDBKeyRangeDataView, blink::IndexedDBKeyRange> {
  static const blink::IndexedDBKey& lower(
      const blink::IndexedDBKeyRange& key_range) {
    return key_range.lower();
  }
  static const blink::IndexedDBKey& upper(
      const blink::IndexedDBKeyRange& key_range) {
    return key_range.upper();
  }
  static bool lower_open(const blink::IndexedDBKeyRange& key_range) {
    return key_range.lower_open();
  }
  static bool upper_open(const blink::IndexedDBKeyRange& key_range) {
    return key_range.upper_open();
  }
  static bool Read(blink::mojom::IDBKeyRangeDataView data,
                   blink::IndexedDBKeyRange* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::IDBObjectStoreMetadataDataView,
                 blink::IndexedDBObjectStoreMetadata> {
  static int64_t id(const blink::IndexedDBObjectStoreMetadata& metadata) {
    return metadata.id;
  }
  static const std::u16string& name(
      const blink::IndexedDBObjectStoreMetadata& metadata) {
    return metadata.name;
  }
  static const blink::IndexedDBKeyPath& key_path(
      const blink::IndexedDBObjectStoreMetadata& metadata) {
    return metadata.key_path;
  }
  static bool auto_increment(
      const blink::IndexedDBObjectStoreMetadata& metadata) {
    return metadata.auto_increment;
  }
  static int64_t max_index_id(
      const blink::IndexedDBObjectStoreMetadata& metadata) {
    return metadata.max_index_id;
  }
  static std::map<int64_t, blink::IndexedDBIndexMetadata> indexes(
      const blink::IndexedDBObjectStoreMetadata& metadata) {
    return metadata.indexes;
  }
  static bool Read(blink::mojom::IDBObjectStoreMetadataDataView data,
                   blink::IndexedDBObjectStoreMetadata* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXED_DB_DEFAULT_MOJOM_TRAITS_H_
