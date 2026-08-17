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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_DOCUMENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_DOCUMENT_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_document.h"

namespace blink {

class PluginDocument;
class WebPlugin;

// Wraps a WebDocument for full page plugins.
class BLINK_EXPORT WebPluginDocument final : public WebDocument {
 public:
  WebPluginDocument() = default;
  WebPluginDocument(const WebPluginDocument& e) = default;

  WebPluginDocument& operator=(const WebPluginDocument& e) {
    WebNode::Assign(e);
    return *this;
  }
  void Assign(const WebPluginDocument& d) { WebNode::Assign(d); }

  WebPlugin* Plugin();

#if INSIDE_BLINK
  WebPluginDocument(PluginDocument*);
  WebPluginDocument& operator=(PluginDocument*);
  operator PluginDocument*() const;
#endif
};

DECLARE_WEB_NODE_TYPE_CASTS(WebPluginDocument);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_DOCUMENT_H_
