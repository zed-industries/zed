// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_AREA_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_AREA_MAP_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class is used to represent the in-memory version of the data in a
// StorageArea. This class is responsible for enforcing a quota, and for
// providing somewhat efficient iteration over all the items in the map via
// GetLength/GetKey/GetItem.
// Any modifications to the data in the map can cause all items to be reordered.
// Nothing is guaranteed about the order of items in the map.
// For the purpose of quota each character in the key and value strings is
// counted as two bytes, even if the actual in-memory representation of the
// string only uses one byte per character.
class MODULES_EXPORT StorageAreaMap {
  USING_FAST_MALLOC(StorageAreaMap);

 public:
  explicit StorageAreaMap(size_t quota);

  unsigned GetLength() const;
  String GetKey(unsigned index) const;
  String GetItem(const String& key) const;

  // Returns false iff quota would be exceeded.
  bool SetItem(const String& key, const String& value, String* old_value);
  void SetItemIgnoringQuota(const String& key, const String& value);
  // Returns fals iff item wasn't found.
  bool RemoveItem(const String& key, String* old_value);

  size_t quota_used() const { return quota_used_; }
  size_t memory_used() const { return memory_used_; }
  size_t quota() const { return quota_; }

 private:
  void ResetKeyIterator() const;
  bool SetItemInternal(const String& key,
                       const String& value,
                       String* old_value,
                       bool check_quota);

  HashMap<String, String> keys_values_;

  // To make iterating over all keys somewhat less inefficient, we keep track of
  // an iterator to and index of the last key returned by GetKey().
  mutable HashMap<String, String>::const_iterator key_iterator_;
  mutable unsigned last_key_index_;

  size_t quota_used_ = 0;
  size_t memory_used_ = 0;
  const size_t quota_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_AREA_MAP_H_
