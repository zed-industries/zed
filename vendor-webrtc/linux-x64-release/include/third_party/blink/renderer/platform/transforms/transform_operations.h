/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2006 Graham Dennis (graham.dennis@gmail.com)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSFORM_OPERATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSFORM_OPERATIONS_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/transforms/transform_operation.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/size_f.h"

namespace gfx {
class BoxF;
}

namespace blink {

class PLATFORM_EXPORT EmptyTransformOperations final {
  DISALLOW_NEW();
};

class PLATFORM_EXPORT TransformOperations {
  DISALLOW_NEW();

 public:
  TransformOperations() = default;
  TransformOperations(const EmptyTransformOperations&) {}

  void Trace(Visitor* visitor) const { visitor->Trace(operations_); }

  bool operator==(const TransformOperations& o) const;
  bool operator!=(const TransformOperations& o) const { return !(*this == o); }

  // Constructs a transformation matrix from the operations. The parameter
  // |border_box_size| is used when computing styles that are size-dependent.
  void Apply(const gfx::SizeF& border_box_size, gfx::Transform& t) const {
    for (auto& operation : operations_)
      operation->Apply(t, border_box_size);
  }

  // Constructs a transformation matrix from the operations starting from index
  // |start|. This process facilitates mixing pairwise operations for a common
  // prefix and matrix interpolation for the remainder.  The parameter
  // |border_box_size| is used when computing styles that are size-dependent.
  void ApplyRemaining(const gfx::SizeF& border_box_size,
                      wtf_size_t start,
                      gfx::Transform& t) const;

  // Return true if any of the operation types are 3D operation types (even if
  // the values describe affine transforms)
  bool Has3DOperation() const {
    for (auto& operation : operations_)
      if (operation->Is3DOperation())
        return true;
    return false;
  }

  // Return true if any of the operation types are non-perspective 3D operation
  // types (even if the values describe affine transforms).
  bool HasNonPerspective3DOperation() const {
    for (auto& operation : operations_) {
      if (operation->Is3DOperation() &&
          operation->GetType() != TransformOperation::kPerspective)
        return true;
    }
    return false;
  }

  bool PreservesAxisAlignment() const {
    for (auto& operation : operations_) {
      if (!operation->PreservesAxisAlignment())
        return false;
    }
    return true;
  }

  bool IsIdentityOrTranslation() const {
    for (auto& operation : operations_) {
      if (!operation->IsIdentityOrTranslation())
        return false;
    }
    return true;
  }

  // Returns true if any operation has a non-trivial component in the Z axis.
  bool HasNonTrivial3DComponent() const {
    for (auto& operation : operations_) {
      if (operation->HasNonTrivial3DComponent())
        return true;
    }
    return false;
  }

  // Returns true if any operation is perspective.
  bool HasPerspective() const {
    for (auto& operation : operations_) {
      if (operation->GetType() == TransformOperation::kPerspective)
        return true;
    }
    return false;
  }

  TransformOperation::BoxSizeDependency BoxSizeDependencies(
      wtf_size_t start = 0) const;

  wtf_size_t MatchingPrefixLength(const TransformOperations&) const;

  void clear() { operations_.clear(); }

  HeapVector<Member<TransformOperation>, 2>& Operations() {
    return operations_;
  }
  const HeapVector<Member<TransformOperation>, 2>& Operations() const {
    return operations_;
  }

  wtf_size_t size() const { return operations_.size(); }
  const TransformOperation* at(wtf_size_t index) const {
    return index < operations_.size() ? operations_.at(index).Get() : nullptr;
  }

  bool BlendedBoundsForBox(const gfx::BoxF&,
                           const TransformOperations& from,
                           const double& min_progress,
                           const double& max_progress,
                           gfx::BoxF* bounds) const;

  // Registered custom property interpolations cannot currently represent
  // interpolated values if functions need to be combined into a matrix and
  // those function values depend on the box size via percentage values, since
  // percentages need to be converted into numbers and the percentages should
  // be kept for the computed value. Until there is a standardized mix()
  // function for representing such transform values, make such interpolations
  // discrete with BoxSizeDependentMatrixBlending::kDisallow.
  //
  // The standard CSS transform property still allows such interpolations, but
  // the computed value in typed-om incorrectly returns values with percentages
  // resolved in this case. getComputedStyle().transform returns a resolved
  // value per spec.
  enum class BoxSizeDependentMatrixBlending {
    kAllow,
    kDisallow,
  };

  TransformOperation* BlendRemainingByUsingMatrixInterpolation(
      const TransformOperations& from,
      wtf_size_t matching_prefix_length,
      double progress,
      BoxSizeDependentMatrixBlending box_size_dependent =
          BoxSizeDependentMatrixBlending::kAllow) const;

  TransformOperations Blend(const TransformOperations& from,
                            double progress,
                            BoxSizeDependentMatrixBlending box_size_dependent =
                                BoxSizeDependentMatrixBlending::kAllow) const;
  TransformOperations Add(const TransformOperations& addend) const;
  TransformOperations Zoom(double factor) const;

  // Perform accumulation of |to| onto |this|, as specified in
  // https://drafts.csswg.org/css-transforms-2/#combining-transform-lists
  TransformOperations Accumulate(const TransformOperations& to) const;

 private:
  HeapVector<Member<TransformOperation>, 2> operations_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSFORM_OPERATIONS_H_
