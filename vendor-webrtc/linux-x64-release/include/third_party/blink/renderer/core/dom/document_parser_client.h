// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PARSER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PARSER_CLIENT_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DocumentParserClient : public GarbageCollectedMixin {
 public:
  // This callback is called when all data pushed to parser has been consumed.
  virtual void NotifyParserStopped() = 0;

  void Trace(Visitor* visitor) const override {}

 protected:
  DocumentParserClient() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PARSER_CLIENT_H_
