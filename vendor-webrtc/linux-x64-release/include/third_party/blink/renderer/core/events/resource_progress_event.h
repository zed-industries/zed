/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_RESOURCE_PROGRESS_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_RESOURCE_PROGRESS_EVENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/events/progress_event.h"

namespace blink {

// ResourceProgressEvent is a non-standard class that is simply a ProgressEvent
// with an additional read-only "url" property containing a string URL. This is
// used by the Chromium NaCl integration to indicate to which resource the
// event applies. This is useful because the NaCl integration will download
// (and translate in the case of PNaCl) multiple binary files. It is not
// constructable by web content at all, and so does not provide the usual
// EventInit pattern for Event construction.
class CORE_EXPORT ResourceProgressEvent final : public ProgressEvent {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ResourceProgressEvent(const AtomicString& type,
                        bool length_computable,
                        double loaded,
                        double total,
                        const String& url);

  const String& url() const;

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  String url_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_RESOURCE_PROGRESS_EVENT_H_
