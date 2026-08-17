// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_H_

#include <stddef.h>

#include <string>
#include <vector>

#include "base/check_op.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-shared.h"

namespace blink {

class BLINK_COMMON_EXPORT IndexedDBKey {
 public:
  typedef std::vector<IndexedDBKey> KeyArray;

  // Non-standard limits, selected to avoid breaking real-world use of the API
  // while also preventing buggy (or malicious) code from causing crashes.
  static constexpr size_t kMaximumDepth = 2000;
  static constexpr size_t kMaximumArraySize = 1000000;

  IndexedDBKey();  // Defaults to mojom::IDBKeyType::Invalid.
  explicit IndexedDBKey(mojom::IDBKeyType);  // must be Null or Invalid
  explicit IndexedDBKey(KeyArray array);
  explicit IndexedDBKey(std::string binary);
  explicit IndexedDBKey(std::u16string string);
  IndexedDBKey(double number,
               mojom::IDBKeyType type);  // must be date or number
  IndexedDBKey(const IndexedDBKey& other);
  IndexedDBKey(IndexedDBKey&& other);
  ~IndexedDBKey();
  IndexedDBKey& operator=(const IndexedDBKey& other);

  bool IsValid() const;

  bool IsLessThan(const IndexedDBKey& other) const;
  bool Equals(const IndexedDBKey& other) const;

  mojom::IDBKeyType type() const { return type_; }
  const std::vector<IndexedDBKey>& array() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Array);
    return array_;
  }
  const std::string& binary() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Binary);
    return binary_;
  }
  const std::u16string& string() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::String);
    return string_;
  }
  double date() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Date);
    return number_;
  }
  double number() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Number);
    return number_;
  }

  size_t size_estimate() const { return size_estimate_; }

  // Tests if this array-type key has "holes". Used in cases where a compound
  // key references an auto-generated primary key.
  bool HasHoles() const;

  // Returns a copy of this array-type key, but with "holes" replaced by the
  // given primary key. Used in cases where a compound key references an
  // auto-generated primary key.
  [[nodiscard]] IndexedDBKey FillHoles(const IndexedDBKey&) const;

  std::string DebugString() const;

 private:
  int CompareTo(const IndexedDBKey& other) const;

  mojom::IDBKeyType type_;
  std::vector<IndexedDBKey> array_;
  std::string binary_;
  std::u16string string_;
  double number_ = 0;

  size_t size_estimate_;
};

// An index id, and corresponding set of keys to insert.
struct IndexedDBIndexKeys {
  int64_t id;
  std::vector<IndexedDBKey> keys;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_H_
