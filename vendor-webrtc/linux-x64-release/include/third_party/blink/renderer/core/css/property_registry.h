// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_REGISTRY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/property_registration.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class CORE_EXPORT PropertyRegistry : public GarbageCollected<PropertyRegistry> {
 public:
  using RegistrationMap =
      HeapHashMap<AtomicString, Member<PropertyRegistration>>;

  // Registers a property (CSS.registerProperty).
  void RegisterProperty(const AtomicString&, PropertyRegistration&);
  // Registers a property (@property).
  void DeclareProperty(const AtomicString&, PropertyRegistration&);

  // Removes all registrations originating from @property. Has no effect on
  // properties originating from CSS.registerProperty.
  void RemoveDeclaredProperties();

  // Register property for devtools inspector.
  void AddRegistrationForInspector(const AtomicString&, PropertyRegistration&);
  // Removes property registration for devtools inspector.
  void RemoveRegistrationForInspector(const AtomicString&);

  // Returns the registration originating from CSS.registerProperty if present,
  // otherwise returns the registration originating from @property (which may
  // be nullptr).
  //
  // https://drafts.css-houdini.org/css-properties-values-api-1/#determining-registration
  const PropertyRegistration* Registration(const AtomicString&) const;

  bool IsEmpty() const;

  // The viewport unit flags across all registration and declarations.
  //
  // See `ViewportUnitFlag`.
  unsigned GetViewportUnitFlags() const {
    return registered_viewport_unit_flags_ | declared_viewport_unit_flags_;
  }

  // Returns a number that increases by one every time there's a change to the
  // PropertyRegistry.
  size_t Version() const { return version_; }

  // Returns true for properties registered with RegisterProperty/
  // (CSS.registerProperty). Ignores declared properties (@property).
  //
  // https://drafts.css-houdini.org/css-properties-values-api-1/#dom-window-registeredpropertyset-slot
  bool IsInRegisteredPropertySet(const AtomicString&) const;

  // Produces all active registrations.
  //
  // This means all registrations originating from CSS.registerProperty,
  // plus all registration originating from @property that don't conflict
  // with any CSS.registerProperty-registrations.
  //
  // https://drafts.css-houdini.org/css-properties-values-api-1/#determining-registration
  class CORE_EXPORT Iterator {
    STACK_ALLOCATED();
    using MapIterator = RegistrationMap::const_iterator;

   public:
    Iterator(const RegistrationMap& registered_properties,
             const RegistrationMap& declared_properties,
             MapIterator registered_iterator,
             MapIterator declared_iterator);

    void operator++();
    RegistrationMap::ValueType operator*() const;
    bool operator==(const Iterator&) const;
    bool operator!=(const Iterator& o) const { return !(*this == o); }

   private:
    // True if declared_iterator_ points to a registration that has already
    // been emitted by registered_iterator_.
    bool CurrentDeclaredIteratorIsMasked();

    MapIterator registered_iterator_;
    MapIterator declared_iterator_;
    const RegistrationMap& registered_properties_;
    const RegistrationMap& declared_properties_;
  };

  Iterator begin() const;
  Iterator end() const;

  void Trace(Visitor* visitor) const {
    visitor->Trace(registered_properties_);
    visitor->Trace(declared_properties_);
  }

  // Whenever a registered custom property is referenced by anything using
  // var(), it is marked as referenced (globally). This information is used
  // when determining whether or not a custom property animation can run
  // on the compositor.
  void MarkReferenced(const AtomicString&) const;
  bool WasReferenced(const AtomicString&) const;

 private:
  RegistrationMap registered_properties_;
  RegistrationMap declared_properties_;
  unsigned registered_viewport_unit_flags_ = 0;
  unsigned declared_viewport_unit_flags_ = 0;
  size_t version_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTY_REGISTRY_H_
