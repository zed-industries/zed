// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_VIEW_H_

#include <cstring>
#include <type_traits>

#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/dcheck_is_on.h"
#include "base/numerics/safe_conversions.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/get_ptr.h"
#include "third_party/blink/renderer/platform/wtf/text/string_impl.h"

#if DCHECK_IS_ON()
#include "base/memory/scoped_refptr.h"
#endif

namespace WTF {

class AtomicString;
class CodePointIterator;
class String;

enum class Utf8ConversionMode : uint8_t {
  // Unpaired surrogates are encoded using the standard UTF-8 encoding scheme,
  // even though surrogate characters should not be present in a valid UTF-8
  // string.
  kLenient,
  // Conversion terminates at the first unpaired surrogate, if any.
  kStrict,
  // Unpaired surrogates are replaced with U+FFFD (REPLACEMENT CHARACTER).
  kStrictReplacingErrors
};

// A string like object that wraps either an 8bit or 16bit byte sequence
// and keeps track of the length and the type, it does NOT own the bytes.
//
// Since StringView does not own the bytes creating a StringView from a String,
// then calling clear() on the String will result in a use-after-free. Asserts
// in ~StringView attempt to enforce this for most common cases.
//
// See base/strings/string_piece.h for more details.
class WTF_EXPORT StringView {
  DISALLOW_NEW();

 public:
  // A buffer that allows for short strings to be held on the stack during a
  // transform. This is a performance optimization for very hot paths and
  // should rarely need to be used.
  class StackBackingStore {
   public:
    // Returns a span of size |length| that is valid for as long the
    // StackBackingStore object is alive and Realloc() has not been called
    // again.
    template <typename CharT>
    base::span<CharT> Realloc(wtf_size_t length) {
      size_t size = length * sizeof(CharT);
      if (size > sizeof(stackbuf16_)) [[unlikely]] {
        heapbuf_.reset(reinterpret_cast<char*>(
            WTF::Partitions::BufferMalloc(size, "StackBackingStore")));
        // SAFETY: `heapbuf_` is the result of BufferMalloc() for `length`.
        return UNSAFE_BUFFERS(
            base::span(reinterpret_cast<CharT*>(heapbuf_.get()), length));
      }

      // If the Realloc() shrinks the buffer size, |heapbuf_| will keep a copy
      // of the old string. A reset can be added here, but given this is a
      // transient usage, deferring to the destructor is just as good and avoids
      // another branch.
      static_assert(alignof(decltype(stackbuf16_)) % alignof(CharT) == 0,
                    "stack buffer must be sufficiently aligned");
      // SAFETY: `length` is smaller than the size of `stackbuf16_`.
      return UNSAFE_BUFFERS(
          base::span(reinterpret_cast<CharT*>(&stackbuf16_[0]), length));
    }

   public:
    struct BufferDeleter {
      void operator()(void* buffer) { WTF::Partitions::BufferFree(buffer); }
    };

    static_assert(sizeof(UChar) != sizeof(char),
                  "A char array will trigger -fstack-protect an produce "
                  "overkill stack canaries all over v8 bindings");

    // The size 64 is just a guess on a good size. No data was used in its
    // selection.
    UChar stackbuf16_[64];
    std::unique_ptr<char[], BufferDeleter> heapbuf_;
  };

  // Null string.
  StringView() { Clear(); }

  // From a StringView:
  StringView(const StringView&, unsigned offset, unsigned length);
  StringView(const StringView& view, unsigned offset)
      : StringView(view, offset, view.length_ - offset) {}

  // From a StringImpl:
  StringView(const StringImpl*);
  StringView(const StringImpl*, unsigned offset);
  StringView(const StringImpl*, unsigned offset, unsigned length);

  // From a non-null StringImpl.
  StringView(const StringImpl& impl)
      : impl_(const_cast<StringImpl*>(&impl)),
        bytes_(impl.Bytes()),
        length_(impl.length()) {}

  // From a non-null StringImpl, avoids the null check.
  StringView(StringImpl& impl)
      : impl_(&impl), bytes_(impl.Bytes()), length_(impl.length()) {}
  StringView(StringImpl&, unsigned offset);
  StringView(StringImpl&, unsigned offset, unsigned length);

  // From a String, implemented in wtf_string.h
  inline StringView(const String& string LIFETIME_BOUND,
                    unsigned offset,
                    unsigned length);
  inline StringView(const String& string LIFETIME_BOUND, unsigned offset);
  // NOLINTNEXTLINE(google-explicit-constructor)
  inline StringView(const String& string LIFETIME_BOUND);

  // From an AtomicString, implemented in atomic_string.h
  inline StringView(const AtomicString& string LIFETIME_BOUND,
                    unsigned offset,
                    unsigned length);
  inline StringView(const AtomicString& string LIFETIME_BOUND, unsigned offset);
  // NOLINTNEXTLINE(google-explicit-constructor)
  inline StringView(const AtomicString& string LIFETIME_BOUND);

  // From a literal string or LChar buffer:
  explicit StringView(base::span<const LChar> chars)
      : impl_(StringImpl::empty_),
        bytes_(chars.data()),
        length_(base::checked_cast<wtf_size_t>(chars.size())) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  StringView(const LChar* chars)
      : impl_(StringImpl::empty_),
        bytes_(chars),
        length_(chars ? base::checked_cast<unsigned>(
                            strlen(reinterpret_cast<const char*>(chars)))
                      : 0) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  StringView(const char* chars)
      : StringView(reinterpret_cast<const LChar*>(chars)) {}

  // From a wide literal string or UChar buffer.
  explicit StringView(base::span<const UChar> chars)
      : impl_(StringImpl::empty16_bit_),
        bytes_(chars.data()),
        length_(base::checked_cast<wtf_size_t>(chars.size())) {}
  StringView(const UChar* chars);

  // StringView(const T*, unsigned) are deleted explicitly because `const T*` is
  // converted to a StringView implicitly and StringView(const StringView&,
  // unsigned offset) would be used unexpectedly.
  StringView(const LChar*, unsigned) = delete;
  StringView(const char*, unsigned) = delete;
  StringView(const UChar*, unsigned) = delete;

#if DCHECK_IS_ON()
  ~StringView();
#endif

  bool IsNull() const { return !bytes_; }
  bool empty() const { return !length_; }

  unsigned length() const { return length_; }

  bool Is8Bit() const {
    DCHECK(impl_);
    return impl_->Is8Bit();
  }

  [[nodiscard]] std::string Utf8(
      Utf8ConversionMode mode = Utf8ConversionMode::kLenient) const;

  bool IsAtomic() const { return SharedImpl() && SharedImpl()->IsAtomic(); }

  bool IsLowerASCII() const;
  bool ContainsOnlyASCIIOrEmpty() const;

  bool SubstringContainsOnlyWhitespaceOrEmpty(unsigned from, unsigned to) const;

  void Clear();

  UChar operator[](unsigned i) const {
    SECURITY_DCHECK(i < length());
    if (Is8Bit())
      return static_cast<const LChar*>(bytes_)[i];
    return static_cast<const UChar*>(bytes_)[i];
  }

  // Use Span16() instead.
  UNSAFE_BUFFER_USAGE const LChar* Characters8() const {
    DCHECK(Is8Bit());
    return static_cast<const LChar*>(bytes_);
  }

  // Use Span16() instead.
  UNSAFE_BUFFER_USAGE const UChar* Characters16() const {
    DCHECK(!Is8Bit());
    return static_cast<const UChar*>(bytes_);
  }

  base::span<const LChar> Span8() const {
    DCHECK(Is8Bit());
    return {static_cast<const LChar*>(bytes_), length_};
  }

  base::span<const UChar> Span16() const {
    DCHECK(!Is8Bit());
    return {static_cast<const UChar*>(bytes_), length_};
  }

  base::span<const uint16_t> SpanUint16() const {
    DCHECK(!Is8Bit());
    return {static_cast<const uint16_t*>(bytes_), length_};
  }

  // Returns the Unicode code point starting at the specified offset of this
  // string. If the offset points an unpaired surrogate, this function returns
  // the surrogate code unit as is. If you'd like to check such surroagtes,
  // use U_IS_SURROGATE() defined in unicode/utf.h.
  UChar32 CodepointAt(unsigned i) const;

  // Returns i+2 if a pair of [i] and [i+1] is a valid surrogate pair.
  // Returns i+1 otherwise.
  unsigned NextCodePointOffset(unsigned i) const;

  // Does `CodepointAt()`, and the specified `i` is updated by
  // `NextCodePointOffset()`.
  UChar32 CodePointAtAndNext(unsigned& i) const;

  const void* Bytes() const { return bytes_; }

  base::span<const uint8_t> RawByteSpan() const {
    return {reinterpret_cast<const uint8_t*>(bytes_),
            length_ * (Is8Bit() ? sizeof(LChar) : sizeof(UChar))};
  }

  // This is not named impl() like String because it has different semantics.
  // String::impl() is never null if String::isNull() is false. For StringView
  // sharedImpl() can be null if the StringView was created with a non-zero
  // offset, or a length that made it shorter than the underlying impl.
  StringImpl* SharedImpl() const {
    // If this StringView is backed by a StringImpl, and was constructed
    // with a zero offset and the same length we can just access the impl
    // directly since this == StringView(m_impl).
    if (impl_->Bytes() == Bytes() && length_ == impl_->length())
      return GetPtr(impl_);
    return nullptr;
  }

  // This will return a StringView with a version of |this| that has all ASCII
  // characters lowercased. The returned StringView is guarantee to be valid for
  // as long as |backing_store| is valid.
  //
  // The odd lifetime of the returned object occurs because lowercasing may
  // require allocation. When that happens, |backing_store| is used as the
  // backing store and the returned StringView has the same lifetime.
  StringView LowerASCIIMaybeUsingBuffer(StackBackingStore& backing_store) const;

  String ToString() const;
  AtomicString ToAtomicString() const;

  // Returns a version suitable for gtest and base/logging.*.  It prepends and
  // appends double-quotes, and escapes characters other than ASCII printables.
  [[nodiscard]] String EncodeForDebugging() const;

  // Find characters. Returns the index of the match, or `kNotFound`.
  wtf_size_t Find(CharacterMatchFunctionPtr match_function,
                  wtf_size_t start = 0) const;

  template <bool isSpecialCharacter(UChar)>
  bool IsAllSpecialCharacters() const;

  // Iterator support
  //
  // begin() and end() return iterators for UChar32, neither UChar nor LChar.
  // If you'd like to iterate code units, just use [] and length().
  //
  // * Iterate code units
  //    for (unsigned i = 0; i < view.length(); ++i) {
  //      UChar code_unit = view[i];
  //      ...
  // * Iterate code points
  //    for (UChar32 code_point : view) {
  //      ...
  CodePointIterator begin() const;
  CodePointIterator end() const;

 private:
  void Set(const StringImpl&, unsigned offset, unsigned length);

// We use the StringImpl to mark for 8bit or 16bit, even for strings where
// we were constructed from a char pointer. So m_impl->bytes() might have
// nothing to do with this view's bytes().
#if DCHECK_IS_ON()
  scoped_refptr<StringImpl> impl_;
#else
  StringImpl* impl_;
#endif
  const void* bytes_;
  unsigned length_;
};

inline StringView::StringView(const StringView& view,
                              unsigned offset,
                              unsigned length)
    : impl_(view.impl_), length_(length) {
  SECURITY_DCHECK(offset <= view.length());
  SECURITY_DCHECK(length <= view.length() - offset);
  if (Is8Bit())
    bytes_ = view.Characters8() + offset;
  else
    bytes_ = view.Characters16() + offset;
}

inline StringView::StringView(const StringImpl* impl) {
  if (!impl) {
    Clear();
    return;
  }
  impl_ = const_cast<StringImpl*>(impl);
  length_ = impl->length();
  bytes_ = impl->Bytes();
}

inline StringView::StringView(const StringImpl* impl, unsigned offset) {
  impl ? Set(*impl, offset, impl->length() - offset) : Clear();
}

inline StringView::StringView(const StringImpl* impl,
                              unsigned offset,
                              unsigned length) {
  impl ? Set(*impl, offset, length) : Clear();
}

inline StringView::StringView(StringImpl& impl, unsigned offset) {
  Set(impl, offset, impl.length() - offset);
}

inline StringView::StringView(StringImpl& impl,
                              unsigned offset,
                              unsigned length) {
  Set(impl, offset, length);
}

inline void StringView::Clear() {
  length_ = 0;
  bytes_ = nullptr;
  impl_ = StringImpl::empty_;  // mark as 8 bit.
}

inline void StringView::Set(const StringImpl& impl,
                            unsigned offset,
                            unsigned length) {
  SECURITY_DCHECK(offset <= impl.length());
  SECURITY_DCHECK(length <= impl.length() - offset);
  length_ = length;
  impl_ = const_cast<StringImpl*>(&impl);
  if (impl.Is8Bit())
    bytes_ = impl.Characters8() + offset;
  else
    bytes_ = impl.Characters16() + offset;
}

// Unicode aware case insensitive string matching. Non-ASCII characters might
// match to ASCII characters. These functions are rarely used to implement web
// platform features.
// These functions are deprecated. Use EqualIgnoringASCIICase(), or introduce
// EqualIgnoringUnicodeCase(). See crbug.com/627682
WTF_EXPORT bool DeprecatedEqualIgnoringCase(const StringView&,
                                            const StringView&);
WTF_EXPORT bool DeprecatedEqualIgnoringCaseAndNullity(const StringView&,
                                                      const StringView&);

WTF_EXPORT bool EqualIgnoringASCIICase(const StringView&, const StringView&);

template <size_t N>
inline bool EqualIgnoringASCIICase(const StringView& a,
                                   const char (&literal)[N]) {
  if (a.length() != N - 1 || (N == 1 && a.IsNull()))
    return false;
  base::span<const char> span = base::span(literal).template first<N - 1>();
  return a.Is8Bit() ? EqualIgnoringASCIICase(a.Span8(), span)
                    : EqualIgnoringASCIICase(a.Span16(), span);
}

// TODO(esprehn): Can't make this an overload of WTF::equal since that makes
// calls to equal() that pass literal strings ambiguous. Figure out if we can
// replace all the callers with equalStringView and then rename it to equal().
WTF_EXPORT bool EqualStringView(const StringView&, const StringView&);

inline bool operator==(const StringView& a, const StringView& b) {
  return EqualStringView(a, b);
}

inline bool operator!=(const StringView& a, const StringView& b) {
  return !(a == b);
}

inline wtf_size_t StringView::Find(CharacterMatchFunctionPtr match_function,
                                   wtf_size_t start) const {
  return Is8Bit() ? WTF::Find(Span8(), match_function, start)
                  : WTF::Find(Span16(), match_function, start);
}

template <bool isSpecialCharacter(UChar)>
inline bool StringView::IsAllSpecialCharacters() const {
  if (empty()) {
    return true;
  }
  return Is8Bit() ? std::ranges::all_of(Span8(), isSpecialCharacter)
                  : std::ranges::all_of(Span16(), isSpecialCharacter);
}

WTF_EXPORT std::ostream& operator<<(std::ostream&, const StringView&);

}  // namespace WTF

using WTF::StringView;
using WTF::EqualIgnoringASCIICase;
using WTF::DeprecatedEqualIgnoringCase;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_STRING_VIEW_H_
