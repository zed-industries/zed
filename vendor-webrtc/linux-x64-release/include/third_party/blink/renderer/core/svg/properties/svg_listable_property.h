/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LISTABLE_PROPERTY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LISTABLE_PROPERTY_H_

#include "third_party/blink/renderer/core/svg/properties/svg_property.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class SVGListPropertyBase;

// Subclass of SVGPropertyBase for SVG properties that can reside in lists that
// are exposed via SVG DOM.
class SVGListablePropertyBase : public SVGPropertyBase {
 public:
  SVGListablePropertyBase(const SVGListablePropertyBase&) = delete;
  SVGListablePropertyBase& operator=(const SVGListablePropertyBase&) = delete;

  SVGListPropertyBase* OwnerList() const { return owner_list_.Get(); }

  void SetOwnerList(SVGListPropertyBase* owner_list) {
    // Previous owner list must be cleared before setting new owner list.
    DCHECK((!owner_list && owner_list_) || (owner_list && !owner_list_));

    owner_list_ = owner_list;
  }

 protected:
  SVGListablePropertyBase() : owner_list_(nullptr) {}

 private:
  // Oilpan: the back reference to the owner should be a Member, but this can
  // create cycles when SVG properties meet the off-heap InterpolationValue
  // hierarchy.  Not tracing it is safe, albeit an undesirable state of affairs.
  // See http://crbug.com/528275 for details.
  UntracedMember<SVGListPropertyBase> owner_list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_PROPERTIES_SVG_LISTABLE_PROPERTY_H_
