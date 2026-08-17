// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Defines classes for field comparison.

#ifndef GOOGLE_PROTOBUF_UTIL_FIELD_COMPARATOR_H__
#define GOOGLE_PROTOBUF_UTIL_FIELD_COMPARATOR_H__

#include <cstdint>
#include <string>
#include <vector>

#include "google/protobuf/stubs/common.h"
#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

class Message;
class EnumValueDescriptor;
class FieldDescriptor;

namespace util {

class FieldContext;
class MessageDifferencer;

// Base class specifying the interface for comparing protocol buffer fields.
// Regular users should consider using or subclassing DefaultFieldComparator
// rather than this interface.
// Currently, this does not support comparing unknown fields.
class PROTOBUF_EXPORT FieldComparator {
 public:
  FieldComparator();
  FieldComparator(const FieldComparator&) = delete;
  FieldComparator& operator=(const FieldComparator&) = delete;
  virtual ~FieldComparator();

  enum ComparisonResult {
    SAME,       // Compared fields are equal. In case of comparing submessages,
                // user should not recursively compare their contents.
    DIFFERENT,  // Compared fields are different. In case of comparing
                // submessages, user should not recursively compare their
                // contents.
    RECURSE,    // Compared submessages need to be compared recursively.
                // FieldComparator does not specify the semantics of recursive
                // comparison. This value should not be returned for simple
                // values.
  };

  // Compares the values of a field in two protocol buffer messages.
  // Returns SAME or DIFFERENT for simple values, and SAME, DIFFERENT or RECURSE
  // for submessages. Returning RECURSE for fields not being submessages is
  // illegal.
  // In case the given FieldDescriptor points to a repeated field, the indices
  // need to be valid. Otherwise they should be ignored.
  //
  // FieldContext contains information about the specific instances of the
  // fields being compared, versus FieldDescriptor which only contains general
  // type information about the fields.
  virtual ComparisonResult Compare(const Message& message_1,
                                   const Message& message_2,
                                   const FieldDescriptor* field, int index_1,
                                   int index_2,
                                   const util::FieldContext* field_context) = 0;
};

// Basic implementation of FieldComparator.  Supports three modes of floating
// point value comparison: exact, approximate using MathUtil::AlmostEqual
// method, and arbitrarily precise using MathUtil::WithinFractionOrMargin.
class PROTOBUF_EXPORT SimpleFieldComparator : public FieldComparator {
 public:
  enum FloatComparison {
    EXACT,        // Floats and doubles are compared exactly.
    APPROXIMATE,  // Floats and doubles are compared using the
                  // MathUtil::AlmostEqual method or
                  // MathUtil::WithinFractionOrMargin method.
    // TODO: Introduce third value to differentiate uses of AlmostEqual
    //               and WithinFractionOrMargin.
  };

  // Creates new comparator with float comparison set to EXACT.
  SimpleFieldComparator();
  SimpleFieldComparator(const SimpleFieldComparator&) = delete;
  SimpleFieldComparator& operator=(const SimpleFieldComparator&) = delete;

  ~SimpleFieldComparator() override;

  void set_float_comparison(FloatComparison float_comparison) {
    float_comparison_ = float_comparison;
  }

  FloatComparison float_comparison() const { return float_comparison_; }

  // Set whether the FieldComparator shall treat floats or doubles that are both
  // NaN as equal (treat_nan_as_equal = true) or as different
  // (treat_nan_as_equal = false). Default is treating NaNs always as different.
  void set_treat_nan_as_equal(bool treat_nan_as_equal) {
    treat_nan_as_equal_ = treat_nan_as_equal;
  }

  bool treat_nan_as_equal() const { return treat_nan_as_equal_; }

  // Sets the fraction and margin for the float comparison of a given field.
  // Uses MathUtil::WithinFractionOrMargin to compare the values.
  //
  // REQUIRES: field->cpp_type == FieldDescriptor::CPPTYPE_DOUBLE or
  //           field->cpp_type == FieldDescriptor::CPPTYPE_FLOAT
  // REQUIRES: float_comparison_ == APPROXIMATE
  void SetFractionAndMargin(const FieldDescriptor* field, double fraction,
                            double margin);

  // Sets the fraction and margin for the float comparison of all float and
  // double fields, unless a field has been given a specific setting via
  // SetFractionAndMargin() above.
  // Uses MathUtil::WithinFractionOrMargin to compare the values.
  //
  // REQUIRES: float_comparison_ == APPROXIMATE
  void SetDefaultFractionAndMargin(double fraction, double margin);

 protected:
  // Returns the comparison result for the given field in two messages.
  //
  // This function is called directly by DefaultFieldComparator::Compare.
  // Subclasses can call this function to compare fields they do not need to
  // handle specially.
  ComparisonResult SimpleCompare(const Message& message_1,
                                 const Message& message_2,
                                 const FieldDescriptor* field, int index_1,
                                 int index_2,
                                 const util::FieldContext* field_context);

  // Compare using the provided message_differencer. For example, a subclass can
  // use this method to compare some field in a certain way using the same
  // message_differencer instance and the field context.
  bool CompareWithDifferencer(MessageDifferencer* differencer,
                              const Message& message1, const Message& message2,
                              const util::FieldContext* field_context);

  // Returns FieldComparator::SAME if boolean_result is true and
  // FieldComparator::DIFFERENT otherwise.
  ComparisonResult ResultFromBoolean(bool boolean_result) const;

 private:
  // Defines the tolerance for floating point comparison (fraction and margin).
  struct Tolerance {
    double fraction;
    double margin;
    Tolerance() : fraction(0.0), margin(0.0) {}
    Tolerance(double f, double m) : fraction(f), margin(m) {}
  };

  friend class MessageDifferencer;
  // The following methods get executed when CompareFields is called for the
  // basic types (instead of submessages). They return true on success. One
  // can use ResultFromBoolean() to convert that boolean to a ComparisonResult
  // value.
  bool CompareBool(const FieldDescriptor& /* unused */, bool value_1,
                   bool value_2) {
    return value_1 == value_2;
  }

  // Uses CompareDoubleOrFloat, a helper function used by both CompareDouble and
  // CompareFloat.
  bool CompareDouble(const FieldDescriptor& field, double value_1,
                     double value_2);

  bool CompareEnum(const FieldDescriptor& field,
                   const EnumValueDescriptor* value_1,
                   const EnumValueDescriptor* value_2);

  // Uses CompareDoubleOrFloat, a helper function used by both CompareDouble and
  // CompareFloat.
  bool CompareFloat(const FieldDescriptor& field, float value_1, float value_2);

  bool CompareInt32(const FieldDescriptor& /* unused */, int32_t value_1,
                    int32_t value_2) {
    return value_1 == value_2;
  }

  bool CompareInt64(const FieldDescriptor& /* unused */, int64_t value_1,
                    int64_t value_2) {
    return value_1 == value_2;
  }

  bool CompareString(const FieldDescriptor& /* unused */,
                     absl::string_view value_1, absl::string_view value_2) {
    return value_1 == value_2;
  }

  bool CompareUInt32(const FieldDescriptor& /* unused */, uint32_t value_1,
                     uint32_t value_2) {
    return value_1 == value_2;
  }

  bool CompareUInt64(const FieldDescriptor& /* unused */, uint64_t value_1,
                     uint64_t value_2) {
    return value_1 == value_2;
  }

  // This function is used by CompareDouble and CompareFloat to avoid code
  // duplication. There are no checks done against types of the values passed,
  // but it's likely to fail if passed non-numeric arguments.
  template <typename T>
  bool CompareDoubleOrFloat(const FieldDescriptor& field, T value_1, T value_2);

  FloatComparison float_comparison_;

  // If true, floats and doubles that are both NaN are considered to be
  // equal. Otherwise, two floats or doubles that are NaN are considered to be
  // different.
  bool treat_nan_as_equal_;

  // True iff default_tolerance_ has been explicitly set.
  //
  // If false, then the default tolerance for floats and doubles is that which
  // is used by MathUtil::AlmostEquals().
  bool has_default_tolerance_;

  // Default float/double tolerance. Only meaningful if
  // has_default_tolerance_ == true.
  Tolerance default_tolerance_;

  // Field-specific float/double tolerances, which override any default for
  // those particular fields.
  absl::flat_hash_map<const FieldDescriptor*, Tolerance> map_tolerance_;
};

// Default field comparison: use the basic implementation of FieldComparator.
class PROTOBUF_EXPORT DefaultFieldComparator final
    : public SimpleFieldComparator {
 public:
  ComparisonResult Compare(const Message& message_1, const Message& message_2,
                           const FieldDescriptor* field, int index_1,
                           int index_2,
                           const util::FieldContext* field_context) override {
    return SimpleCompare(message_1, message_2, field, index_1, index_2,
                         field_context);
  }
};

}  // namespace util
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_UTIL_FIELD_COMPARATOR_H__
