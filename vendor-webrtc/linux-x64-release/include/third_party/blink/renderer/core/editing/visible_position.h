/*
 * Copyright (C) 2004, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_VISIBLE_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_VISIBLE_POSITION_H_

#include <iosfwd>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/position_with_affinity.h"
#include "third_party/blink/renderer/core/editing/text_affinity.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// |VisiblePosition| is an immutable object representing "canonical position"
// with affinity.
//
// "canonical position" is roughly equivalent to a position where we can place
// caret, see |canonicalPosition()| for actual definition.
//
// "affinity" represents a place of caret at wrapped line. UPSTREAM affinity
// means caret is placed at end of line. DOWNSTREAM affinity means caret is
// placed at start of line.
//
// Example of affinity:
//    abc^def where "^" represent |Position|
// When above text line wrapped after "abc"
//    abc|  UPSTREAM |VisiblePosition|
//    |def  DOWNSTREAM |VisiblePosition|
//
// NOTE: UPSTREAM affinity will be used only if pos is at end of a wrapped line,
// otherwise it will be converted to DOWNSTREAM.
template <typename Strategy>
class VisiblePositionTemplate final {
  DISALLOW_NEW();

 public:
  VisiblePositionTemplate();

  // Node: Other than |createVisiblePosition()| and
  // |createVisiblePositionDeprecated()|, we should not use |create()|.
  static VisiblePositionTemplate Create(
      const PositionWithAffinityTemplate<Strategy>&);

  // Intentionally delete |operator==()| and |operator!=()| for reducing
  // compilation error message.
  // TODO(yosin) We'll have |equals()| when we have use cases of checking
  // equality of both position and affinity.
  bool operator==(const VisiblePositionTemplate&) const = delete;
  bool operator!=(const VisiblePositionTemplate&) const = delete;

  bool IsValid() const;
  bool IsValidFor(const Document&) const;

  // TODO(editing-dev): We should have |DCHECK(isValid())| in the following
  // functions. However, there are some clients storing a VisiblePosition and
  // inspecting its properties after mutation. This should be fixed.
  // See crbug.com/648949 for details.
  bool IsNull() const { return position_with_affinity_.IsNull(); }
  bool IsNotNull() const { return position_with_affinity_.IsNotNull(); }
  bool IsOrphan() const { return DeepEquivalent().IsOrphan(); }

  PositionTemplate<Strategy> DeepEquivalent() const {
    return position_with_affinity_.GetPosition();
  }
  PositionTemplate<Strategy> ToParentAnchoredPosition() const {
    return DeepEquivalent().ParentAnchoredEquivalent();
  }
  PositionWithAffinityTemplate<Strategy> ToPositionWithAffinity() const {
    return position_with_affinity_;
  }
  TextAffinity Affinity() const { return position_with_affinity_.Affinity(); }

  static VisiblePositionTemplate<Strategy> AfterNode(const Node&);
  static VisiblePositionTemplate<Strategy> BeforeNode(const Node&);
  static VisiblePositionTemplate<Strategy> FirstPositionInNode(const Node&);
  static VisiblePositionTemplate<Strategy> InParentAfterNode(const Node&);
  static VisiblePositionTemplate<Strategy> InParentBeforeNode(const Node&);
  static VisiblePositionTemplate<Strategy> LastPositionInNode(const Node&);

  void Trace(Visitor*) const;

#if DCHECK_IS_ON()
  void ShowTreeForThis() const;
#endif

 private:
  explicit VisiblePositionTemplate(
      const PositionWithAffinityTemplate<Strategy>&);

  PositionWithAffinityTemplate<Strategy> position_with_affinity_;

#if DCHECK_IS_ON()
  uint64_t dom_tree_version_;
  uint64_t style_version_;
#endif
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    VisiblePositionTemplate<EditingStrategy>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    VisiblePositionTemplate<EditingInFlatTreeStrategy>;

using VisiblePosition = VisiblePositionTemplate<EditingStrategy>;
using VisiblePositionInFlatTree =
    VisiblePositionTemplate<EditingInFlatTreeStrategy>;

CORE_EXPORT VisiblePosition
CreateVisiblePosition(const Position&, TextAffinity = TextAffinity::kDefault);
CORE_EXPORT VisiblePosition CreateVisiblePosition(const PositionWithAffinity&);
CORE_EXPORT VisiblePositionInFlatTree
CreateVisiblePosition(const PositionInFlatTree&,
                      TextAffinity = TextAffinity::kDefault);
CORE_EXPORT VisiblePositionInFlatTree
CreateVisiblePosition(const PositionInFlatTreeWithAffinity&);

CORE_EXPORT std::ostream& operator<<(std::ostream&, const VisiblePosition&);
CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const VisiblePositionInFlatTree&);

}  // namespace blink

#if DCHECK_IS_ON()
// Outside the blink namespace for ease of invocation from gdb.
void ShowTree(const blink::VisiblePosition*);
void ShowTree(const blink::VisiblePosition&);
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_VISIBLE_POSITION_H_
