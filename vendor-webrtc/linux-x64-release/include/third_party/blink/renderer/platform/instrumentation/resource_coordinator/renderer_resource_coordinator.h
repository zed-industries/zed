// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_RESOURCE_COORDINATOR_RENDERER_RESOURCE_COORDINATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_RESOURCE_COORDINATOR_RENDERER_RESOURCE_COORDINATOR_H_

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// TODO(chrisha): Remove knowledge of ExecutionContext class from this code!
class ExecutionContext;
class Frame;
class HTMLFrameOwnerElement;
class ScriptState;

// This object is a process-wide singleton, and thread-safe.
class PLATFORM_EXPORT RendererResourceCoordinator {
 public:
  static void Set(RendererResourceCoordinator* instance);

  // This will always return a valid object. However, unless an explicit
  // implementation has been provided via Set, it will be a dummy
  // implementation.
  static RendererResourceCoordinator* Get();

  RendererResourceCoordinator() = default;
  RendererResourceCoordinator(const RendererResourceCoordinator&) = delete;
  RendererResourceCoordinator& operator=(const RendererResourceCoordinator&) =
      delete;
  virtual ~RendererResourceCoordinator() = default;

  // Notifies the browser that the main thread task load is low.
  // TODO(chrisha): Move this to a per-agent interface, and drive this off of
  // a signal from each agent scheduler when those exist.
  virtual void SetMainThreadTaskLoadIsLow(bool) = 0;

  // Used for tracking content javascript contexts (frames, workers, worklets,
  // etc). These functions are thread-safe.

  // Called when a |script_state| is created. Note that |execution_context| may
  // be nullptr if the |script_state| is not associated with an
  // |execution_context|.
  virtual void OnScriptStateCreated(ScriptState* script_state,
                                    ExecutionContext* execution_context) = 0;
  // Called when the |script_state| has been detached from the v8::Context
  // (and ExecutionContext, if applicable) it was associated with at creation.
  // At this point the associated v8::Context is considered "detached" until it
  // is garbage collected.
  virtual void OnScriptStateDetached(ScriptState* script_state) = 0;
  // Called when the |script_state| itself is garbage collected.
  virtual void OnScriptStateDestroyed(ScriptState* script_state) = 0;

  // Called when |frame| is about to be set as the ContentFrame of |owner|.
  virtual void OnBeforeContentFrameAttached(
      const Frame& frame,
      const HTMLFrameOwnerElement& owner) = 0;
  // Called when |frame| is about to be unset as the ContentFrame of |owner|.
  virtual void OnBeforeContentFrameDetached(
      const Frame& frame,
      const HTMLFrameOwnerElement& owner) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_INSTRUMENTATION_RESOURCE_COORDINATOR_RENDERER_RESOURCE_COORDINATOR_H_
