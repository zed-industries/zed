// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_RANGE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_RANGE_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/indexeddb/indexeddb_key.h"

namespace blink {

class BLINK_COMMON_EXPORT IndexedDBKeyRange {
 public:
  IndexedDBKeyRange();
  explicit IndexedDBKeyRange(const blink::IndexedDBKey& key);
  IndexedDBKeyRange(const blink::IndexedDBKey& lower,
                    const blink::IndexedDBKey& upper,
                    bool lower_open,
                    bool upper_open);
  IndexedDBKeyRange(const IndexedDBKeyRange& other);
  ~IndexedDBKeyRange();
  IndexedDBKeyRange& operator=(const IndexedDBKeyRange& other);

  const blink::IndexedDBKey& lower() const { return lower_; }
  const blink::IndexedDBKey& upper() const { return upper_; }
  bool lower_open() const { return lower_open_; }
  bool upper_open() const { return upper_open_; }

  bool IsOnlyKey() const;
  bool IsEmpty() const;

 private:
  blink::IndexedDBKey lower_ = blink::IndexedDBKey(mojom::IDBKeyType::None);
  blink::IndexedDBKey upper_ = blink::IndexedDBKey(mojom::IDBKeyType::None);
  bool lower_open_ = false;
  bool upper_open_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INDEXEDDB_INDEXEDDB_KEY_RANGE_H_
