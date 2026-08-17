// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_RANGE_H_

#include <stdint.h>

#include <ostream>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/modules/accessibility/ax_position.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AXObject;

class MODULES_EXPORT AXRange final {
  DISALLOW_NEW();

 public:
  AXRange(const AXPosition& start, const AXPosition& end);
  AXRange(const AXRange&) = default;
  AXRange& operator=(const AXRange&) = default;
  ~AXRange() = default;

  const AXPosition Start() const { return start_; }
  const AXPosition End() const { return end_; }

  // Returns the lowest common ancestor of the container objects of the start
  // and end positions, or nullptr if the range is invalid.
  AXObject* CommonAncestorContainer() const;

  // The |AXRange| is collapsed if the start position is equal to the end
  // position or if the range is invalid.
  bool IsCollapsed() const;

  // The range is invalid if either the start or end position is invalid, or if
  // the positions are in two separate documents.
  bool IsValid() const;

  // Creates an |AXRange| encompassing the contents of the given |AXObject|.
  static AXRange RangeOfContents(const AXObject&);

  // Returns a string representation of this object.
  String ToString() const;

 private:
  AXPosition start_;
  AXPosition end_;

#if DCHECK_IS_ON()
  // TODO(ax-dev): Use layout tree version in place of DOM and style versions.
  uint64_t dom_tree_version_;
  uint64_t style_version_;
#endif
};

MODULES_EXPORT bool operator==(const AXRange&, const AXRange&);
MODULES_EXPORT bool operator!=(const AXRange&, const AXRange&);
MODULES_EXPORT std::ostream& operator<<(std::ostream&, const AXRange&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_RANGE_H_
