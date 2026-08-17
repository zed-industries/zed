/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_METADATA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_METADATA_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_key_path.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// The lifecycle of the IndexedDB metadata objects defined below is managed by
// reference counting (RefPtr). We don't have to worry about cycles because
// these objects form a tree with the hierarchy shown below.
//     IDBDatabaseMetadata -> IDBObjectStoreMetadata -> IDBIndexMetadata

class IDBIndexMetadata : public RefCounted<IDBIndexMetadata> {
  USING_FAST_MALLOC(IDBIndexMetadata);

 public:
  static constexpr int64_t kInvalidId = -1;

  IDBIndexMetadata();
  IDBIndexMetadata(const String& name,
                   int64_t id,
                   const IDBKeyPath&,
                   bool unique,
                   bool multi_entry);

  static scoped_refptr<IDBIndexMetadata> Create();

  String name;
  int64_t id;
  IDBKeyPath key_path;
  bool unique;
  bool multi_entry;
};

class MODULES_EXPORT IDBObjectStoreMetadata
    : public RefCounted<IDBObjectStoreMetadata> {
  USING_FAST_MALLOC(IDBObjectStoreMetadata);

 public:
  static constexpr int64_t kInvalidId = -1;

  IDBObjectStoreMetadata();
  IDBObjectStoreMetadata(const String& name,
                         int64_t id,
                         const IDBKeyPath&,
                         bool auto_increment,
                         int64_t max_index_id);

  static scoped_refptr<IDBObjectStoreMetadata> Create();

  // Creates a deep copy of the object metadata, which includes copies of index
  // metadata items.
  scoped_refptr<IDBObjectStoreMetadata> CreateCopy() const;

  String name;
  int64_t id;
  IDBKeyPath key_path;
  bool auto_increment;
  int64_t max_index_id;
  HashMap<int64_t, scoped_refptr<IDBIndexMetadata>> indexes;
};

struct MODULES_EXPORT IDBDatabaseMetadata {
  DISALLOW_NEW();

  // FIXME: These can probably be collapsed into 0.
  enum { kNoVersion = -1, kDefaultVersion = 0 };

  IDBDatabaseMetadata();
  IDBDatabaseMetadata(const String& name,
                      int64_t id,
                      int64_t version,
                      int64_t max_object_store_id,
                      bool was_cold_open);

  // Overwrites the database metadata, but does not change the object store and
  // index metadata.
  void CopyFrom(const IDBDatabaseMetadata&);

  String name;
  int64_t id;
  int64_t version;
  int64_t max_object_store_id;
  HashMap<int64_t, scoped_refptr<IDBObjectStoreMetadata>> object_stores;
  bool was_cold_open;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_METADATA_H_
