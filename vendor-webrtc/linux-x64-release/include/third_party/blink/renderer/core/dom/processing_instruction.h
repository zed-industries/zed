/*
 * Copyright (C) 2000 Peter Kelly (pmk@post.com)
 * Copyright (C) 2006 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PROCESSING_INSTRUCTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PROCESSING_INSTRUCTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/character_data.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleSheet;
class EventListener;

class CORE_EXPORT ProcessingInstruction final : public CharacterData,
                                                private ResourceClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ProcessingInstruction(Document&, const String& target, const String& data);
  ~ProcessingInstruction() override;
  void Trace(Visitor*) const override;

  const String& target() const { return target_; }
  const String& LocalHref() const { return local_href_; }
  StyleSheet* sheet() const { return sheet_.Get(); }

  bool IsCSS() const { return is_css_; }
  bool IsXSL() const { return is_xsl_; }

  void DidAttributeChanged();
  bool IsLoading() const;

  // For XSLT
  class DetachableEventListener : public GarbageCollectedMixin {
   public:
    virtual ~DetachableEventListener() = default;
    virtual EventListener* ToEventListener() = 0;
    // Detach event listener from its processing instruction.
    virtual void Detach() = 0;

    void Trace(Visitor* visitor) const override {}
  };

  void SetEventListenerForXSLT(DetachableEventListener* listener) {
    listener_for_xslt_ = listener;
  }
  EventListener* EventListenerForXSLT();
  void ClearEventListenerForXSLT();

 private:
  String nodeName() const override;
  CharacterData* CloneWithData(Document&, const String&) const override;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void DetachLayoutTree(bool performing_reattach) final {}

  bool CheckStyleSheet(String& href, String& charset);
  void Process(const String& href, const String& charset);

  void NotifyFinished(Resource*) override;

  bool SheetLoaded() override;

  void ParseStyleSheet(const String& sheet);
  void ClearSheet();
  void RemovePendingSheet();

  String DebugName() const override { return "ProcessingInstruction"; }

  String target_;
  String local_href_;
  String title_;
  String media_;
  Member<StyleSheet> sheet_;
  bool loading_;
  bool alternate_;
  bool is_css_;
  bool is_xsl_;

  Member<DetachableEventListener> listener_for_xslt_;
};

template <>
struct DowncastTraits<ProcessingInstruction> {
  static bool AllowFrom(const Node& node) {
    return node.getNodeType() == Node::kProcessingInstructionNode;
  }
};

inline bool IsXSLStyleSheet(const Node& node) {
  return node.getNodeType() == Node::kProcessingInstructionNode &&
         To<ProcessingInstruction>(node).IsXSL();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_PROCESSING_INSTRUCTION_H_
