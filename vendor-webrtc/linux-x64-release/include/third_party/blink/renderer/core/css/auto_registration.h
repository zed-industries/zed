// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_AUTO_REGISTRATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_AUTO_REGISTRATION_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/property_registry.h"
#include "third_party/blink/renderer/core/dom/document.h"

namespace blink {

// Temporarily registers a custom property.
class AutoRegistration {
  STACK_ALLOCATED();

 public:
  AutoRegistration(Document& document,
                   const AtomicString& name,
                   PropertyRegistration& registration)
      : document_(document), name_(name) {
    document_.EnsurePropertyRegistry().AddRegistrationForInspector(
        name_, registration);
  }
  ~AutoRegistration() {
    document_.EnsurePropertyRegistry().RemoveRegistrationForInspector(name_);
  }

 private:
  Document& document_;
  const AtomicString& name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_AUTO_REGISTRATION_H_
