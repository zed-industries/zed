/*
 * Copyright (C) 2007, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/page/drag_actions.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "ui/base/dragdrop/mojom/drag_drop_types.mojom-blink.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/rect.h"

namespace gfx {
class RectF;
}

namespace blink {

class DataTransfer;
class Document;
class DragData;
class DragImage;
class DragState;
class LocalFrame;
class FrameSelection;
class HTMLInputElement;
class Node;
class Page;
class WebMouseEvent;

class CORE_EXPORT DragController final
    : public GarbageCollected<DragController>,
      public ExecutionContextLifecycleObserver {
 public:
  explicit DragController(Page*);
  DragController(const DragController&) = delete;
  DragController& operator=(const DragController&) = delete;

  // Holds the drag operation and whether the document is handling it.  Also see
  // DragTargetDragEnter() in widget.mojom for further details.
  struct Operation {
    // The current drag operation as negotiated by the source and destination.
    // When not equal to DragOperationNone, the drag data can be dropped onto
    // the current drop target in this WebView (the drop target can accept the
    // drop).
    ui::mojom::blink::DragOperation operation =
        ui::mojom::blink::DragOperation::kNone;

    // True if the document intends to handle the drag.  This means the drag
    // controller will pass the data to the document, but the document might
    // still decide not to handle it by not calling preventDefault().
    bool document_is_handling_drag = false;
  };

  Operation DragEnteredOrUpdated(DragData*, LocalFrame& local_root);
  void DragExited(DragData*, LocalFrame& local_root);
  void PerformDrag(DragData*, LocalFrame& local_root);

  enum SelectionDragPolicy {
    kImmediateSelectionDragResolution,
    kDelayedSelectionDragResolution,
  };
  Node* DraggableNode(const LocalFrame*,
                      Node*,
                      const gfx::Point&,
                      SelectionDragPolicy,
                      DragSourceAction&) const;
  void DragEnded();

  bool PopulateDragDataTransfer(LocalFrame* src,
                                const DragState&,
                                const gfx::Point& drag_origin);

  // The parameter `drag_event` is the event that triggered the drag operation,
  // and `drag_initiation_location` is the where the drag originated.  The
  // event's location does NOT match the initiation location for a mouse-drag:
  // the drag is triggered by a mouse-move event but the initiation location is
  // that of a mouse-down event.
  bool StartDrag(LocalFrame*,
                 const DragState&,
                 const WebMouseEvent& drag_event,
                 const gfx::Point& drag_initiation_location);

  DragState& GetDragState();

  static std::unique_ptr<DragImage> DragImageForSelection(LocalFrame&, float);

  // Return the selection bounds in absolute coordinates for the frame, clipped
  // to the visual viewport.
  static gfx::RectF ClippedSelection(const LocalFrame&);

  // ExecutionContextLifecycleObserver.
  void ContextDestroyed() final;

  void Trace(Visitor*) const final;

 private:
  DispatchEventResult DispatchTextInputEventFor(LocalFrame*, DragData*);
  bool CanProcessDrag(DragData*, LocalFrame& local_root);
  bool ConcludeEditDrag(DragData*);
  ui::mojom::blink::DragOperation OperationForLoad(DragData*,
                                                   LocalFrame& local_root);
  bool TryDocumentDrag(DragData*,
                       DragDestinationAction,
                       ui::mojom::blink::DragOperation&,
                       LocalFrame& local_root);
  bool TryDHTMLDrag(DragData*,
                    ui::mojom::blink::DragOperation&,
                    LocalFrame& local_root);
  ui::mojom::blink::DragOperation GetDragOperation(DragData*);
  // Clear the selection from the document this drag is exiting.
  void ClearDragCaret();
  bool DragIsMove(FrameSelection&, DragData*);
  bool IsCopyKeyDown(DragData*);

  void MouseMovedIntoDocument(Document*);

  void DoSystemDrag(DragImage*,
                    const gfx::Rect& drag_obj_rect,
                    const gfx::Point& drag_initiation_location,
                    DataTransfer*,
                    LocalFrame*);

  Member<Page> page_;

  // The document the mouse was last dragged over.
  Member<Document> document_under_mouse_;
  // The window (if any) that initiated the drag.
  Member<LocalDOMWindow> drag_initiator_;

  Member<DragState> drag_state_;

  Member<HTMLInputElement> file_input_element_under_mouse_;
  bool document_is_handling_drag_;

  DragDestinationAction drag_destination_action_;
  bool did_initiate_drag_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_CONTROLLER_H_
