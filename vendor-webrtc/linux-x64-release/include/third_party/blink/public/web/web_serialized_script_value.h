/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SERIALIZED_SCRIPT_VALUE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SERIALIZED_SCRIPT_VALUE_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "v8/include/v8-local-handle.h"

namespace v8 {
class Isolate;
class Value;
}

namespace blink {

class SerializedScriptValue;

// FIXME: Should this class be in platform?
class BLINK_EXPORT WebSerializedScriptValue {
 public:
  ~WebSerializedScriptValue() { Reset(); }

  WebSerializedScriptValue() = default;
  WebSerializedScriptValue(const WebSerializedScriptValue& d) { Assign(d); }
  WebSerializedScriptValue& operator=(const WebSerializedScriptValue& d) {
    Assign(d);
    return *this;
  }

  static WebSerializedScriptValue Serialize(v8::Isolate*, v8::Local<v8::Value>);

  // Create a WebSerializedScriptValue that represents a serialization error.
  static WebSerializedScriptValue CreateInvalid();

  void Reset();
  void Assign(const WebSerializedScriptValue&);

  bool IsNull() const { return private_.IsNull(); }

  // Convert the serialized value to a parsed v8 value.
  v8::Local<v8::Value> Deserialize(v8::Isolate*);

#if INSIDE_BLINK
  WebSerializedScriptValue(scoped_refptr<SerializedScriptValue>);
  WebSerializedScriptValue& operator=(scoped_refptr<SerializedScriptValue>);
  operator scoped_refptr<SerializedScriptValue>() const;
#endif

 private:
  WebPrivatePtrForRefCounted<SerializedScriptValue> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SERIALIZED_SCRIPT_VALUE_H_
