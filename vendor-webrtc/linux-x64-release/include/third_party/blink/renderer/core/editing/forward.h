// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains forward declarations of template classes in editing/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FORWARD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FORWARD_H_

namespace blink {

enum class TextAffinity;

class NodeTraversal;
class FlatTreeTraversal;

template <typename Traversal>
class EditingAlgorithm;
using EditingStrategy = EditingAlgorithm<NodeTraversal>;
using EditingInFlatTreeStrategy = EditingAlgorithm<FlatTreeTraversal>;

template <typename Strategy>
class PositionTemplate;
using Position = PositionTemplate<EditingStrategy>;
using PositionInFlatTree = PositionTemplate<EditingInFlatTreeStrategy>;

template <typename Strategy>
class EphemeralRangeTemplate;
using EphemeralRange = EphemeralRangeTemplate<EditingStrategy>;
using EphemeralRangeInFlatTree =
    EphemeralRangeTemplate<EditingInFlatTreeStrategy>;

template <typename Strategy>
class PositionWithAffinityTemplate;
using PositionWithAffinity = PositionWithAffinityTemplate<EditingStrategy>;
using PositionInFlatTreeWithAffinity =
    PositionWithAffinityTemplate<EditingInFlatTreeStrategy>;

template <typename Strategy>
class SelectionTemplate;
using SelectionInDOMTree = SelectionTemplate<EditingStrategy>;
using SelectionInFlatTree = SelectionTemplate<EditingInFlatTreeStrategy>;

template <typename Strategy>
class VisiblePositionTemplate;
using VisiblePosition = VisiblePositionTemplate<EditingStrategy>;
using VisiblePositionInFlatTree =
    VisiblePositionTemplate<EditingInFlatTreeStrategy>;

template <typename Strategy>
class VisibleSelectionTemplate;
using VisibleSelection = VisibleSelectionTemplate<EditingStrategy>;
using VisibleSelectionInFlatTree =
    VisibleSelectionTemplate<EditingInFlatTreeStrategy>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_FORWARD_H_
