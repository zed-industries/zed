/*
 * Copyright 2005 Frerich Raabe <raabe@kde.org>
 * Copyright (C) 2006 Apple Computer, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/xml/xpath_node_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

namespace xpath {

struct EvaluationContext;

class ValueData final : public GarbageCollected<ValueData> {
 public:
  ValueData() : node_set_(NodeSet::Create()) {}
  explicit ValueData(const NodeSet& node_set)
      : node_set_(NodeSet::Create(node_set)) {}
  explicit ValueData(NodeSet* node_set) : node_set_(node_set) {}
  explicit ValueData(const String& string)
      : string_(string), node_set_(NodeSet::Create()) {}

  void Trace(Visitor*) const;
  NodeSet& GetNodeSet() { return *node_set_; }

  String string_;

 private:
  Member<NodeSet> node_set_;
};

// Copying Value objects makes their data partially shared, so care has to be
// taken when dealing with copies.
class CORE_EXPORT Value {
  DISALLOW_NEW();

 public:
  enum Type { kNodeSetValue, kBooleanValue, kNumberValue, kStringValue };

  Value(unsigned value) : type_(kNumberValue), bool_(false), number_(value) {}
  Value(uint64_t value) : type_(kNumberValue), bool_(false), number_(value) {}
  Value(double value) : type_(kNumberValue), bool_(false), number_(value) {}

  Value(const char* value)
      : type_(kStringValue),
        bool_(false),
        number_(0),
        data_(MakeGarbageCollected<ValueData>(value)) {}
  Value(const String& value)
      : type_(kStringValue),
        bool_(false),
        number_(0),
        data_(MakeGarbageCollected<ValueData>(value)) {}
  Value(const NodeSet& value)
      : type_(kNodeSetValue),
        bool_(false),
        number_(0),
        data_(MakeGarbageCollected<ValueData>(value)) {}
  Value(Node* value)
      : type_(kNodeSetValue),
        bool_(false),
        number_(0),
        data_(MakeGarbageCollected<ValueData>()) {
    data_->GetNodeSet().Append(value);
  }
  void Trace(Visitor*) const;

  // This is needed to safely implement constructing from bool - with normal
  // function overloading, any pointer type would match.
  template <typename T>
  Value(T);

  static const struct AdoptTag {
  } kAdopt;
  Value(NodeSet* value, const AdoptTag&)
      : type_(kNodeSetValue),
        bool_(false),
        number_(0),
        data_(MakeGarbageCollected<ValueData>(value)) {}

  Type GetType() const { return type_; }

  bool IsNodeSet() const { return type_ == kNodeSetValue; }
  bool IsBoolean() const { return type_ == kBooleanValue; }
  bool IsNumber() const { return type_ == kNumberValue; }
  bool IsString() const { return type_ == kStringValue; }

  // If this is called during XPathExpression::evaluate(), EvaluationContext
  // should be passed to record type conversion error.
  const NodeSet& ToNodeSet(EvaluationContext*) const;
  NodeSet& ModifiableNodeSet(EvaluationContext&);
  bool ToBoolean() const;
  double ToNumber() const;
  String ToString() const;

 private:
  Type type_;
  bool bool_;
  double number_;
  Member<ValueData> data_;
};

template <>
inline Value::Value(bool value)
    : type_(kBooleanValue), bool_(value), number_(0) {}

}  // namespace xpath

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_XML_XPATH_VALUE_H_
