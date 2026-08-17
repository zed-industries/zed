/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_BLOB_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_BLOB_H_

#include "third_party/blink/public/mojom/blob/serialized_blob.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "v8/include/v8-local-handle.h"

namespace v8 {
class Isolate;
class Value;
}

namespace blink {

class Blob;

class BLINK_EXPORT WebBlob {
 public:
  ~WebBlob() { Reset(); }

  WebBlob() = default;
  WebBlob(const WebBlob& b) { Assign(b); }
  WebBlob& operator=(const WebBlob& b) {
    Assign(b);
    return *this;
  }

  static WebBlob CreateFromSerializedBlob(mojom::SerializedBlobPtr blob);
  static WebBlob CreateFromFile(v8::Isolate* isolate,
                                const WebString& path,
                                uint64_t size);
  static WebBlob FromV8Value(v8::Isolate* isolate, v8::Local<v8::Value>);

  void Reset();
  void Assign(const WebBlob&);
  WebString Uuid();

  bool IsNull() const { return private_.IsNull(); }

  v8::Local<v8::Value> ToV8Value(v8::Isolate*);

#if INSIDE_BLINK
  WebBlob(Blob*);
  WebBlob& operator=(Blob*);
#endif

 protected:
  WebPrivatePtrForGC<Blob> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_BLOB_H_
