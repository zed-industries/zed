// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_CONSOLE_MESSAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_CONSOLE_MESSAGE_H_

#include <optional>

#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DocumentLoader;
class LocalFrame;
class WorkerThread;
struct WebConsoleMessage;

class CORE_EXPORT ConsoleMessage final
    : public GarbageCollected<ConsoleMessage> {
 public:
  using Source = mojom::blink::ConsoleMessageSource;
  using Level = mojom::blink::ConsoleMessageLevel;

  // This constructor captures current location if available.
  ConsoleMessage(Source,
                 Level,
                 const String& message,
                 const String& url,
                 DocumentLoader*,
                 uint64_t request_identifier);
  // Creates message from WorkerMessageSource.
  ConsoleMessage(Level,
                 const String& message,
                 std::unique_ptr<SourceLocation>,
                 WorkerThread*);
  // Creates a ConsoleMessage from a similar WebConsoleMessage.
  ConsoleMessage(const WebConsoleMessage&, LocalFrame*);
  // If provided, source_location must be non-null.
  ConsoleMessage(Source,
                 Level,
                 const String& message,
                 std::unique_ptr<SourceLocation> source_location =
                     CaptureSourceLocation());
  ~ConsoleMessage();

  SourceLocation* Location() const;
  const String& RequestIdentifier() const;
  double Timestamp() const;
  Source GetSource() const;
  Level GetLevel() const;
  const String& Message() const;
  const String& WorkerId() const;
  LocalFrame* Frame() const;
  Vector<DOMNodeId>& Nodes();
  void SetNodes(LocalFrame*, Vector<DOMNodeId> nodes);
  const std::optional<mojom::blink::ConsoleMessageCategory>& Category() const;
  void SetCategory(mojom::blink::ConsoleMessageCategory category);

  void Trace(Visitor*) const;

 private:
  Source source_;
  Level level_;
  std::optional<mojom::blink::ConsoleMessageCategory> category_;
  String message_;
  std::unique_ptr<SourceLocation> location_;
  String request_identifier_;
  double timestamp_;
  String worker_id_;
  WeakMember<LocalFrame> frame_;
  Vector<DOMNodeId> nodes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_CONSOLE_MESSAGE_H_
