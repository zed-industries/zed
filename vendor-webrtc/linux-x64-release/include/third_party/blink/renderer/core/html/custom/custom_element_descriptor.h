// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_DESCRIPTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_DESCRIPTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Describes what elements a custom element definition applies to.
// https://html.spec.whatwg.org/C/#custom-elements-core-concepts
//
// There are two kinds of definitions:
//
// [Autonomous] These have their own tag name. In that case "name"
// (the definition name) and local name (the tag name) are identical.
//
// [Customized built-in] The name is still a valid custom element
// name; but the local name will be a built-in element name, or an
// unknown element name that is *not* a valid custom element name.
//
// CustomElementDescriptor used when the kind of custom element
// definition is known, and generally the difference is important. For
// example, a definition for "my-element", "my-element" must not be
// applied to an element <button is="my-element">.
class CORE_EXPORT CustomElementDescriptor final {
  DISALLOW_NEW();

 public:
  CustomElementDescriptor() = default;

  CustomElementDescriptor(const AtomicString& name,
                          const AtomicString& local_name)
      : name_(name), local_name_(local_name) {}

  bool operator==(const CustomElementDescriptor& other) const {
    return name_ == other.name_ && local_name_ == other.local_name_;
  }

  const AtomicString& GetName() const { return name_; }
  const AtomicString& LocalName() const { return local_name_; }

  bool Matches(const Element& element) const {
    return LocalName() == element.localName() &&
           (IsAutonomous() || GetName() == element.IsValue()) &&
           element.namespaceURI() == html_names::xhtmlNamespaceURI;
  }

  bool IsAutonomous() const { return name_ == local_name_; }

 private:
  friend struct WTF::HashTraits<blink::CustomElementDescriptor>;
  AtomicString name_;
  AtomicString local_name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CUSTOM_CUSTOM_ELEMENT_DESCRIPTOR_H_
