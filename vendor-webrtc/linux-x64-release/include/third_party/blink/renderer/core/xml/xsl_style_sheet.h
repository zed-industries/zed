/*
 * This file is part of the XSL implementation.
 *
 * Copyright (C) 2004, 2006, 2008, 2012 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XSL_STYLE_SHEET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XSL_STYLE_SHEET_H_

#include <libxml/tree.h>
#include <libxslt/transform.h>
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/style_sheet.h"
#include "third_party/blink/renderer/core/dom/processing_instruction.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class XSLStyleSheet final : public StyleSheet {
 public:
  XSLStyleSheet(Node* parent_node,
                const String& original_url,
                const KURL& final_url,
                bool embedded);
  // Taking an arbitrary node is unsafe, because owner node pointer can become
  // stale. XSLTProcessor ensures that the stylesheet doesn't outlive its
  // parent, in part by not exposing it to JavaScript.
  XSLStyleSheet(Document* owner_document,
                Node* style_sheet_root_node,
                const String& original_url,
                const KURL& final_url,
                bool embedded);
  XSLStyleSheet(XSLStyleSheet* parent_style_sheet,
                const String& original_url,
                const KURL& final_url);
  ~XSLStyleSheet() override;

  bool ParseString(const String&);

  void CheckLoaded();

  Document* OwnerDocument();
  XSLStyleSheet* parentStyleSheet() const override {
    return parent_style_sheet_.Get();
  }

  xmlDocPtr GetDocument();
  xsltStylesheetPtr CompileStyleSheet();
  xmlDocPtr LocateStylesheetSubResource(xmlDocPtr parent_doc,
                                        const xmlChar* uri);

  void ClearDocuments();

  void MarkAsProcessed();
  bool Processed() const { return processed_; }

  String type() const override { return "text/xml"; }
  bool disabled() const override { return is_disabled_; }
  void setDisabled(bool b) override { is_disabled_ = b; }
  Node* ownerNode() const override { return owner_node_.Get(); }
  String href() const override { return original_url_; }
  String title() const override { return g_empty_string; }

  void ClearOwnerNode() override { owner_node_ = nullptr; }
  KURL BaseURL() const override { return final_url_; }
  bool IsLoading() const override { return false; }

  void Trace(Visitor*) const override;

 private:
  void LoadChildSheets();
  void LoadChildSheet(const String& href);

  Member<Node> owner_node_;
  String original_url_;
  KURL final_url_;
  bool is_disabled_;

  HeapVector<Member<XSLStyleSheet>> children_;

  bool embedded_;
  bool processed_;

  xmlDocPtr stylesheet_doc_;
  bool stylesheet_doc_taken_;
  bool compilation_failed_;

  Member<XSLStyleSheet> parent_style_sheet_;
  Member<Document> owner_document_;
};

template <>
struct DowncastTraits<XSLStyleSheet> {
  static bool AllowFrom(const StyleSheet& sheet) {
    return !sheet.IsCSSStyleSheet();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XSL_STYLE_SHEET_H_
