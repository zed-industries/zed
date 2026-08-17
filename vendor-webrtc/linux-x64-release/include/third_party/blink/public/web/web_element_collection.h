/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_ELEMENT_COLLECTION_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_ELEMENT_COLLECTION_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

class HTMLCollection;
class WebElement;

// Provides readonly access to some properties of a DOM node.
class BLINK_EXPORT WebElementCollection {
 public:
  ~WebElementCollection() { Reset(); }

  WebElementCollection() : current_(0) {}
  WebElementCollection(const WebElementCollection& n) { Assign(n); }
  WebElementCollection& operator=(const WebElementCollection& n) {
    Assign(n);
    return *this;
  }

  bool IsNull() const { return private_.IsNull(); }
  explicit operator bool() const { return !IsNull(); }

  void Reset();
  void Assign(const WebElementCollection&);

  unsigned length() const;
  WebElement NextItem() const;
  WebElement FirstItem() const;

#if INSIDE_BLINK
  WebElementCollection(HTMLCollection*);
  WebElementCollection& operator=(HTMLCollection*);
#endif

 private:
  WebPrivatePtrForGC<HTMLCollection> private_;
  mutable unsigned current_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_ELEMENT_COLLECTION_H_
