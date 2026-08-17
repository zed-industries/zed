// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_TRACE_EVENT_TRACE_ARGUMENTS_H_
#define BASE_TRACE_EVENT_TRACE_ARGUMENTS_H_

#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <memory>
#include <string>
#include <utility>

#include "base/base_export.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/trace_event/common/trace_event_common.h"
#include "base/tracing_buildflags.h"
#include "third_party/perfetto/include/perfetto/protozero/scattered_heap_buffer.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value.h"
#include "third_party/perfetto/protos/perfetto/trace/track_event/debug_annotation.pbzero.h"

// Trace macro can have one or two optional arguments, each one of them
// identified by a name (a C string literal) and a value, which can be an
// integer, enum, floating point, boolean, string pointer or reference, or
// std::unique_ptr<ConvertableToTraceFormat> compatible values. Additionally,
// custom data types need to be supported, like time values or WTF::CString.
//
// TraceArguments is a helper class used to store 0 to 2 named arguments
// corresponding to an individual trace macro call. As efficiently as possible,
// and with the minimal amount of generated machine code (since this affects
// any TRACE macro call). Each argument has:
//
//  - A name (C string literal, e.g "dumps")
//  - An 8-bit type value, corresponding to the TRACE_VALUE_TYPE_XXX macros.
//  - A value, stored in a TraceValue union
//
// IMPORTANT: For a TRACE_VALUE_TYPE_CONVERTABLE types, the TraceArguments
// instance owns the pointed ConvertableToTraceFormat object, i.e. it will
// delete it automatically on destruction.
//
// TraceArguments instances should be built using one of specialized
// constructors declared below. One cannot modify an instance once it has
// been built, except for move operations, Reset() and destruction. Examples:
//
//    TraceArguments args;    // No arguments.
//    // args.size() == 0
//
//    TraceArguments("foo", 100);
//    // args.size() == 1
//    // args.types()[0] == TRACE_VALUE_TYPE_INT
//    // args.names()[0] == "foo"
//    // args.values()[0].as_int == 100
//
//    TraceArguments("bar", 1ULL);
//    // args.size() == 1
//    // args.types()[0] == TRACE_VALUE_TYPE_UINT
//    // args.names()[0] == "bar"
//    // args.values()[0].as_uint == 100
//
//    TraceArguments("foo", "Hello", "bar", "World");
//    // args.size() == 2
//    // args.types()[0] == TRACE_VALUE_TYPE_STRING
//    // args.types()[1] == TRACE_VALUE_TYPE_STRING
//    // args.names()[0] == "foo"
//    // args.names()[1] == "bar"
//    // args.values()[0].as_string == "Hello"
//    // args.values()[1].as_string == "World"
//
//    std::string some_string = ...;
//    TraceArguments("str1", some_string);
//    // args.size() == 1
//    // args.types()[0] == TRACE_VALUE_TYPE_COPY_STRING
//    // args.names()[0] == "str1"
//    // args.values()[0].as_string == some_string.c_str()
//
// Note that TRACE_VALUE_TYPE_COPY_STRING corresponds to string pointers
// that point to temporary values that may disappear soon. The
// TraceArguments::CopyStringTo() method can be used to copy their content
// into a StringStorage memory block, and update the |as_string| value pointers
// to it to avoid keeping any dangling pointers. This is used by TraceEvent
// to keep copies of such strings in the log after their initialization values
// have disappeared.
//
// The TraceStringWithCopy helper class can be used to initialize a value
// from a regular string pointer with TRACE_VALUE_TYPE_COPY_STRING too, as in:
//
//     const char str[] = "....";
//     TraceArguments("foo", str, "bar", TraceStringWithCopy(str));
//     // args.size() == 2
//     // args.types()[0] == TRACE_VALUE_TYPE_STRING
//     // args.types()[1] == TRACE_VALUE_TYPE_COPY_STRING
//     // args.names()[0] == "foo"
//     // args.names()[1] == "bar"
//     // args.values()[0].as_string == str
//     // args.values()[1].as_string == str
//
//     StringStorage storage;
//     args.CopyStringTo(&storage, false, nullptr, nullptr);
//     // args.size() == 2
//     // args.types()[0] == TRACE_VALUE_TYPE_STRING
//     // args.types()[1] == TRACE_VALUE_TYPE_COPY_STRING
//     // args.names()[0] == "foo"
//     // args.names()[1] == "bar"
//     // args.values()[0].as_string == str
//     // args.values()[1].as_string == Address inside |storage|.
//
// Initialization from a std::unique_ptr<ConvertableToTraceFormat>
// is supported but will move ownership of the pointer objects to the
// TraceArguments instance:
//
//     class MyConvertableType :
//         public base::trace_event::AsConvertableToTraceFormat {
//        ...
//     };
//
//     {
//       TraceArguments args("foo" , std::make_unique<MyConvertableType>(...));
//       // args.size() == 1
//       // args.values()[0].as_convertable == address of MyConvertable object.
//     } // Calls |args| destructor, which will delete the object too.
//
// Finally, it is possible to support initialization from custom values by
// specializing the TraceValue::Helper<> template struct as described below.
//
// This is how values of custom types like WTF::CString can be passed directly
// to trace macros.

namespace base {

class Time;
class TimeTicks;
class ThreadTicks;

namespace trace_event {

// For any argument of type TRACE_VALUE_TYPE_CONVERTABLE the provided
// class must implement this interface. Note that unlike other values,
// these objects will be owned by the TraceArguments instance that points
// to them.
class BASE_EXPORT ConvertableToTraceFormat : public perfetto::DebugAnnotation {
 public:
  ConvertableToTraceFormat() = default;
  ConvertableToTraceFormat(const ConvertableToTraceFormat&) = delete;
  ConvertableToTraceFormat& operator=(const ConvertableToTraceFormat&) = delete;
  ~ConvertableToTraceFormat() override = default;

  // Append the class info to the provided |out| string. The appended
  // data must be a valid JSON object. Strings must be properly quoted, and
  // escaped. There is no processing applied to the content after it is
  // appended.
  virtual void AppendAsTraceFormat(std::string* out) const = 0;

  // Append the class info directly into the Perfetto-defined proto
  // format; this is attempted first and if this returns true,
  // AppendAsTraceFormat is not called. The ProtoAppender interface
  // acts as a bridge to avoid proto/Perfetto dependencies in base.
  class BASE_EXPORT ProtoAppender {
   public:
    virtual ~ProtoAppender() = default;

    virtual void AddBuffer(uint8_t* begin, uint8_t* end) = 0;
    // Copy all of the previous buffers registered with AddBuffer
    // into the proto, with the given |field_id|.
    virtual size_t Finalize(uint32_t field_id) = 0;
  };
  virtual bool AppendToProto(ProtoAppender* appender) const;

  // DebugAnnotation implementation.
  void Add(perfetto::protos::pbzero::DebugAnnotation*) const override;
};

const int kTraceMaxNumArgs = 2;

// A union used to hold the values of individual trace arguments.
//
// This is a POD union for performance reason. Initialization from an
// explicit C++ trace argument should be performed with the Init()
// templated method described below.
//
// Initialization from custom types is possible by implementing a custom
// TraceValue::Helper<> instantiation as described below.
//
// IMPORTANT: Pointer storage inside a TraceUnion follows specific rules:
//
//   - |as_pointer| is for raw pointers that should be treated as a simple
//     address and will never be dereferenced. Associated with the
//     TRACE_VALUE_TYPE_POINTER type.
//
//   - |as_string| is for C-string pointers, associated with both
//     TRACE_VALUE_TYPE_STRING and TRACE_VALUE_TYPE_COPY_STRING. The former
//     indicates that the string pointer is persistent (e.g. a C string
//     literal), while the second indicates that the pointer belongs to a
//     temporary variable that may disappear soon. The TraceArguments class
//     provides a CopyStringTo() method to copy these strings into a
//     StringStorage instance, which is useful if the instance needs to
//     survive longer than the temporaries.
//
//   - |as_convertable| is equivalent to
//     std::unique_ptr<ConvertableToTraceFormat>, except that it is a pointer
//     to keep this union POD and avoid un-necessary declarations and potential
//     code generation. This means that its ownership is passed to the
//     TraceValue instance when Init(std::unique_ptr<ConvertableToTraceFormat>)
//     is called, and that it will be deleted by the containing TraceArguments
//     destructor, or Reset() method.
//
union BASE_EXPORT TraceValue {
  bool as_bool;
  unsigned long long as_uint;
  long long as_int;
  double as_double;
  // This field is not a raw_ptr<> because it was filtered by the rewriter for:
  // #union
  RAW_PTR_EXCLUSION const void* as_pointer;
  const char* as_string;
  // This field is not a raw_ptr<> because it was filtered by the rewriter for:
  // #union
  RAW_PTR_EXCLUSION ConvertableToTraceFormat* as_convertable;
  // This field is not a raw_ptr<> because it was filtered by the rewriter for:
  // #union
  RAW_PTR_EXCLUSION protozero::HeapBuffered<
      perfetto::protos::pbzero::DebugAnnotation>* as_proto;

  // Static method to create a new TraceValue instance from a given
  // initialization value. Note that this deduces the TRACE_VALUE_TYPE_XXX
  // type but doesn't return it, use ForType<T>::value for this.
  //
  // Usage example:
  //     auto v = TraceValue::Make(100);
  //     auto v2 = TraceValue::Make("Some text string");
  //
  // IMPORTANT: Experience shows that the compiler generates worse code when
  // using this method rather than calling Init() directly on an existing
  // TraceValue union :-(
  //
  template <typename T>
  static TraceValue Make(T&& value) {
    TraceValue ret;
    ret.Init(std::forward<T>(value));
    return ret;
  }

  // Output current value as a JSON string. |type| must be a valid
  // TRACE_VALUE_TYPE_XXX value.
  void AppendAsJSON(unsigned char type, std::string* out) const;

  // Output current value as a string. If the output string is to be used
  // in a JSON format use AppendAsJSON instead. |type| must be valid
  // TRACE_VALUE_TYPE_XXX value.
  void AppendAsString(unsigned char type, std::string* out) const;

 private:
  void Append(unsigned char type, bool as_json, std::string* out) const;

 public:
  // TraceValue::Helper is used to provide information about initialization
  // value types and an initialization function. It is a struct that should
  // provide the following for supported initialization value types:
  //
  //    - kType: is a static TRACE_VALUE_TYPE_XXX constant.
  //
  //    - SetValue(TraceValue*, T): is a static inline method that sets
  //        TraceValue value from a given T value. Second parameter type
  //        can also be const T& or T&& to restrict uses.
  //
  // IMPORTANT: The type T must be std::decay_t<Q>, where Q is the real C++
  // argument type. I.e. you should not have to deal with reference types
  // in your specialization.
  //
  // Specializations are defined for integers, enums, floating point, pointers,
  // constant C string literals and pointers, std::string, time values below.
  //
  // Specializations for custom types are possible provided that there exists
  // a corresponding Helper specialization, for example:
  //
  //    template <>
  //    struct base::trace_event::TraceValue::Helper<Foo> {
  //      static constexpr unsigned char kTypes = TRACE_VALUE_TYPE_COPY_STRING;
  //      static inline void SetValue(TraceValue* v, const Foo& value) {
  //        v->as_string = value.c_str();
  //      }
  //    };
  //
  // Will allow code like:
  //
  //    Foo foo = ...;
  //    auto v = TraceValue::Make(foo);
  //
  // Or even:
  //    Foo foo = ...;
  //    TraceArguments args("foo_arg1", foo);
  //
  template <typename T>
  struct Helper {};

  template <typename T>
  static constexpr bool HasHelperSupport =
      requires { TraceValue::Helper<std::decay_t<T>>::kType; };

  // TypeCheck<T> is true iff T can be used to initialize a TraceValue instance.
  template <typename T>
  static constexpr bool TypeCheck =
      HasHelperSupport<T> ||
      perfetto::internal::has_traced_value_support<std::decay_t<T>>::value;

  // InitialValue<T>() returns the TRACE_VALUE_TYPE_XXX
  // corresponding to initialization values of type T.
  template <typename T>
    requires(TypeCheck<T>)
  static consteval unsigned char InitialValue() {
    if constexpr (HasHelperSupport<T>) {
      return Helper<std::decay_t<T>>::kType;
    }
    return TRACE_VALUE_TYPE_PROTO;
  }

  // There is no constructor to keep this structure POD intentionally.
  // This avoids un-needed initialization when only 0 or 1 arguments are
  // used to construct a TraceArguments instance. Use Init() instead to
  // perform explicit initialization from a given C++ value.

  // Initialize TraceValue instance from a C++ trace value.
  // This relies on the proper specialization of TraceValue::Helper<>
  // described below. Usage is simply:
  //
  //  TraceValue v;
  //  v.Init(<value>);
  //
  // NOTE: For ConvertableToTraceFormat values, see the notes above.
  template <class T>
    requires(HasHelperSupport<T>)
  void Init(T&& value) {
    using ValueType = std::decay_t<T>;
    Helper<ValueType>::SetValue(this, std::forward<T>(value));
  }

  template <class T>
    requires(
        !HasHelperSupport<T> &&
        perfetto::internal::has_traced_value_support<std::decay_t<T>>::value)
  void Init(T&& value) {
    as_proto = new protozero::HeapBuffered<
        perfetto::protos::pbzero::DebugAnnotation>();
    perfetto::WriteIntoTracedValue(
        perfetto::internal::CreateTracedValueFromProto(as_proto->get()),
        std::forward<T>(value));
  }
};

// TraceValue::Helper for integers and enums.
template <typename T>
  requires(std::is_integral_v<T> || std::is_enum_v<T>)
struct TraceValue::Helper<T> {
  static constexpr unsigned char kType =
      std::is_signed_v<T> ? TRACE_VALUE_TYPE_INT : TRACE_VALUE_TYPE_UINT;
  static inline void SetValue(TraceValue* v, T value) {
    v->as_uint = static_cast<unsigned long long>(value);
  }
};

// TraceValue::Helper for floating-point types
template <typename T>
  requires(std::is_floating_point_v<T>)
struct TraceValue::Helper<T> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_DOUBLE;
  static inline void SetValue(TraceValue* v, T value) { v->as_double = value; }
};

// TraceValue::Helper for bool.
template <>
struct TraceValue::Helper<bool> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_BOOL;
  static inline void SetValue(TraceValue* v, bool value) { v->as_bool = value; }
};

//  TraceValue::Helper for generic pointer types.
template <>
struct TraceValue::Helper<const void*> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_POINTER;
  static inline void SetValue(TraceValue* v, const void* value) {
    v->as_pointer = value;
  }
};

template <>
struct TraceValue::Helper<void*> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_POINTER;
  static inline void SetValue(TraceValue* v, void* value) {
    v->as_pointer = value;
  }
};

// TraceValue::Helper for raw persistent C strings.
template <>
struct TraceValue::Helper<const char*> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_STRING;
  static inline void SetValue(TraceValue* v, const char* value) {
    v->as_string = value;
  }
};

// TraceValue::Helper for std::string values.
template <>
struct TraceValue::Helper<std::string> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_COPY_STRING;
  static inline void SetValue(TraceValue* v, const std::string& value) {
    v->as_string = value.c_str();
  }
};

// Special case for scoped pointers to convertables to trace format.
// |CONVERTABLE_TYPE| must be a type whose pointers can be converted to a
// ConvertableToTraceFormat* pointer as well (e.g. a derived class).
// IMPORTANT: This takes an std::unique_ptr<CONVERTABLE_TYPE> value, and takes
// ownership of the pointed object!
template <typename CONVERTABLE_TYPE>
  requires(std::is_convertible_v<CONVERTABLE_TYPE*, ConvertableToTraceFormat*>)
struct TraceValue::Helper<std::unique_ptr<CONVERTABLE_TYPE>> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_CONVERTABLE;
  static inline void SetValue(TraceValue* v,
                              std::unique_ptr<CONVERTABLE_TYPE> value) {
    v->as_convertable = value.release();
  }
};

// Specialization for time-based values like base::Time, which provide a
// a ToInternalValue() method.
template <typename T>
  requires(std::is_same_v<T, base::Time> ||
           std::is_same_v<T, base::TimeTicks> ||
           std::is_same_v<T, base::ThreadTicks>)
struct TraceValue::Helper<T> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_INT;
  static inline void SetValue(TraceValue* v, const T& value) {
    v->as_int = value.ToInternalValue();
  }
};

// Simple container for const char* that should be copied instead of retained.
// The goal is to indicate that the C string is copyable, unlike the default
// Init(const char*) implementation. Usage is:
//
//    const char* str = ...;
//    v.Init(TraceStringWithCopy(str));
//
// Which will mark the string as TRACE_VALUE_TYPE_COPY_STRING, instead of
// TRACE_VALUE_TYPE_STRING.
//
class TraceStringWithCopy {
 public:
  explicit TraceStringWithCopy(const char* str) : str_(str) {}
  const char* str() const { return str_; }

 private:
  const char* str_;
};

template <>
struct TraceValue::Helper<TraceStringWithCopy> {
  static constexpr unsigned char kType = TRACE_VALUE_TYPE_COPY_STRING;
  static inline void SetValue(TraceValue* v, const TraceStringWithCopy& value) {
    v->as_string = value.str();
  }
};

class TraceArguments;

// A small class used to store a copy of all strings from a given
// TraceArguments instance (see below). When empty, this should only
// take the size of a pointer. Otherwise, this will point to a heap
// allocated block containing a size_t value followed by all characters
// in the storage area. For most cases, this is more efficient
// than using a std::unique_ptr<std::string> or an std::vector<char>.
class BASE_EXPORT StringStorage {
 public:
  constexpr StringStorage() = default;

  explicit StringStorage(size_t alloc_size) { Reset(alloc_size); }

  ~StringStorage() {
    if (data_) {
      ::free(data_);
    }
  }

  StringStorage(StringStorage&& other) noexcept : data_(other.data_) {
    other.data_ = nullptr;
  }

  StringStorage& operator=(StringStorage&& other) noexcept {
    if (this != &other) {
      if (data_) {
        ::free(data_);
      }
      data_ = other.data_;
      other.data_ = nullptr;
    }
    return *this;
  }

  // Reset storage area to new allocation size. Existing content might not
  // be preserved. If |alloc_size| is 0, this will free the storage area
  // as well.
  void Reset(size_t alloc_size = 0);

  // Accessors.
  constexpr size_t size() const { return data_ ? data_->size : 0u; }
  constexpr const char* data() const { return data_ ? data_->chars : nullptr; }
  constexpr char* data() { return data_ ? data_->chars : nullptr; }

  constexpr const char* begin() const { return data(); }
  constexpr const char* end() const { return data() + size(); }
  inline char* begin() { return data(); }
  inline char* end() { return data() + size(); }

  // True iff storage is empty.
  constexpr bool empty() const { return size() == 0; }

  // Returns true if |ptr| is inside the storage area, false otherwise.
  // Used during unit-testing.
  constexpr bool Contains(const void* ptr) const {
    const char* char_ptr = static_cast<const char*>(ptr);
    return (char_ptr >= begin() && char_ptr < end());
  }

  // Returns true if all string pointers in |args| are contained in this
  // storage area.
  bool Contains(const TraceArguments& args) const;

 private:
  // Heap allocated data block (variable size), made of:
  //
  //   - size: a size_t field, giving the size of the following |chars| array.
  //   - chars: an array of |size| characters, holding all zero-terminated
  //     strings referenced from a TraceArguments instance.
  struct Data {
    size_t size = 0;
    char chars[1];  // really |size| character items in storage.
  };

  // This is an owning pointer. Normally, using a std::unique_ptr<> would be
  // enough, but the compiler will then complaing about inlined constructors
  // and destructors being too complex (!), resulting in larger code for no
  // good reason.
  // RAW_PTR_EXCLUSION: As above, inlining bloats code for no good reason.
  RAW_PTR_EXCLUSION Data* data_ = nullptr;
};

// TraceArguments models an array of kMaxSize trace-related items,
// each one of them having:
//   - a name, which is a constant char array literal.
//   - a type, as described by TRACE_VALUE_TYPE_XXX macros.
//   - a value, stored in a TraceValue union.
//
// IMPORTANT: For TRACE_VALUE_TYPE_CONVERTABLE, the value holds an owning
//            pointer to an AsConvertableToTraceFormat instance, which will
//            be destroyed with the array (or moved out of it when passed
//            to a TraceEvent instance).
//
// For TRACE_VALUE_TYPE_COPY_STRING, the value holds a const char* pointer
// whose content will be copied when creating a TraceEvent instance.
//
// IMPORTANT: Most constructors and the destructor are all inlined
// intentionally, in order to let the compiler remove un-necessary operations
// and reduce machine code.
//
class BASE_EXPORT TraceArguments {
 public:
  // Maximum number of arguments held by this structure.
  static constexpr size_t kMaxSize = 2;

  // Default constructor, no arguments.
  TraceArguments() = default;

  // Constructor for a single argument.
  template <typename T>
    requires(TraceValue::TypeCheck<T>)
  TraceArguments(const char* arg1_name, T&& arg1_value) : size_(1) {
    types_[0] = TraceValue::InitialValue<T>();
    names_[0] = arg1_name;
    values_[0].Init(std::forward<T>(arg1_value));
  }

  // Constructor for two arguments.
  template <typename T1, typename T2>
    requires(TraceValue::TypeCheck<T1> && TraceValue::TypeCheck<T2>)
  TraceArguments(const char* arg1_name,
                 T1&& arg1_value,
                 const char* arg2_name,
                 T2&& arg2_value)
      : size_(2) {
    types_[0] = TraceValue::InitialValue<T1>();
    types_[1] = TraceValue::InitialValue<T2>();
    names_[0] = arg1_name;
    names_[1] = arg2_name;
    values_[0].Init(std::forward<T1>(arg1_value));
    values_[1].Init(std::forward<T2>(arg2_value));
  }

  // Constructor used to convert a legacy set of arguments when there
  // are no convertable values at all.
  TraceArguments(int num_args,
                 const char* const* arg_names,
                 const unsigned char* arg_types,
                 const unsigned long long* arg_values);

  // Constructor used to convert legacy set of arguments, where the
  // convertable values are also provided by an array of CONVERTABLE_TYPE.
  template <typename CONVERTABLE_TYPE>
  TraceArguments(int num_args,
                 const char* const* arg_names,
                 const unsigned char* arg_types,
                 const unsigned long long* arg_values,
                 CONVERTABLE_TYPE* arg_convertables) {
    static int max_args = static_cast<int>(kMaxSize);
    if (num_args > max_args) {
      num_args = max_args;
    }
    size_ = static_cast<unsigned char>(num_args);
    for (size_t n = 0; n < size_; ++n) {
      types_[n] = arg_types[n];
      names_[n] = arg_names[n];
      if (arg_types[n] == TRACE_VALUE_TYPE_CONVERTABLE) {
        values_[n].Init(
            std::forward<CONVERTABLE_TYPE>(std::move(arg_convertables[n])));
      } else {
        values_[n].as_uint = arg_values[n];
      }
    }
  }

  // Destructor. NOTE: Intentionally inlined (see note above).
  ~TraceArguments() {
    for (size_t n = 0; n < size_; ++n) {
      if (types_[n] == TRACE_VALUE_TYPE_CONVERTABLE) {
        delete values_[n].as_convertable;
      }
      if (types_[n] == TRACE_VALUE_TYPE_PROTO) {
        delete values_[n].as_proto;
      }
    }
  }

  // Disallow copy operations.
  TraceArguments(const TraceArguments&) = delete;
  TraceArguments& operator=(const TraceArguments&) = delete;

  // Allow move operations.
  TraceArguments(TraceArguments&& other) noexcept {
    ::memcpy(this, &other, sizeof(*this));
    // All owning pointers were copied to |this|. Setting |other.size_| will
    // mask the pointer values still in |other|.
    other.size_ = 0;
  }

  TraceArguments& operator=(TraceArguments&&) noexcept;

  // Accessors
  size_t size() const { return size_; }
  const unsigned char* types() const { return types_; }
  const char* const* names() const { return names_; }
  const TraceValue* values() const { return values_; }

  // Reset to empty arguments list.
  void Reset();

  // Use |storage| to copy all copyable strings.
  // If |copy_all_strings| is false, then only the TRACE_VALUE_TYPE_COPY_STRING
  // values will be copied into storage. If it is true, then argument names are
  // also copied to storage, as well as the strings pointed to by
  // |*extra_string1| and |*extra_string2|.
  // NOTE: If there are no strings to copy, |*storage| is left untouched.
  void CopyStringsTo(StringStorage* storage,
                     bool copy_all_strings,
                     const char** extra_string1,
                     const char** extra_string2);

  // Append debug string representation to |*out|.
  void AppendDebugString(std::string* out);

 private:
  unsigned char size_ = 0;
  unsigned char types_[kMaxSize];
  const char* names_[kMaxSize];
  TraceValue values_[kMaxSize];
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACE_ARGUMENTS_H_
