// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_V8_BINDING_FOR_MODULES_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_V8_BINDING_FOR_MODULES_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class IDBKey;
class IDBKeyPath;
class IDBValue;
class SerializedScriptValue;
class WebBlobInfo;

// Exposed for unit testing:
MODULES_EXPORT v8::Local<v8::Value> DeserializeIDBValue(ScriptState*,
                                                        const IDBValue*);

v8::Local<v8::Value> DeserializeIDBValueArray(
    ScriptState*,
    const Vector<std::unique_ptr<IDBValue>>&);

MODULES_EXPORT bool InjectV8KeyIntoV8Value(v8::Isolate*,
                                           v8::Local<v8::Value> key,
                                           v8::Local<v8::Value>,
                                           const IDBKeyPath&);

// For use by Source/modules/indexeddb (and unit testing):
MODULES_EXPORT bool CanInjectIDBKeyIntoScriptValue(v8::Isolate*,
                                                   const ScriptValue&,
                                                   const IDBKeyPath&);
ScriptValue DeserializeScriptValue(ScriptState*,
                                   SerializedScriptValue*,
                                   const Vector<WebBlobInfo>*);

#if DCHECK_IS_ON()
void AssertPrimaryKeyValidOrInjectable(ScriptState*, const IDBValue*);
#endif

// Used by Indexed DB when converting an explicit value to a key.
// https://w3c.github.io/IndexedDB/#convert-value-to-key
//
// Returns an Invalid key if the conversion is a failure (per spec).
//
// Note that an Array key may contain Invalid members, as the "multi-entry"
// index case allows these, and will filter them out later. Use IsValid() to
// recursively check.
MODULES_EXPORT std::unique_ptr<IDBKey> CreateIDBKeyFromValue(
    v8::Isolate* isolate,
    v8::Local<v8::Value> value,
    ExceptionState& exception_state);

// Used by Indexed DB when generating the primary key for a record that is
// being stored in an object store that uses in-line keys.
// https://w3c.github.io/IndexedDB/#extract-key-from-value
//
// Evaluates the given key path against the script value to produce an
// IDBKey. Returns either:
// * A nullptr, if key path evaluation fails.
// * An Invalid key, if the evaluation yielded a non-key.
// * An IDBKey, otherwise.
//
// Note that an Array key may contain with Invalid members, as the
// "multi-entry" index case allows these, and will filter them out later.
// Use IsValid() to recursively check.
MODULES_EXPORT std::unique_ptr<IDBKey> CreateIDBKeyFromValueAndKeyPath(
    v8::Isolate* isolate,
    v8::Local<v8::Value> v8_value,
    const String& key_path,
    ExceptionState& exception_state);
MODULES_EXPORT std::unique_ptr<IDBKey> CreateIDBKeyFromValueAndKeyPath(
    v8::Isolate* isolate,
    v8::Local<v8::Value> value,
    const IDBKeyPath& key_path,
    ExceptionState& exception_state);

// Used by Indexed DB when generating the index key for a record that is being
// stored.
// https://w3c.github.io/IndexedDB/#extract-key-from-value
//
// Evaluates the given key path against the script value to produce an IDBKey.
// Returns either:
// * A nullptr, if key path evaluation fails.
// * An Invalid key, if the evaluation yielded a non-key.
// * An IDBKey, otherwise.
//
// Note that an Array key may contain Invalid members, as the
// "multi-entry" index case allows these, and will filter them out later. Use
// IsValid() to recursively check.
//
// If evaluating an index's key path which is an array, and the sub-key path
// matches the object store's key path, and that evaluation fails, then a
// None key member will be present in the Array key result. This should only
// occur when the store has a key generator, which would fill in the primary
// key lazily.
MODULES_EXPORT std::unique_ptr<IDBKey> CreateIDBKeyFromValueAndKeyPaths(
    v8::Isolate* isolate,
    v8::Local<v8::Value> value,
    const IDBKeyPath& store_key_path,
    const IDBKeyPath& index_key_path,
    ExceptionState& exception_state);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_V8_BINDING_FOR_MODULES_H_
