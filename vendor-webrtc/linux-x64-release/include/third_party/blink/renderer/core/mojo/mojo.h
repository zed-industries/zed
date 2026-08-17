// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_MOJO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_MOJO_H_

#include "mojo/public/c/system/types.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MojoCreateDataPipeOptions;
class MojoCreateDataPipeResult;
class MojoCreateMessagePipeResult;
class MojoCreateSharedBufferResult;
class MojoHandle;
class ScriptState;
class V8MojoScope;

class CORE_EXPORT Mojo final : public ScriptWrappable,
                               public Supplementable<Mojo> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Mojo() = default;
  Mojo(const Mojo&) = delete;
  Mojo& operator=(const Mojo&) = delete;

  // MojoResult
  static const MojoResult kResultOk = MOJO_RESULT_OK;
  static const MojoResult kResultCancelled = MOJO_RESULT_CANCELLED;
  static const MojoResult kResultUnknown = MOJO_RESULT_UNKNOWN;
  static const MojoResult kResultInvalidArgument = MOJO_RESULT_INVALID_ARGUMENT;
  static const MojoResult kResultDeadlineExceeded =
      MOJO_RESULT_DEADLINE_EXCEEDED;
  static const MojoResult kResultNotFound = MOJO_RESULT_NOT_FOUND;
  static const MojoResult kResultAlreadyExists = MOJO_RESULT_ALREADY_EXISTS;
  static const MojoResult kResultPermissionDenied =
      MOJO_RESULT_PERMISSION_DENIED;
  static const MojoResult kResultResourceExhausted =
      MOJO_RESULT_RESOURCE_EXHAUSTED;
  static const MojoResult kResultFailedPrecondition =
      MOJO_RESULT_FAILED_PRECONDITION;
  static const MojoResult kResultAborted = MOJO_RESULT_ABORTED;
  static const MojoResult kResultOutOfRange = MOJO_RESULT_OUT_OF_RANGE;
  static const MojoResult kResultUnimplemented = MOJO_RESULT_UNIMPLEMENTED;
  static const MojoResult kResultInternal = MOJO_RESULT_INTERNAL;
  static const MojoResult kResultUnavailable = MOJO_RESULT_UNAVAILABLE;
  static const MojoResult kResultDataLoss = MOJO_RESULT_DATA_LOSS;
  static const MojoResult kResultBusy = MOJO_RESULT_BUSY;
  static const MojoResult kResultShouldWait = MOJO_RESULT_SHOULD_WAIT;

  static MojoCreateMessagePipeResult* createMessagePipe();
  static MojoCreateDataPipeResult* createDataPipe(
      const MojoCreateDataPipeOptions*);
  static MojoCreateSharedBufferResult* createSharedBuffer(unsigned num_bytes);

  static void bindInterface(ScriptState*,
                            const String& interface_name,
                            MojoHandle*,
                            const V8MojoScope& scope,
                            ExceptionState& exception_state);

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
    Supplementable<Mojo>::Trace(visitor);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MOJO_MOJO_H_
