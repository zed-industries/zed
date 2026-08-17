/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_KEY_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_KEY_RANGE_H_

#include <memory>
#include <utility>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_key.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class ScriptState;
class ScriptValue;

class MODULES_EXPORT IDBKeyRange final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum LowerBoundType { kLowerBoundOpen, kLowerBoundClosed };
  enum UpperBoundType { kUpperBoundOpen, kUpperBoundClosed };

  static IDBKeyRange* Create(const IDBKey* key) {
    std::unique_ptr<IDBKey> lower_key = IDBKey::Clone(key);
    std::unique_ptr<IDBKey> upper_key = IDBKey::Clone(key);
    return IDBKeyRange::Create(std::move(lower_key), std::move(upper_key),
                               kLowerBoundClosed, kUpperBoundClosed);
  }

  static IDBKeyRange* Create(std::unique_ptr<IDBKey> lower,
                             std::unique_ptr<IDBKey> upper,
                             LowerBoundType lower_type,
                             UpperBoundType upper_type) {
    if ((!lower || !lower->IsValid()) && (!upper || !upper->IsValid()))
      return nullptr;

    IDBKey* upper_compressed = upper.get();
    return MakeGarbageCollected<IDBKeyRange>(std::move(lower), upper_compressed,
                                             std::move(upper), lower_type,
                                             upper_type);
  }

  IDBKeyRange(std::unique_ptr<IDBKey> lower,
              IDBKey* upper,
              std::unique_ptr<IDBKey> upper_if_distinct,
              LowerBoundType lower_type,
              UpperBoundType upper_type);

  // Null if the script value is null or undefined, the range if it is one,
  // otherwise tries to convert to a key and throws if it fails.
  static IDBKeyRange* FromScriptValue(ExecutionContext*,
                                      const ScriptValue&,
                                      ExceptionState&);

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
  }

  // Implement the IDBKeyRange IDL
  IDBKey* Lower() const { return lower_.get(); }
  IDBKey* Upper() const { return upper_; }

  ScriptValue LowerValue(ScriptState*) const;
  ScriptValue UpperValue(ScriptState*) const;
  bool lowerOpen() const { return lower_type_ == kLowerBoundOpen; }
  bool upperOpen() const { return upper_type_ == kUpperBoundOpen; }

  static IDBKeyRange* only(ScriptState*,
                           const ScriptValue& key,
                           ExceptionState&);
  static IDBKeyRange* lowerBound(ScriptState*,
                                 const ScriptValue& bound,
                                 bool open,
                                 ExceptionState&);
  static IDBKeyRange* upperBound(ScriptState*,
                                 const ScriptValue& bound,
                                 bool open,
                                 ExceptionState&);
  static IDBKeyRange* bound(ScriptState*,
                            const ScriptValue& lower,
                            const ScriptValue& upper,
                            bool lower_open,
                            bool upper_open,
                            ExceptionState&);

  static IDBKeyRange* only(std::unique_ptr<IDBKey> value, ExceptionState&);

  bool includes(ScriptState*, const ScriptValue& key, ExceptionState&);

 private:
  // IDBKeyRange has two possible internal representations of keys.
  //
  // The normal representation uses distinct IDBKey instances for the lower and
  // upper keys. In this case, upper_if_distinct_ owns the upper key, and upper_
  // points to it.
  //
  // The compressed representation uses the same IDBKey instance for the lower
  // and upper keys, which are equal. This representation is used in the (fairly
  // common) case where a range is created out of a single key. In this case,
  // upper_if_distinct_ is null, and upper_ points to the same IDBKey instance
  // as lower_.
  //
  // The two representations are an implementation detail, and should not be
  // visible to the class' consumers.
  //
  // It may be tempting to think that a nullptr upper_if_distinct_ implies a
  // compressed representation. However, ranges without an upper key
  // (open to infinity on the right side) have a null upper_if_distinct_, but
  // are not considered compressed, as the left key is different from the right
  // key.

  // Owns the range's lower key, and possibly the upper key.
  std::unique_ptr<IDBKey> lower_;

  // Owns the range's upper key, if not null.
  std::unique_ptr<IDBKey> upper_if_distinct_;

  // Non-owning reference to the range's upper key.
  //
  // Points to either upper_if_distinct_ or lower_, or is null.
  const raw_ptr<IDBKey> upper_;

  const LowerBoundType lower_type_;
  const UpperBoundType upper_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_KEY_RANGE_H_
