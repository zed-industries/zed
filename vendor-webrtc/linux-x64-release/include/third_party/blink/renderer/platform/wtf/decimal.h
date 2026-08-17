/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DECIMAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DECIMAL_H_

#include <cstdint>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class represents decimal base floating point number.
//
// FIXME: Once all C++ compiler support decimal type, we should replace this
// class to compiler supported one. See below URI for current status of decimal
// type for C++:
// http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2006/n1977.html
class WTF_EXPORT Decimal {
  USING_FAST_MALLOC(Decimal);

 public:
  enum Sign {
    kPositive,
    kNegative,
  };

  // You should not use EncodedData other than unit testing.
  class EncodedData {
    DISALLOW_NEW();
    // For accessing FormatClass.
    friend class Decimal;

   public:
    EncodedData(Sign, int exponent, uint64_t coefficient);

    bool operator==(const EncodedData&) const;
    bool operator!=(const EncodedData& another) const {
      return !operator==(another);
    }

    uint64_t Coefficient() const { return coefficient_; }
    int CountDigits() const;
    int Exponent() const { return exponent_; }
    bool IsFinite() const { return !IsSpecial(); }
    bool IsInfinity() const { return format_class_ == kClassInfinity; }
    bool IsNaN() const { return format_class_ == kClassNaN; }
    bool IsSpecial() const {
      return format_class_ == kClassInfinity || format_class_ == kClassNaN;
    }
    bool IsZero() const { return format_class_ == kClassZero; }
    Sign GetSign() const { return sign_; }
    void SetSign(Sign sign) { sign_ = sign; }

   private:
    enum FormatClass {
      kClassInfinity,
      kClassNormal,
      kClassNaN,
      kClassZero,
    };

    EncodedData(Sign, FormatClass);
    FormatClass GetFormatClass() const { return format_class_; }

    uint64_t coefficient_;
    int16_t exponent_;
    FormatClass format_class_;
    Sign sign_;
  };

  Decimal(int32_t = 0);
  Decimal(Sign, int exponent, uint64_t coefficient);
  Decimal(const Decimal&);

  Decimal& operator=(const Decimal&);
  Decimal& operator+=(const Decimal&);
  Decimal& operator-=(const Decimal&);
  Decimal& operator*=(const Decimal&);
  Decimal& operator/=(const Decimal&);

  Decimal operator-() const;

  bool operator==(const Decimal&) const;
  bool operator!=(const Decimal&) const;
  bool operator<(const Decimal&) const;
  bool operator<=(const Decimal&) const;
  bool operator>(const Decimal&) const;
  bool operator>=(const Decimal&) const;

  Decimal operator+(const Decimal&) const;
  Decimal operator-(const Decimal&) const;
  Decimal operator*(const Decimal&)const;
  Decimal operator/(const Decimal&) const;

  int Exponent() const {
    DCHECK(IsFinite());
    return data_.Exponent();
  }

  bool IsFinite() const { return data_.IsFinite(); }
  bool IsInfinity() const { return data_.IsInfinity(); }
  bool IsNaN() const { return data_.IsNaN(); }
  bool IsNegative() const { return GetSign() == kNegative; }
  bool IsPositive() const { return GetSign() == kPositive; }
  bool IsSpecial() const { return data_.IsSpecial(); }
  bool IsZero() const { return data_.IsZero(); }

  Decimal Abs() const;
  Decimal Ceil() const;
  Decimal Floor() const;
  Decimal Remainder(const Decimal&) const;
  Decimal Round() const;

  double ToDouble() const;
  // Note: toString method supports infinity and nan but fromString not.
  String ToString() const;

  static Decimal FromDouble(double);
  // fromString supports following syntax EBNF:
  //  number ::= sign? digit+ ('.' digit*) (exponent-marker sign? digit+)?
  //          | sign? '.' digit+ (exponent-marker sign? digit+)?
  //  sign ::= '+' | '-'
  //  exponent-marker ::= 'e' | 'E'
  //  digit ::= '0' | '1' | ... | '9'
  // Note: fromString doesn't support "infinity" and "nan".
  static Decimal FromString(const String&);
  static Decimal Infinity(Sign);
  static Decimal Nan();
  static Decimal Zero(Sign);

  // You should not use below methods. We expose them for unit testing.
  explicit Decimal(const EncodedData&);
  const EncodedData& Value() const { return data_; }

 private:
  struct AlignedOperands {
    uint64_t lhs_coefficient;
    uint64_t rhs_coefficient;
    int exponent;
  };

  Decimal(double);
  Decimal CompareTo(const Decimal&) const;

  static AlignedOperands AlignOperands(const Decimal& lhs, const Decimal& rhs);
  static inline Sign InvertSign(Sign sign) {
    return sign == kNegative ? kPositive : kNegative;
  }

  Sign GetSign() const { return data_.GetSign(); }

  EncodedData data_;
};

WTF_EXPORT std::ostream& operator<<(std::ostream&, const Decimal&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_DECIMAL_H_
