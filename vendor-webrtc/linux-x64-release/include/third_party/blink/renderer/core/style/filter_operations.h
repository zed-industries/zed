/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILTER_OPERATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILTER_OPERATIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/filter_operation.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

#include <iosfwd>

namespace gfx {
class RectF;
}

namespace blink {

class SVGResourceClient;

class CORE_EXPORT FilterOperations {
  DISALLOW_NEW();

 public:
  FilterOperations();
  FilterOperations(const FilterOperations& other) = default;
  FilterOperations(FilterOperations&& other) = default;

  FilterOperations& operator=(const FilterOperations&) = default;
  FilterOperations& operator=(FilterOperations&&) = default;

  bool operator==(const FilterOperations&) const;
  bool operator!=(const FilterOperations& o) const { return !(*this == o); }

  void clear() { operations_.clear(); }

  typedef HeapVector<Member<FilterOperation>> FilterOperationVector;

  FilterOperationVector& Operations() { return operations_; }
  const FilterOperationVector& Operations() const { return operations_; }

  bool IsEmpty() const { return !operations_.size(); }
  wtf_size_t size() const { return operations_.size(); }
  const FilterOperation* at(wtf_size_t index) const {
    return index < operations_.size() ? operations_.at(index).Get() : nullptr;
  }

  bool CanInterpolateWith(const FilterOperations&) const;

  // Maps "forward" to determine which pixels in a destination rect are
  // affected by pixels in the source rect.
  // See also FilterEffect::MapRect.
  gfx::RectF MapRect(const gfx::RectF&) const;

  bool HasFilterThatAffectsOpacity() const;
  bool HasFilterThatMovesPixels() const;
  bool HasReferenceFilter() const;
  bool UsesCurrentColor() const;

  void AddClient(SVGResourceClient&) const;
  void RemoveClient(SVGResourceClient&) const;

  void Trace(Visitor*) const;

 private:
  FilterOperationVector operations_;

  friend std::ostream& operator<<(std::ostream& stream,
                                  const FilterOperations& operations);
};

inline std::ostream& operator<<(std::ostream& stream,
                                const FilterOperations& operations) {
  stream << "FilterOperations{";
  for (const Member<FilterOperation>& operation : operations.operations_) {
    stream << *operation;
    stream << ",";
  }
  return stream << "}";
}

// Wrapper object for the FilterOperations part object.
class FilterOperationsWrapper
    : public GarbageCollected<FilterOperationsWrapper> {
 public:
  FilterOperationsWrapper() = default;
  explicit FilterOperationsWrapper(const FilterOperations& operations)
      : operations_(operations) {}

  const FilterOperations& Operations() const { return operations_; }

  void Trace(Visitor* visitor) const { visitor->Trace(operations_); }

 private:
  FilterOperations operations_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILTER_OPERATIONS_H_
