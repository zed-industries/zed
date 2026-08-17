/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_KEY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_KEY_H_

#include <memory>
#include <utility>

#include "base/check_op.h"
#include "base/memory/ptr_util.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-shared.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8-forward.h"

namespace blink {
class ScriptState;

// An IndexedDB primary or index key.
//
// The IndexedDB backing store regards script values written as object store
// record values as fairly opaque data (see IDBValue and IDBValueWrapping).
// However, it needs a fair amount of visibility into script values used as
// primary keys and index keys. For this reason, keys are represented using a
// dedicated data type that fully exposes its contents to the backing store.
class MODULES_EXPORT IDBKey {
  USING_FAST_MALLOC(IDBKey);

 public:
  typedef Vector<std::unique_ptr<IDBKey>> KeyArray;

  static std::unique_ptr<IDBKey> CreateInvalid() {
    return base::WrapUnique(new IDBKey());
  }

  static std::unique_ptr<IDBKey> CreateNone() {
    return base::WrapUnique(new IDBKey(mojom::IDBKeyType::None));
  }

  static std::unique_ptr<IDBKey> CreateNumber(double number) {
    return base::WrapUnique(new IDBKey(mojom::IDBKeyType::Number, number));
  }

  static std::unique_ptr<IDBKey> CreateBinary(
      scoped_refptr<base::RefCountedData<Vector<char>>> binary) {
    return base::WrapUnique(new IDBKey(std::move(binary)));
  }

  static std::unique_ptr<IDBKey> CreateString(const String& string) {
    return base::WrapUnique(new IDBKey(string));
  }

  static std::unique_ptr<IDBKey> CreateDate(double date) {
    return base::WrapUnique(new IDBKey(mojom::IDBKeyType::Date, date));
  }

  static std::unique_ptr<IDBKey> CreateArray(KeyArray array) {
    return base::WrapUnique(new IDBKey(std::move(array)));
  }

  static std::unique_ptr<IDBKey> Clone(const std::unique_ptr<IDBKey>& rkey_in) {
    return IDBKey::Clone(rkey_in.get());
  }

  static std::unique_ptr<IDBKey> Clone(const IDBKey* rkey);

  // Disallow copy and assign.
  IDBKey(const IDBKey&) = delete;
  IDBKey& operator=(const IDBKey&) = delete;

  ~IDBKey();

  mojom::IDBKeyType GetType() const { return type_; }
  bool IsValid() const;

  const KeyArray& Array() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Array);
    return array_;
  }

  scoped_refptr<base::RefCountedData<Vector<char>>> Binary() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Binary);
    return binary_;
  }

  const String& GetString() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::String);
    return string_;
  }

  double Date() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Date);
    return number_;
  }

  double Number() const {
    DCHECK_EQ(type_, mojom::IDBKeyType::Number);
    return number_;
  }

  int Compare(const IDBKey* other) const;
  bool IsLessThan(const IDBKey* other) const;
  bool IsEqual(const IDBKey* other) const;
  size_t SizeEstimate() const { return size_estimate_; }

  v8::Local<v8::Value> ToV8(ScriptState*) const;

  // Returns a new key array with invalid keys and duplicates removed.
  //
  // The items in the key array are moved out of the given IDBKey, which must be
  // an array. For this reason, the method is a static method that receives its
  // argument via an std::unique_ptr.
  //
  // The return value will be pasesd to the backing store, which requires
  // Web types. Returning the correct types directly avoids copying later on
  // (wasted CPU cycles and code size).
  static Vector<std::unique_ptr<IDBKey>> ToMultiEntryArray(
      std::unique_ptr<IDBKey> array_key);

 private:
  IDBKey();
  explicit IDBKey(mojom::IDBKeyType type);
  IDBKey(mojom::IDBKeyType type, double number);
  explicit IDBKey(const String& value);
  explicit IDBKey(scoped_refptr<base::RefCountedData<Vector<char>>> value);
  explicit IDBKey(KeyArray key_array);

  mojom::IDBKeyType type_;
  KeyArray array_;
  scoped_refptr<base::RefCountedData<Vector<char>>> binary_;
  const String string_;
  const double number_ = 0;

  // Initialized in IDBKey constructors based on key type and value size (see
  // idb_key.cc).  Returned via SizeEstimate() and used in IndexedDB code to
  // verify that a given key is small enough to pass over IPC.
  size_t size_estimate_;
};

// An index id, and corresponding set of keys to insert.
struct IDBIndexKeys {
  int64_t id;
  Vector<std::unique_ptr<IDBKey>> keys;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_KEY_H_
