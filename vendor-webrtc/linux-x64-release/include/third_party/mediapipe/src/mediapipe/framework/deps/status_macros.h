// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Helper macros and methods to return and propagate errors with
// `absl::Status`.
//
// The owners of mediapipe do not endorse use of these macros as a good
// programming practice, and would prefer that you write the equivalent C++
// directly. The macros are provided and supported for those that disagree,
// with the goal of having a single, consistent, and robust implementation.

#ifndef MEDIAPIPE_DEPS_STATUS_MACROS_H_
#define MEDIAPIPE_DEPS_STATUS_MACROS_H_

#include "mediapipe/framework/deps/status.h"
#include "mediapipe/framework/deps/status_builder.h"

// Evaluates an expression that produces a `absl::Status`. If the status
// is not ok, returns it from the current function.
//
// For example:
//   absl::Status MultiStepFunction() {
//     MP_RETURN_IF_ERROR(Function(args...));
//     MP_RETURN_IF_ERROR(foo.Method(args...));
//     return absl::OkStatus();
//   }
//
// The macro ends with a `mediapipe::StatusBuilder` which allows the returned
// status to be extended with more details.  Any chained expressions after the
// macro will not be evaluated unless there is an error.
//
// For example:
//   absl::Status MultiStepFunction() {
//     MP_RETURN_IF_ERROR(Function(args...)) << "in MultiStepFunction";
//     MP_RETURN_IF_ERROR(foo.Method(args...)).Log(base_logging::ERROR)
//         << "while processing query: " << query.DebugString();
//     return absl::OkStatus();
//   }
//
// `mediapipe::StatusBuilder` supports adapting the builder chain using a
// `With` method and a functor.  This allows for powerful extensions to the
// macro.
//
// For example, teams can define local policies to use across their code:
//
//   StatusBuilder TeamPolicy(StatusBuilder builder) {
//     return std::move(builder.Log(base_logging::WARNING).Attach(...));
//   }
//
//   MP_RETURN_IF_ERROR(foo()).With(TeamPolicy);
//   MP_RETURN_IF_ERROR(bar()).With(TeamPolicy);
//
// Changing the return type allows the macro to be used with Task and Rpc
// interfaces.  See `mediapipe::TaskReturn` and `rpc::RpcSetStatus` for
// details.
//
//   void Read(StringPiece name, mediapipe::Task* task) {
//     int64 id;
//     MP_RETURN_IF_ERROR(GetIdForName(name, &id)).With(TaskReturn(task));
//     MP_RETURN_IF_ERROR(ReadForId(id)).With(TaskReturn(task));
//     task->Return();
//   }
//
// If using this macro inside a lambda, you need to annotate the return type
// to avoid confusion between a `mediapipe::StatusBuilder` and a
// `absl::Status` type. E.g.
//
//   []() -> absl::Status {
//     MP_RETURN_IF_ERROR(Function(args...));
//     MP_RETURN_IF_ERROR(foo.Method(args...));
//     return absl::OkStatus();
//   }
#define MP_RETURN_IF_ERROR(expr)                                     \
  MP_STATUS_MACROS_IMPL_ELSE_BLOCKER_                                \
  if (mediapipe::status_macro_internal::StatusAdaptorForMacros       \
          status_macro_internal_adaptor = {(expr), MEDIAPIPE_LOC}) { \
  } else /* NOLINT */                                                \
    return status_macro_internal_adaptor.Consume()

// Executes an expression `rexpr` that returns a `absl::StatusOr<T>`. On
// OK, extracts its value into the variable defined by `lhs`, otherwise returns
// from the current function. By default the error status is returned
// unchanged, but it may be modified by an `error_expression`. If there is an
// error, `lhs` is not evaluated; thus any side effects that `lhs` may have
// only occur in the success case.
//
// Interface:
//
//   MP_ASSIGN_OR_RETURN(lhs, rexpr)
//   MP_ASSIGN_OR_RETURN(lhs, rexpr, error_expression);
//
// WARNING: expands into multiple statements; it cannot be used in a single
// statement (e.g. as the body of an if statement without {})!
//
// Example: Declaring and initializing a new variable (ValueType can be anything
//          that can be initialized with assignment, including references):
//   MP_ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(arg));
//
// Example: Assigning to an existing variable:
//   ValueType value;
//   MP_ASSIGN_OR_RETURN(value, MaybeGetValue(arg));
//
// Example: Assigning to an expression with side effects:
//   MyProto data;
//   MP_ASSIGN_OR_RETURN(*data.mutable_str(), MaybeGetValue(arg));
//   // No field "str" is added on error.
//
// Example: Assigning to a std::unique_ptr.
//   MP_ASSIGN_OR_RETURN(std::unique_ptr<T> ptr, MaybeGetPtr(arg));
//
// If passed, the `error_expression` is evaluated to produce the return
// value. The expression may reference any variable visible in scope, as
// well as a `mediapipe::StatusBuilder` object populated with the error and
// named by a single underscore `_`. The expression typically uses the
// builder to modify the status and is returned directly in manner similar
// to MP_RETURN_IF_ERROR. The expression may, however, evaluate to any type
// returnable by the function, including (void). For example:
//
// Example: Adjusting the error message.
//   MP_ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    _ << "while processing query " << query.DebugString());
//
// Example: Logging the error on failure.
//   MP_ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query), _.LogError());
//
#define MP_ASSIGN_OR_RETURN(...)                                  \
  MP_STATUS_MACROS_IMPL_GET_VARIADIC_(                            \
      (__VA_ARGS__, MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_3_, \
       MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_2_))             \
  (__VA_ARGS__)

// =================================================================
// == Implementation details, do not rely on anything below here. ==
// =================================================================

// MSVC incorrectly expands variadic macros, splice together a macro call to
// work around the bug.
#define MP_STATUS_MACROS_IMPL_GET_VARIADIC_HELPER_(_1, _2, _3, NAME, ...) NAME
#define MP_STATUS_MACROS_IMPL_GET_VARIADIC_(args) \
  MP_STATUS_MACROS_IMPL_GET_VARIADIC_HELPER_ args

#define MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_2_(lhs, rexpr)               \
  MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_(                                  \
      MP_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__), lhs, rexpr,   \
      return mediapipe::StatusBuilder(                                         \
          std::move(MP_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__)) \
              .status(),                                                       \
          MEDIAPIPE_LOC))
#define MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_3_(lhs, rexpr,               \
                                                     error_expression)         \
  MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_(                                  \
      MP_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__), lhs, rexpr,   \
      mediapipe::StatusBuilder _(                                              \
          std::move(MP_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__)) \
              .status(),                                                       \
          MEDIAPIPE_LOC);                                                      \
      (void)_; /* error_expression is allowed to not use this variable */      \
      return (error_expression))
#define MP_STATUS_MACROS_IMPL_MP_ASSIGN_OR_RETURN_(statusor, lhs, rexpr, \
                                                   error_expression)     \
  auto statusor = (rexpr);                                               \
  if (ABSL_PREDICT_FALSE(!statusor.ok())) {                              \
    error_expression;                                                    \
  }                                                                      \
  lhs = std::move(statusor).value()

// Internal helper for concatenating macro values.
#define MP_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y) x##y
#define MP_STATUS_MACROS_IMPL_CONCAT_(x, y) \
  MP_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y)

// The GNU compiler emits a warning for code like:
//
//   if (foo)
//     if (bar) { } else baz;
//
// because it thinks you might want the else to bind to the first if.  This
// leads to problems with code like:
//
//   if (do_expr) MP_RETURN_IF_ERROR(expr) << "Some message";
//
// The "switch (0) case 0:" idiom is used to suppress this.
#define MP_STATUS_MACROS_IMPL_ELSE_BLOCKER_ \
  switch (0)                                \
  case 0:                                   \
  default:  // NOLINT

namespace mediapipe {
namespace status_macro_internal {

// Provides a conversion to bool so that it can be used inside an if statement
// that declares a variable.
class StatusAdaptorForMacros {
 public:
  StatusAdaptorForMacros(const absl::Status& status, source_location location)
      : builder_(status, location) {}

  StatusAdaptorForMacros(absl::Status&& status, source_location location)
      : builder_(std::move(status), location) {}

  StatusAdaptorForMacros(const StatusBuilder& builder,
                         source_location /*location*/)
      : builder_(builder) {}

  StatusAdaptorForMacros(StatusBuilder&& builder, source_location /*location*/)
      : builder_(std::move(builder)) {}

  StatusAdaptorForMacros(const StatusAdaptorForMacros&) = delete;
  StatusAdaptorForMacros& operator=(const StatusAdaptorForMacros&) = delete;

  explicit operator bool() const { return ABSL_PREDICT_TRUE(builder_.ok()); }

  StatusBuilder&& Consume() { return std::move(builder_); }

 private:
  StatusBuilder builder_;
};

}  // namespace status_macro_internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_STATUS_MACROS_H_
