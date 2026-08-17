/*
 * Copyright (C) 2003, 2006, 2008 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_FAMILY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_FAMILY_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace blink {

class SharedFontFamily;

class PLATFORM_EXPORT FontFamily {
  DISALLOW_NEW();

 public:
  // https://drafts.csswg.org/css-fonts/#font-family-prop
  enum class Type : uint8_t { kFamilyName, kGenericFamily };

  FontFamily(const AtomicString& family_name,
             Type family_type,
             scoped_refptr<SharedFontFamily> next = nullptr)
      : family_name_(family_name),
        next_(std::move(next)),
        family_type_(family_type) {}
  FontFamily() = default;
  ~FontFamily();

  // Return this font family's name. Note that it is never quoted nor escaped.
  // For web-exposed serialization, please rely instead on the functions
  // ComputedStyleUtils::ValueForFontFamily(const FontFamily&) and
  // CSSValue::CssText() in order to match formatting rules from the CSSOM
  // specification.
  const AtomicString& FamilyName() const { return family_name_; }
  bool FamilyIsGeneric() const { return family_type_ == Type::kGenericFamily; }

  const FontFamily* Next() const;

  scoped_refptr<SharedFontFamily> ReleaseNext();

  bool IsPrewarmed() const { return is_prewarmed_; }
  void SetIsPrewarmed() const { is_prewarmed_ = true; }

  // Returns this font family's name followed by all subsequent linked
  // families separated ", " (comma and space). Font family names are never
  // quoted nor escaped. For web-exposed serialization, please rely instead on
  // the functions ComputedStyleUtils::ValueForFontFamily(const FontFamily&) and
  // CSSValue::CssText() in order to match formatting rules from the CSSOM
  // specification.
  String ToString() const;

  // Return kGenericFamily if family_name is equal to one of the supported
  // <generic-family> keyword from the CSS fonts module spec and kFamilyName
  // otherwise.
  static Type InferredTypeFor(const AtomicString& family_name);

 private:
  AtomicString family_name_;
  scoped_refptr<SharedFontFamily> next_;
  Type family_type_ = Type::kFamilyName;
  mutable bool is_prewarmed_ = false;
};

class PLATFORM_EXPORT SharedFontFamily : public FontFamily,
                                         public RefCounted<SharedFontFamily> {
  USING_FAST_MALLOC(SharedFontFamily);
 public:
  SharedFontFamily(const SharedFontFamily&) = delete;
  SharedFontFamily& operator=(const SharedFontFamily&) = delete;

  static scoped_refptr<SharedFontFamily> Create(
      const AtomicString& family_name,
      Type family_type,
      scoped_refptr<SharedFontFamily> next = nullptr) {
    return base::AdoptRef(
        new SharedFontFamily(family_name, family_type, std::move(next)));
  }

 private:
  SharedFontFamily(const AtomicString& family_name,
                   Type family_type,
                   scoped_refptr<SharedFontFamily> next)
      : FontFamily(family_name, family_type, std::move(next)) {}
};

PLATFORM_EXPORT bool operator==(const FontFamily&, const FontFamily&);
inline bool operator!=(const FontFamily& a, const FontFamily& b) {
  return !(a == b);
}

inline FontFamily::~FontFamily() {
  scoped_refptr<SharedFontFamily> reaper = std::move(next_);
  while (reaper && reaper->HasOneRef()) {
    // implicitly protects reaper->next, then derefs reaper
    reaper = reaper->ReleaseNext();
  }
}

inline const FontFamily* FontFamily::Next() const {
  return next_.get();
}

inline scoped_refptr<SharedFontFamily> FontFamily::ReleaseNext() {
  return std::move(next_);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_FAMILY_H_
