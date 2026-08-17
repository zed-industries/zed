// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_UNION_TYPES_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_UNION_TYPES_TEST_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class V8UnionDoubleOrInternalEnum;
class V8UnionDoubleOrString;
class V8UnionDoubleOrStringOrStringSequence;
class V8UnionElementOrNodeList;

class UnionTypesTest final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  UnionTypesTest() = default;
  ~UnionTypesTest() override = default;

  V8UnionDoubleOrStringOrStringSequence*
  doubleOrStringOrStringSequenceAttribute() const;
  void setDoubleOrStringOrStringSequenceAttribute(
      const V8UnionDoubleOrStringOrStringSequence* value);

  String doubleOrStringArg(V8UnionDoubleOrString* arg);
  String doubleOrInternalEnumArg(V8UnionDoubleOrInternalEnum* arg);
  String doubleOrStringSequenceArg(
      const HeapVector<Member<V8UnionDoubleOrString>>& sequence);

  String nodeListOrElementArg(const V8UnionElementOrNodeList* arg);
  String nodeListOrElementOrNullArg(const V8UnionElementOrNodeList* arg);

  String doubleOrStringOrStringSequenceArg(
      const V8UnionDoubleOrStringOrStringSequence* arg);

 private:
  enum AttributeSpecificType {
    kSpecificTypeDouble,
    kSpecificTypeString,
    kSpecificTypeStringSequence,
  };
  AttributeSpecificType attribute_type_ = kSpecificTypeDouble;
  double attribute_double_ = 0.;
  String attribute_string_;
  Vector<String> attribute_string_sequence_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_UNION_TYPES_TEST_H_
