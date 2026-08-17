// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_RELOCATABLE_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_RELOCATABLE_POSITION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/range.h"

namespace blink {

// |RelocatablePosition| is a helper class for keeping track of a |Position| in
// a document upon DOM tree changes even if the given |Position|'s original
// anchor node is moved out of document. The class is implemented by using a
// temporary |Range| object to keep track of the |Position|, and disposing the
// |Range| when out of scope.
class CORE_EXPORT RelocatablePosition final
    : public GarbageCollected<RelocatablePosition> {
 public:
  explicit RelocatablePosition(const Position&);

  void SetPosition(const Position&);
  Position GetPosition() const;

  void Trace(Visitor* visitor) const;

 private:
  Member<Range> const range_;
  Position original_position_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_RELOCATABLE_POSITION_H_
