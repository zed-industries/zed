// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACED_VALUE_H_
#define BASE_TRACE_EVENT_TRACED_VALUE_H_

#include <stddef.h>

#include <memory>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/trace_event/trace_arguments.h"

namespace base {

class Value;

namespace trace_event {

class BASE_EXPORT TracedValue : public ConvertableToTraceFormat {
 public:
  // TODO(oysteine): |capacity| is not used in any production code. Consider
  // removing it.
  explicit TracedValue(size_t capacity = 0);
  TracedValue(const TracedValue&) = delete;
  TracedValue& operator=(const TracedValue&) = delete;
  ~TracedValue() override;

  void EndDictionary();
  void EndArray();

  // These methods assume that |name| is a long lived "quoted" string.
  void SetInteger(const char* name, int value);
  void SetDouble(const char* name, double value);
  void SetBoolean(const char* name, bool value);
  void SetString(const char* name, std::string_view value);
  void SetValue(const char* name, TracedValue* value);
  void SetPointer(const char* name, const void* value);
  void BeginDictionary(const char* name);
  void BeginArray(const char* name);

  // These, instead, can be safely passed a temporary string.
  void SetIntegerWithCopiedName(std::string_view name, int value);
  void SetDoubleWithCopiedName(std::string_view name, double value);
  void SetBooleanWithCopiedName(std::string_view name, bool value);
  void SetStringWithCopiedName(std::string_view name, std::string_view value);
  void SetValueWithCopiedName(std::string_view name, TracedValue* value);
  void SetPointerWithCopiedName(std::string_view name, const void* value);
  void BeginDictionaryWithCopiedName(std::string_view name);
  void BeginArrayWithCopiedName(std::string_view name);

  void AppendInteger(int);
  void AppendDouble(double);
  void AppendBoolean(bool);
  void AppendString(std::string_view);
  void AppendPointer(const void*);
  void BeginArray();
  void BeginDictionary();

  // ConvertableToTraceFormat implementation.
  void AppendAsTraceFormat(std::string* out) const override;
  bool AppendToProto(ProtoAppender* appender) const override;

  // Helper to auto-close an array. The call to |ArrayScope::~ArrayScope| closes
  // the array.
  //
  // To be constructed using:
  //   |TracedValue::AppendArrayScoped|
  //   |TracedValue::BeginArrayScoped|
  //   |TracedValue::BeginArrayScopedWithCopiedName|
  //
  // |ArrayScope| holds a |TracedValue| pointer which should remain a valid
  // pointer until |ArrayScope::~ArrayScope| is called.
  //
  // |ArrayScope::~ArrayScope| calls |TracedValue::EndArray| (which checks if
  // the held |TracedValue*| is in array state).
  //
  // Example:
  //   std::unique_ptr<TracedValue> value(new TracedValue());
  //   {
  //     auto scope = value->BeginArrayScoped("array_name");
  //     value->AppendBoolean(false);
  //   }
  class BASE_EXPORT ArrayScope {
   public:
    ArrayScope(const ArrayScope&) = delete;
    ArrayScope(ArrayScope&&) = default;
    ArrayScope& operator=(const ArrayScope&) = delete;
    ArrayScope& operator=(ArrayScope&&) = default;
    ~ArrayScope();

   private:
    explicit ArrayScope(TracedValue* value);

    raw_ptr<TracedValue> value_;

    friend class TracedValue;
  };

  // Call |BeginArray| or |BeginArrayWithCopiedName| with no / the same
  // parameter and return an |ArrayScope| holding |this|.
  [[nodiscard]] ArrayScope AppendArrayScoped();
  [[nodiscard]] ArrayScope BeginArrayScoped(const char* name);
  [[nodiscard]] ArrayScope BeginArrayScopedWithCopiedName(
      std::string_view name);

  // Helper to auto-close a dictionary. The call to
  // |DictionaryScope::~DictionaryScope| closes the dictionary.
  //
  // To be constructed using:
  //   |TracedValue::AppendDictionaryScoped|
  //   |TracedValue::BeginDictionaryScoped|
  //   |TracedValue::BeginDictionaryScopedWithCopiedName|
  //
  // |DictionaryScope| holds a |TracedValue| pointer which should remain a valid
  // pointer until |DictionaryScope::~DictionaryScope| is called.
  //
  // |DictionaryScope::~DictionaryScope| calls |TracedValue::EndDictionary|
  // (which checks if the held |TracedValue*| is in dictionary state).
  //
  // Example:
  //   std::unique_ptr<TracedValue> value(new TracedValue());
  //   {
  //     auto scope = value->BeginDictionaryScoped("dictionary_name");
  //     value->SetBoolean("my_boolean", false);
  //   }
  class BASE_EXPORT DictionaryScope {
   public:
    DictionaryScope(const DictionaryScope&) = delete;
    DictionaryScope(DictionaryScope&&) = default;
    DictionaryScope& operator=(const DictionaryScope&) = delete;
    DictionaryScope& operator=(DictionaryScope&&) = default;
    ~DictionaryScope();

   private:
    explicit DictionaryScope(TracedValue* value);

    raw_ptr<TracedValue> value_;

    friend class TracedValue;
  };

  // Call |BeginDictionary| or |BeginDictionaryWithCopiedName| with no / the
  // same parameter and return a |DictionaryScope| holding |this|.
  [[nodiscard]] DictionaryScope AppendDictionaryScoped();
  [[nodiscard]] DictionaryScope BeginDictionaryScoped(const char* name);
  [[nodiscard]] DictionaryScope BeginDictionaryScopedWithCopiedName(
      std::string_view name);

  class BASE_EXPORT Array;
  class BASE_EXPORT Dictionary;
  class BASE_EXPORT ValueHolder;
  class BASE_EXPORT ArrayItem;
  class BASE_EXPORT DictionaryItem;

  // Helper to enable easier initialization of |TracedValue|. This is intended
  // for quick local debugging as there is overhead of creating
  // |std::initializer_list| of name-value objects (in the case of containers
  // the value is also a |std::initializer_list|). Generally the helper types
  // |TracedValue::Dictionary|, |TracedValue::Array|,
  // |TracedValue::DictionaryItem|, |TracedValue::ArrayItem| must be valid as
  // well as their internals (e.g., |std::string_view| data should be valid
  // when |TracedValue::Build| is called; |TracedValue::Array| or
  // |TracedValue::Dictionary| holds a |std::initializer_list| whose underlying
  // array needs to be valid when calling |TracedValue::Build|).
  //
  // Example:
  //    auto value = TracedValue::Build({
  //      {"int_var_name", 42},
  //      {"double_var_name", 3.14},
  //      {"string_var_name", "hello world"},
  //      {"empty_array", TracedValue::Array({})},
  //      {"dictionary", TracedValue::Dictionary({
  //        {"my_ptr", static_cast<void*>(my_ptr)},
  //        {"nested_array", TracedValue::Array({1, false, 0.5})},
  //      })},
  //    });
  static std::unique_ptr<TracedValue> Build(
      const std::initializer_list<DictionaryItem> items);

  // An |Array| instance represents an array of |ArrayItem| objects. This is a
  // helper to allow initializer list like construction of arrays using
  // |TracedValue::Build|.
  //
  // An instance holds an |std::initializer_list<TracedValue::ArrayItem>| and is
  // cheap to copy (copying the initializer_list does not copy the underlying
  // objects). The underlying array must exist at the time when
  // |TracedValue::Build| is called.
  class Array {
   public:
    // This constructor expects that the initializer_list is valid when
    // |TracedValue::Build| is called.
    Array(const std::initializer_list<ArrayItem> items);
    Array(Array&&);
    void WriteToValue(TracedValue* value) const;

   private:
    std::initializer_list<ArrayItem> items_;
  };

  // A helper to hold a dictionary. Similar to |TracedValue::Array|.
  class Dictionary {
   public:
    // This constructor expects that the initializer_list is valid when
    // |TracedValue::Build| is called.
    Dictionary(const std::initializer_list<DictionaryItem> items);
    Dictionary(Dictionary&&);
    void WriteToValue(TracedValue* value) const;

   private:
    std::initializer_list<DictionaryItem> items_;
  };

  // A |ValueHolder| holds a single value or a container (int, double... or an
  // |Array| / |Dictionary|). Not to be used outside of the context of
  // |TracedValue::Build| (has one parameter implicit constructors).
  //
  // Base class for |TracedValue::ArrayItem| and |TracedValue::DictionaryItem|.
  class ValueHolder {
   public:
    // Implicit constructors allow constructing |DictionaryItem| without having
    // to write |{"name", TracedValue::ValueHolder(1)}|.
    ValueHolder(int value);     // NOLINT(google-explicit-constructor)
    ValueHolder(double value);  // NOLINT(google-explicit-constructor)
    ValueHolder(bool value);    // NOLINT(google-explicit-constructor)
    ValueHolder(void* value);   // NOLINT(google-explicit-constructor)
    // std::string_view's backing storage / const char* pointer needs to remain
    // valid until TracedValue::Build is called.
    // NOLINTNEXTLINE(google-explicit-constructor)
    ValueHolder(std::string_view value);
    // Create a copy to avoid holding a reference to a non-existing string:
    //
    // Example:
    //   TracedValue::Build({{"my_string", std::string("std::string value")}});
    // Explanation:
    //   1. std::string temporary is passed to the constructor of |ValueHolder|.
    //   2. |ValueHolder| is passed to the constructor of |DictionaryItem|.
    //   3. |Build| iterates initializer_list of |DictionaryItems|.
    //
    //   If the original |ValueHolder| kept just a reference to the string (or
    //   a |std::string_view|) then |Build| is undefined behaviour, as it is
    //   passing a reference to an out-of-scope temporary to
    //   |TracedValue::SetString|.
    // NOLINTNEXTLINE(google-explicit-constructor)
    ValueHolder(std::string value);
    // Define an explicit overload for const char* to resolve the ambiguity
    // between the std::string_view, void*, and bool constructors for string
    // literals.
    ValueHolder(const char* value);  // NOLINT(google-explicit-constructor)
    ValueHolder(Array& value);       // NOLINT(google-explicit-constructor)
    ValueHolder(Dictionary& value);  // NOLINT(google-explicit-constructor)
    ValueHolder(ValueHolder&&);

   protected:
    void WriteToValue(TracedValue* value) const;
    void WriteToValue(const char* name, TracedValue* value) const;

   private:
    union KeptValue {
      // Copy is handled by the holder (based on
      // |TracedValue::ValueHolder::kept_value_type_|).
      int int_value;
      double double_value;
      bool bool_value;
      std::string_view string_piece_value;
      std::string std_string_value;
      // This field is not a raw_ptr<> because it was filtered by the rewriter
      // for: #union
      RAW_PTR_EXCLUSION void* void_ptr_value;
      Array array_value;
      Dictionary dictionary_value;

      // Default constructor is implicitly deleted because union field has a
      // non-trivial default constructor.
      KeptValue() {}   // NOLINT(modernize-use-equals-default)
      ~KeptValue() {}  // NOLINT(modernize-use-equals-default)
    };

    // Reimplementing a subset of C++17 std::variant.
    enum class KeptValueType {
      kIntType,
      kDoubleType,
      kBoolType,
      kStringPieceType,
      kStdStringType,
      kVoidPtrType,
      kArrayType,
      kDictionaryType,
    };

    KeptValue kept_value_;
    KeptValueType kept_value_type_;
  };

  // |ArrayItem| is a |ValueHolder| which can be used to construct an |Array|.
  class ArrayItem : public ValueHolder {
   public:
    // Implicit constructors allow calling |TracedValue::Array({1, true, 3.14})|
    // instead of |TracedValue::Array({TracedValue::ArrayItem(1),
    // TracedValue::ArrayItem(true), TracedValue::ArrayItem(3.14)})|.
    template <typename T>
    // NOLINTNEXTLINE(google-explicit-constructor)
    ArrayItem(T value) : ValueHolder(value) {}

    void WriteToValue(TracedValue* value) const;
  };

  // |DictionaryItem| instance represents a single name-value pair.
  //
  // |name| is assumed to be a long lived "quoted" string.
  class DictionaryItem : public ValueHolder {
   public:
    // These constructors assume that |name| is a long lived "quoted" string.
    template <typename T>
    DictionaryItem(const char* name, T value)
        : ValueHolder(value), name_(name) {}

    void WriteToValue(TracedValue* value) const;

   private:
    const char* name_;
  };

  // A custom serialization class can be supplied by implementing the
  // Writer interface and supplying a factory class to SetWriterFactoryCallback.
  // Primarily used by Perfetto to write TracedValues directly into its proto
  // format, which lets us do a direct memcpy() in AppendToProto() rather than
  // a JSON serialization step in AppendAsTraceFormat.
  class BASE_EXPORT Writer {
   public:
    virtual ~Writer() = default;

    virtual void BeginArray() = 0;
    virtual void BeginDictionary() = 0;
    virtual void EndDictionary() = 0;
    virtual void EndArray() = 0;

    // These methods assume that |name| is a long lived "quoted" string.
    virtual void SetInteger(const char* name, int value) = 0;
    virtual void SetDouble(const char* name, double value) = 0;
    virtual void SetBoolean(const char* name, bool value) = 0;
    virtual void SetString(const char* name, std::string_view value) = 0;
    virtual void SetValue(const char* name, Writer* value) = 0;
    virtual void BeginDictionary(const char* name) = 0;
    virtual void BeginArray(const char* name) = 0;

    // These, instead, can be safely passed a temporary string.
    virtual void SetIntegerWithCopiedName(std::string_view name, int value) = 0;
    virtual void SetDoubleWithCopiedName(std::string_view name,
                                         double value) = 0;
    virtual void SetBooleanWithCopiedName(std::string_view name,
                                          bool value) = 0;
    virtual void SetStringWithCopiedName(std::string_view name,
                                         std::string_view value) = 0;
    virtual void SetValueWithCopiedName(std::string_view name,
                                        Writer* value) = 0;
    virtual void BeginDictionaryWithCopiedName(std::string_view name) = 0;
    virtual void BeginArrayWithCopiedName(std::string_view name) = 0;

    virtual void AppendInteger(int) = 0;
    virtual void AppendDouble(double) = 0;
    virtual void AppendBoolean(bool) = 0;
    virtual void AppendString(std::string_view) = 0;

    virtual void AppendAsTraceFormat(std::string* out) const = 0;

    virtual bool AppendToProto(ProtoAppender* appender);

    virtual bool IsPickleWriter() const = 0;
    virtual bool IsProtoWriter() const = 0;
  };

  typedef std::unique_ptr<Writer> (*WriterFactoryCallback)(size_t capacity);
  static void SetWriterFactoryCallback(WriterFactoryCallback callback);

 protected:
  TracedValue(size_t capacity, bool forced_json);

  std::unique_ptr<base::Value> ToBaseValue() const;

 private:
  mutable std::unique_ptr<Writer> writer_;

#ifndef NDEBUG
  // In debug builds checks the pairings of {Start,End}{Dictionary,Array}
  std::vector<bool> nesting_stack_;
#endif
};

// TracedValue that is convertable to JSON format. This has lower performance
// than the default TracedValue in production code, and should be used only for
// testing and debugging. Should be avoided in tracing. It's for
// testing/debugging code calling value dumping function designed for tracing,
// like the following:
//
//   TracedValueJSON value;
//   AsValueInto(&value);  // which is designed for tracing.
//   return value.ToJSON();
//
// If the code is merely for testing/debugging, base::Value should be used
// instead.
class BASE_EXPORT TracedValueJSON : public TracedValue {
 public:
  explicit TracedValueJSON(size_t capacity = 0)
      : TracedValue(capacity, /*forced_josn*/ true) {}

  using TracedValue::ToBaseValue;

  // Converts the value into a JSON string without formatting. Suitable for
  // printing a simple value or printing a value in a single line context.
  std::string ToJSON() const;

  // Converts the value into a formatted JSON string, with indentation, spaces
  // and new lines for better human readability of complex values.
  std::string ToFormattedJSON() const;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACED_VALUE_H_
