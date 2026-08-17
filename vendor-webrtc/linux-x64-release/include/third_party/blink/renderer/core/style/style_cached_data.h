// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_CACHED_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_CACHED_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class ComputedStyle;

using PseudoElementStyleCache = GCedHeapVector<Member<const ComputedStyle>, 4>;

class CORE_EXPORT StyleCachedData final
    : public GarbageCollected<StyleCachedData> {
 public:
  void Trace(Visitor* visitor) const {
    visitor->Trace(pseudo_element_styles_);
  }

 private:
  friend class ComputedStyle;
  friend class ComputedStyleBuilder;

  // This cache stores ComputedStyles for pseudo elements originating from this
  // ComputedStyle's element. Pseudo elements which are represented by
  // PseudoElement in DOM store the ComputedStyle on those elements, so this
  // cache is for:
  //
  // 1. Pseudo elements which do not generate a PseudoElement internally like
  //    ::first-line and ::selection.
  //
  // 2. Pseudo element style requested from getComputedStyle() where the element
  //    currently doesn't generate a PseudoElement. E.g.:
  //
  //    <style>
  //      #div::before { color: green /* no content property! */}
  //    </style>
  //    <div id=div></div>
  //    <script>
  //      getComputedStyle(div, "::before").color // still green.
  //    </script>
  Member<PseudoElementStyleCache> pseudo_element_styles_;

  // Stores the names of of all custom properties on a given ComputedStyle.
  std::unique_ptr<Vector<AtomicString>> variable_names_;

  // If this style is a "decorating box" stores the list of applied text
  // decorations (with the most recent decoration from this box being at the
  // end of the Vector).
  scoped_refptr<base::RefCountedData<Vector<AppliedTextDecoration, 1>>>
      applied_text_decorations_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_CACHED_DATA_H_
