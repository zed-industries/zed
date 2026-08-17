/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXCEPTION_MESSAGES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXCEPTION_MESSAGES_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-exception.h"

namespace blink {

class Decimal;

class PLATFORM_EXPORT ExceptionMessages {
  STATIC_ONLY(ExceptionMessages);

 public:
  enum BoundType {
    kInclusiveBound,
    kExclusiveBound,
  };

  static String AddContextToMessage(v8::ExceptionContext type,
                                    const char* class_name,
                                    const String& property_name,
                                    const String& message);

  static String ArgumentNullOrIncorrectType(int argument_index,
                                            const String& expected_type);
  static String ArgumentNotOfType(int argument_index,
                                  const char* expected_type);
  static String ConstructorNotCallableAsFunction(const char* type);
  static String ConstructorCalledAsFunction();

  static String FailedToConvertJSValue(const char* type);

  static String FailedToConstruct(const char* type, const String& detail);
  static String FailedToEnumerate(const char* type, const String& detail);
  static String FailedToExecute(const char* method,
                                const char* type,
                                const String& detail) {
    return FailedToExecute(String(method), type, detail);
  }
  static String FailedToExecute(const String& method,
                                const char* type,
                                const String& detail);
  static String FailedToGet(const String& property,
                            const char* type,
                            const String& detail);
  static String FailedToSet(const String& property,
                            const char* type,
                            const String& detail);
  static String FailedToDelete(const String& property,
                               const char* type,
                               const String& detail);
  static String FailedToGetIndexed(const String& property,
                                   const char* type,
                                   const String& detail);
  static String FailedToSetIndexed(const String& property,
                                   const char* type,
                                   const String& detail);
  static String FailedToDeleteIndexed(const String& property,
                                      const char* type,
                                      const String& detail);
  static String FailedToGetNamed(const String& property,
                                 const char* type,
                                 const String& detail);
  static String FailedToSetNamed(const String& property,
                                 const char* type,
                                 const String& detail);
  static String FailedToDeleteNamed(const String& property,
                                    const char* type,
                                    const String& detail);

  template <typename NumType>
  static String FormatNumber(NumType number) {
    return FormatFiniteNumber(number);
  }

  static String IncorrectPropertyType(const String& property,
                                      const String& detail);

  template <typename NumberType>
  static String IndexExceedsMaximumBound(const char* name,
                                         NumberType given,
                                         NumberType bound) {
    return IndexExceedsMaximumBound(name, given == bound, FormatNumber(given),
                                    FormatNumber(bound));
  }

  template <typename NumberType>
  static String IndexExceedsMinimumBound(const char* name,
                                         NumberType given,
                                         NumberType bound) {
    return IndexExceedsMinimumBound(name, given == bound, FormatNumber(given),
                                    FormatNumber(bound));
  }

  template <typename NumberType>
  static String IndexOutsideRange(const char* name,
                                  NumberType given,
                                  NumberType lower_bound,
                                  BoundType lower_type,
                                  NumberType upper_bound,
                                  BoundType upper_type) {
    return IndexOutsideRange(name, FormatNumber(given),
                             FormatNumber(lower_bound), lower_type,
                             FormatNumber(upper_bound), upper_type);
  }

  static String InvalidArity(const char* expected, unsigned provided);

  static String NotASequenceTypeProperty(const String& property_name);
  static String NotAFiniteNumber(double value,
                                 const char* name = "value provided");
  static String NotAFiniteNumber(const Decimal& value,
                                 const char* name = "value provided");

  static String NotEnoughArguments(unsigned expected, unsigned provided);

  static String ReadOnly(const char* detail = nullptr);

  static String SharedArrayBufferNotAllowed(const char* expected_type);

  static String ResizableArrayBufferNotAllowed(const char* expected_type);

  static String ValueNotOfType(const char* expected_type);

 private:
  template <typename NumType>
  static String FormatFiniteNumber(NumType number) {
    if (number > 1e20 || number < -1e20)
      return String::Format("%e", 1.0 * number);
    return String::Number(number);
  }

  template <typename NumType>
  static String FormatPotentiallyNonFiniteNumber(NumType number) {
    if (std::isnan(number))
      return "NaN";
    if (std::isinf(number))
      return number > 0 ? "Infinity" : "-Infinity";
    if (number > 1e20 || number < -1e20)
      return String::Format("%e", number);
    return String::Number(number);
  }

  static String OrdinalNumber(int number);

  static String IndexExceedsMaximumBound(const char* name,
                                         bool eq,
                                         const String& given,
                                         const String& bound);

  static String IndexExceedsMinimumBound(const char* name,
                                         bool eq,
                                         const String& given,
                                         const String& bound);

  static String IndexOutsideRange(const char* name,
                                  const String& given,
                                  const String& lower_bound,
                                  BoundType lower_type,
                                  const String& upper_bound,
                                  BoundType upper_type);
};

template <>
PLATFORM_EXPORT String ExceptionMessages::FormatNumber<float>(float number);

template <>
PLATFORM_EXPORT String ExceptionMessages::FormatNumber<double>(double number);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_EXCEPTION_MESSAGES_H_
