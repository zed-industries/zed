// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SHAPE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SHAPE_VALUE_H_
#include <initializer_list>

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/css_value_pair.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/svg/svg_path_data.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

namespace cssvalue {

// https://drafts.csswg.org/css-shapes-2/#typedef-shape-command
// The sequence of <shape-command>s represent further path data commands.
// Each command’s starting point is the previous command’s ending point.
class CSSShapeCommand : public GarbageCollected<CSSShapeCommand> {
 public:
  using Type = SVGPathSegType;
  Type GetType() const { return type_; }
  const CSSValue& GetEndPoint() const { return *end_point_; }
  bool IsAbsolute() const { return IsAbsolutePathSegType(type_); }

  String CSSText() const;
  bool operator==(const CSSShapeCommand& other) const;
  virtual void Trace(Visitor* visitor) const { visitor->Trace(end_point_); }

  CSSShapeCommand(Type type, const CSSValue& end_point)
      : type_(type), end_point_(end_point) {
    CHECK(type != Type::kPathSegClosePath);
  }

  static const CSSShapeCommand* Close() {
    return MakeGarbageCollected<CSSShapeCommand>();
  }

  // This should be private, but can't because of MakeGarbageCollected.
  CSSShapeCommand() : type_(Type::kPathSegClosePath) {}

 private:
  Type type_;
  Member<const CSSValue> end_point_;
};

class CSSShapeArcCommand : public CSSShapeCommand {
 public:
  CSSShapeArcCommand(Type type,
                     const CSSValue& end_point,
                     const CSSPrimitiveValue& angle,
                     const CSSValuePair& radius,
                     CSSValueID size,
                     CSSValueID sweep)
      : CSSShapeCommand(type, end_point),
        angle_(angle),
        radius_(radius),
        size_(size),
        sweep_(sweep) {
    CHECK(type == Type::kPathSegArcAbs || type == Type::kPathSegArcRel);
    CHECK(sweep == CSSValueID::kCw || sweep == CSSValueID::kCcw);
    CHECK(size == CSSValueID::kLarge || size == CSSValueID::kSmall);
  }
  const CSSPrimitiveValue& Angle() const { return *angle_; }
  const CSSValuePair& Radius() const { return *radius_; }
  CSSValueID Size() const { return size_; }
  CSSValueID Sweep() const { return sweep_; }
  bool operator==(const CSSShapeArcCommand& other) const {
    return CSSShapeCommand::operator==(other) && sweep_ == other.sweep_ &&
           size_ == other.size_ && radius_ == other.radius_ &&
           angle_ == other.angle_;
  }
  void Trace(Visitor* visitor) const override {
    visitor->Trace(angle_);
    visitor->Trace(radius_);
    CSSShapeCommand::Trace(visitor);
  }

 private:
  Member<const CSSPrimitiveValue> angle_;
  Member<const CSSValuePair> radius_;
  CSSValueID size_;
  CSSValueID sweep_;
};

using CSSShapeControlPoint = std::pair<CSSValueID, Member<const CSSValuePair>>;

template <wtf_size_t NumControlPoints>
class CSSShapeCurveCommand : public CSSShapeCommand {
 public:
  CSSShapeCurveCommand<1>(Type type,
                          const CSSValuePair& end_point,
                          const CSSShapeControlPoint control_point)
      : CSSShapeCommand(type, end_point), control_points_{control_point} {}
  CSSShapeCurveCommand(Type type,
                       const CSSValuePair& end_point,
                       const CSSShapeControlPoint control_point1,
                       const CSSShapeControlPoint control_point2)
      : CSSShapeCommand(type, end_point),
        control_points_{control_point1, control_point2} {}

  bool operator==(const CSSShapeCurveCommand<NumControlPoints>& other) const {
    return CSSShapeCommand::operator==(other) &&
           control_points_ == other.control_points_;
  }

  void Trace(Visitor* visitor) const override {
    CSSShapeCommand::Trace(visitor);
    visitor->Trace(control_points_.at(0).second);
    if (NumControlPoints == 2) {
      visitor->Trace(control_points_.at(1).second);
    }
  }

  const std::array<CSSShapeControlPoint, NumControlPoints>& GetControlPoints()
      const {
    return control_points_;
  }

 private:
  std::array<CSSShapeControlPoint, NumControlPoints> control_points_;
};

class CSSShapeValue : public CSSValue {
 public:
  CSSShapeValue(WindRule wind_rule,
                const CSSValuePair& origin,
                HeapVector<Member<const CSSShapeCommand>> commands)
      : CSSValue(kShapeClass),
        wind_rule_(wind_rule),
        origin_(origin),
        commands_(std::move(commands)) {}
  String CustomCSSText() const;

  WindRule GetWindRule() const { return wind_rule_; }
  const CSSValuePair& GetOrigin() const { return *origin_; }
  const HeapVector<Member<const CSSShapeCommand>>& Commands() const {
    return commands_;
  }

  bool Equals(const CSSShapeValue& other) const {
    return wind_rule_ == other.wind_rule_ && *origin_ == *other.origin_ &&
           commands_ == other.commands_;
  }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  WindRule wind_rule_;
  Member<const CSSValuePair> origin_;
  HeapVector<Member<const CSSShapeCommand>> commands_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSShapeValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsShapeValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SHAPE_VALUE_H_
