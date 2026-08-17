// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_RESOLVER_H_

#include "base/containers/adapters.h"
#include "base/memory/values_equivalent.h"
#include "base/types/strong_alias.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_name.h"
#include "third_party/blink/renderer/core/css/properties/css_property.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_filter.h"
#include "third_party/blink/renderer/core/css/resolver/cascade_origin.h"
#include "third_party/blink/renderer/core/css/rule_set.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

class CSSProperty;
class CSSVariableData;
class CSSProperty;

namespace cssvalue {

class CSSPendingSubstitutionValue;

}  // namespace cssvalue

// CascadeResolver is an object passed on the stack during Apply. Its most
// important job is to detect cycles during Apply (in general, keep track of
// which properties we're currently applying).
class CORE_EXPORT CascadeResolver {
  STACK_ALLOCATED();

 public:
  // Represents a "node" in the cycle graph, as described in css-variables [1].
  //
  // Originally designed for just custom properties, cycles can now form
  // via a number of different constructs, e.g. an attribute an refer to
  // a custom function, which refers to a custom property, which refers back
  // to the same attribute.
  //
  // [1] https://drafts.csswg.org/css-variables-1/#cycles
  class CycleNode {
    DISALLOW_NEW();

   public:
    enum Type {
      // var()
      kProperty,
      // attr()
      kAttribute,
      // var() (within @function)
      kLocalVariable,
      // <dashed-function>
      kFunction,
    };

    bool operator==(const CycleNode& o) const {
      auto property_name_eq = [](const CSSProperty& a, const CSSProperty& b) {
        return a.HasEqualCSSPropertyName(b);
      };
      return type == o.type &&
             base::ValuesEquivalent(property, o.property, property_name_eq) &&
             name == o.name && function == o.function;
    }

    void Trace(blink::Visitor* visitor) const;

    const Type type;
    // Used for Type::kProperty.
    const CSSProperty* property;
    // Used for Type::kAttribute and Type::kLocalVariable.
    const AtomicString name;
    // Used for Type::kFunction and Type::kLocalVariable.
    const Member<const StyleRuleFunction> function;
  };

  // TODO(crbug.com/985047): Probably use a HashMap for this.
  using CycleStack = HeapVector<CycleNode, 8>;

  // A 'locked' property is a property we are in the process of applying.
  // In other words, once a property is locked, locking it again would form
  // a cycle, and is therefore an error.
  bool IsLocked(const CSSProperty& property) const {
    return IsLocked(
        CycleNode{.type = CycleNode::Type::kProperty, .property = &property});
  }
  bool IsLocked(const CycleNode& node) const { return Find(node) != kNotFound; }

  // Returns the property we're currently applying.
  const CSSProperty* CurrentProperty() const {
    for (const CycleNode& node : base::Reversed(stack_)) {
      if (node.type == CycleNode::Type::kProperty) {
        return node.property;
      }
    }
    return nullptr;
  }

  // We do not allow substitution of animation-tainted values into
  // an animation-affecting property.
  //
  // https://drafts.csswg.org/css-variables/#animation-tainted
  bool AllowSubstitution(CSSVariableData*) const;

  bool Rejects(const CSSProperty& property) {
    if (!filter_.Rejects(property)) {
      return false;
    }
    rejected_flags_ |= property.GetFlags();
    return true;
  }

  // Collects CSSProperty::Flags from the given property. The Flags() function
  // can then be used to see which flags have been observed..
  void CollectFlags(const CSSProperty& property, CascadeOrigin origin) {
    CSSProperty::Flags flags = property.GetFlags();
    author_flags_ |= (origin == CascadeOrigin::kAuthor ? flags : 0);
    flags_ |= flags;
  }

  CSSProperty::Flags Flags() const { return flags_; }

  // Like Flags, but for the author origin only.
  CSSProperty::Flags AuthorFlags() const { return author_flags_; }

  // The CSSProperty::Flags of all properties rejected by the CascadeFilter.
  CSSProperty::Flags RejectedFlags() const { return rejected_flags_; }

  // Automatically locks and unlocks the given property. (See
  // CascadeResolver::IsLocked).
  class CORE_EXPORT AutoLock {
    STACK_ALLOCATED();

   public:
    AutoLock(const CSSProperty& property, CascadeResolver& resolver)
        : AutoLock(CycleNode{.type = CycleNode::Type::kProperty,
                             .property = &property},
                   resolver) {}
    AutoLock(const CycleNode&, CascadeResolver&);
    ~AutoLock();

   private:
    CascadeResolver& resolver_;
  };

 private:
  friend class AutoLock;
  friend class StyleCascade;
  friend class TestCascadeResolver;

  CascadeResolver(CascadeFilter filter, uint8_t generation)
      : filter_(filter), generation_(generation) {}

  // If the given property is already being applied, returns true.
  //
  // When a cycle is detected, a portion of the stack is effectively marked
  // as "in cycle". The function InCycle() may be used to check if we are
  // currently inside a marked portion of the stack.
  //
  // The marked range of the stack shrinks during ~AutoLock, such that we won't
  // be InCycle whenever we move outside that of that range.
  bool DetectCycle(const CSSProperty& property) {
    return DetectCycle(
        CycleNode{.type = CycleNode::Type::kProperty, .property = &property});
  }
  bool DetectCycle(const CycleNode& node) { return DetectCycle(Find(node)); }
  bool DetectCycle(wtf_size_t index);
  // Returns true whenever the CascadeResolver is in a cycle state.
  // This DOES NOT detect cycles; the caller must call DetectCycle first.
  bool InCycle() const;
  // Returns the index of the given CycleNode (by value, see operator==),
  // or kNotFound if the node is not present in stack_.
  wtf_size_t Find(const CycleNode&) const;

  CycleStack stack_;
  // If we're in a cycle, cycle_start_ is the index of the stack_ item that
  // "started" the cycle, i.e. the item in the cycle with the smallest index.
  wtf_size_t cycle_start_ = kNotFound;
  // If we're in a cycle, cycle_end_ is set to the size of stack_. This causes
  // InCycle to return true while we're on the portion of the stack between
  // cycle_start_ and cycle_end_.
  wtf_size_t cycle_end_ = kNotFound;
  CascadeFilter filter_;
  const uint8_t generation_ = 0;
  CSSProperty::Flags author_flags_ = 0;
  CSSProperty::Flags flags_ = 0;
  CSSProperty::Flags rejected_flags_ = 0;

  // A very simple cache for CSSPendingSubstitutionValues. We cache only the
  // most recently parsed CSSPendingSubstitutionValue, such that consecutive
  // calls to ResolvePendingSubstitution with the same value don't need to
  // do the same parsing job all over again.
  struct {
    STACK_ALLOCATED();

   public:
    const cssvalue::CSSPendingSubstitutionValue* value = nullptr;
    HeapVector<CSSPropertyValue, 64> parsed_properties;
  } shorthand_cache_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(
    blink::CascadeResolver::CycleNode)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CASCADE_RESOLVER_H_
