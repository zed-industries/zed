/*
 * Copyright (C) 2013 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_EVENT_TARGET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_EVENT_TARGET_H_

#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class XMLHttpRequestEventTarget : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  DEFINE_ATTRIBUTE_EVENT_LISTENER(abort, kAbort)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(load, kLoad)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadend, kLoadend)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(loadstart, kLoadstart)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(progress, kProgress)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(timeout, kTimeout)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_EVENT_TARGET_H_
