/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2004, 2006, 2008, 2009 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_EMBED_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_EMBED_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_plugin_element.h"

namespace blink {

class CORE_EXPORT HTMLEmbedElement final : public HTMLPlugInElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  HTMLEmbedElement(Document&, const CreateElementFlags = CreateElementFlags());

  // Returns attributes that should be checked against Trusted Types
  const AttrNameToTrustedType& GetCheckedAttributeTypes() const override;

  bool IsExposed() const;

  FrameOwnerElementType OwnerType() const final {
    return FrameOwnerElementType::kEmbed;
  }

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;

  bool LayoutObjectIsNeeded(const DisplayStyle&) const override;

  bool IsURLAttribute(const Attribute&) const override;
  const QualifiedName& SubResourceAttributeName() const override;

  LayoutEmbeddedContent* ExistingLayoutEmbeddedContent() const override;

  void UpdatePluginInternal() override;

  void ParametersForPlugin(PluginParameters& plugin_params);

  NamedItemType GetNamedItemType() const override {
    return NamedItemType::kName;
  }
  bool IsInteractiveContent() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_EMBED_ELEMENT_H_
