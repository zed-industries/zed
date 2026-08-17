// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_POSITION_WITH_AFFINITY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_POSITION_WITH_AFFINITY_H_

#include <iosfwd>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/position.h"

namespace blink {

enum class TextAffinity;

template <typename Strategy>
class PositionWithAffinityTemplate {
  DISALLOW_NEW();

 public:
  PositionWithAffinityTemplate(const PositionTemplate<Strategy>&, TextAffinity);
  explicit PositionWithAffinityTemplate(const PositionTemplate<Strategy>&);
  PositionWithAffinityTemplate();
  ~PositionWithAffinityTemplate();

  explicit operator bool() const { return IsNotNull(); }

  TextAffinity Affinity() const { return affinity_; }
  const PositionTemplate<Strategy>& GetPosition() const { return position_; }

  // Returns true if both |this| and |other| is null or both |position_|
  // and |affinity_| equal.
  bool operator==(const PositionWithAffinityTemplate& other) const;
  bool operator!=(const PositionWithAffinityTemplate& other) const {
    return !operator==(other);
  }

  bool IsValidFor(const Document& document) const {
    return position_.IsValidFor(document);
  }

  bool IsNotNull() const { return position_.IsNotNull(); }
  bool IsNull() const { return position_.IsNull(); }
  bool IsOrphan() const { return position_.IsOrphan(); }
  bool IsConnected() const { return position_.IsConnected(); }

  Node* AnchorNode() const { return position_.AnchorNode(); }
  Document* GetDocument() const { return position_.GetDocument(); }

  void Trace(Visitor*) const;

 private:
  PositionTemplate<Strategy> position_;
  TextAffinity affinity_;
};

extern template class CORE_EXTERN_TEMPLATE_EXPORT
    PositionWithAffinityTemplate<EditingStrategy>;
extern template class CORE_EXTERN_TEMPLATE_EXPORT
    PositionWithAffinityTemplate<EditingInFlatTreeStrategy>;

using PositionWithAffinity = PositionWithAffinityTemplate<EditingStrategy>;
using PositionInFlatTreeWithAffinity =
    PositionWithAffinityTemplate<EditingInFlatTreeStrategy>;

PositionWithAffinity ToPositionInDOMTreeWithAffinity(
    const PositionWithAffinity&);
PositionWithAffinity ToPositionInDOMTreeWithAffinity(
    const PositionInFlatTreeWithAffinity&);
PositionInFlatTreeWithAffinity ToPositionInFlatTreeWithAffinity(
    const PositionWithAffinity&);
PositionInFlatTreeWithAffinity ToPositionInFlatTreeWithAffinity(
    const PositionInFlatTreeWithAffinity&);

template <typename Strategy>
PositionWithAffinityTemplate<Strategy> FromPositionInDOMTree(
    const PositionWithAffinity&);

template <>
inline PositionWithAffinity FromPositionInDOMTree<EditingStrategy>(
    const PositionWithAffinity& position_with_affinity) {
  return position_with_affinity;
}

template <>
inline PositionInFlatTreeWithAffinity
FromPositionInDOMTree<EditingInFlatTreeStrategy>(
    const PositionWithAffinity& position_with_affinity) {
  return PositionInFlatTreeWithAffinity(
      ToPositionInFlatTree(position_with_affinity.GetPosition()),
      position_with_affinity.Affinity());
}

CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const PositionWithAffinity&);
CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                     const PositionInFlatTreeWithAffinity&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_POSITION_WITH_AFFINITY_H_
