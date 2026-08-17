/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2008, 2009, 2010, 2011 Google Inc. All rights reserved.
 * Copyright (C) 2011 Igalia S.L.
 * Copyright (C) 2011 Motorola Mobility. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_STYLED_MARKUP_SERIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_STYLED_MARKUP_SERIALIZER_H_

#include "third_party/blink/renderer/core/dom/node_traversal.h"
#include "third_party/blink/renderer/core/editing/editing_strategy.h"
#include "third_party/blink/renderer/core/editing/editing_style.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/core/editing/serializers/create_markup_options.h"
#include "third_party/blink/renderer/core/editing/serializers/styled_markup_accumulator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

template <typename... ElementTypes>
inline Node* FirstAncestorOfTypes(const Node& current);
template <typename Strategy>
class StyledMarkupSerializer final {
  STACK_ALLOCATED();

 public:
  StyledMarkupSerializer(const PositionTemplate<Strategy>& start,
                         const PositionTemplate<Strategy>& end,
                         Node* highest_node_to_be_serialized,
                         const CreateMarkupOptions& options);

  String CreateMarkup();

 private:
  bool ShouldAnnotate() const {
    return options_.ShouldAnnotateForInterchange();
  }

  bool DetermineParentTagAndUpdateLastClosed(Node* first_node, Node* past_end);

  const PositionTemplate<Strategy> start_;
  const PositionTemplate<Strategy> end_;
  Node* const highest_node_to_be_serialized_;
  const CreateMarkupOptions options_;
  Node* last_closed_;
  EditingStyle* wrapping_style_;
};

extern template class StyledMarkupSerializer<EditingStrategy>;
extern template class StyledMarkupSerializer<EditingInFlatTreeStrategy>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_STYLED_MARKUP_SERIALIZER_H_
