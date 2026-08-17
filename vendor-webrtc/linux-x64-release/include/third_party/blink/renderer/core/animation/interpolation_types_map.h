// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_TYPES_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_TYPES_MAP_H_

#include <memory>

#include "third_party/blink/renderer/core/animation/interpolation_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSSyntaxDefinition;
class PropertyHandle;
class PropertyRegistry;
class PropertyRegistration;

using InterpolationTypes = Vector<std::unique_ptr<const InterpolationType>>;

class CORE_EXPORT InterpolationTypesMap {
  STACK_ALLOCATED();

 public:
  InterpolationTypesMap(const PropertyRegistry* registry,
                        const Document& document);

  const InterpolationTypes& Get(const PropertyHandle&) const;
  size_t Version() const;

  static InterpolationTypes CreateInterpolationTypesForCSSSyntax(
      const AtomicString& property_name,
      const CSSSyntaxDefinition&,
      const PropertyRegistration&);

 private:
  const Document& document_;
  const PropertyRegistry* registry_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLATION_TYPES_MAP_H_
