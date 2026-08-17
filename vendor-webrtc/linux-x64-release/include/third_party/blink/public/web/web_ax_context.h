// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_CONTEXT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_CONTEXT_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/web/web_ax_object.h"
#include "third_party/blink/public/web/web_document.h"
#include "ui/accessibility/ax_error_types.h"
#include "ui/accessibility/ax_event.h"

namespace ui {
class AXMode;
struct AXTreeUpdate;
}  // namespace ui

namespace blink {

class AXContext;

// An instance of this class, while kept alive, enables accessibility
// support for the given document.
class BLINK_EXPORT WebAXContext {
 public:
  explicit WebAXContext(WebDocument document, const ui::AXMode& mode);
  ~WebAXContext();

  bool HasActiveDocument() const;
  bool HasAXObjectCache() const;

  const ui::AXMode& GetAXMode() const;

  void SetAXMode(const ui::AXMode&) const;

  // Recompute the entire tree and reserialize it.
  // This method is useful when something that potentially affects most of the
  // page occurs, such as an inertness change or a fullscreen toggle.
  // This keeps the existing nodes, but recomputes all of their properties and
  // reserializes everything.
  // Compared with ResetSerializer() and MarkAXObjectDirtyWithDetails() with
  // subtree = true, this does more work, because it recomputes the entire tree
  // structure and properties of each node.
  void MarkDocumentDirty();

  void SetSerializationResetToken(uint32_t reset_token) const;

  // Compared with MarkDocumentDirty(), this does less work, because it assumes
  // the AXObjectCache's tree of objects and properties is correct, but needs to
  // be reserialized.
  void ResetSerializer();

  bool SerializeEntireTree(
      size_t max_node_count,
      base::TimeDelta timeout,
      ui::AXTreeUpdate* response,
      std::set<ui::AXSerializationErrorFlag>* out_error = nullptr);

  // Returns a vector of the images found in |updates|.
  void GetImagesToAnnotate(ui::AXTreeUpdate& updates,
                           std::vector<ui::AXNodeData*>&);

  // Ensure that accessibility is clean and up-to-date for both the main and
  // popup document. Ensures layout is clean as well.
  void UpdateAXForAllDocuments();

  // If the document is loaded, fire a load complete event.
  void FireLoadCompleteIfLoaded();

  // Ensures that a serialization of all pending events and dirty objects is
  // sent to the client as soon as possible at the next lifecycle update.
  // Technically, ensures that a call to
  // RenderAccessibilityImpl::SendAccessibilitySerialization() will occur as
  // soon as possible.
  void ScheduleImmediateSerialization();

  // Add an event to the queue of events to be processed as well as mark the
  // AXObject dirty. If immediate_serialization is set, it schedules a
  // serialization to be done at the next lifecycle update without delays.
  void AddEventToSerializationQueue(const ui::AXEvent& event,
                                    bool immediate_serialization);

  // Inform AXObjectCacheImpl that the last serialization was received by the
  // browser successfully.
  void OnSerializationReceived();

  // Inform AXObjectCacheImpl that a serialization was cancelled. It's only
  // required for legacy, non-lifecycle mode. Check IsSerializationInFlight
  // details.
  // TODO(accessibility): This method can eventually be moved to AXObjectCache
  // when legacy mode is removed.
  void OnSerializationCancelled();

  // Inform AXObjectCacheImpl that a serialization just started to be sent to
  // the browser. Check IsSerializationInFlight details.
  // TODO(accessibility): This method can eventually be moved to AXObjectCache
  // when legacy mode is removed.
  void OnSerializationStartSend();

  // Determine if a serialization is in progress or not. Sometimes when
  // serializing the events, more events are generated and a new lifecycle
  // update occurs. Without this variable, we could end up in an infinite loop
  // of sending updates so we keep track when an update is in progress and avoid
  // starting any new updates while it's true. It becomes false again when the
  // update reaches the browser via a call to OnSerializationReceived().
  // TODO(accessibility): Again, this method is here only for legacy mode and
  // would be moved to AXObjectCache later on.
  bool IsSerializationInFlight() const;

 private:
  std::unique_ptr<AXContext> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_CONTEXT_H_
