/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2006, 2008, 2009, 2010, 2012 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_query.h"
#include "third_party/blink/renderer/core/layout/geometry/axis.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSRule;
class CSSStyleSheet;
class ExceptionState;
class ExecutionContext;
class MediaList;
class MediaQuery;
class MediaQuerySetOwner;

class CORE_EXPORT MediaQuerySet : public GarbageCollected<MediaQuerySet> {
 public:
  static MediaQuerySet* Create() {
    return MakeGarbageCollected<MediaQuerySet>();
  }
  static MediaQuerySet* Create(const String& media_string, ExecutionContext*);

  MediaQuerySet();
  MediaQuerySet(const MediaQuerySet&);
  explicit MediaQuerySet(HeapVector<Member<const MediaQuery>>);
  void Trace(Visitor*) const;

  const MediaQuerySet* CopyAndAdd(const String&, ExecutionContext*) const;
  const MediaQuerySet* CopyAndRemove(const String&, ExecutionContext*) const;

  const HeapVector<Member<const MediaQuery>>& QueryVector() const {
    return queries_;
  }

  String MediaText() const;

 private:
  HeapVector<Member<const MediaQuery>> queries_;
};

class CORE_EXPORT MediaList final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit MediaList(CSSStyleSheet* parent_sheet);
  explicit MediaList(CSSRule* parent_rule);

  unsigned length() const { return Queries()->QueryVector().size(); }
  String item(unsigned index) const;
  void deleteMedium(ExecutionContext*,
                    const String& old_medium,
                    ExceptionState&);
  void appendMedium(ExecutionContext*, const String& new_medium);

  // Note that this getter doesn't require the ExecutionContext (except for
  // crbug.com/1268860 use-counting), but the attribute is marked as
  // [CallWith=ExecutionContext] so that the setter can have access to the
  // ExecutionContext.
  //
  // Prefer MediaTextInternal for internal use. (Avoids use-counter).
  String mediaText(ExecutionContext*) const;
  void setMediaText(ExecutionContext*, const String&);
  String MediaTextInternal() const { return Queries()->MediaText(); }

  // Not part of CSSOM.
  CSSRule* ParentRule() const { return parent_rule_.Get(); }
  CSSStyleSheet* ParentStyleSheet() const { return parent_style_sheet_.Get(); }

  const MediaQuerySet* Queries() const;

  void Trace(Visitor*) const override;

 private:
  MediaQuerySetOwner* Owner() const;
  void NotifyMutation();

  Member<CSSStyleSheet> parent_style_sheet_;
  Member<CSSRule> parent_rule_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_LIST_H_
