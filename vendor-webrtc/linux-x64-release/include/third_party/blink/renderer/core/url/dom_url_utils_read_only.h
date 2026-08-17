/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 * Copyright (C) 2012 Motorola Mobility Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_URL_DOM_URL_UTILS_READ_ONLY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_URL_DOM_URL_UTILS_READ_ONLY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class CORE_EXPORT DOMURLUtilsReadOnly {
 public:
  virtual KURL Url() const = 0;
  virtual ~DOMURLUtilsReadOnly() = default;

  // href() returns Url() if it is non-null, or Input() otherwise.
  String href();
  virtual String Input() const = 0;

  static String origin(const KURL&);
  String origin() { return origin(Url()); }

  static String protocol(const KURL& url) { return url.Protocol() + ":"; }
  String protocol() { return protocol(Url()); }

  static String username(const KURL& url) { return url.User().ToString(); }
  String username() { return username(Url()); }

  static String password(const KURL& url) { return url.Pass().ToString(); }
  String password() { return password(Url()); }

  static String host(const KURL&);
  String host() { return host(Url()); }

  static String hostname(const KURL& url) { return url.Host().ToString(); }
  String hostname() { return hostname(Url()); }

  static String port(const KURL&);
  String port() { return port(Url()); }

  static String pathname(const KURL& url) { return url.GetPath().ToString(); }
  String pathname() { return pathname(Url()); }

  static String search(const KURL&);
  String search() { return search(Url()); }

  static String hash(const KURL&);
  String hash() { return hash(Url()); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_URL_DOM_URL_UTILS_READ_ONLY_H_
