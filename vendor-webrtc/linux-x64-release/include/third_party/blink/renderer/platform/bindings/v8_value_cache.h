/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_VALUE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_VALUE_CACHE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/bindings/v8_global_value_map.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class StringCacheMapTraits
    : public V8GlobalValueMapTraits<StringImpl*,
                                    v8::String,
                                    v8::kWeakWithParameter> {
  STATIC_ONLY(StringCacheMapTraits);

 public:
  // Weak traits:
  typedef StringImpl WeakCallbackDataType;
  typedef v8::GlobalValueMap<StringImpl*, v8::String, StringCacheMapTraits>
      MapType;

  static WeakCallbackDataType* WeakCallbackParameter(
      MapType* map,
      StringImpl* key,
      v8::Local<v8::String>& value) {
    return key;
  }
  static void DisposeCallbackData(WeakCallbackDataType* callback_data) {}

  static MapType* MapFromWeakCallbackInfo(
      const v8::WeakCallbackInfo<WeakCallbackDataType>&);

  static StringImpl* KeyFromWeakCallbackInfo(
      const v8::WeakCallbackInfo<WeakCallbackDataType>& data) {
    return data.GetParameter();
  }

  static void OnWeakCallback(
      const v8::WeakCallbackInfo<WeakCallbackDataType>&) {}

  static void Dispose(v8::Isolate*,
                      v8::Global<v8::String> value,
                      StringImpl* key);
  static void DisposeWeak(const v8::WeakCallbackInfo<WeakCallbackDataType>&);
};

class ParkableStringCacheMapTraits
    : public V8GlobalValueMapTraits<ParkableStringImpl*,
                                    v8::String,
                                    v8::kWeakWithParameter> {
  STATIC_ONLY(ParkableStringCacheMapTraits);

 public:
  // Weak traits:
  typedef ParkableStringImpl WeakCallbackDataType;
  typedef v8::GlobalValueMap<ParkableStringImpl*,
                             v8::String,
                             ParkableStringCacheMapTraits>
      MapType;

  static WeakCallbackDataType* WeakCallbackParameter(
      MapType* map,
      ParkableStringImpl* key,
      v8::Local<v8::String>& value) {
    return key;
  }
  static void DisposeCallbackData(WeakCallbackDataType* callback_data) {}

  static MapType* MapFromWeakCallbackInfo(
      const v8::WeakCallbackInfo<WeakCallbackDataType>&);

  static ParkableStringImpl* KeyFromWeakCallbackInfo(
      const v8::WeakCallbackInfo<WeakCallbackDataType>& data) {
    return data.GetParameter();
  }

  static void OnWeakCallback(const v8::WeakCallbackInfo<WeakCallbackDataType>&);

  static void Dispose(v8::Isolate*,
                      v8::Global<v8::String> value,
                      ParkableStringImpl* key);
  static void DisposeWeak(const v8::WeakCallbackInfo<WeakCallbackDataType>&);
};

// String cache helps convert WTF strings (StringImpl*) into v8 strings by
// only creating a v8::String for a particular StringImpl* once and caching it
// for future use. It is held by and can be retrieved from V8PerIsolateData, and
// is cleared when the isolate is destroyed. Entries are removed from the
// backing global value map when weak references to the values are collected.
class PLATFORM_EXPORT StringCache {
  USING_FAST_MALLOC(StringCache);

 public:
  explicit StringCache(v8::Isolate* isolate)
      : string_cache_(isolate), parkable_string_cache_(isolate) {}
  StringCache(const StringCache&) = delete;
  StringCache& operator=(const StringCache&) = delete;

  v8::Local<v8::String> V8ExternalString(v8::Isolate* isolate,
                                         StringImpl* string_impl);
  v8::Local<v8::String> V8ExternalString(v8::Isolate* isolate,
                                         const ParkableString& string);

  void SetReturnValueFromString(v8::ReturnValue<v8::Value> return_value,
                                StringImpl* string_impl);

  void Dispose();

  friend class StringCacheMapTraits;
  friend class ParkableStringCacheMapTraits;

 private:
  v8::Local<v8::String> CreateStringAndInsertIntoCache(v8::Isolate*,
                                                       StringImpl*);
  v8::Local<v8::String> CreateStringAndInsertIntoCache(v8::Isolate*,
                                                       ParkableString);

  StringCacheMapTraits::MapType string_cache_;
  ParkableStringCacheMapTraits::MapType parkable_string_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_VALUE_CACHE_H_
