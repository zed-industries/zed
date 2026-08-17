/*
 * Copyright (C) 2011 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_TOKEN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_TOKEN_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class VTTTokenTypes {
  STATIC_ONLY(VTTTokenTypes);

 public:
  enum Type {
    kUninitialized,
    kCharacter,
    kStartTag,
    kEndTag,
    kTimestampTag,
  };
};

class VTTToken {
  STACK_ALLOCATED();

 public:
  typedef VTTTokenTypes Type;

  VTTToken() : type_(Type::kUninitialized) {}

  static VTTToken StringToken(const String& character_data) {
    return VTTToken(Type::kCharacter, character_data);
  }
  static VTTToken StartTag(const String& tag_name,
                           const AtomicString& classes = g_empty_atom,
                           const AtomicString& annotation = g_empty_atom) {
    VTTToken token(Type::kStartTag, tag_name);
    token.classes_ = classes;
    token.annotation_ = annotation;
    return token;
  }
  static VTTToken EndTag(const String& tag_name) {
    return VTTToken(Type::kEndTag, tag_name);
  }
  static VTTToken TimestampTag(const String& timestamp_data) {
    return VTTToken(Type::kTimestampTag, timestamp_data);
  }

  Type::Type GetType() const { return type_; }
  const String& GetName() const { return data_; }
  const String& Characters() const { return data_; }
  const AtomicString& Classes() const { return classes_; }
  const AtomicString& Annotation() const { return annotation_; }

 private:
  VTTToken(Type::Type type, const String& data) : type_(type), data_(data) {}

  Type::Type type_;
  String data_;
  AtomicString annotation_;
  AtomicString classes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_TOKEN_H_
