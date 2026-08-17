/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_TYPE_CONVERSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_TYPE_CONVERSIONS_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class TypeConversions final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TypeConversions() = default;

  int32_t testLong() { return long_; }
  void setTestLong(int32_t value) { long_ = value; }
  uint32_t testUnsignedLong() { return unsigned_long_; }
  void setTestUnsignedLong(uint32_t value) { unsigned_long_ = value; }

  int64_t testLongLong() { return long_long_; }
  void setTestLongLong(int64_t value) { long_long_ = value; }
  uint64_t testUnsignedLongLong() { return unsigned_long_long_; }
  void setTestUnsignedLongLong(uint64_t value) { unsigned_long_long_ = value; }

  int8_t testByte() { return byte_; }
  void setTestByte(int8_t value) { byte_ = value; }
  uint8_t testOctet() { return octet_; }
  void setTestOctet(uint8_t value) { octet_ = value; }

  int16_t testShort() { return short_; }
  void setTestShort(int16_t value) { short_ = value; }
  uint16_t testUnsignedShort() { return unsigned_short_; }
  void setTestUnsignedShort(uint16_t value) { unsigned_short_ = value; }

  const String& testByteString() const { return byte_string_; }
  void setTestByteString(const String& value) { byte_string_ = value; }

  const String& testUSVString() const { return usv_string_; }
  void setTestUSVString(const String& value) { usv_string_ = value; }

  const String& testUSVStringOrNull() const { return usv_string_or_null_; }
  void setTestUSVStringOrNull(const String& value) {
    usv_string_or_null_ = value;
  }

 private:
  int32_t long_ = 0;
  uint32_t unsigned_long_ = 0;
  int64_t long_long_ = 0;
  uint64_t unsigned_long_long_ = 0;
  int8_t byte_ = 0;
  uint8_t octet_ = 0;
  int16_t short_ = 0;
  uint16_t unsigned_short_ = 0;
  String byte_string_;
  String usv_string_;
  String usv_string_or_null_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_TYPE_CONVERSIONS_H_
