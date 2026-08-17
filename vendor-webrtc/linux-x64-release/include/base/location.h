// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_LOCATION_H_
#define BASE_LOCATION_H_

#include <compare>
#include <string>

#include "base/base_export.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/trace_event/base_tracing_forward.h"
#include "build/build_config.h"

namespace base {

// Location provides basic info where of an object was constructed, or was
// significantly brought to life.
class BASE_EXPORT Location {
 public:
  Location();
  Location(const Location& other);
  Location(Location&& other) noexcept;
  Location& operator=(const Location& other);

  static Location CreateForTesting(const char* function_name,
                                   const char* file_name,
                                   int line_number,
                                   const void* program_counter) {
    return Location(function_name, file_name, line_number, program_counter);
  }

  // Comparator for testing. The program counter should uniquely
  // identify a location.
  friend bool operator==(const Location& lhs, const Location& rhs) {
    return lhs.program_counter_ == rhs.program_counter_;
  }

  // The program counter should uniquely identify a location. There is no
  // guarantee that a program counter corresponds to unique function/file/line
  // values, based on how it's constructed, and therefore equivalent locations
  // could be distinguishable.
  friend std::weak_ordering operator<=>(const Location& lhs,
                                        const Location& rhs) {
    return lhs.program_counter_ <=> rhs.program_counter_;
  }

  // Returns true if there is source code location info. If this is false,
  // the Location object only contains a program counter or is
  // default-initialized (the program counter is also null).
  bool has_source_info() const { return function_name_ && file_name_; }

  // Will be nullptr for default initialized Location objects and when source
  // names are disabled.
  const char* function_name() const { return function_name_; }

  // Will be nullptr for default initialized Location objects and when source
  // names are disabled.
  const char* file_name() const { return file_name_; }

  // Will be -1 for default initialized Location objects and when source names
  // are disabled.
  int line_number() const { return line_number_; }

  // The address of the code generating this Location object. Should always be
  // valid except for default initialized Location objects, which will be
  // nullptr.
  const void* program_counter() const { return program_counter_; }

  // Converts to the most user-readable form possible. If function and filename
  // are not available, this will return "pc:<hex address>".
  std::string ToString() const;

  // Write a representation of this object into a trace.
  void WriteIntoTrace(perfetto::TracedValue context) const;

  static Location Current(const char* function_name = __builtin_FUNCTION(),
                          const char* file_name = __builtin_FILE(),
                          int line_number = __builtin_LINE());

  static Location CurrentWithoutFunctionName(
      const char* file_name = __builtin_FILE(),
      int line_number = __builtin_LINE());

 private:
  // Only initializes the file name and program counter, the source information
  // will be null for the strings, and -1 for the line number.
  // TODO(http://crbug.com/760702) remove file name from this constructor.
  Location(const char* file_name, const void* program_counter);

  // Constructor should be called with a long-lived char*, such as __FILE__.
  // It assumes the provided value will persist as a global constant, and it
  // will not make a copy of it.
  Location(const char* function_name,
           const char* file_name,
           int line_number,
           const void* program_counter);

  const char* function_name_ = nullptr;
  const char* file_name_ = nullptr;
  int line_number_ = -1;

  // `program_counter_` is not a raw_ptr<...> for performance reasons (based on
  // analysis of sampling profiler data and tab_search:top100:2020).
  RAW_PTR_EXCLUSION const void* program_counter_ = nullptr;
};

BASE_EXPORT const void* GetProgramCounter();

#define FROM_HERE ::base::Location::Current()

}  // namespace base

#endif  // BASE_LOCATION_H_
