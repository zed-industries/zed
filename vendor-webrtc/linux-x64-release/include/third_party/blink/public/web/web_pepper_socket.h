/*
 * Copyright (C) 2011, 2012 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PEPPER_SOCKET_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PEPPER_SOCKET_H_

#include <memory>
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

class WebArrayBuffer;
class WebDocument;
class WebPepperSocketClient;
class WebURL;

class BLINK_EXPORT WebPepperSocket {
 public:
  enum CloseEventCode {
    kCloseEventCodeNotSpecified = -1,
    kCloseEventCodeNormalClosure = 1000,
    kCloseEventCodeGoingAway = 1001,
    kCloseEventCodeProtocolError = 1002,
    kCloseEventCodeUnsupportedData = 1003,
    kCloseEventCodeFrameTooLarge = 1004,
    kCloseEventCodeNoStatusRcvd = 1005,
    kCloseEventCodeAbnormalClosure = 1006,
    kCloseEventCodeInvalidFramePayloadData = 1007,
    kCloseEventCodePolicyViolation = 1008,
    kCloseEventCodeMessageTooBig = 1009,
    kCloseEventCodeMandatoryExt = 1010,
    kCloseEventCodeInternalError = 1011,
    kCloseEventCodeTLSHandshake = 1015,
    kCloseEventCodeMinimumUserDefined = 3000,
    kCloseEventCodeMaximumUserDefined = 4999
  };

  static std::unique_ptr<WebPepperSocket> Create(const WebDocument&,
                                                 WebPepperSocketClient*);
  virtual ~WebPepperSocket() = default;

  virtual void Connect(const WebURL&, const WebString& protocol) = 0;
  virtual WebString Subprotocol() = 0;
  virtual bool SendText(const WebString&) = 0;
  virtual bool SendArrayBuffer(const WebArrayBuffer&) = 0;
  virtual void Close(int code, const WebString& reason) = 0;
  virtual void Fail(const WebString& reason) = 0;
  virtual void Disconnect() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PEPPER_SOCKET_H_
