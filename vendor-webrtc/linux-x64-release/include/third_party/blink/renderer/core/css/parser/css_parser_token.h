// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_TOKEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_TOKEN_H_

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/parser/at_rule_descriptors.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

namespace blink {

class ExecutionContext;
enum class CSSValueID;

enum CSSParserTokenType {
  kIdentToken = 0,
  kFunctionToken,
  kAtKeywordToken,
  kHashToken,
  kUrlToken,
  kBadUrlToken,
  kDelimiterToken,
  kNumberToken,
  kPercentageToken,
  kDimensionToken,
  kIncludeMatchToken,
  kDashMatchToken,
  kPrefixMatchToken,
  kSuffixMatchToken,
  kSubstringMatchToken,
  kColumnToken,
  kUnicodeRangeToken,
  kWhitespaceToken,
  kCDOToken,
  kCDCToken,
  kColonToken,
  kSemicolonToken,
  kCommaToken,
  kLeftParenthesisToken,
  kRightParenthesisToken,
  kLeftBracketToken,
  kRightBracketToken,
  kLeftBraceToken,
  kRightBraceToken,
  kStringToken,
  kBadStringToken,
  kEOFToken,
  kCommentToken,
};

enum NumericSign {
  kNoSign,
  kPlusSign,
  kMinusSign,
};

enum NumericValueType {
  kIntegerValueType,
  kNumberValueType,
};

enum HashTokenType {
  kHashTokenId,
  kHashTokenUnrestricted,
};

class CORE_EXPORT CSSParserToken {
 public:
  enum BlockType {
    kNotBlock,
    kBlockStart,
    kBlockEnd,
  };

  // NOTE: There are some fields that we don't actually use (marked here
  // as “Don't care”), but we still set them explicitly, since otherwise,
  // Clang works really hard to preserve their contents.
  explicit CSSParserToken(CSSParserTokenType type,
                          BlockType block_type = kNotBlock)
      : type_(type),
        block_type_(block_type),
        numeric_value_type_(0),  // Don't care.
        numeric_sign_(0),        // Don't care.
        unit_(0),                // Don't care.
        value_is_inline_(false),
        value_is_8bit_(false),  // Don't care.
        padding_(0)             // Don't care.
  {}

  // The resulting CSSParserToken may hold a reference to the data in value.
  CSSParserToken(CSSParserTokenType type,
                 StringView value,
                 BlockType block_type = kNotBlock,
                 int id = -1)
      : type_(type), block_type_(block_type), id_(id) {
    InitValueFromStringView(value);
  }

  CSSParserToken(CSSParserTokenType, UChar);  // for DelimiterToken
  CSSParserToken(CSSParserTokenType,
                 double,
                 NumericValueType,
                 NumericSign);  // for NumberToken
  CSSParserToken(CSSParserTokenType,
                 UChar32,
                 UChar32);  // for UnicodeRangeToken

  CSSParserToken(HashTokenType, StringView);

  bool operator==(const CSSParserToken& other) const;
  bool operator!=(const CSSParserToken& other) const {
    return !(*this == other);
  }

  // Converts NumberToken to DimensionToken.
  void ConvertToDimensionWithUnit(StringView);

  // Converts NumberToken to PercentageToken.
  void ConvertToPercentage();

  CSSParserTokenType GetType() const {
    return static_cast<CSSParserTokenType>(type_);
  }
  StringView Value() const {
    return value_is_8bit_ ? StringView(Span8()) : StringView(Span16());
  }

  bool IsEOF() const { return type_ == static_cast<unsigned>(kEOFToken); }

  UChar Delimiter() const;
  NumericSign GetNumericSign() const;
  NumericValueType GetNumericValueType() const;
  double NumericValue() const;
  HashTokenType GetHashTokenType() const {
    DCHECK_EQ(type_, static_cast<unsigned>(kHashToken));
    return hash_token_type_;
  }
  BlockType GetBlockType() const { return static_cast<BlockType>(block_type_); }
  CSSPrimitiveValue::UnitType GetUnitType() const {
    return static_cast<CSSPrimitiveValue::UnitType>(unit_);
  }
  UChar32 UnicodeRangeStart() const {
    DCHECK_EQ(type_, static_cast<unsigned>(kUnicodeRangeToken));
    return unicode_range_.start;
  }
  UChar32 UnicodeRangeEnd() const {
    DCHECK_EQ(type_, static_cast<unsigned>(kUnicodeRangeToken));
    return unicode_range_.end;
  }

  CSSValueID Id() const;

  CSSValueID FunctionId() const {
    if (type_ != kFunctionToken) {
      return CSSValueID::kInvalid;
    }
    return static_cast<CSSValueID>(id_);
  }

  bool HasStringBacking() const;

  CSSPropertyID ParseAsUnresolvedCSSPropertyID(
      const ExecutionContext* execution_context,
      CSSParserMode mode = kHTMLStandardMode) const;
  AtRuleDescriptorID ParseAsAtRuleDescriptorID() const;

  void Serialize(StringBuilder&) const;

  CSSParserToken CopyWithUpdatedString(const StringView&) const;
  CSSParserToken CopyWithoutValue() const {
    CSSParserToken token = *this;
    token.value_is_inline_ = false;
    token.value_length_ = 0;
    token.value_data_char_raw_ = nullptr;
    return token;
  }

  static CSSParserTokenType ClosingTokenType(CSSParserTokenType opening_type) {
    switch (opening_type) {
      case kFunctionToken:
      case kLeftParenthesisToken:
        return kRightParenthesisToken;
      case kLeftBracketToken:
        return kRightBracketToken;
      case kLeftBraceToken:
        return kRightBraceToken;
      default:
        NOTREACHED();
    }
  }

  // For debugging/logging only.
  friend std::ostream& operator<<(std::ostream& stream,
                                  const CSSParserToken& token) {
    if (token.GetType() == kEOFToken) {
      return stream << "<EOF>";
    } else if (token.GetType() == kCommentToken) {
      return stream << "/* comment */";
    } else {
      StringBuilder sb;
      token.Serialize(sb);
      return stream << sb.ToString();
    }
  }

 private:
  void InitValueFromStringView(StringView string) {
    value_length_ = string.length();
    value_is_8bit_ = string.Is8Bit();
    if (value_is_8bit_ && value_length_ <= sizeof(value_data_char_inline_)) {
      UNSAFE_TODO(
          memcpy(value_data_char_inline_, string.Bytes(), value_length_));
      value_is_inline_ = true;
    } else {
      value_data_char_raw_ = string.Bytes();
      value_is_inline_ = false;
    }
  }
  bool ValueDataCharRawEqual(const CSSParserToken& other) const;
  const void* ValueDataCharRaw() const {
    if (value_is_inline_) {
      return value_data_char_inline_;
    } else {
      return value_data_char_raw_;
    }
  }
  base::span<const LChar> Span8() const {
    DCHECK(value_is_8bit_);
    // SAFETY: InitValueFromStringView() ensures the expression is safe.
    return UNSAFE_BUFFERS(
        {static_cast<const LChar*>(ValueDataCharRaw()), value_length_});
  }
  base::span<const UChar> Span16() const {
    DCHECK(!value_is_8bit_);
    DCHECK(!value_is_inline_);
    // SAFETY: InitValueFromStringView() ensures the expression is safe.
    return UNSAFE_BUFFERS(
        {static_cast<const UChar*>(value_data_char_raw_), value_length_});
  }

  // Bitfields are all declared as type `unsigned` based on observation that
  // on Windows, adjacent bitfields of differing types do not get packed
  // together, for binary compatibility with code generated by MSVC.

  unsigned type_ : 6;                // CSSParserTokenType
  unsigned block_type_ : 2;          // BlockType
  unsigned numeric_value_type_ : 1;  // NumericValueType
  unsigned numeric_sign_ : 2;        // NumericSign
  unsigned unit_ : 7;                // CSSPrimitiveValue::UnitType

  // The variables below are only used if the token type is string-backed
  // (which depends on type_; see HasStringBacking() for the list).

  // Short strings (eight bytes or fewer) may be stored directly into the
  // CSSParserToken, freeing us from allocating a backing string for the
  // contents (saving RAM and a little time). If so, value_is_inline_
  // is set to mark that the buffer contains the string itself instead of
  // a pointer to the string. It also guarantees value_is_8bit_ == true.
  unsigned value_is_inline_ : 1;

  // value_... is an unpacked StringView so that we can pack it
  // tightly with the rest of this object for a smaller object size.
  unsigned value_is_8bit_ : 1;

  // These are free bits. You may take from them if you need.
  [[maybe_unused]] unsigned padding_ : 12;

  unsigned value_length_;
  union {
    char value_data_char_inline_[8];   // If value_is_inline_ is true.
    const void* value_data_char_raw_;  // Either LChar* or UChar*.
  };

  union {
    UChar delimiter_;
    HashTokenType hash_token_type_;
    // NOTE: For DimensionToken, this value stores the numeric part,
    // value_data_char_raw_ (or value_data_char_inline_) stores the
    // unit as text, and unit_ stores the unit as enum (assuming it
    // is a valid unit). So for e.g. “100px”, numeric_value_ = 100.0,
    // value_length_ = 2, value_data_char_inline_ = "px", and
    // unit_ = kPixels.
    double numeric_value_;
    mutable int id_;

    struct {
      UChar32 start;
      UChar32 end;
    } unicode_range_;
  };
};

// If this assert fails, check the comment above about bitfields.
static_assert(sizeof(CSSParserToken) == 24);

bool NeedsInsertedComment(const CSSParserToken& a, const CSSParserToken& b);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_TOKEN_H_
