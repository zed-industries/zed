// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_INDEXED_DB_BLINK_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_INDEXED_DB_BLINK_MOJOM_TRAITS_H_

#include <stdint.h>

#include <memory>

#include "base/containers/span.h"
#include "mojo/public/cpp/bindings/map_traits_wtf_hash_map.h"
#include "third_party/blink/public/mojom/blob/blob.mojom-blink.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_transfer_token.mojom-blink.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_metadata.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace mojo {

template <>
struct MODULES_EXPORT StructTraits<blink::mojom::IDBDatabaseMetadataDataView,
                                   blink::IDBDatabaseMetadata> {
  static int64_t id(const blink::IDBDatabaseMetadata& metadata) {
    return metadata.id;
  }
  static WTF::String name(const blink::IDBDatabaseMetadata& metadata) {
    if (metadata.name.IsNull())
      return g_empty_string;
    return metadata.name;
  }
  static int64_t version(const blink::IDBDatabaseMetadata& metadata) {
    return metadata.version;
  }
  static int64_t max_object_store_id(
      const blink::IDBDatabaseMetadata& metadata) {
    return metadata.max_object_store_id;
  }
  static const HashMap<int64_t, scoped_refptr<blink::IDBObjectStoreMetadata>>&
  object_stores(const blink::IDBDatabaseMetadata& metadata) {
    return metadata.object_stores;
  }
  static bool was_cold_open(const blink::IDBDatabaseMetadata& metadata) {
    return metadata.was_cold_open;
  }
  static bool Read(blink::mojom::IDBDatabaseMetadataDataView data,
                   blink::IDBDatabaseMetadata* out);
};

template <>
struct MODULES_EXPORT
    StructTraits<blink::mojom::IDBIndexKeysDataView, blink::IDBIndexKeys> {
  static int64_t index_id(const blink::IDBIndexKeys& index_keys) {
    return index_keys.id;
  }
  static const Vector<std::unique_ptr<blink::IDBKey>>& index_keys(
      const blink::IDBIndexKeys& index_keys) {
    return index_keys.keys;
  }
  static bool Read(blink::mojom::IDBIndexKeysDataView data,
                   blink::IDBIndexKeys* out);
};

template <>
struct MODULES_EXPORT StructTraits<blink::mojom::IDBIndexMetadataDataView,
                                   scoped_refptr<blink::IDBIndexMetadata>> {
  static int64_t id(const scoped_refptr<blink::IDBIndexMetadata>& metadata) {
    return metadata->id;
  }
  static WTF::String name(
      const scoped_refptr<blink::IDBIndexMetadata>& metadata) {
    if (metadata->name.IsNull())
      return g_empty_string;
    return metadata->name;
  }
  static const blink::IDBKeyPath& key_path(
      const scoped_refptr<blink::IDBIndexMetadata>& metadata) {
    return metadata->key_path;
  }
  static bool unique(const scoped_refptr<blink::IDBIndexMetadata>& metadata) {
    return metadata->unique;
  }
  static bool multi_entry(
      const scoped_refptr<blink::IDBIndexMetadata>& metadata) {
    return metadata->multi_entry;
  }
  static bool Read(blink::mojom::IDBIndexMetadataDataView data,
                   scoped_refptr<blink::IDBIndexMetadata>* out);
};

template <>
struct MODULES_EXPORT
    UnionTraits<blink::mojom::IDBKeyDataView, std::unique_ptr<blink::IDBKey>> {
  static blink::mojom::IDBKeyDataView::Tag GetTag(
      const std::unique_ptr<blink::IDBKey>& key);
  static bool Read(blink::mojom::IDBKeyDataView data,
                   std::unique_ptr<blink::IDBKey>* out);
  static const Vector<std::unique_ptr<blink::IDBKey>>& key_array(
      const std::unique_ptr<blink::IDBKey>& key);
  static base::span<const uint8_t> binary(
      const std::unique_ptr<blink::IDBKey>& key);
  static const WTF::String string(const std::unique_ptr<blink::IDBKey>& key) {
    String key_string = key->GetString();
    if (key_string.IsNull())
      key_string = g_empty_string;
    return key_string;
  }
  static double date(const std::unique_ptr<blink::IDBKey>& key) {
    return key->Date();
  }
  static double number(const std::unique_ptr<blink::IDBKey>& key) {
    return key->Number();
  }
  static bool other_invalid(const std::unique_ptr<blink::IDBKey>& key) {
    return key->GetType() == blink::mojom::IDBKeyType::Invalid;
  }
  static bool other_none(const std::unique_ptr<blink::IDBKey>& key) {
    return key->GetType() == blink::mojom::IDBKeyType::None;
  }
};

template <>
struct MODULES_EXPORT StructTraits<blink::mojom::IDBValueDataView,
                                   std::unique_ptr<blink::IDBValue>> {
  static base::span<const uint8_t> bits(
      const std::unique_ptr<blink::IDBValue>& input);
  static Vector<blink::mojom::blink::IDBExternalObjectPtr> external_objects(
      const std::unique_ptr<blink::IDBValue>& input);
  static bool Read(blink::mojom::IDBValueDataView data,
                   std::unique_ptr<blink::IDBValue>* out);
};

template <>
struct MODULES_EXPORT
    StructTraits<blink::mojom::IDBKeyPathDataView, blink::IDBKeyPath> {
  static blink::mojom::blink::IDBKeyPathDataPtr data(
      const blink::IDBKeyPath& key_path);
  static bool Read(blink::mojom::IDBKeyPathDataView data,
                   blink::IDBKeyPath* out);
};

template <>
struct MODULES_EXPORT
    StructTraits<blink::mojom::IDBObjectStoreMetadataDataView,
                 scoped_refptr<blink::IDBObjectStoreMetadata>> {
  static int64_t id(
      const scoped_refptr<blink::IDBObjectStoreMetadata>& metadata) {
    return metadata->id;
  }
  static WTF::String name(
      const scoped_refptr<blink::IDBObjectStoreMetadata>& metadata) {
    if (metadata->name.IsNull())
      return g_empty_string;
    return metadata->name;
  }
  static const blink::IDBKeyPath& key_path(
      const scoped_refptr<blink::IDBObjectStoreMetadata>& metadata) {
    return metadata->key_path;
  }
  static bool auto_increment(
      const scoped_refptr<blink::IDBObjectStoreMetadata>& metadata) {
    return metadata->auto_increment;
  }
  static int64_t max_index_id(
      const scoped_refptr<blink::IDBObjectStoreMetadata>& metadata) {
    return metadata->max_index_id;
  }
  static const HashMap<int64_t, scoped_refptr<blink::IDBIndexMetadata>>&
  indexes(const scoped_refptr<blink::IDBObjectStoreMetadata>& metadata) {
    return metadata->indexes;
  }
  static bool Read(blink::mojom::IDBObjectStoreMetadataDataView data,
                   scoped_refptr<blink::IDBObjectStoreMetadata>* out);
};

template <>
struct TypeConverter<blink::mojom::blink::IDBKeyRangePtr,
                     const blink::IDBKeyRange*> {
  static blink::mojom::blink::IDBKeyRangePtr Convert(
      const blink::IDBKeyRange* input);
};

template <>
struct TypeConverter<blink::mojom::blink::IDBKeyRangePtr, blink::IDBKeyRange*> {
  static blink::mojom::blink::IDBKeyRangePtr Convert(blink::IDBKeyRange* input);
};

template <>
struct TypeConverter<blink::IDBKeyRange*, blink::mojom::blink::IDBKeyRangePtr> {
  static blink::IDBKeyRange* Convert(
      const blink::mojom::blink::IDBKeyRangePtr& input);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_INDEXED_DB_BLINK_MOJOM_TRAITS_H_
