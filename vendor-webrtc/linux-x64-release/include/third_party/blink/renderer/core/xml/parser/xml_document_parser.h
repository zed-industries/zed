/*
 * Copyright (C) 2000 Peter Kelly (pmk@post.com)
 * Copyright (C) 2005, 2006, 2007 Apple Inc. All rights reserved.
 * Copyright (C) 2007 Samuel Weinig (sam@webkit.org)
 * Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_PARSER_XML_DOCUMENT_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_PARSER_XML_DOCUMENT_PARSER_H_

#include <libxml/tree.h>

#include <memory>

#include "base/notreached.h"
#include "third_party/blink/renderer/core/dom/parser_content_policy.h"
#include "third_party/blink/renderer/core/dom/scriptable_document_parser.h"
#include "third_party/blink/renderer/core/script/xml_parser_script_runner.h"
#include "third_party/blink/renderer/core/script/xml_parser_script_runner_host.h"
#include "third_party/blink/renderer/core/xml/parser/xml_errors.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/text/segmented_string.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class ContainerNode;
class Document;
class DocumentFragment;
class Element;
class LocalFrameView;
class Text;

struct xmlSAX2Attributes;
struct xmlSAX2Namespace;

class XMLParserContext : public RefCounted<XMLParserContext> {
  USING_FAST_MALLOC(XMLParserContext);

 public:
  static scoped_refptr<XMLParserContext> CreateMemoryParser(
      xmlSAXHandlerPtr,
      void* user_data,
      const std::string& chunk);
  static scoped_refptr<XMLParserContext> CreateStringParser(xmlSAXHandlerPtr,
                                                            void* user_data);
  ~XMLParserContext();
  xmlParserCtxtPtr Context() const { return context_; }

 private:
  XMLParserContext(xmlParserCtxtPtr context) : context_(context) {}

  xmlParserCtxtPtr context_;
};

class XMLDocumentParser final : public ScriptableDocumentParser,
                                public XMLParserScriptRunnerHost {
 public:
  explicit XMLDocumentParser(Document&, LocalFrameView* = nullptr);
  XMLDocumentParser(DocumentFragment*, Element*, ParserContentPolicy);
  ~XMLDocumentParser() override;
  void Trace(Visitor*) const override;

  // Exposed for callbacks:
  void HandleError(XMLErrors::ErrorType, const char* message, TextPosition);

  void SetIsXHTMLDocument(bool is_xhtml) { is_xhtml_document_ = is_xhtml; }
  bool IsXHTMLDocument() const { return is_xhtml_document_; }

  bool IsCurrentlyParsing8BitChunk() {
    return is_currently_parsing8_bit_chunk_;
  }

  static bool ParseDocumentFragment(const String&,
                                    DocumentFragment*,
                                    Element* parent,
                                    ParserContentPolicy,
                                    ExceptionState&);

  // Used by the XMLHttpRequest to check if the responseXML was well formed.
  bool WellFormed() const override { return !saw_error_; }

  TextPosition GetTextPosition() const override;

  static bool SupportsXMLVersion(const String&);

  class PendingCallback {
    USING_FAST_MALLOC(PendingCallback);

   public:
    virtual ~PendingCallback() = default;
    virtual void Call(XMLDocumentParser*) = 0;

    TextPosition GetTextPosition() const { return text_position_; }
    OrdinalNumber LineNumber() const { return text_position_.line_; }
    OrdinalNumber ColumnNumber() const { return text_position_.column_; }

   protected:
    PendingCallback(TextPosition text_position)
        : text_position_(text_position) {}

   private:
    TextPosition text_position_;
  };

  void SetScriptStartPosition(TextPosition);

 private:
  // From DocumentParser
  void insert(const String&) override { NOTREACHED(); }
  void Append(const String&) override;
  void Finish() override;
  void ExecuteScriptsWaitingForResources() final;
  bool IsWaitingForScripts() const override;
  void StopParsing() override;
  void Detach() override;
  OrdinalNumber LineNumber() const override;
  OrdinalNumber ColumnNumber() const;
  void DidAddPendingParserBlockingStylesheet() final;
  void DidLoadAllPendingParserBlockingStylesheets() final;

  // XMLParserScriptRunnerHost
  void NotifyScriptExecuted() override;

  void end();

  void PauseParsing();
  void ResumeParsing();

  bool AppendFragmentSource(const String&);

 public:
  // Callbacks from parser SAX
  PRINTF_FORMAT(3, 0)
  void GetError(XMLErrors::ErrorType, const char* message, va_list args);
  void StartElementNs(const AtomicString& local_name,
                      const AtomicString& prefix,
                      const AtomicString& uri,
                      base::span<const xmlSAX2Namespace> namespaces,
                      base::span<const xmlSAX2Attributes> attributes,
                      int defaulted_count);
  void EndElementNs();
  void Characters(base::span<const xmlChar> chars);
  void GetProcessingInstruction(const String& target, const String& data);
  void CdataBlock(const String&);
  void Comment(const String&);
  void StartDocument(const String& version,
                     const String& encoding,
                     int standalone);
  void InternalSubset(const String& name,
                      const String& external_id,
                      const String& system_id);
  void EndDocument();

 private:
  void InitializeParserContext(const std::string& chunk = std::string());

  void PushCurrentNode(ContainerNode*);
  void PopCurrentNode();
  void ClearCurrentNodeStack();

  void InsertErrorMessageBlock();

  void CreateLeafTextNodeIfNeeded();
  bool UpdateLeafTextNode();

  void DoWrite(const String&);
  void DoEnd();

  void CheckIfBlockingStyleSheetAdded();

  SegmentedString original_source_for_transform_;

  xmlParserCtxtPtr Context() const {
    return context_ ? context_->Context() : nullptr;
  }
  scoped_refptr<XMLParserContext> context_;
  Deque<std::unique_ptr<PendingCallback>> pending_callbacks_;
  std::unique_ptr<PendingCallback> callback_;
  Vector<xmlChar> buffered_text_;

  Member<ContainerNode> current_node_;
  HeapVector<Member<ContainerNode>> current_node_stack_;

  Member<Text> leaf_text_node_;

  // Tracks whether we're processing a new input chunk. This is set right before
  // submitting a new chunk to libxml and is reset by most emitted parse events.
  // We use this as a signal to merge CDATA sections when they span a chunk
  // boundary.
  bool is_start_of_new_chunk_ = false;

  bool is_currently_parsing8_bit_chunk_;
  bool saw_error_;
  bool saw_css_;
  bool saw_xsl_transform_;
  bool saw_first_element_;
  bool is_xhtml_document_;
  bool parser_paused_;
  bool requesting_script_;
  bool finish_called_;
  bool waiting_for_stylesheets_ = false;
  bool added_pending_parser_blocking_stylesheet_ = false;

  XMLErrors xml_errors_;

  Member<Document> document_;
  Member<XMLParserScriptRunner> script_runner_;
  TextPosition script_start_position_;

  bool parsing_fragment_;
  AtomicString default_namespace_uri_;

  typedef HashMap<AtomicString, AtomicString> PrefixForNamespaceMap;
  PrefixForNamespaceMap prefix_to_namespace_map_;
  SegmentedString pending_src_;
};

xmlDocPtr XmlDocPtrForString(Document*,
                             const String& source,
                             const String& url);
HashMap<String, String> ParseAttributes(const String&, bool& attrs_ok);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_PARSER_XML_DOCUMENT_PARSER_H_
