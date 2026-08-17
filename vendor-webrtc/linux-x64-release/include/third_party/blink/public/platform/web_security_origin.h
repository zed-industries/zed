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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SECURITY_ORIGIN_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SECURITY_ORIGIN_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"
#include "url/origin.h"

#if INSIDE_BLINK
#include "base/memory/scoped_refptr.h"
#endif

namespace blink {

class SecurityOrigin;
class WebURL;

class BLINK_PLATFORM_EXPORT WebSecurityOrigin {
 public:
  ~WebSecurityOrigin() { Reset(); }

  WebSecurityOrigin() = default;
  WebSecurityOrigin(const WebSecurityOrigin& s) { Assign(s); }
  WebSecurityOrigin& operator=(const WebSecurityOrigin& s) {
    Assign(s);
    return *this;
  }

  static WebSecurityOrigin CreateFromString(const WebString&);
  static WebSecurityOrigin Create(const WebURL&);

  void Reset();
  void Assign(const WebSecurityOrigin&);

  bool IsNull() const { return private_.IsNull(); }

  WebString Protocol() const;
  WebString Host() const;

  // Like url::Origin::port, this returns the default port for standard URLs
  // with no explicit port set.
  uint16_t Port() const;

  // A unique WebSecurityOrigin is the least privileged WebSecurityOrigin.
  bool IsOpaque() const;

  // Returns true if this WebSecurityOrigin can script objects in the given
  // SecurityOrigin. For example, call this function before allowing
  // script from one security origin to read or write objects from
  // another SecurityOrigin.
  bool CanAccess(const WebSecurityOrigin&) const;

  // Returns true if this WebSecurityOrigin can read content retrieved from
  // the given URL. For example, call this function before allowing script
  // from a given security origin to receive contents from a given URL.
  bool CanRequest(const WebURL&) const;

  // Returns true if this WebSecurityOrigin can display content from the given
  // URL (e.g., in an iframe or as an image). For example, web sites generally
  // cannot display content from the user's files system.
  bool CanDisplay(const WebURL&) const;

  // Returns true if the origin loads resources either from the local
  // machine or over the network from a
  // cryptographically-authenticated origin, as described in
  // https://w3c.github.io/webappsec-secure-contexts/#is-origin-trustworthy.
  bool IsPotentiallyTrustworthy() const;

  // Returns a string representation of the WebSecurityOrigin.  The empty
  // WebSecurityOrigin is represented by "null".  The representation of a
  // non-empty WebSecurityOrigin resembles a standard URL.
  WebString ToString() const;

  // Returns true if this WebSecurityOrigin can access usernames and
  // passwords stored in password manager.
  bool CanAccessPasswordManager() const;

  // This method implements HTML's "same origin" check, which verifies equality
  // of opaque origins, or exact (scheme,host,port) matches. Note that
  // `document.domain` does not come into play for this comparison.
  //
  // This method does not take the "universal access" flag into account. It does
  // take the "local access" flag into account, considering `file:` origins that
  // set the flag to be same-origin with all other `file:` origins that set the
  // flag.
  //
  // https://html.spec.whatwg.org/#same-origin
  bool IsSameOriginWith(const WebSecurityOrigin&) const;

#if INSIDE_BLINK
  WebSecurityOrigin(scoped_refptr<const SecurityOrigin>);
  WebSecurityOrigin& operator=(scoped_refptr<const SecurityOrigin>);
  operator scoped_refptr<const SecurityOrigin>() const;
  const SecurityOrigin* Get() const;
#endif
  WebSecurityOrigin(const url::Origin&);
  operator url::Origin() const;

#if DCHECK_IS_ON()
  bool operator==(const WebSecurityOrigin&) const;
#endif

 private:
  WebPrivatePtrForRefCounted<const SecurityOrigin> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SECURITY_ORIGIN_H_
