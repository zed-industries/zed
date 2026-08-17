// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_CONTEXT_H_

#include "base/auto_reset.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_resource_fetch_restriction.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class CSSStyleSheet;
class Document;
class ExecutionContext;
class StyleSheetContents;
enum class SecureContextMode;

class CORE_EXPORT CSSParserContext final
    : public GarbageCollected<CSSParserContext> {
 public:
  // All three of these constructors copy the context and override the current
  // Document handle used for UseCounter.
  CSSParserContext(const CSSParserContext*, const CSSStyleSheet*);
  CSSParserContext(const CSSParserContext*, const StyleSheetContents*);
  // FIXME: This constructor shouldn't exist if we properly piped the UseCounter
  // through the CSS subsystem. Currently the UseCounter life time is too crazy
  // and we need a way to override it.
  explicit CSSParserContext(const CSSParserContext* other,
                            const Document* use_counter_document = nullptr);

  // Creates a context with most of its constructor attributes provided by
  // copying from |other|, except that the remaining constructor arguments take
  // precedence over the corresponding characteristics of |other|. This is
  // useful for initializing @imported sheets' contexts, which inherit most of
  // their characteristics from their parents.
  CSSParserContext(const CSSParserContext* other,
                   const KURL& base_url_override,
                   bool origin_clean,
                   const Referrer& referrer,
                   const WTF::TextEncoding& charset_override,
                   const Document* use_counter_document);
  CSSParserContext(CSSParserMode,
                   SecureContextMode,
                   const Document* use_counter_document = nullptr);
  explicit CSSParserContext(const Document&);
  CSSParserContext(const Document&, const KURL& base_url_override);
  CSSParserContext(const Document&,
                   const KURL& base_url_override,
                   bool origin_clean,
                   const Referrer& referrer,
                   const WTF::TextEncoding& charset = WTF::TextEncoding(),
                   ResourceFetchRestriction resource_fetch_restriction =
                       ResourceFetchRestriction::kNone);

  // This is used for workers, where we don't have a document.
  CSSParserContext(const ExecutionContext& context);

  CSSParserContext(const KURL& base_url,
                   bool origin_clean,
                   const WTF::TextEncoding& charset,
                   CSSParserMode,
                   const Referrer& referrer,
                   bool is_html_document,
                   SecureContextMode,
                   const DOMWrapperWorld* world,
                   const Document* use_counter_document,
                   ResourceFetchRestriction resource_fetch_restriction);

  bool operator==(const CSSParserContext&) const;
  bool operator!=(const CSSParserContext& other) const {
    return !(*this == other);
  }

  CSSParserMode Mode() const { return mode_; }
  const KURL& BaseURL() const { return base_url_; }
  const WTF::TextEncoding& Charset() const { return charset_; }
  const Referrer& GetReferrer() const { return referrer_; }
  bool IsAdRelated() const { return is_ad_related_; }
  bool IsHTMLDocument() const { return is_html_document_; }
  enum ResourceFetchRestriction ResourceFetchRestriction() const {
    return resource_fetch_restriction_;
  }

  bool IsOriginClean() const;
  bool IsSecureContext() const;

  // FIXME: This setter shouldn't exist, however the current lifetime of
  // CSSParserContext is not well understood and thus we sometimes need to
  // override this field.
  void SetMode(CSSParserMode mode) { mode_ = mode; }
  CSSParserMode GetMode() const { return mode_; }

  void SetIsAdRelated() { is_ad_related_ = true; }

  KURL CompleteURL(const String& url) const;

  // Like CompleteURL(), but if `url` is empty a null KURL is returned.
  KURL CompleteNonEmptyURL(const String& url) const;

  SecureContextMode GetSecureContextMode() const {
    return secure_context_mode_;
  }

  void Count(WebFeature) const;
  void Count(WebDXFeature) const;
  void Count(CSSParserMode, CSSPropertyID) const;
  void CountDeprecation(WebFeature) const;
  bool IsUseCounterRecordingEnabled() const { return document_ != nullptr; }
  bool IsDocumentHandleEqual(const Document* other) const;
  const Document* GetDocument() const;
  ExecutionContext* GetExecutionContext() const;

  const DOMWrapperWorld* JavascriptWorld() const { return world_.Get(); }

  bool IsForMarkupSanitization() const;

  // Returns true if we are in a parsing mode where the result will be used in
  // an element context. This function is used to fail parsing of functions such
  // as sibling-index() which do not make sense in @page or @font-face
  // descriptors, for instance.
  bool InElementContext() const;

  // Overrides |mode_| of a CSSParserContext within the scope, allowing us to
  // switching parsing mode while parsing different parts of a style sheet.
  // TODO(xiaochengh): This isn't the right approach, as it breaks the
  // immutability of CSSParserContext. We should introduce some local context.
  class ParserModeOverridingScope {
    STACK_ALLOCATED();

   public:
    ParserModeOverridingScope(const CSSParserContext& context,
                              CSSParserMode mode)
        : mode_reset_(const_cast<CSSParserMode*>(&context.mode_), mode) {}

   private:
    base::AutoReset<CSSParserMode> mode_reset_;
  };

  void Trace(Visitor*) const;

 private:
  friend class ParserModeOverridingScope;

  KURL base_url_;

  const Member<const DOMWrapperWorld> world_;

  // If true, allows reading and modifying of the CSS rules.
  // https://drafts.csswg.org/cssom/#concept-css-style-sheet-origin-clean-flag
  const bool origin_clean_;

  CSSParserMode mode_;
  Referrer referrer_;

  // Whether the associated stylesheet's ResourceRequest is an ad resource. If
  // there is no associated ResourceRequest, whether ad script is on the v8 call
  // stack at stylesheet creation. Not set for presentation attributes.
  bool is_ad_related_ = false;
  bool is_html_document_;
  SecureContextMode secure_context_mode_;

  WTF::TextEncoding charset_;

  WeakMember<const Document> document_;

  // Flag indicating whether images with a URL scheme other than "data" are
  // allowed.
  const enum ResourceFetchRestriction resource_fetch_restriction_;
};

CORE_EXPORT const CSSParserContext* StrictCSSParserContext(SecureContextMode);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_CONTEXT_H_
