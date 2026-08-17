/*
 * Copyright (C) 2013 Apple Inc. All rights reserved.
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
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CONSOLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CONSOLE_H_

#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ConsoleMessage;
class DocumentLoader;
class LocalFrame;
class ResourceError;
class ResourceResponse;
class SourceLocation;

// FrameConsole takes per-frame console messages and routes them up through the
// Page to the ChromeClient and Inspector.  It's meant as an abstraction
// around ChromeClient calls and the way that Blink core/ can add messages to
// the console.
class CORE_EXPORT FrameConsole final : public GarbageCollected<FrameConsole> {
 public:
  explicit FrameConsole(LocalFrame&);

  void AddMessage(ConsoleMessage*, bool discard_duplicates = false);

  bool AddMessageToStorage(ConsoleMessage*, bool discard_duplicates = false);
  void ReportMessageToClient(mojom::ConsoleMessageSource,
                             mojom::ConsoleMessageLevel,
                             const String& message,
                             SourceLocation*);

  void ReportResourceResponseReceived(DocumentLoader*,
                                      uint64_t request_identifier,
                                      const ResourceResponse&);

  void DidFailLoading(DocumentLoader*,
                      uint64_t request_identifier,
                      const ResourceError&);

  void Trace(Visitor*) const;

 private:
  Member<LocalFrame> frame_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CONSOLE_H_
