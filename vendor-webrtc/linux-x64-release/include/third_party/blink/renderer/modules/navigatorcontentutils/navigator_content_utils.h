/*
 * Copyright (C) 2011, Google Inc. All rights reserved.
 * Copyright (C) 2012, Samsung Electronics. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NAVIGATORCONTENTUTILS_NAVIGATOR_CONTENT_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NAVIGATORCONTENTUTILS_NAVIGATOR_CONTENT_UTILS_H_

#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class LocalFrame;
class NavigatorContentUtilsClient;
enum class ProtocolHandlerSecurityLevel;

// Verify custom handler schemes for errors as described in steps 1 and 2
// https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers.
// Callers should surface an error with |error_message| if it returns false.
bool VerifyCustomHandlerScheme(const String& scheme,
                               String& error_message,
                               ProtocolHandlerSecurityLevel security_level);

// Verify custom handler URLs for syntax errors as described in step 3
// https://html.spec.whatwg.org/multipage/system-state.html#custom-handlers.
// Callers should surface an error with |error_message| if it returns false.
// |full_url| is calculated URL that needs to resolve to a valid URL.
// |base_url| is used for the error message and is generally the Document URL.
// |user_url| is the URL provided by the user, which may be relative.
bool VerifyCustomHandlerURLSyntax(const KURL& full_url,
                                  const KURL& base_url,
                                  const String& user_url,
                                  String& error_message);

// It is owned by Navigator, and an instance is created lazily by calling
// NavigatorContentUtils::From() via [register/unregister]ProtocolHandler.
class MODULES_EXPORT NavigatorContentUtils final
    : public GarbageCollected<NavigatorContentUtils>,
      public Supplement<Navigator> {
 public:
  static const char kSupplementName[];

  NavigatorContentUtils(Navigator& navigator,
                        NavigatorContentUtilsClient* client)
      : Supplement<Navigator>(navigator), client_(client) {}
  ~NavigatorContentUtils();

  static void registerProtocolHandler(Navigator&,
                                      const String& scheme,
                                      const String& url,
                                      ExceptionState&);
  static void unregisterProtocolHandler(Navigator&,
                                        const String& scheme,
                                        const String& url,
                                        ExceptionState&);

  void Trace(Visitor*) const override;

  void SetClientForTest(NavigatorContentUtilsClient* client) {
    client_ = client;
  }

 private:
  static NavigatorContentUtils& From(Navigator&, LocalFrame& frame);

  NavigatorContentUtilsClient* Client() { return client_.Get(); }

  Member<NavigatorContentUtilsClient> client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NAVIGATORCONTENTUTILS_NAVIGATOR_CONTENT_UTILS_H_
