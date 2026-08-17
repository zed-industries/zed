// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TRANSITION_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TRANSITION_DATA_H_

#include <memory>

#include "base/check_op.h"
#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/animation/css/css_timing_data.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CORE_EXPORT CSSTransitionData final : public CSSTimingData {
 public:
  enum TransitionAnimationType {
    kTransitionNone,
    kTransitionKnownProperty,
    kTransitionUnknownProperty,
  };

  enum TransitionBehavior { kNormal, kAllowDiscrete };

  // FIXME: We shouldn't allow 'none' to be used alongside other properties.
  struct TransitionProperty {
    DISALLOW_NEW();
    TransitionProperty(CSSPropertyID id)
        : property_type(kTransitionKnownProperty), unresolved_property(id) {
      DCHECK_NE(id, CSSPropertyID::kInvalid);
    }

    TransitionProperty(const AtomicString& string)
        : property_type(kTransitionUnknownProperty),
          unresolved_property(CSSPropertyID::kInvalid),
          property_string(string) {}

    explicit TransitionProperty(TransitionAnimationType type)
        : property_type(type), unresolved_property(CSSPropertyID::kInvalid) {
      DCHECK_EQ(type, kTransitionNone);
    }

    bool operator==(const TransitionProperty& other) const {
      return property_type == other.property_type &&
             unresolved_property == other.unresolved_property &&
             property_string == other.property_string;
    }

    TransitionAnimationType property_type;
    CSSPropertyID unresolved_property;
    AtomicString property_string;
  };

  using TransitionPropertyVector = Vector<TransitionProperty, 1>;
  using TransitionBehaviorVector = Vector<TransitionBehavior, 1>;

  std::unique_ptr<CSSTransitionData> Clone() {
    return base::WrapUnique(new CSSTransitionData(*this));
  }

  CSSTransitionData();
  explicit CSSTransitionData(const CSSTransitionData&);

  bool TransitionsMatchForStyleRecalc(const CSSTransitionData& other) const;
  bool operator==(const CSSTransitionData& other) const {
    return TransitionsMatchForStyleRecalc(other);
  }

  Timing ConvertToTiming(size_t index) const;

  const TransitionPropertyVector& PropertyList() const {
    return property_list_;
  }
  TransitionPropertyVector& PropertyList() { return property_list_; }

  const TransitionBehaviorVector& BehaviorList() const {
    return behavior_list_;
  }
  TransitionBehaviorVector& BehaviorList() { return behavior_list_; }

  static std::optional<double> InitialDuration() { return 0; }

  static TransitionProperty InitialProperty() {
    return TransitionProperty(CSSPropertyID::kAll);
  }

  static TransitionBehavior InitialBehavior() {
    return TransitionBehavior::kNormal;
  }

 private:
  TransitionPropertyVector property_list_;
  TransitionBehaviorVector behavior_list_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_CSS_TRANSITION_DATA_H_
