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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSFORM_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSFORM_OPERATION_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size_f.h"
#include "ui/gfx/geometry/transform.h"

namespace blink {

// CSS Transforms (may become part of CSS3)

class PLATFORM_EXPORT TransformOperation
    : public GarbageCollected<TransformOperation> {
 public:
  enum OperationType {
    kScaleX,
    kScaleY,
    kScale,
    kTranslateX,
    kTranslateY,
    kTranslate,
    kRotate,
    kRotateZ,
    kSkewX,
    kSkewY,
    kSkew,
    kMatrix,
    kScaleZ,
    kScale3D,
    kTranslateZ,
    kTranslate3D,
    kRotateX,
    kRotateY,
    kRotate3D,
    kMatrix3D,
    kPerspective,
    kInterpolated,
    kRotateAroundOrigin,
  };

  TransformOperation() = default;
  TransformOperation(const TransformOperation&) = delete;
  TransformOperation& operator=(const TransformOperation&) = delete;
  virtual ~TransformOperation() = default;

  virtual void Trace(Visitor*) const {}

  bool operator==(const TransformOperation& o) const {
    return IsSameType(o) && IsEqualAssumingSameType(o);
  }
  bool operator!=(const TransformOperation& o) const { return !(*this == o); }

  virtual void Apply(gfx::Transform&,
                     const gfx::SizeF& border_box_size) const = 0;

  // Implements the accumulative behavior described in
  // https://drafts.csswg.org/css-transforms-2/#combining-transform-lists
  virtual TransformOperation* Accumulate(const TransformOperation& other) = 0;

  virtual TransformOperation* Blend(const TransformOperation* from,
                                    double progress,
                                    bool blend_to_identity = false) = 0;
  virtual TransformOperation* Zoom(double factor) = 0;

  virtual OperationType GetType() const = 0;

  // https://drafts.csswg.org/css-transforms/#transform-primitives
  virtual OperationType PrimitiveType() const { return GetType(); }

  bool IsSameType(const TransformOperation& other) const {
    return other.GetType() == GetType();
  }
  bool CanBlendWith(const TransformOperation& other) const {
    return PrimitiveType() == other.PrimitiveType();
  }

  virtual bool PreservesAxisAlignment() const { return false; }
  virtual bool IsIdentityOrTranslation() const { return false; }

  bool Is3DOperation() const {
    OperationType op_type = GetType();
    return op_type == kScaleZ || op_type == kScale3D ||
           op_type == kTranslateZ || op_type == kTranslate3D ||
           op_type == kRotateX || op_type == kRotateY || op_type == kRotate3D ||
           op_type == kMatrix3D || op_type == kPerspective ||
           op_type == kInterpolated;
  }

  virtual bool HasNonTrivial3DComponent() const { return Is3DOperation(); }

  enum BoxSizeDependency {
    kDependsNone = 0,
    kDependsWidth = 0x01,
    kDependsHeight = 0x02,
    kDependsBoth = kDependsWidth | kDependsHeight
  };
  virtual BoxSizeDependency BoxSizeDependencies() const { return kDependsNone; }

  static inline BoxSizeDependency CombineDependencies(BoxSizeDependency a,
                                                      BoxSizeDependency b) {
    return static_cast<BoxSizeDependency>(a | b);
  }

 protected:
  virtual bool IsEqualAssumingSameType(const TransformOperation&) const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_TRANSFORM_OPERATION_H_
