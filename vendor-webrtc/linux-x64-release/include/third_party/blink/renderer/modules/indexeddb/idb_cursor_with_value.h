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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_CURSOR_WITH_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_CURSOR_WITH_VALUE_H_

#include <memory>

#include "third_party/blink/renderer/modules/indexeddb/idb_cursor.h"
#include "third_party/blink/renderer/modules/indexeddb/indexed_db.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class IDBRequest;
class IDBTransaction;

class IDBCursorWithValue final : public IDBCursor {
  DEFINE_WRAPPERTYPEINFO();

 public:
  IDBCursorWithValue(mojo::PendingAssociatedRemote<mojom::blink::IDBCursor>,
                     mojom::IDBCursorDirection,
                     IDBRequest*,
                     const Source*,
                     IDBTransaction*);
  ~IDBCursorWithValue() override;

  // The value attribute defined in the IDL is simply implemented in IDBCursor
  // (but not exposed via its IDL). This is to make the implementation more
  // simple while matching what the spec says.

  bool IsKeyCursor() const override { return false; }
  bool IsCursorWithValue() const override { return true; }
};

template <>
struct DowncastTraits<IDBCursorWithValue> {
  static bool AllowFrom(const IDBCursor& cursor) {
    return cursor.IsCursorWithValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_CURSOR_WITH_VALUE_H_
