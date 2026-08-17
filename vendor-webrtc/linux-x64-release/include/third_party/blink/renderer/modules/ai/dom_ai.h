// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_DOM_AI_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_DOM_AI_H_

#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AI;
class ExecutionContext;

class DOMAI final : public GarbageCollected<DOMAI>,
                    public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  static DOMAI& From(ExecutionContext&);
  static AI* ai(ExecutionContext&);

  explicit DOMAI(ExecutionContext&);
  DOMAI(const DOMAI&) = delete;
  DOMAI& operator=(const DOMAI&) = delete;

  void Trace(Visitor*) const override;

 private:
  AI* ai();

  Member<AI> ai_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_DOM_AI_H_
