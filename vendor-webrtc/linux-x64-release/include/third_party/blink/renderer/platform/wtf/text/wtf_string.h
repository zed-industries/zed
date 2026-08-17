/*
 * (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012, 2013 Apple Inc.
 * All rights reserved.
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

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_WTF_STRING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_WTF_STRING_H_

// This file would be called String.h, but that conflicts with <string.h>
// on systems without case-sensitive file systems.

#include <array>
#include <iosfwd>
#include <string_view>
#include <type_traits>

#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/integer_to_string_conversion.h"
#include "third_party/blink/renderer/platform/wtf/text/string_impl.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace WTF {

class CodePointIterator;

#define DISPATCH_CASE_OP(case_sensitivity, op, args)  \
  ((case_sensitivity == kTextCaseSensitive) ? op args \
                                            : op##IgnoringASCIICase args)

// You can find documentation about this class in README.md in this directory.
class WTF_EXPORT String {
  USING_FAST_MALLOC(String);

 public:
  // Construct a null string, distinguishable from an empty string.
  String() = default;

  // Construct a string with UTF-16 data.
  explicit String(base::span<const UChar> utf16_data);

  // Construct a string by copying the contents of a vector.
  // This method will never create a null string. Vectors with size() == 0
  // will return the empty string.
  // NOTE: This is different from String(vector.data(), vector.size())
  // which will sometimes return a null string when vector.data() is null
  // which can only occur for vectors without inline capacity.
  // See: https://bugs.webkit.org/show_bug.cgi?id=109792
  template <wtf_size_t inlineCapacity>
  explicit String(const Vector<UChar, inlineCapacity>&);

  // Construct a string with UTF-16 data, from a null-terminated source.
  String(const UChar*);

  // Construct a string with latin1 data.
  explicit String(base::span<const LChar> latin1_data);
  explicit String(base::span<const char> latin1_data)
      : String(base::as_bytes(latin1_data)) {}
  explicit String(const std::string& s) : String(base::as_byte_span(s)) {}

  // Construct a string with latin1 data, from a null-terminated source. The
  // `LChar` constructor is explicit to avoid misinterpreting byte arrays.
  // If the conversion is implicit, functions with both `String` and
  // `base::span<const uint8_t>` overloads become ambiguous when called on
  // `uint8_t[N]`.
  explicit String(const LChar* characters)
      : String(reinterpret_cast<const char*>(characters)) {}
  String(const char* characters)  // NOLINT(google-explicit-constructor)
      : String(base::span(characters, characters ? strlen(characters) : 0)) {}

  // Construct a string referencing an existing StringImpl.
  String(StringImpl* impl) : impl_(impl) {}
  String(scoped_refptr<StringImpl> impl) : impl_(std::move(impl)) {}

  // Copying a String is a relatively inexpensive, since the underlying data is
  // immutable and refcounted.
  String(const String&) = default;
  String& operator=(const String&) = default;

  String(String&&) noexcept = default;
  String& operator=(String&&) = default;

  void swap(String& o) { impl_.swap(o.impl_); }

  template <typename CharType>
  static String Adopt(StringBuffer<CharType>& buffer) {
    if (!buffer.length())
      return StringImpl::empty_;
    return String(buffer.Release());
  }

  explicit operator bool() const { return !IsNull(); }
  bool IsNull() const { return !impl_; }
  bool empty() const { return !impl_ || !impl_->length(); }

  StringImpl* Impl() const { return impl_.get(); }
  scoped_refptr<StringImpl> ReleaseImpl() { return std::move(impl_); }

  unsigned length() const {
    if (!impl_)
      return 0;
    return impl_->length();
  }

  // Prefer Span8() and Span16() to Characters8() and Characters16().
  base::span<const LChar> Span8() const {
    if (!impl_)
      return {};
    DCHECK(impl_->Is8Bit());
    return impl_->Span8();
  }

  base::span<const UChar> Span16() const {
    if (!impl_)
      return {};
    DCHECK(!impl_->Is8Bit());
    return impl_->Span16();
  }

  base::span<const uint16_t> SpanUint16() const {
    if (!impl_) {
      return {};
    }
    DCHECK(!impl_->Is8Bit());
    return impl_->SpanUint16();
  }

  // This exposes the underlying representation of the string. Use with
  // care. When interpreting the string as a sequence of code units
  // Span8()/Span16() should be used.
  base::span<const uint8_t> RawByteSpan() const {
    if (!impl_) {
      return {};
    }
    return impl_->RawByteSpan();
  }

  // Use Span8() instead.
  UNSAFE_BUFFER_USAGE const LChar* Characters8() const {
    if (!impl_)
      return nullptr;
    DCHECK(impl_->Is8Bit());
    return impl_->Characters8();
  }

  // Use Span16() instead.
  UNSAFE_BUFFER_USAGE const UChar* Characters16() const {
    if (!impl_)
      return nullptr;
    DCHECK(!impl_->Is8Bit());
    return impl_->Characters16();
  }

  ALWAYS_INLINE const void* Bytes() const {
    if (!impl_)
      return nullptr;
    return impl_->Bytes();
  }

  // Return characters8() or characters16() depending on CharacterType.
  template <typename CharacterType>
  inline const CharacterType* GetCharacters() const;

  bool Is8Bit() const { return impl_->Is8Bit(); }

  [[nodiscard]] std::string Ascii() const;
  [[nodiscard]] std::string Latin1() const;
  [[nodiscard]] std::string Utf8(
      Utf8ConversionMode mode = Utf8ConversionMode::kLenient) const {
    return StringView(*this).Utf8(mode);
  }
  // Returns a std::u16string_view pointing this string.
  // This should be called only if !Is8Bit().
  //
  // This function should be removed after enabling C++23 because
  // std::u16string_view(Span16()) will work with C++23.
  std::u16string_view View16() const LIFETIME_BOUND {
    auto chars = Span16();
    return std::u16string_view(chars.begin(), chars.end());
  }

  UChar operator[](wtf_size_t index) const {
    if (!impl_ || index >= impl_->length())
      return 0;
    return (*impl_)[index];
  }

  // `begin()` and `end()` return iterators for `UChar32`, neither `UChar` nor
  // `LChar`. If you'd like to iterate code units, use `[]` and `length()`.
  CodePointIterator begin() const;
  CodePointIterator end() const;

  template <typename IntegerType>
  static String Number(IntegerType number) {
    IntegerToStringConverter<IntegerType> converter(number);
    return StringImpl::Create(converter.Span());
  }

  static String Boolean(bool value) { return String(value ? "true" : "false"); }

  [[nodiscard]] static String Number(float);

  [[nodiscard]] static String Number(double, unsigned precision = 6);

  // Number to String conversion following the ECMAScript definition.
  [[nodiscard]] static String NumberToStringECMAScript(double);
  [[nodiscard]] static String NumberToStringFixedWidth(double,
                                                       unsigned decimal_places);

  // Find characters.
  wtf_size_t find(UChar c, wtf_size_t start = 0) const {
    return impl_ ? impl_->Find(c, start) : kNotFound;
  }
  wtf_size_t find(LChar c, wtf_size_t start = 0) const {
    return impl_ ? impl_->Find(c, start) : kNotFound;
  }
  wtf_size_t find(char c, wtf_size_t start = 0) const {
    return find(static_cast<LChar>(c), start);
  }
  wtf_size_t Find(CharacterMatchFunctionPtr match_function,
                  wtf_size_t start = 0) const {
    return impl_ ? impl_->Find(match_function, start) : kNotFound;
  }
  wtf_size_t Find(base::RepeatingCallback<bool(UChar)> match_callback,
                  wtf_size_t index = 0) const;

  // Find substrings.
  wtf_size_t Find(const StringView& value, wtf_size_t start = 0) const {
    return impl_ ? impl_->Find(value, start) : kNotFound;
  }
  wtf_size_t Find(const StringView& value,
                  wtf_size_t start,
                  TextCaseSensitivity case_sensitivity) const {
    return impl_
               ? DISPATCH_CASE_OP(case_sensitivity, impl_->Find, (value, start))
               : kNotFound;
  }

  // Unicode aware case insensitive string matching. Non-ASCII characters might
  // match to ASCII characters. This function is rarely used to implement web
  // platform features.  See crbug.com/40476285.
  wtf_size_t DeprecatedFindIgnoringCase(const StringView& value,
                                        unsigned start = 0) const {
    return impl_ ? impl_->DeprecatedFindIgnoringCase(value, start) : kNotFound;
  }

  // ASCII case insensitive string matching.
  wtf_size_t FindIgnoringASCIICase(const StringView& value,
                                   unsigned start = 0) const {
    return impl_ ? impl_->FindIgnoringASCIICase(value, start) : kNotFound;
  }

  bool Contains(char c) const { return find(c) != kNotFound; }
  bool Contains(
      const StringView& value,
      TextCaseSensitivity case_sensitivity = kTextCaseSensitive) const {
    return Find(value, 0, case_sensitivity) != kNotFound;
  }

  // Find the last instance of a single character or string.
  wtf_size_t ReverseFind(UChar c, unsigned start = UINT_MAX) const {
    return impl_ ? impl_->ReverseFind(c, start) : kNotFound;
  }
  wtf_size_t ReverseFind(const StringView& value,
                         unsigned start = UINT_MAX) const {
    return impl_ ? impl_->ReverseFind(value, start) : kNotFound;
  }

  // Returns the Unicode code point starting at the specified offset of this
  // string. If the offset points an unpaired surrogate, this function returns
  // 0.
  UChar32 CharacterStartingAt(unsigned) const;

  bool StartsWith(const StringView& prefix) const {
    return impl_ ? impl_->StartsWith(prefix) : prefix.empty();
  }
  bool StartsWith(const StringView& prefix,
                  TextCaseSensitivity case_sensitivity) const {
    return impl_
               ? DISPATCH_CASE_OP(case_sensitivity, impl_->StartsWith, (prefix))
               : prefix.empty();
  }
  // Unicode aware case insensitive string matching. Non-ASCII characters might
  // match to ASCII characters. This function is rarely used to implement web
  // platform features.  See crbug.com/40476285.
  bool DeprecatedStartsWithIgnoringCase(const StringView& prefix) const {
    return impl_ ? impl_->DeprecatedStartsWithIgnoringCase(prefix)
                 : prefix.empty();
  }
  bool StartsWithIgnoringCaseAndAccents(const StringView& prefix) const {
    return impl_ ? impl_->StartsWithIgnoringCaseAndAccents(prefix)
                 : prefix.empty();
  }
  bool StartsWithIgnoringASCIICase(const StringView& prefix) const {
    return impl_ ? impl_->StartsWithIgnoringASCIICase(prefix) : prefix.empty();
  }
  bool StartsWith(UChar character) const {
    return impl_ ? impl_->StartsWith(character) : false;
  }

  bool EndsWith(const StringView& suffix) const {
    return impl_ ? impl_->EndsWith(suffix) : suffix.empty();
  }
  bool EndsWith(const StringView& suffix,
                TextCaseSensitivity case_sensitivity) const {
    return impl_ ? DISPATCH_CASE_OP(case_sensitivity, impl_->EndsWith, (suffix))
                 : suffix.empty();
  }
  // Unicode aware case insensitive string matching. Non-ASCII characters might
  // match to ASCII characters. This function is rarely used to implement web
  // platform features.  See crbug.com/40476285.
  bool DeprecatedEndsWithIgnoringCase(const StringView& prefix) const {
    return impl_ ? impl_->DeprecatedEndsWithIgnoringCase(prefix)
                 : prefix.empty();
  }
  bool EndsWithIgnoringASCIICase(const StringView& prefix) const {
    return impl_ ? impl_->EndsWithIgnoringASCIICase(prefix) : prefix.empty();
  }
  bool EndsWith(UChar character) const {
    return impl_ ? impl_->EndsWith(character) : false;
  }

  // TODO(esprehn): replace strangely both modifies this String *and* return a
  // value. It should only do one of those.
  String& Replace(UChar pattern, UChar replacement) {
    if (impl_)
      impl_ = impl_->Replace(pattern, replacement);
    return *this;
  }
  String& Replace(UChar pattern, const StringView& replacement) {
    if (impl_)
      impl_ = impl_->Replace(pattern, replacement);
    return *this;
  }
  String& Replace(const StringView& pattern, const StringView& replacement) {
    if (impl_)
      impl_ = impl_->Replace(pattern, replacement);
    return *this;
  }
  String& replace(unsigned index,
                  unsigned length_to_replace,
                  const StringView& replacement) {
    if (impl_)
      impl_ = impl_->Replace(index, length_to_replace, replacement);
    return *this;
  }

  void Fill(UChar c) {
    if (impl_)
      impl_ = impl_->Fill(c);
  }

  void Ensure16Bit();

  void Truncate(unsigned length);
  void Remove(unsigned start, unsigned length = 1);

  [[nodiscard]] String Substring(unsigned pos, unsigned len = UINT_MAX) const;
  [[nodiscard]] String Left(unsigned len) const { return Substring(0, len); }
  [[nodiscard]] String Right(unsigned len) const {
    return Substring(length() - len, len);
  }

  // Returns a lowercase version of the string. This function might convert
  // non-ASCII characters to ASCII characters. For example, DeprecatedLower()
  // for U+212A is 'k'.
  // This function is rarely used to implement web platform features. See
  // crbug.com/627682.
  // This function is deprecated. We should use LowerASCII() or CaseMap.
  [[nodiscard]] String DeprecatedLower() const;

  // Returns a lowercase version of the string.
  // This function converts ASCII characters only.
  [[nodiscard]] String LowerASCII() const;
  // Returns a uppercase version of the string.
  // This function converts ASCII characters only.
  [[nodiscard]] String UpperASCII() const;

  // Returns the length of the string after stripping white spaces.
  // This is equivalent (minus the allocation overhead) of doing:
  // `string.StripWhiteSpace().length()`
  [[nodiscard]] unsigned LengthWithStrippedWhiteSpace() const;
  [[nodiscard]] String StripWhiteSpace() const;
  [[nodiscard]] String StripWhiteSpace(IsWhiteSpaceFunctionPtr) const;
  [[nodiscard]] String SimplifyWhiteSpace(
      StripBehavior = kStripExtraWhiteSpace) const;
  [[nodiscard]] String SimplifyWhiteSpace(
      IsWhiteSpaceFunctionPtr,
      StripBehavior = kStripExtraWhiteSpace) const;

  [[nodiscard]] String RemoveCharacters(CharacterMatchFunctionPtr) const;
  template <bool isSpecialCharacter(UChar)>
  bool IsAllSpecialCharacters() const;

  // Return the string with case folded for case insensitive comparison.
  [[nodiscard]] String FoldCase() const;

  // Takes a printf format and args and prints into a String.
  // This function supports Latin-1 characters only.
  [[nodiscard]] PRINTF_FORMAT(1, 2) static String
      Format(const char* format, ...);

  // Returns a version suitable for gtest and base/logging.*.  It prepends and
  // appends double-quotes, and escapes characters other than ASCII printables.
  [[nodiscard]] String EncodeForDebugging() const;

  // Returns an uninitialized string. The characters needs to be written
  // into the buffer returned in `data` before the returned string is used.
  // Failure to do this will have unpredictable results.
  [[nodiscard]] static String CreateUninitialized(unsigned length,
                                                  base::span<UChar>& data) {
    return StringImpl::CreateUninitialized(length, data);
  }
  [[nodiscard]] static String CreateUninitialized(unsigned length,
                                                  base::span<LChar>& data) {
    return StringImpl::CreateUninitialized(length, data);
  }

  void Split(const StringView& separator,
             bool allow_empty_entries,
             Vector<String>& result) const;
  void Split(const StringView& separator, Vector<String>& result) const {
    Split(separator, false, result);
  }
  void Split(UChar separator,
             bool allow_empty_entries,
             Vector<String>& result) const;
  void Split(UChar separator, Vector<String>& result) const {
    Split(separator, false, result);
  }

  // Copy characters out of the string. See StringImpl.h for detailed docs.
  size_t CopyTo(base::span<UChar> buffer, wtf_size_t start) const {
    return impl_ ? impl_->CopyTo(buffer, start) : 0;
  }
  template <typename BufferType>
  void AppendTo(BufferType&,
                unsigned start = 0,
                unsigned length = UINT_MAX) const;

  // Convert the string into a number.

  // The following ToFooStrict functions accept:
  //  - leading '+'
  //  - leading Unicode whitespace
  //  - trailing Unicode whitespace
  //  - no "-0" (ToUIntStrict and ToUInt64Strict)
  //  - no out-of-range numbers which the resultant type can't represent
  //
  // If the input string is not acceptable, 0 is returned and |*ok| becomes
  // |false|.
  //
  // We can use these functions to implement a Web Platform feature only if the
  // input string is already valid according to the specification of the
  // feature.
  int ToIntStrict(bool* ok = nullptr) const;
  unsigned ToUIntStrict(bool* ok = nullptr) const;
  unsigned HexToUIntStrict(bool* ok) const;
  uint64_t HexToUInt64Strict(bool* ok) const;
  int64_t ToInt64Strict(bool* ok = nullptr) const;
  uint64_t ToUInt64Strict(bool* ok = nullptr) const;

  // The following ToFoo functions accept:
  //  - leading '+'
  //  - leading Unicode whitespace
  //  - trailing garbage
  //  - no "-0" (ToUInt and ToUInt64)
  //  - no out-of-range numbers which the resultant type can't represent
  //
  // If the input string is not acceptable, 0 is returned and |*ok| becomes
  // |false|.
  //
  // We can use these functions to implement a Web Platform feature only if the
  // input string is already valid according to the specification of the
  // feature.
  int ToInt(bool* ok = nullptr) const;
  unsigned ToUInt(bool* ok = nullptr) const;

  // These functions accepts:
  //  - leading '+'
  //  - numbers without leading zeros such as ".5"
  //  - numbers ending with "." such as "3."
  //  - scientific notation
  //  - leading whitespace (IsASCIISpace, not IsHTMLSpace)
  //  - no trailing whitespace
  //  - no trailing garbage
  //  - no numbers such as "NaN" "Infinity"
  //
  // A huge absolute number which a double/float can't represent is accepted,
  // and +Infinity or -Infinity is returned.
  //
  // A small absolute numbers which a double/float can't represent is accepted,
  // and 0 is returned
  //
  // If the input string is not acceptable, 0.0 is returned and |*ok| becomes
  // |false|.
  //
  // We can use these functions to implement a Web Platform feature only if the
  // input string is already valid according to the specification of the
  // feature.
  //
  // FIXME: Like the strict functions above, these give false for "ok" when
  // there is trailing garbage.  Like the non-strict functions above, these
  // return the value when there is trailing garbage.  It would be better if
  // these were more consistent with the above functions instead.
  double ToDouble(bool* ok = nullptr) const;
  float ToFloat(bool* ok = nullptr) const;

#ifdef __OBJC__
  String(NSString*);

  // This conversion maps null string to "", which loses the meaning of null
  // string, but we need this mapping because AppKit crashes when passed nil
  // NSStrings.
  operator NSString*() const {
    if (!impl_)
      return @"";
    return *impl_;
  }
#endif

  [[nodiscard]] static String Make8BitFrom16BitSource(base::span<const UChar>);
  [[nodiscard]] static String Make16BitFrom8BitSource(base::span<const LChar>);

  // String::FromUTF8 will return a null string if
  // the input data contains invalid UTF-8 sequences.
  // Does not strip BOMs.
  [[nodiscard]] static String FromUTF8(base::span<const uint8_t>);
  [[nodiscard]] static String FromUTF8(const char* s);
  [[nodiscard]] static String FromUTF8(std::string_view s) {
    return FromUTF8(base::as_byte_span(s));
  }

  // Tries to convert the passed in string to UTF-8, but will fall back to
  // Latin-1 if the string is not valid UTF-8.
  [[nodiscard]] static String FromUTF8WithLatin1Fallback(
      base::span<const uint8_t>);
  [[nodiscard]] static String FromUTF8WithLatin1Fallback(std::string_view s) {
    return FromUTF8WithLatin1Fallback(base::as_byte_span(s));
  }

  bool IsLowerASCII() const { return !impl_ || impl_->IsLowerASCII(); }

  bool ContainsOnlyASCIIOrEmpty() const {
    return !impl_ || impl_->ContainsOnlyASCIIOrEmpty();
  }
  bool ContainsOnlyLatin1OrEmpty() const;
  bool ContainsOnlyWhitespaceOrEmpty() const {
    return !impl_ || impl_->ContainsOnlyWhitespaceOrEmpty();
  }

  size_t CharactersSizeInBytes() const {
    return impl_ ? impl_->CharactersSizeInBytes() : 0;
  }

#ifndef NDEBUG
  // For use in the debugger.
  void Show() const;
#endif

  void WriteIntoTrace(perfetto::TracedValue context) const;

 private:
  friend struct HashTraits<String>;

  scoped_refptr<StringImpl> impl_;
};

#undef DISPATCH_CASE_OP

inline bool operator==(const String& a, const String& b) {
  // We don't use equalStringView here since we want the isAtomic() fast path
  // inside WTF::equal.
  return Equal(a.Impl(), b.Impl());
}
inline bool operator==(const String& a, const char* b) {
  return EqualStringView(a, b);
}
inline bool operator==(const char* a, const String& b) {
  return b == a;
}

inline bool operator!=(const String& a, const String& b) {
  return !(a == b);
}
inline bool operator!=(const String& a, const char* b) {
  return !(a == b);
}
inline bool operator!=(const char* a, const String& b) {
  return !(a == b);
}

inline bool EqualIgnoringNullity(const String& a, const String& b) {
  return EqualIgnoringNullity(a.Impl(), b.Impl());
}

template <wtf_size_t inlineCapacity>
inline bool EqualIgnoringNullity(const Vector<UChar, inlineCapacity>& a,
                                 const String& b) {
  return EqualIgnoringNullity(a, b.Impl());
}

inline void swap(String& a, String& b) {
  a.swap(b);
}

// Definitions of string operations

template <wtf_size_t inlineCapacity>
String::String(const Vector<UChar, inlineCapacity>& vector)
    : impl_(vector.size() ? StringImpl::Create(vector) : StringImpl::empty_) {}

template <>
inline const LChar* String::GetCharacters<LChar>() const {
  DCHECK(Is8Bit());
  return Characters8();
}

template <>
inline const UChar* String::GetCharacters<UChar>() const {
  DCHECK(!Is8Bit());
  return Characters16();
}

inline bool String::ContainsOnlyLatin1OrEmpty() const {
  if (empty())
    return true;

  if (Is8Bit())
    return true;

  return std::ranges::all_of(Span16(), [](UChar ch) { return ch < 0x0100; });
}

#ifdef __OBJC__
// This is for situations in WebKit where the long standing behavior has been
// "nil if empty", so we try to maintain longstanding behavior for the sake of
// entrenched clients
inline NSString* NsStringNilIfEmpty(const String& str) {
  return str.empty() ? nil : (NSString*)str;
}
#endif

// Compare strings using code units, matching Javascript string ordering.  See
// https://infra.spec.whatwg.org/#code-unit-less-than.
WTF_EXPORT int CodeUnitCompare(const String&, const String&);

inline bool CodeUnitCompareLessThan(const String& a, const String& b) {
  return CodeUnitCompare(a.Impl(), b.Impl()) < 0;
}

WTF_EXPORT int CodeUnitCompareIgnoringASCIICase(const String&, const char*);

inline bool CodeUnitCompareIgnoringASCIICaseLessThan(const String& a,
                                                     const String& b) {
  return CodeUnitCompareIgnoringASCIICase(a.Impl(), b.Impl()) < 0;
}

template <bool isSpecialCharacter(UChar)>
inline bool String::IsAllSpecialCharacters() const {
  return StringView(*this).IsAllSpecialCharacters<isSpecialCharacter>();
}

template <typename BufferType>
void String::AppendTo(BufferType& result,
                      unsigned position,
                      unsigned length) const {
  if (!impl_)
    return;
  impl_->AppendTo(result, position, length);
}

template <typename T>
struct HashTraits;
// Defined in string_hash.h.
template <>
struct HashTraits<String>;

// Shared global empty string.
WTF_EXPORT extern const String& g_empty_string;
WTF_EXPORT extern const String& g_empty_string16_bit;
WTF_EXPORT extern const String& g_xmlns_with_colon;

// Table representing common HTML strings of type '\n<space>*'.
class WTF_EXPORT NewlineThenWhitespaceStringsTable {
 public:
  // The constant is kept small to minimize the overhead of the table (496
  // bytes).
  static constexpr size_t kTableSize = 32;
  using TableType = std::array<String, kTableSize>;

  static void Init();

  static inline String GetStringForLength(size_t string_length) {
    return g_table_[string_length];
  }

  static bool IsNewlineThenWhitespaces(const StringView& view);

 private:
  static const TableType& g_table_;
};

// Pretty printer for gtest and base/logging.*.  It prepends and appends
// double-quotes, and escapes characters other than ASCII printables.
WTF_EXPORT std::ostream& operator<<(std::ostream&, const String&);

inline StringView::StringView(const String& string LIFETIME_BOUND,
                              unsigned offset,
                              unsigned length)
    : StringView(string.Impl(), offset, length) {}
inline StringView::StringView(const String& string LIFETIME_BOUND,
                              unsigned offset)
    : StringView(string.Impl(), offset) {}
inline StringView::StringView(const String& string LIFETIME_BOUND)
    : StringView(string.Impl()) {}

}  // namespace WTF

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(String)

using WTF::Equal;
using WTF::Find;
using WTF::g_empty_string;
using WTF::g_empty_string16_bit;
using WTF::String;
using WTF::Utf8ConversionMode;

#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_WTF_STRING_H_
