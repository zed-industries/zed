/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * (C) 2002-2003 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2002, 2006, 2008, 2012 Apple Inc. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_IMPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_IMPORT_H_

#include "third_party/blink/renderer/core/css/css_origin_clean.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class MediaQuerySet;
class StyleScope;
class StyleSheetContents;

class StyleRuleImport : public StyleRuleBase {
  USING_PRE_FINALIZER(StyleRuleImport, Dispose);

 public:
  StyleRuleImport(const String& href,
                  LayerName&& layer,
                  const StyleScope*,
                  bool supported,
                  String supports,
                  const MediaQuerySet*,
                  OriginClean origin_clean);
  ~StyleRuleImport();

  StyleSheetContents* ParentStyleSheet() const {
    return parent_style_sheet_.Get();
  }
  void SetParentStyleSheet(StyleSheetContents* sheet) {
    DCHECK(sheet);
    parent_style_sheet_ = sheet;
  }
  void ClearParentStyleSheet() { parent_style_sheet_ = nullptr; }

  String Href() const { return str_href_; }
  StyleSheetContents* GetStyleSheet() const { return style_sheet_.Get(); }

  bool IsLoading() const;

  const MediaQuerySet* MediaQueries() const { return media_queries_.Get(); }
  void SetMediaQueries(const MediaQuerySet* media_queries) {
    media_queries_ = media_queries;
  }

  void SetPositionHint(const TextPosition& position_hint) {
    position_hint_ = position_hint;
  }
  void RequestStyleSheet();

  bool IsLayered() const { return layer_.size(); }
  const LayerName& GetLayerName() const { return layer_; }
  String GetLayerNameAsString() const;

  const StyleScope* GetScope() const { return scope_.Get(); }

  bool IsSupported() const { return supported_; }
  String GetSupportsString() const { return supports_string_; }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  // FIXME: inherit from ResourceClient directly to eliminate back
  // pointer, as there are no space savings in this.
  // NOTE: We put the ResourceClient in a member instead of inheriting
  // from it to avoid adding a vptr to StyleRuleImport.
  class ImportedStyleSheetClient final
      : public GarbageCollected<ImportedStyleSheetClient>,
        public ResourceClient {
   public:
    ImportedStyleSheetClient(StyleRuleImport* owner_rule)
        : owner_rule_(owner_rule) {}
    ~ImportedStyleSheetClient() override = default;
    void NotifyFinished(Resource* resource) override {
      owner_rule_->NotifyFinished(resource);
    }
    void Dispose() { ClearResource(); }

    String DebugName() const override { return "ImportedStyleSheetClient"; }

    void Trace(Visitor* visitor) const override {
      visitor->Trace(owner_rule_);
      ResourceClient::Trace(visitor);
    }

   private:
    Member<StyleRuleImport> owner_rule_;
  };

  void NotifyFinished(Resource*);

  void Dispose();

  Member<StyleSheetContents> parent_style_sheet_;

  Member<ImportedStyleSheetClient> style_sheet_client_;
  String str_href_;
  LayerName layer_;
  Member<const StyleScope> scope_;
  String supports_string_;
  Member<const MediaQuerySet> media_queries_;
  Member<StyleSheetContents> style_sheet_;
  bool loading_;
  bool supported_;
  // Whether the style sheet that has this import rule is origin-clean:
  // https://drafts.csswg.org/cssom-1/#concept-css-style-sheet-origin-clean-flag
  const OriginClean origin_clean_;

  // If set, this holds the position of the import rule (start of the `@import`)
  // in the stylesheet text. The position is used to encode accurate initiator
  // info on the stylesheet request in order to report accurate failures.
  std::optional<TextPosition> position_hint_;
};

template <>
struct DowncastTraits<StyleRuleImport> {
  static bool AllowFrom(const StyleRuleBase& rule) {
    return rule.IsImportRule();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_RULE_IMPORT_H_
