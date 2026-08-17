// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_HTML_IFRAME_ELEMENT_PAYMENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_HTML_IFRAME_ELEMENT_PAYMENTS_H_

namespace blink {

class HTMLIFrameElement;
class QualifiedName;

class HTMLIFrameElementPayments final {
 public:
  static bool FastHasAttribute(const HTMLIFrameElement&, const QualifiedName&);

 private:
  HTMLIFrameElementPayments() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PAYMENTS_HTML_IFRAME_ELEMENT_PAYMENTS_H_
