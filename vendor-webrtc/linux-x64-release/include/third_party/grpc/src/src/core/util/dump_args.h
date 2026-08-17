// Copyright 2024 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_UTIL_DUMP_ARGS_H
#define GRPC_SRC_CORE_UTIL_DUMP_ARGS_H

#include <ostream>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"

namespace grpc_core {
namespace dump_args_detail {

class DumpArgs {
 public:
  template <typename... Args>
  explicit DumpArgs(const char* arg_string, const Args&... args)
      : arg_string_(arg_string) {
    (AddDumper(&args), ...);
  }

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const DumpArgs& dumper) {
    CustomSinkImpl<Sink> custom_sink(sink);
    dumper.Stringify(custom_sink);
  }

  friend std::ostream& operator<<(std::ostream& out, const DumpArgs& dumper) {
    return out << absl::StrCat(dumper);
  }

 private:
  class CustomSink {
   public:
    virtual void Append(absl::string_view x) = 0;

   protected:
    ~CustomSink() = default;
  };

  template <typename Sink>
  class CustomSinkImpl final : public CustomSink {
   public:
    explicit CustomSinkImpl(Sink& sink) : sink_(sink) {}
    void Append(absl::string_view x) override { sink_.Append(x); }

   private:
    Sink& sink_;
  };

  template <typename T>
  int AddDumper(T* p) {
    arg_dumpers_.push_back(
        [p](CustomSink& os) { os.Append(absl::StrCat(*p)); });
    return 0;
  }

  int AddDumper(void const* const* p) {
    arg_dumpers_.push_back(
        [p](CustomSink& os) { os.Append(absl::StrFormat("%p", *p)); });
    return 0;
  }

  template <typename T>
  int AddDumper(T const* const* p) {
    return AddDumper(reinterpret_cast<void const* const*>(p));
  }

  template <typename T>
  int AddDumper(T* const* p) {
    return AddDumper(const_cast<T const* const*>(p));
  }

  template <typename T>
  int AddDumper(T const** p) {
    return AddDumper(const_cast<T const* const*>(p));
  }

  void Stringify(CustomSink& sink) const;

  const char* arg_string_;
  std::vector<absl::AnyInvocable<void(CustomSink&) const>> arg_dumpers_;
};

}  // namespace dump_args_detail
}  // namespace grpc_core

// Helper to print a list of variables and their values.
// Each type must be streamable to std::ostream.
// Usage:
//   int a = 1;
//   int b = 2;
//   LOG(INFO) << GRPC_DUMP_ARGS(a, b)
// Output:
//   a = 1, b = 2
#define GRPC_DUMP_ARGS(...) \
  grpc_core::dump_args_detail::DumpArgs(#__VA_ARGS__, __VA_ARGS__)

#endif  // GRPC_SRC_CORE_UTIL_DUMP_ARGS_H
