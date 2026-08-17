/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_PER_CONTEXT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_PER_CONTEXT_DATA_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "gin/public/context_holder.h"
#include "gin/public/gin_embedders.h"
#include "third_party/blink/renderer/platform/bindings/scoped_persistent.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

class V8DOMActivityLogger;
class V8PerContextData;
struct WrapperTypeInfo;

// Used to hold data that is associated with a single v8::Context object, and
// has a 1:1 relationship with v8::Context.
class PLATFORM_EXPORT V8PerContextData final
    : public GarbageCollected<V8PerContextData> {
 public:
  explicit V8PerContextData(v8::Local<v8::Context>);
  V8PerContextData(const V8PerContextData&) = delete;
  V8PerContextData& operator=(const V8PerContextData&) = delete;

  ~V8PerContextData();

  void Trace(Visitor* visitor) const;
  void Dispose();

  v8::Local<v8::Context> GetContext() { return context_.NewLocal(isolate_); }

  // To create JS Wrapper objects, we create a cache of a 'boiler plate'
  // object, and then simply Clone that object each time we need a new one.
  // This is faster than going through the full object creation process.
  v8::Local<v8::Object> CreateWrapperFromCache(v8::Isolate* isolate,
                                               const WrapperTypeInfo* type) {
    if (auto it = wrapper_boilerplates_.find(type);
        it != wrapper_boilerplates_.end()) {
      v8::Local<v8::Object> obj = it->value.Get(isolate_);
      return obj->Clone(isolate);
    }
    return CreateWrapperFromCacheSlowCase(isolate, type);
  }

  // Returns the interface object that is appropriately initialized (e.g.
  // context-dependent properties are installed).
  v8::Local<v8::Function> ConstructorForType(const WrapperTypeInfo* type) {
    auto it = constructor_map_.find(type);
    return it != constructor_map_.end() ? it->value.Get(isolate_)
                                        : ConstructorForTypeSlowCase(type);
  }

  v8::Local<v8::Object> PrototypeForType(const WrapperTypeInfo*);

  // Gets the constructor and prototype for a type, if they have already been
  // created. Returns true if they exist, and sets the existing values in
  // |prototypeObject| and |interfaceObject|. Otherwise, returns false, and the
  // values are set to empty objects (non-null).
  bool GetExistingConstructorAndPrototypeForType(
      const WrapperTypeInfo*,
      v8::Local<v8::Object>* prototype_object,
      v8::Local<v8::Function>* interface_object);

  V8DOMActivityLogger* ActivityLogger() const { return activity_logger_; }
  void SetActivityLogger(V8DOMActivityLogger* activity_logger) {
    activity_logger_ = activity_logger;
  }

  // Garbage collected classes that use V8PerContextData to hold an instance
  // should subclass Data, and use addData / clearData / getData to manage the
  // instance.
  class PLATFORM_EXPORT Data : public GarbageCollectedMixin {};

  void AddData(const char* key, Data*);
  void ClearData(const char* key);
  Data* GetData(const char* key);

 private:
  v8::Local<v8::Object> CreateWrapperFromCacheSlowCase(v8::Isolate*,
                                                       const WrapperTypeInfo*);
  v8::Local<v8::Function> ConstructorForTypeSlowCase(const WrapperTypeInfo*);

  const raw_ptr<v8::Isolate> isolate_;

  // For each possible type of wrapper, we keep a boilerplate object.
  // The boilerplate is used to create additional wrappers of the same type.
  HeapHashMap<const WrapperTypeInfo*, TraceWrapperV8Reference<v8::Object>>
      wrapper_boilerplates_;

  HeapHashMap<const WrapperTypeInfo*, TraceWrapperV8Reference<v8::Function>>
      constructor_map_;

  std::unique_ptr<gin::ContextHolder> context_holder_;

  ScopedPersistent<v8::Context> context_;

  // This is owned by a static hash map in V8DOMActivityLogger.
  raw_ptr<V8DOMActivityLogger, DanglingUntriaged> activity_logger_;

  using DataMap = HeapHashMap<const char*, Member<Data>>;
  DataMap data_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_PER_CONTEXT_DATA_H_
