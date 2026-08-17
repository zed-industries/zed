// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_PATH_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_PATH_H_

#include <string>
#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-shared.h"

namespace blink {

class BLINK_COMMON_EXPORT IndexedDBKeyPath {
 public:
  IndexedDBKeyPath();  // Defaults to blink::WebIDBKeyPathTypeNull.
  explicit IndexedDBKeyPath(const std::u16string&);
  explicit IndexedDBKeyPath(const std::vector<std::u16string>&);
  IndexedDBKeyPath(const IndexedDBKeyPath& other);
  IndexedDBKeyPath(IndexedDBKeyPath&& other);
  ~IndexedDBKeyPath();
  IndexedDBKeyPath& operator=(const IndexedDBKeyPath& other);
  IndexedDBKeyPath& operator=(IndexedDBKeyPath&& other);

  bool IsNull() const { return type_ == blink::mojom::IDBKeyPathType::Null; }
  bool operator==(const IndexedDBKeyPath& other) const;

  mojom::IDBKeyPathType type() const { return type_; }
  const std::vector<std::u16string>& array() const;
  const std::u16string& string() const;

 private:
  mojom::IDBKeyPathType type_;
  std::u16string string_;
  std::vector<std::u16string> array_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_PATH_H_
