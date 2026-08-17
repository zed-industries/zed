/*
 * Copyright (C) 2004 Apple Computer, Inc.  All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TEXT_AFFINITY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TEXT_AFFINITY_H_

#include <iosfwd>
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

enum class TextAffinity {
  kUpstream,
  kDownstream,

  // PositionWithAffiity default affinity is downstream because the callers do
  // not really care (they just want the deep position without regard to line
  // position), and this is cheaper than kUpstream.
  kDefault = kDownstream,

  // Callers who do not know where on the line the position is, but would like
  // kUpstream if at a line break or kDownstream otherwise, need a clear way to
  // specify that. The constructors auto-correct kUpstream to kDownstream if the
  // position is not at a line break.
  kUpstreamIfPossible = kUpstream,
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, TextAffinity);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TEXT_AFFINITY_H_
