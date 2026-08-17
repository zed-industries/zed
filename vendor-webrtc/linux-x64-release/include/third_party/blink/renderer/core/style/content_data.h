/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008, 2010 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2006 Graham Dennis (graham.dennis@gmail.com)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CONTENT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CONTENT_DATA_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_image.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

#include <iosfwd>

namespace blink {

class LayoutObject;
class TreeScope;

class ContentData : public GarbageCollected<ContentData> {
 public:
  virtual ~ContentData() = default;

  virtual bool IsCounter() const { return false; }
  virtual bool IsImage() const { return false; }
  virtual bool IsQuote() const { return false; }
  virtual bool IsText() const { return false; }
  virtual bool IsAltText() const { return false; }
  virtual bool IsNone() const { return false; }

  // Create a layout object for this piece of content. `owner` is the layout
  // object that has the content property, e.g. a pseudo element, or an @page
  // margin box.
  virtual LayoutObject* CreateLayoutObject(LayoutObject& owner) const = 0;

  virtual ContentData* Clone() const;

  ContentData* Next() const { return next_.Get(); }
  void SetNext(ContentData* next) {
    DCHECK(!IsNone());
    DCHECK(!next || !next->IsNone());
    next_ = next;
  }

  virtual bool Equals(const ContentData&) const = 0;

  virtual void Trace(Visitor*) const;

  // For debugging/logging only.
  virtual String DebugString() const { return "<unknown>"; }

 private:
  virtual ContentData* CloneInternal() const = 0;

  Member<ContentData> next_;

  friend std::ostream& operator<<(std::ostream& stream,
                                  const ContentData& content_data);
};

inline std::ostream& operator<<(std::ostream& stream,
                                const ContentData& content_data) {
  const ContentData* ptr = &content_data;
  stream << "ContentData{";
  while (ptr) {
    stream << content_data.DebugString();
    stream << ",";
    ptr = ptr->next_.Get();
  }
  return stream << "}";
}

class ImageContentData final : public ContentData {
  friend class ContentData;

 public:
  explicit ImageContentData(StyleImage* image) : image_(image) {
    DCHECK(image_);
  }

  const StyleImage* GetImage() const { return image_.Get(); }
  StyleImage* GetImage() { return image_.Get(); }
  void SetImage(StyleImage* image) {
    DCHECK(image);
    image_ = image;
  }

  bool IsImage() const override { return true; }
  LayoutObject* CreateLayoutObject(LayoutObject& owner) const override;

  bool Equals(const ContentData& data) const override {
    if (!data.IsImage()) {
      return false;
    }
    return *static_cast<const ImageContentData&>(data).GetImage() ==
           *GetImage();
  }

  void Trace(Visitor*) const override;

  String DebugString() const override {
    StringBuilder str;
    str.Append("<image: ");
    if (image_->IsImageResource()) {
      str.Append("[is_resource]");
    }
    if (image_->IsPendingImage()) {
      str.Append("[pending]");
    }
    if (image_->IsGeneratedImage()) {
      str.Append("[generated]");
    }
    if (image_->IsContentful()) {
      str.Append("[contentful]");
    }
    if (image_->IsImageResourceSet()) {
      str.Append("[resourceset]");
    }
    if (image_->IsPaintImage()) {
      str.Append("[paint]");
    }
    if (image_->IsCrossfadeImage()) {
      str.Append("[crossfade]");
    }
    return str + image_->CssValue()->CssText() + ">";
  }

 private:
  ContentData* CloneInternal() const override {
    StyleImage* image = const_cast<StyleImage*>(GetImage());
    return MakeGarbageCollected<ImageContentData>(image);
  }

  Member<StyleImage> image_;
};

template <>
struct DowncastTraits<ImageContentData> {
  static bool AllowFrom(const ContentData& content) {
    return content.IsImage();
  }
};

class TextContentData final : public ContentData {
  friend class ContentData;

 public:
  explicit TextContentData(const String& text) : text_(text) {}

  const String& GetText() const { return text_; }
  void SetText(const String& text) { text_ = text; }

  bool IsText() const override { return true; }
  LayoutObject* CreateLayoutObject(LayoutObject& owner) const override;

  bool Equals(const ContentData& data) const override {
    if (!data.IsText()) {
      return false;
    }
    return static_cast<const TextContentData&>(data).GetText() == GetText();
  }

  String DebugString() const override { return text_; }

 private:
  ContentData* CloneInternal() const override {
    return MakeGarbageCollected<TextContentData>(GetText());
  }

  String text_;
};

template <>
struct DowncastTraits<TextContentData> {
  static bool AllowFrom(const ContentData& content) { return content.IsText(); }
};

class AltTextContentData final : public ContentData {
  friend class ContentData;

 public:
  explicit AltTextContentData(const String& text) : text_(text) {}

  String GetText() const { return text_; }
  String ConcatenateAltText() const {
    StringBuilder alt_text;
    for (const ContentData* content_data = this; content_data;
         content_data = content_data->Next()) {
      alt_text.Append(To<AltTextContentData>(content_data)->GetText());
    }
    return alt_text.ToString();
  }
  void SetText(const String& text) { text_ = text; }

  bool IsAltText() const override { return true; }
  LayoutObject* CreateLayoutObject(LayoutObject& owner) const override;

  bool Equals(const ContentData& data) const override {
    if (!data.IsAltText()) {
      return false;
    }
    return static_cast<const AltTextContentData&>(data).GetText() == GetText();
  }

  String DebugString() const override { return "<alt: " + text_ + ">"; }

 private:
  ContentData* CloneInternal() const override {
    return MakeGarbageCollected<AltTextContentData>(GetText());
  }
  String text_;
};

template <>
struct DowncastTraits<AltTextContentData> {
  static bool AllowFrom(const ContentData& content) {
    return content.IsAltText();
  }
};

class CounterContentData final : public ContentData {
  friend class ContentData;

 public:
  explicit CounterContentData(const AtomicString& identifier,
                              const AtomicString& style,
                              const AtomicString& separator,
                              const TreeScope* tree_scope)
      : identifier_(identifier),
        list_style_(style),
        separator_(separator),
        tree_scope_(tree_scope) {}

  bool IsCounter() const override { return true; }
  LayoutObject* CreateLayoutObject(LayoutObject& owner) const override;

  const AtomicString& Identifier() const { return identifier_; }
  const AtomicString& ListStyle() const { return list_style_; }
  const AtomicString& Separator() const { return separator_; }
  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }

  void Trace(Visitor*) const override;

  String DebugString() const override { return "<counter>"; }

 private:
  ContentData* CloneInternal() const override {
    return MakeGarbageCollected<CounterContentData>(identifier_, list_style_,
                                                    separator_, tree_scope_);
  }

  bool Equals(const ContentData& data) const override {
    if (!data.IsCounter()) {
      return false;
    }
    const CounterContentData& other =
        static_cast<const CounterContentData&>(data);
    return Identifier() == other.Identifier() &&
           ListStyle() == other.ListStyle() &&
           Separator() == other.Separator() &&
           GetTreeScope() == other.GetTreeScope();
  }

  AtomicString identifier_;
  AtomicString list_style_;
  AtomicString separator_;
  Member<const TreeScope> tree_scope_;
};

template <>
struct DowncastTraits<CounterContentData> {
  static bool AllowFrom(const ContentData& content) {
    return content.IsCounter();
  }
};

class QuoteContentData final : public ContentData {
  friend class ContentData;

 public:
  explicit QuoteContentData(QuoteType quote) : quote_(quote) {}

  QuoteType Quote() const { return quote_; }
  void SetQuote(QuoteType quote) { quote_ = quote; }

  bool IsQuote() const override { return true; }
  LayoutObject* CreateLayoutObject(LayoutObject& owner) const override;

  bool Equals(const ContentData& data) const override {
    if (!data.IsQuote()) {
      return false;
    }
    return static_cast<const QuoteContentData&>(data).Quote() == Quote();
  }

  String DebugString() const override { return "<quote>"; }

 private:
  ContentData* CloneInternal() const override {
    return MakeGarbageCollected<QuoteContentData>(Quote());
  }

  QuoteType quote_;
};

template <>
struct DowncastTraits<QuoteContentData> {
  static bool AllowFrom(const ContentData& content) {
    return content.IsQuote();
  }
};

class NoneContentData final : public ContentData {
  friend class ContentData;

 public:
  explicit NoneContentData() {}

  bool IsNone() const override { return true; }
  LayoutObject* CreateLayoutObject(LayoutObject& owner) const override;

  bool Equals(const ContentData& data) const override { return data.IsNone(); }

  String DebugString() const override { return "<none>"; }

 private:
  ContentData* CloneInternal() const override {
    return MakeGarbageCollected<NoneContentData>();
  }
};

template <>
struct DowncastTraits<NoneContentData> {
  static bool AllowFrom(const ContentData& content) { return content.IsNone(); }
};

inline bool operator==(const ContentData& a, const ContentData& b) {
  const ContentData* ptr_a = &a;
  const ContentData* ptr_b = &b;

  while (ptr_a && ptr_b && ptr_a->Equals(*ptr_b)) {
    ptr_a = ptr_a->Next();
    ptr_b = ptr_b->Next();
  }

  return !ptr_a && !ptr_b;
}

// In order for an image to be rendered from the content property on an actual
// element, there can be at most one piece of image content data, followed by
// some optional alternative text.
inline bool ShouldUseContentDataForElement(const ContentData* content_data) {
  if (!content_data) {
    return false;
  }
  if (!content_data->IsImage()) {
    return false;
  }
  if (content_data->Next() && !content_data->Next()->IsAltText()) {
    return false;
  }

  return true;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_CONTENT_DATA_H_
