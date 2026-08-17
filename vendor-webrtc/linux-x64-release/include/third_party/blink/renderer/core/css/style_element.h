/*
 * Copyright (C) 2006, 2007 Rob Buis
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_style_sheet.h"
#include "third_party/blink/renderer/core/css/pending_sheet_type.h"
#include "third_party/blink/renderer/platform/loader/fetch/render_blocking_behavior.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class ContainerNode;
class Document;
class Element;
class Node;

class CORE_EXPORT StyleElement : public GarbageCollectedMixin {
 public:
  StyleElement(Document*, bool created_by_parser);
  virtual ~StyleElement();
  void Trace(Visitor*) const override;

 protected:
  enum ProcessingResult { kProcessingSuccessful, kProcessingFatalError };

  virtual const AtomicString& type() const = 0;
  virtual const AtomicString& media() const = 0;

  // Returns whether |this| and |node| are the same object. Helps us verify
  // parameter validity in certain member functions with an Element parameter
  // which should only be called by a subclass with |this|.
  virtual bool IsSameObject(const Node& node) const = 0;

  CSSStyleSheet* sheet() const { return sheet_.Get(); }

  bool IsLoading() const;
  bool SheetLoaded(Document&);
  void SetToPendingState(Document&, Element& element);

  void RemovedFrom(Element&, ContainerNode& insertion_point);
  void BlockingAttributeChanged(Element&);
  ProcessingResult ProcessStyleSheet(Document&, Element&);
  ProcessingResult ChildrenChanged(Element&);
  ProcessingResult FinishParsingChildren(Element&);

  Member<CSSStyleSheet> sheet_;

 protected:
  bool CreatedByParser() const { return created_by_parser_; }

 private:
  ProcessingResult CreateSheet(Element&, const String& text = String());
  ProcessingResult Process(Element&);
  void ClearSheet(Element& owner_element);

  bool has_finished_parsing_children_ : 1;
  bool loading_ : 1;
  bool registered_as_candidate_ : 1;
  bool created_by_parser_ : 1;
  TextPosition start_position_;
  PendingSheetType pending_sheet_type_;
  RenderBlockingBehavior render_blocking_behavior_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ELEMENT_H_
