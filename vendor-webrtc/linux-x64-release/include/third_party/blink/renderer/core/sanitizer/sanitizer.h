// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_sanitizer_presets.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/sanitizer/sanitizer_names.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class Element;
class ExceptionState;
class Node;
class QualifiedName;
class SanitizerConfig;
class SanitizerElementNamespace;
class V8UnionSanitizerConfigOrSanitizerPresets;
class V8UnionSanitizerAttributeNamespaceOrString;
class V8UnionSanitizerElementNamespaceWithAttributesOrString;
class V8UnionSanitizerElementNamespaceOrString;

class CORE_EXPORT Sanitizer final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Called by WebIDL for Sanitizer constructor, new Sanitizer(xxx).
  static Sanitizer* Create(const V8UnionSanitizerConfigOrSanitizerPresets*,
                           ExceptionState&);

  // Called by Sanitizer API, to implement setHTML / setHTMLUnsafe & friends.
  static Sanitizer* Create(const SanitizerConfig*, bool safe, ExceptionState&);
  static Sanitizer* Create(const V8SanitizerPresets::Enum, ExceptionState&);

  Sanitizer() = default;
  ~Sanitizer() override = default;
  Sanitizer(const Sanitizer&) = delete;  // Use MakeGarbageCollected + setFrom.

  // This constructor is meant to be used by generated code to build up the
  // Sanitizer builtins. You probably don't want to use it in regular code.
  Sanitizer(HashSet<QualifiedName>,
            HashSet<QualifiedName>,
            HashSet<QualifiedName>,
            HashSet<QualifiedName>,
            HashSet<QualifiedName>,
            bool,
            bool);

  // API methods:
  void allowElement(
      const V8UnionSanitizerElementNamespaceWithAttributesOrString*);
  void removeElement(const V8UnionSanitizerElementNamespaceOrString*);
  void replaceElementWithChildren(
      const V8UnionSanitizerElementNamespaceOrString*);
  void allowAttribute(const V8UnionSanitizerAttributeNamespaceOrString*);
  void removeAttribute(const V8UnionSanitizerAttributeNamespaceOrString*);
  void setComments(bool);
  void setDataAttributes(bool);
  void removeUnsafe();
  SanitizerConfig* get() const;

  // Internal versions of API methods that use Blink types (rather than IDL):
  void AllowElement(const QualifiedName&);
  void RemoveElement(const QualifiedName&);
  void ReplaceElement(const QualifiedName&);
  void AllowAttribute(const QualifiedName&);
  void RemoveAttribute(const QualifiedName&);

  // Accessors, mainly for testing:
  const SanitizerNameSet& allow_elements() const { return allow_elements_; }
  const SanitizerNameSet& remove_elements() const { return remove_elements_; }
  const SanitizerNameSet& replace_elements() const { return replace_elements_; }
  const SanitizerNameSet& allow_attrs() const { return allow_attrs_; }
  const SanitizerNameSet& remove_attrs() const { return remove_attrs_; }
  const SanitizerNameMap& allow_attrs_per_element() const {
    return allow_attrs_per_element_;
  }
  const SanitizerNameMap& remove_attrs_per_element() const {
    return remove_attrs_per_element_;
  }
  bool allow_data_attrs() const { return allow_data_attrs_; }
  bool allow_comments() const { return allow_comments_; }

  // The core methods (not directly exposed to the API): Recursively sanitize
  // the node according to the current config.
  void SanitizeSafe(Node* node) const;
  void SanitizeUnsafe(Node* node) const;

 private:
  // Helper methods for SanitizeSafe/Unsafe:
  void Sanitize(Node* node, bool safe) const;
  void SanitizeElement(Element* element) const;
  void SanitizeJavascriptNavigationAttributes(Element* element,
                                              bool safe) const;
  void SanitizeTemplate(Node* node, bool safe) const;

  // Helper for copy constructor and Create: Convert from IDL representation
  // to internal.
  bool setFrom(const SanitizerConfig*, bool safe);
  void setFrom(const Sanitizer&);

  // Helpers for get(): Convert from internal to IDL representation.
  QualifiedName getFrom(const String& name, const String& namespaceURI) const;
  QualifiedName getFrom(const SanitizerElementNamespace*) const;
  QualifiedName getFrom(
      const V8UnionSanitizerElementNamespaceWithAttributesOrString*) const;
  QualifiedName getFrom(const V8UnionSanitizerElementNamespaceOrString*) const;
  QualifiedName getFrom(
      const V8UnionSanitizerAttributeNamespaceOrString*) const;

  // Helpers for setFrom(SanitizerConfig*, ...): Count items in config.
  // These are used for error checking.
  int countItemsInSanitizerConfig(const SanitizerConfig*) const;
  int countItemsInSanitizerElement(
      const V8UnionSanitizerElementNamespaceWithAttributesOrString*) const;
  int countItemsInConfig() const;

  // These members are Blink-representation of SanitizerConfig, and the core
  // data structure(s) for Sanitizer. We'll try to keep them simple (sets and
  // maps), and to use QualifiedName, since that's an efficient-to-compare
  // implementation.
  SanitizerNameSet allow_elements_;
  SanitizerNameSet remove_elements_;
  SanitizerNameSet replace_elements_;
  SanitizerNameSet allow_attrs_;
  SanitizerNameSet remove_attrs_;
  SanitizerNameMap allow_attrs_per_element_;
  SanitizerNameMap remove_attrs_per_element_;
  bool allow_data_attrs_;
  bool allow_comments_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_H_
