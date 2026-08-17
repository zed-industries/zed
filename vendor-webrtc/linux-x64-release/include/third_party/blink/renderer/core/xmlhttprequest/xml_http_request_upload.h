/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_UPLOAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_UPLOAD_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/xmlhttprequest/xml_http_request.h"
#include "third_party/blink/renderer/core/xmlhttprequest/xml_http_request_event_target.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ExecutionContext;

class XMLHttpRequestUpload final : public XMLHttpRequestEventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XMLHttpRequestUpload(XMLHttpRequest*);

  XMLHttpRequest* XmlHttpRequest() const { return xml_http_request_.Get(); }

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void DispatchEventAndLoadEnd(const AtomicString&, bool, uint64_t, uint64_t);
  void DispatchProgressEvent(uint64_t, uint64_t);

  void HandleRequestError(const AtomicString&);

  void Trace(Visitor*) const override;

 private:
  Member<XMLHttpRequest> xml_http_request_;

  // Last progress event values; used when issuing the
  // required 'progress' event on a request error or abort.
  uint64_t last_bytes_sent_;
  uint64_t last_total_bytes_to_be_sent_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XMLHTTPREQUEST_XML_HTTP_REQUEST_UPLOAD_H_
