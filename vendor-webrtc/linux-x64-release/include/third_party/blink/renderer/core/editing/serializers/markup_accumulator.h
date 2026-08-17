/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
 * reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_MARKUP_ACCUMULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_MARKUP_ACCUMULATOR_H_

#include <utility>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/editing_strategy.h"
#include "third_party/blink/renderer/core/editing/serializers/markup_formatter.h"
#include "third_party/blink/renderer/core/editing/serializers/serialization.h"
#include "third_party/blink/renderer/core/html/html_template_element.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class Attribute;
class Element;
class Node;

class CORE_EXPORT MarkupAccumulator {
  STACK_ALLOCATED();

 public:
  enum class AttributesMode : uint8_t {
    // Correctly represent the current attributes.
    kSynchronized,
    // Generate possibly incorrect results, avoiding heap modification.
    kUnsynchronized,
  };

  MarkupAccumulator(AbsoluteURLs,
                    SerializationType,
                    const ShadowRootInclusion&,
                    AttributesMode = AttributesMode::kSynchronized);
  MarkupAccumulator(const MarkupAccumulator&) = delete;
  MarkupAccumulator& operator=(const MarkupAccumulator&) = delete;
  virtual ~MarkupAccumulator();

  template <typename Strategy>
  CORE_EXPORT String SerializeNodes(const Node&, ChildrenOnly);

 protected:
  // Determines whether an attribute is emitted as markup.
  enum class EmitAttributeChoice {
    // Emit the attribute as markup.
    kEmit,
    // Do not emit the attribute.
    kIgnore,
  };

  // Determines whether an element is emitted as markup.
  enum class EmitElementChoice {
    // Emit it as markup.
    kEmit,
    // Do not emit the element or any children.
    kIgnore,
    // Emit the element, but not its children.
    kEmitButIgnoreChildren,
  };

  // Returns serialized prefix. It should be passed to AppendEndTag().
  virtual AtomicString AppendElement(const Element&);
  virtual void AppendEndTag(const Element&, const AtomicString& prefix);
  virtual void AppendAttribute(const Element&, const Attribute&);

  // This is called just before emitting markup for `element`. Derived classes
  // may emit markup here, i.e., if they want to provide a substitute for this
  // element.
  virtual EmitElementChoice WillProcessElement(const Element& element);
  // Called just before closing a <template> element used to serialize a
  // shadow root. `auxiliary_tree` is the shadow root that has just been
  // serialized into the <template> element.
  virtual void WillCloseSyntheticTemplateElement(ShadowRoot& auxiliary_tree) {}

  MarkupFormatter formatter_;
  StringBuilder markup_;
  ShadowRootInclusion shadow_root_inclusion_;

 private:
  bool SerializeAsHTML() const;
  String ToString() { return markup_.ToString(); }

  void AppendString(const String&);
  // Serialize a Node, without its children and its end tag.
  void AppendStartMarkup(const Node&);

  class ElementSerializationData;
  // Returns 'ignore namespace definition attribute' flag and the serialized
  // prefix.
  // If the flag is true, we should not serialize xmlns="..." on the element.
  // The prefix should be used in end tag serialization.
  ElementSerializationData AppendStartTagOpen(const Element&);
  void AppendStartTagClose(const Element&);
  void AppendNamespace(const AtomicString& prefix,
                       const AtomicString& namespace_uri,
                       const Document& document);
  void AppendAttributeAsXMLWithNamespace(const Element& element,
                                         const Attribute& attribute,
                                         const String& value);
  bool ShouldAddNamespaceAttribute(const Attribute& attribute,
                                   const AtomicString& candidate_prefix);

  EntityMask EntityMaskForText(const Text&) const;

  void PushNamespaces(const Element& element);
  void PopNamespaces(const Element& element);
  void RecordNamespaceInformation(const Element& element);
  AtomicString RetrievePreferredPrefixString(const AtomicString& ns,
                                             const AtomicString& prefix);
  void AddPrefix(const AtomicString& prefix, const AtomicString& namespace_uri);
  AtomicString LookupNamespaceURI(const AtomicString& prefix);
  AtomicString GeneratePrefix(const AtomicString& new_namespace);

  virtual void AppendCustomAttributes(const Element&);
  virtual EmitAttributeChoice WillProcessAttribute(const Element&,
                                                   const Attribute&) const;

  // Returns a shadow tree that needs also to be serialized. The ShadowRoot is
  // returned as the 1st element in the pair, and can be null if no shadow tree
  // exists. To serialize a ShadowRoot, an enclosing <template shadowrootmode>
  // must be used, and this is returned as the 2nd element in the pair. It can
  // be null if the first element is null.
  virtual std::pair<ShadowRoot*, HTMLTemplateElement*> GetShadowTree(
      const Element&) const;

  template <typename Strategy>
  void SerializeNodesWithNamespaces(const Node& target_node,
                                    ChildrenOnly children_only);

  class NamespaceContext;
  Vector<NamespaceContext> namespace_stack_;

  // https://w3c.github.io/DOM-Parsing/#dfn-generated-namespace-prefix-index
  uint32_t prefix_index_;

  AttributesMode attributes_mode_;
};

extern template String MarkupAccumulator::SerializeNodes<EditingStrategy>(
    const Node&,
    ChildrenOnly);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SERIALIZERS_MARKUP_ACCUMULATOR_H_
