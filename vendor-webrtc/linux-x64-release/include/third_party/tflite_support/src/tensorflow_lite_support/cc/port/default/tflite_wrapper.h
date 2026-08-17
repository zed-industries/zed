/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/
#ifndef TENSORFLOW_LITE_SUPPORT_CC_PORT_DEFAULT_TFLITE_WRAPPER_H_
#define TENSORFLOW_LITE_SUPPORT_CC_PORT_DEFAULT_TFLITE_WRAPPER_H_

#include <memory>
#include <string>
#include <utility>

#include "absl/status/status.h"  // from @com_google_absl
#include "flatbuffers/flatbuffers.h"  // from @flatbuffers
#include "tensorflow/lite/acceleration/configuration/configuration.pb.h"
#include "tensorflow/lite/acceleration/configuration/delegate_registry.h"
#include "tensorflow/lite/c/common.h"
#include "tensorflow/lite/experimental/acceleration/mini_benchmark/mini_benchmark.h"
#include "tensorflow/lite/interpreter.h"
#include "tensorflow/lite/interpreter_builder.h"

namespace tflite {
namespace support {

// Options that are created by `TFLiteInterpreterWrapper` and will help to
// initialize Interpreter in the callback function. `TFLiteInterpreterWrapper`
// retains ownership of the included options, and will ensure that they remain
// valid for the duration of the created interpreter's lifetime.
struct InterpreterCreationResources {
  // The delegate created, based on the parameters in `ComputeSettings`.
  // `TfLiteInterpreterWrapper` exclusively owns the `TfLiteDelegate` object,
  // and maintains it through out the lifetime of `TfLiteInterpreterWrapper`.
  TfLiteDelegate* optional_delegate;

  // Number of threads to use, or -1 to use the default number of threads.
  int num_threads = -1;

  // Apply the InterpreterCreationResources to the InterpreterBuilder.
  // Note: caller is responsible for ensuring that arguments are valid,
  // e.g. that num_threads >= -1.
  void ApplyTo(tflite::InterpreterBuilder* interpreter_builder) const {
    if (optional_delegate != nullptr) {
      interpreter_builder->AddDelegate(optional_delegate);
    }
    if (num_threads != -1) {
      // We ignore the TfLiteStatus return value here; caller is responsible
      // for checking that num_threads is valid.
      (void)interpreter_builder->SetNumThreads(num_threads);
    }
  }
};

// Wrapper for a TfLiteInterpreter that may be accelerated [1]. Meant to be
// substituted for `unique_ptr<tflite::Interpreter>` class members.
//
// This class is in charge of:
// * Picking, instantiating and configuring the right delegate for the provided
//   ComputeSettings [2],
// * Providing methods to initialize and invoke the Interpreter with optional
//   (controlled through the ComputeSettings) automatic fallback to CPU if any
//   acceleration-related error occurs at compilation or runtime.
// * TODO(b/169474250) Cache interpreters for multiple input sizes to enable
//   performant acceleration for the case where input size changes frequently.
//
// IMPORTANT: The only supported delegates are (as defined in [1]) NONE, GPU,
// HEXAGON, NNAPI, EDGETPU (Google internal), and EDGETPU_CORAL. Specifying
// another delegate type may cause an UnimplementedError to be thrown.
//
// Like TfLiteInterpreter, this class is thread-compatible. Use from multiple
// threads must be guarded by synchronization outside this class.
//
// [1]:
// https://github.com/tensorflow/tensorflow/blob/master/tensorflow/lite/acceleration/configuration/configuration.proto
class TfLiteInterpreterWrapper {
 public:
  // Creates an instance to be associated with a TfLite model that could be
  // identified by (`default_model_namespace`, `default_model_id`). Note the
  // model identifier is generally used for the sake of logging.
  TfLiteInterpreterWrapper(const std::string& default_model_namespace,
                           const std::string& default_model_id);
  TfLiteInterpreterWrapper()
      : TfLiteInterpreterWrapper("org.tensorflow.lite.support",
                                 "unknown_model_id") {}

  virtual ~TfLiteInterpreterWrapper() = default;

  // Calls `interpreter_initializer` to construct the Interpreter, then
  // initializes it with the appropriate delegate (if any) specified through
  // `compute_settings` and finally calls AllocateTensors() on it.
  //
  // Whether or not this function automatically falls back to using CPU in case
  // initialization with a delegate fails depends on the FallbackSettings
  // specified in the TFLiteSettings of the provided ComputeSettings: if the
  // `allow_automatic_fallback_on_compilation_error` field is set to true,
  // fallback will automatically happen; otherwise an InternalError will be
  // thrown.
  // This flag allows callers to rely on this function whether or not they
  // actually want fallback to happen; if they don't, it will ensure that the
  // configuration doesn't accidentally trigger fallback.
  absl::Status InitializeWithFallback(
      std::function<absl::Status(const InterpreterCreationResources&,
                                 std::unique_ptr<tflite::Interpreter>*)>
          interpreter_initializer,
      const tflite::proto::ComputeSettings& compute_settings);

  // Deprecated: Use the one above with `InterpreterCreationResources` instead.
  absl::Status InitializeWithFallback(
      std::function<absl::Status(std::unique_ptr<tflite::Interpreter>*)>
          interpreter_initializer,
      const tflite::proto::ComputeSettings& compute_settings);

  // Calls `set_inputs` and then Invoke() on the interpreter.
  //
  // Whether or not this function automatically falls back to using CPU in case
  // invocation with a delegate fails depends on the FallbackSettings
  // specified in the TFLiteSettings of the ComputeSettings provided at
  // initialization: if the `allow_automatic_fallback_on_execution_error`
  // field is set to true, fallback will automatically happen; otherwise an
  // InternalError will be thrown.
  // This flag allows callers to rely on this function whether or not they
  // actually want fallback to happen; if they don't, it will ensure that the
  // configuration doesn't accidentally trigger fallback.
  absl::Status InvokeWithFallback(
      const std::function<absl::Status(tflite::Interpreter* interpreter)>&
          set_inputs);

  // Calls Invoke() on the interpreter. Caller must have set up inputs
  // before-hand.
  absl::Status InvokeWithoutFallback();

  // Cancels the current TFLite **CPU** inference.
  //
  // IMPORTANT: If inference is entirely running on a delegate, this has no
  // effect; if inference is partially delegated, only the CPU part is
  // cancelled.
  //
  // Usually called on a different thread than the one Invoke() is running
  // on. Calling Cancel() while InvokeWithFallback() or InvokeWithoutFallback()
  // is running may cause these methods to return a `CancelledError` with empty
  // results. Calling Cancel() at any other time doesn't have any effect.
  //
  // InvokeWithFallback() and InvokeWithoutFallback() reset the cancel flag
  // right before the underlying Invoke() is called, so these two methods can be
  // called again on the same instance after a call to Cancel().
  //
  // Note that this is the only method that can be called from another thread
  // without locking.
  void Cancel();

  // Accesses the underlying interpreter for other methods.
  tflite::Interpreter& operator*() { return *interpreter_; }
  tflite::Interpreter* operator->() { return interpreter_.get(); }
  tflite::Interpreter& operator*() const { return *interpreter_; }
  tflite::Interpreter* operator->() const { return interpreter_.get(); }
  tflite::Interpreter* get() const { return interpreter_.get(); }

  TfLiteInterpreterWrapper(const TfLiteInterpreterWrapper&) = delete;
  TfLiteInterpreterWrapper& operator=(const TfLiteInterpreterWrapper&) = delete;

  // Whether an error has occurred with the delegate.
  bool HasDelegateError() { return got_error_do_not_delegate_anymore_; }

  // Whether the on-device mini-benchmark has completed for those TfLite
  // acceleration configurations that are specified in passed-in
  // ComputeSettings. If it finishes, the next time this same InterpreterWrapper
  // object is created (i.e. w/ the same model and the same
  // mini-benchmark-related configurations), the best acceleration configuration
  // will be picked up and used.
  bool HasMiniBenchmarkCompleted();

  const tflite::proto::ComputeSettings& compute_settings() const {
    return compute_settings_;
  }

 protected:
  // The delegate used to accelerate inference.
  Interpreter::TfLiteDelegatePtr delegate_;
  // The corresponding delegate plugin.
  std::unique_ptr<tflite::delegates::DelegatePluginInterface> delegate_plugin_;

 private:
  // Performs sanity checks on the provided ComputeSettings.
  static absl::Status SanityCheckComputeSettings(
      const tflite::proto::ComputeSettings& compute_settings);

  // Inner function for initializing an interpreter with fallback, optionally
  // resizing input tensors by calling `resize` on the newly initialized
  // interpreter.
  absl::Status InitializeWithFallbackAndResize(
      std::function<absl::Status(Interpreter* interpreter)> resize =
          [](Interpreter* interpreter) { return absl::OkStatus(); });

  // Initializes the delegate plugin and creates the delegate.
  absl::Status InitializeDelegate();

  // Wrapper around the interpreter's `AllocateTensors()` method converting the
  // returned `TfLiteStatus` to an `absl::Status`.
  absl::Status AllocateTensors();

  absl::Status LoadDelegatePlugin(const std::string&,
                                  const tflite::TFLiteSettings&);

  std::string ModelNamespace();
  std::string ModelID();

  // The interpreter instance being used.
  std::unique_ptr<tflite::Interpreter> interpreter_;
  // The function used to initialize the interpreter and store it into the
  // provided `std::unique_ptr`.
  // This is typically a wrapper function around `tflite::InterpreterBuilder`,
  // giving the caller the opportunity to hook-up a custom `tflite::OpResolver`
  // and / or `tflite::ErrorReporter`.
  std::function<absl::Status(const InterpreterCreationResources&,
                             std::unique_ptr<Interpreter>*)>
      interpreter_initializer_;

  // The ComputeSettings provided at initialization time.
  // Note when TfLite mini-benchmark is enabled, it could be changed to the
  // best TfLite acceleration setting selected.
  tflite::proto::ComputeSettings compute_settings_;

  // Set to true if an occurs with the specified delegate (if any), causing
  // future calls to fallback on CPU.
  bool got_error_do_not_delegate_anymore_;

  // Fallback behavior as specified through the ComputeSettings.
  bool fallback_on_compilation_error_;
  bool fallback_on_execution_error_;

  std::string default_model_namespace_;
  std::string default_model_id_;

  // Used to convert the ComputeSettings proto to FlatBuffer format.
  flatbuffers::FlatBufferBuilder flatbuffers_builder_;

  // Cancellation flag definition.
  struct CancelFlag {
    // Mutex to guard the `cancel_flag`.
    mutable absl::Mutex cancel_mutex;

    // A flag indicates if the caller cancels the TFLite interpreter invocation.
    bool cancel_flag ABSL_GUARDED_BY(cancel_mutex) = false;

    // Returns `cancel_flag`.
    bool Get() const ABSL_LOCKS_EXCLUDED(cancel_mutex) {
      absl::MutexLock cancel_lock(&cancel_mutex);
      return cancel_flag;
    }

    // Sets `cancel_flag` to `value`.
    void Set(bool value) ABSL_LOCKS_EXCLUDED(cancel_mutex) {
      absl::MutexLock cancel_lock(&cancel_mutex);
      cancel_flag = value;
    }
  };
  CancelFlag cancel_flag_;

  std::unique_ptr<tflite::acceleration::MiniBenchmark> mini_benchmark_;

  // Sets up the TFLite invocation cancellation by
  // tflite::Interpreter::SetCancellationFunction().
  void SetTfLiteCancellation();
};

}  // namespace support
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_PORT_DEFAULT_TFLITE_WRAPPER_H_
