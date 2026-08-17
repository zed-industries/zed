// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_EXPANSION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_EXPANSION_H_

#include <limits>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/core/css/properties/longhands/custom_property.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_filter.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_origin.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_priority.h"
#include "third_party/blink/renderer/core/style/computed_style_base_constants.h"

namespace blink {

struct MatchedProperties;

inline uint32_t EncodeMatchResultPosition(uint16_t block,
                                          uint16_t declaration) {
  return (static_cast<uint32_t>(block) << 16) | declaration;
}

inline wtf_size_t DecodeMatchedPropertiesIndex(uint32_t position) {
  return (position >> 16) & 0xFFFF;
}

inline wtf_size_t DecodeDeclarationIndex(uint32_t position) {
  return position & 0xFFFF;
}

// Used by the -inl.h file.
CORE_EXPORT CascadeFilter
CreateExpansionFilter(const MatchedProperties& matched_properties);
CORE_EXPORT bool IsInAllExpansion(CSSPropertyID id);

// CascadeExpansion objects which exceed these limits will emit nothing.
constexpr wtf_size_t kMaxDeclarationIndex =
    std::numeric_limits<uint16_t>::max();
constexpr wtf_size_t kMaxMatchedPropertiesIndex =
    std::numeric_limits<uint16_t>::max();

// CascadeExpansion takes a declaration block (MatchedProperties) and
// expands the declarations found into the final list of declarations observed
// by StyleCascade. It exists to prevent callers to deal with the complexity
// of the 'all' property, '-internal-visited-' properties, '-internal-ua-'
// properties, and filtering of both regular declarations and "generated"
// declarations.
//
// For example, for the declaration block:
//
//   top:1px;
//   all:unset;
//   top:2px;
//
// CascadeExpansion would emit:
//
//   top:1px;
//   animation-delay:unset;
//   animation-direction:unset;
//   /* ... <all longhands affected by 'all'> ... */
//   -webkit-text-emphasis:unset;
//   -webkit-text-stroke:unset;
//   top:2px;
//
// In other words, 'all' is expanded into the actual longhands it represents.
// A similar expansion happens for properties which have companion
// -internal-visited-* properties (depending on inside-link status).
//
// Usage:
//
//   ExpandCascade(..., [](CascadePriority cascade_priority,
//                         const AtomicString& name) {
//                           DoStuffWithCustomProperty(...)
//                         },
//                      [](CascadePriority cascade_priority,
//                         CSSPropertyID id) {
//                           DoStuffWithRegularProperty(...)
//                         });
//
//
// The css_property and name references are not guaranteed to live past the end
// of the callback. The name is guaranteed to be identical to
// css_property.GetCSSPropertyName() (using it generally saves you the virtual
// function call).
//
// The implementation is in cascade_expansion-inl.h, which you will need to
// include if you use this function.

template <class CustomPropertyCallback, class RegularPropertyCallback>
void ExpandCascade(const MatchedProperties& matched_properties,
                   const Document& document,
                   wtf_size_t matched_properties_index,
                   CustomPropertyCallback&& custom_property_callback,
                   RegularPropertyCallback&& regular_property_callback);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_EXPANSION_H_
