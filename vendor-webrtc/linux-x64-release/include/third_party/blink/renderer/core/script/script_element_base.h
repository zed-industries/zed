/*
 * Copyright (C) 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2013 Google Inc. All rights reserved.
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
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_ELEMENT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_ELEMENT_BASE_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/create_element_flags.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;
class Element;
class ExecutionContext;
class HTMLScriptElementOrSVGScriptElement;
class ScriptLoader;

ScriptLoader* ScriptLoaderFromElement(Element*);

class CORE_EXPORT ScriptElementBase : public GarbageCollectedMixin {
 public:
  enum class Type { kHTMLScriptElement, kSVGScriptElement };
  virtual bool AsyncAttributeValue() const = 0;
  virtual String CharsetAttributeValue() const = 0;
  virtual String CrossOriginAttributeValue() const = 0;
  virtual bool DeferAttributeValue() const = 0;
  virtual String EventAttributeValue() const = 0;
  virtual String ForAttributeValue() const = 0;
  virtual String IntegrityAttributeValue() const = 0;
  virtual String SignatureAttributeValue() const = 0;
  virtual String LanguageAttributeValue() const = 0;
  virtual bool NomoduleAttributeValue() const = 0;
  virtual String SourceAttributeValue() const = 0;
  virtual String TypeAttributeValue() const = 0;
  virtual String ReferrerPolicyAttributeValue() const = 0;
  virtual String FetchPriorityAttributeValue() const = 0;

  // This implements https://dom.spec.whatwg.org/#concept-child-text-content
  virtual String ChildTextContent() = 0;
  // This supports
  // https://w3c.github.io/trusted-types/dist/spec/#prepare-script-url-and-text
  virtual String ScriptTextInternalSlot() const = 0;
  virtual bool HasSourceAttribute() const = 0;
  virtual bool HasAttributionsrcAttribute() const = 0;
  virtual bool IsConnected() const = 0;
  virtual bool HasChildren() const = 0;
  virtual const AtomicString& GetNonceForElement() const = 0;
  virtual bool ElementHasDuplicateAttributes() const = 0;

  // https://html.spec.whatwg.org/C/#potentially-render-blocking
  virtual bool IsPotentiallyRenderBlocking() const = 0;

  // Whether the inline script is allowed by the CSP. Must be called
  // synchronously to ensure the correct Javascript world is used for CSP
  // checks.
  virtual bool AllowInlineScriptForCSP(const AtomicString& nonce,
                                       const WTF::OrdinalNumber&,
                                       const String& script_content) = 0;

  // GetDocument() is "element document", to which the script element belongs
  // and a parser is attached (in the case of parser-inserted scripts).
  // GetExecutionContext() is "context window" (LocalDOMWindow),
  // in which script should be fetched and evaluated,
  // and its document() was previously known as "context document".
  // The distinction between the element document and the context document is
  // important in HTML imports, where:
  // The element document is HTML-imported Document, and
  // The context document is the parent Document that triggers HTML imports.
  //
  // TODO(hiroshige): After HTML imports implementation is removed, merge
  // context documents into element documents, because without HTML imports,
  // they are the same wherever scripting is enabled.
  // Even after that, some uses of context window will remain, simply as
  // script element's node document's relevant settings object.
  virtual Document& GetDocument() const = 0;
  virtual ExecutionContext* GetExecutionContext() const = 0;

  virtual V8HTMLOrSVGScriptElement* AsV8HTMLOrSVGScriptElement() = 0;
  virtual DOMNodeId GetDOMNodeId() = 0;

  virtual void DispatchLoadEvent() = 0;
  virtual void DispatchErrorEvent() = 0;

  virtual Type GetScriptElementType() = 0;

 protected:
  ScriptLoader* InitializeScriptLoader(CreateElementFlags);

  virtual ScriptLoader* Loader() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_ELEMENT_BASE_H_
