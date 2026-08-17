// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_POST_MESSAGE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_POST_MESSAGE_HELPER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/messaging/user_activation_snapshot.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class ExecutionContext;
class ExceptionState;
class PostMessageOptions;
class StructuredSerializeOptions;
class ScriptValue;
class SecurityOrigin;
class SerializedScriptValue;
class Transferables;
class WindowPostMessageOptions;

class CORE_EXPORT PostMessageHelper {
  STATIC_ONLY(PostMessageHelper);

 public:
  static scoped_refptr<SerializedScriptValue> SerializeMessageByMove(
      v8::Isolate*,
      const ScriptValue& message,
      const StructuredSerializeOptions* options,
      Transferables& transferables,
      ExceptionState&);

  static scoped_refptr<SerializedScriptValue> SerializeMessageByCopy(
      v8::Isolate*,
      const ScriptValue& message,
      const StructuredSerializeOptions* options,
      Transferables& transferables,
      ExceptionState&);

  // Create a snapshot of the user activation state. Return null if this if the
  // execution context is not a window.
  static mojom::blink::UserActivationSnapshotPtr CreateUserActivationSnapshot(
      ExecutionContext*,
      const PostMessageOptions*);

  // Extracts target origin from |options|. Throws SyntaxError, if the origin
  // provided is an invalid URL.
  static scoped_refptr<const SecurityOrigin> GetTargetOrigin(
      const WindowPostMessageOptions* options,
      const ExecutionContext& context,
      ExceptionState& state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SERIALIZATION_POST_MESSAGE_HELPER_H_
