/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_TREE_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_TREE_BUILDER_H_

#include "base/dcheck_is_on.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/html/parser/html_construction_site.h"
#include "third_party/blink/renderer/core/html/parser/html_element_stack.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_options.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class AtomicHTMLToken;
class DocumentFragment;
class Element;
class HTMLDocument;
class HTMLDocumentParser;

class HTMLTreeBuilder final : public GarbageCollected<HTMLTreeBuilder> {
 public:
  // This constructor is used for main document parsing.
  // TODO(kouhei): HTMLTreeBuilder can be created for non-HTMLDocument
  // (XHTMLDocument) from editing code. Fix editing code to always invoke HTML
  // parser on HTMLDocument.
  HTMLTreeBuilder(HTMLDocumentParser*,
                  Document&,
                  ParserContentPolicy,
                  const HTMLParserOptions&,
                  bool include_shadow_roots);
  // This constructor is used for fragment parsing.
  HTMLTreeBuilder(HTMLDocumentParser*,
                  DocumentFragment*,
                  Element* context_element,
                  ParserContentPolicy,
                  const HTMLParserOptions&,
                  bool include_shadow_roots);

 private:
  HTMLTreeBuilder(HTMLDocumentParser*,
                  Document&,
                  ParserContentPolicy,
                  const HTMLParserOptions&,
                  bool include_shadow_roots,
                  DocumentFragment* for_fragment,
                  Element* fragment_context_element);

 public:
  HTMLTreeBuilder(const HTMLTreeBuilder&) = delete;
  HTMLTreeBuilder& operator=(const HTMLTreeBuilder&) = delete;
  ~HTMLTreeBuilder();
  void Trace(Visitor*) const;

  const HTMLElementStack* OpenElements() const { return tree_.OpenElements(); }

  bool IsParsingFragment() const { return !!fragment_context_.Fragment(); }
  bool IsParsingTemplateContents() const {
    return tree_.OpenElements()->HasTemplateInHTMLScope();
  }
  bool IsParsingFragmentOrTemplateContents() const {
    return IsParsingFragment() || IsParsingTemplateContents();
  }

  void SetDOMPartsAllowedState(DOMPartsAllowed state) {
    DCHECK(RuntimeEnabledFeatures::DOMPartsAPIEnabled());
    tree_.SetDOMPartsAllowedState(state);
  }

  void Detach();

  void ConstructTree(AtomicHTMLToken*);

  ALWAYS_INLINE bool HasParserBlockingScript() const {
    return !!script_to_process_;
  }
  // Must be called to take the parser-blocking script before calling the parser
  // again.
  Element* TakeScriptToProcess(TextPosition& script_start_position);

  // Done, close any open tags, etc.
  void Finished();

  // Synchronously flush pending text and queued tasks, possibly creating more
  // DOM nodes. Flushing pending text depends on |mode|.
  void Flush() { tree_.Flush(); }

  void SetShouldSkipLeadingNewline(bool should_skip) {
    should_skip_leading_newline_ = should_skip;
  }

 private:
  class CharacterTokenBuffer;
  // Represents HTML5 "insertion mode"
  // http://www.whatwg.org/specs/web-apps/current-work/multipage/parsing.html#insertion-mode
  enum InsertionMode {
    kInitialMode,
    kBeforeHTMLMode,
    kBeforeHeadMode,
    kInHeadMode,
    kInHeadNoscriptMode,
    kAfterHeadMode,
    kTemplateContentsMode,
    kInBodyMode,
    kTextMode,
    kInTableMode,
    kInTableTextMode,
    kInCaptionMode,
    kInColumnGroupMode,
    kInTableBodyMode,
    kInRowMode,
    kInCellMode,
    kInSelectMode,
    kInSelectInTableMode,
    kAfterBodyMode,
    kInFramesetMode,
    kAfterFramesetMode,
    kAfterAfterBodyMode,
    kAfterAfterFramesetMode,
  };
#ifndef NDEBUG
  static const char* ToString(InsertionMode);
#endif

  void ProcessToken(AtomicHTMLToken*);

  void ProcessDoctypeToken(AtomicHTMLToken*);
  void ProcessStartTag(AtomicHTMLToken*);
  void ProcessEndTag(AtomicHTMLToken*);
  void ProcessComment(AtomicHTMLToken*);
  void ProcessCharacter(AtomicHTMLToken*);
  void ProcessEndOfFile(AtomicHTMLToken*);
  void ProcessDOMPart(AtomicHTMLToken*);

  bool ProcessStartTagForInHead(AtomicHTMLToken*);
  void ProcessStartTagForInBody(AtomicHTMLToken*);
  void ProcessStartTagForInTable(AtomicHTMLToken*);
  void ProcessEndTagForInBody(AtomicHTMLToken*);
  void ProcessEndTagForInTable(AtomicHTMLToken*);
  void ProcessEndTagForInTableBody(AtomicHTMLToken*);
  void ProcessEndTagForInRow(AtomicHTMLToken*);
  void ProcessEndTagForInCell(AtomicHTMLToken*);

  void ProcessHtmlStartTagForInBody(AtomicHTMLToken*);
  bool ProcessBodyEndTagForInBody(AtomicHTMLToken*);
  bool ProcessTableEndTagForInTable();
  bool ProcessCaptionEndTagForInCaption();
  bool ProcessColgroupEndTagForInColumnGroup();
  bool ProcessTrEndTagForInRow();
  // FIXME: This function should be inlined into its one call site or it
  // needs to assert which tokens it can be called with.
  void ProcessAnyOtherEndTagForInBody(AtomicHTMLToken*);

  void ProcessCharacterBuffer(CharacterTokenBuffer&);
  inline void ProcessCharacterBufferForInBody(CharacterTokenBuffer&);

  void ProcessFakeStartTag(
      html_names::HTMLTag tag,
      const Vector<Attribute>& attributes = Vector<Attribute>());
  void ProcessFakeEndTag(html_names::HTMLTag tag);
  void ProcessFakeEndTag(const HTMLStackItem& stack_item);
  void ProcessFakePEndTagIfPInButtonScope();

  void ProcessGenericRCDATAStartTag(AtomicHTMLToken*);
  void ProcessGenericRawTextStartTag(AtomicHTMLToken*);
  void ProcessScriptStartTag(AtomicHTMLToken*);

  // Default processing for the different insertion modes.
  void DefaultForInitial();
  void DefaultForBeforeHTML();
  void DefaultForBeforeHead();
  void DefaultForInHead();
  void DefaultForInHeadNoscript();
  void DefaultForAfterHead();
  void DefaultForInTableText();

  inline HTMLStackItem* AdjustedCurrentStackItem() const;
  inline bool ShouldProcessTokenInForeignContent(AtomicHTMLToken*);
  void ProcessTokenInForeignContent(AtomicHTMLToken*);

  void CallTheAdoptionAgency(AtomicHTMLToken*);

  void CloseTheCell();

  template <bool shouldClose(const HTMLStackItem*)>
  void ProcessCloseWhenNestedTag(AtomicHTMLToken*);

  void ParseError(AtomicHTMLToken*);

  InsertionMode GetInsertionMode() const { return insertion_mode_; }
  void SetInsertionMode(InsertionMode mode) { insertion_mode_ = mode; }

  void ResetInsertionModeAppropriately();

  void ProcessTemplateStartTag(AtomicHTMLToken*);
  bool ProcessTemplateEndTag(AtomicHTMLToken*);
  bool ProcessEndOfFileForInTemplateContents(AtomicHTMLToken*);

  class FragmentParsingContext {
    DISALLOW_NEW();

   public:
    FragmentParsingContext() = default;
    FragmentParsingContext(const FragmentParsingContext&) = delete;
    FragmentParsingContext& operator=(const FragmentParsingContext&) = delete;
    void Init(DocumentFragment*, Element* context_element);

    DocumentFragment* Fragment() const { return fragment_.Get(); }
    Element* ContextElement() const {
      DCHECK(fragment_);
      return context_element_stack_item_->GetElement();
    }
    HTMLStackItem* ContextElementStackItem() const {
      DCHECK(fragment_);
      return context_element_stack_item_.Get();
    }

    void Trace(Visitor*) const;

   private:
    Member<DocumentFragment> fragment_;
    Member<HTMLStackItem> context_element_stack_item_;
  };

  // https://html.spec.whatwg.org/C/#frameset-ok-flag
  FragmentParsingContext fragment_context_;
  HTMLConstructionSite tree_;

  // http://www.whatwg.org/specs/web-apps/current-work/multipage/parsing.html#insertion-mode
  InsertionMode insertion_mode_;

  // http://www.whatwg.org/specs/web-apps/current-work/multipage/parsing.html#original-insertion-mode
  InsertionMode original_insertion_mode_;

  Vector<InsertionMode> template_insertion_modes_;

  // http://www.whatwg.org/specs/web-apps/current-work/multipage/tokenization.html#pending-table-character-tokens
  StringBuilder pending_table_characters_;

  bool should_skip_leading_newline_;

  const bool include_shadow_roots_;

  bool frameset_ok_;
#if DCHECK_IS_ON()
  bool is_attached_ = true;
#endif

  // We access parser because HTML5 spec requires that we be able to change the
  // state of the tokenizer from within parser actions. We also need it to track
  // the current position.
  Member<HTMLDocumentParser> parser_;

  // <script> tag which needs processing before resuming the parser.
  Member<Element> script_to_process_;

  // Starting line number of the script tag needing processing.
  TextPosition script_to_process_start_position_;

  HTMLParserOptions options_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_TREE_BUILDER_H_
