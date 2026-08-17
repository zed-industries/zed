/*
 * Copyright (C) 2001 Peter Kelly (pmk@post.com)
 * Copyright (C) 2001 Tobias Anton (anton@stud.fbi.fh-darmstadt.de)
 * Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2008 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_TRANSFER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_TRANSFER_H_

#include <memory>

#include "third_party/blink/public/common/page/drag_operation.h"
#include "third_party/blink/renderer/core/clipboard/data_object.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/keywords.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_content.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "ui/base/dragdrop/mojom/drag_drop_types.mojom-blink-forward.h"
#include "ui/gfx/geometry/point.h"

namespace blink {

class DataTransferItemList;
class DragImage;
class Element;
class FileList;
class FrameSelection;
class LocalFrame;
class Node;
class PaintRecordBuilder;
class PropertyTreeState;

enum class DataTransferAccessPolicy;

// Used for drag and drop and copy/paste.
// Drag and Drop:
// https://html.spec.whatwg.org/multipage/dnd.html
// Clipboard API (copy/paste):
// https://w3c.github.io/clipboard-apis/
class CORE_EXPORT DataTransfer final : public ScriptWrappable,
                                       public DataObject::Observer {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Whether this transfer is serving a drag-drop, copy-paste, spellcheck,
  // auto-correct or similar request.
  enum DataTransferType {
    kCopyAndPaste,
    kDragAndDrop,
    kInsertReplacementText,
  };

  static DataTransfer* Create();
  static DataTransfer* Create(DataTransferType,
                              DataTransferAccessPolicy,
                              DataObject*);

  DataTransfer(DataTransferType, DataTransferAccessPolicy, DataObject*);
  ~DataTransfer() override;

  bool IsForCopyAndPaste() const { return transfer_type_ == kCopyAndPaste; }
  bool IsForDragAndDrop() const { return transfer_type_ == kDragAndDrop; }

  AtomicString dropEffect() const {
    return DropEffectIsInitialized() ? drop_effect_ : keywords::kNone;
  }
  void setDropEffect(const AtomicString&);
  bool DropEffectIsInitialized() const { return !drop_effect_.IsNull(); }
  AtomicString effectAllowed() const { return effect_allowed_; }
  void setEffectAllowed(const AtomicString&);

  void clearData(const String& type = String());
  String getData(const String& type) const;
  void setData(const String& type, const String& data);

  // Used by the bindings code to determine whether to call types() again.
  bool hasDataStoreItemListChanged() const;

  Vector<String> types();
  FileList* files() const;

  // Returns drag location (offset) within the dragged object.  This is (0,0)
  // unless set by setDragImage().
  gfx::Point DragLocation() const { return drag_loc_; }

  void setDragImage(Element*, int x, int y);
  void ClearDragImage();
  void SetDragImageResource(ImageResourceContent*, const gfx::Point&);
  void SetDragImageElement(Node*, const gfx::Point&);

  std::unique_ptr<DragImage> CreateDragImage(gfx::Point& drag_location,
                                             float device_scale_factor,
                                             LocalFrame*) const;
  void DeclareAndWriteDragImage(Element*,
                                const KURL& link_url,
                                const KURL& image_url,
                                const String& title);
  void WriteURL(Node*, const KURL&, const String&);
  void WriteSelection(const FrameSelection&);

  void SetAccessPolicy(DataTransferAccessPolicy);
  bool CanReadTypes() const;
  bool CanReadData() const;
  bool CanWriteData() const;
  // Note that the spec doesn't actually allow drag image modification outside
  // the dragstart event. This capability is maintained for backwards
  // compatiblity for ports that have supported this in the past. On many ports,
  // attempting to set a drag image outside the dragstart operation is a no-op
  // anyway.
  bool CanSetDragImage() const;

  DragOperationsMask SourceOperation() const;
  ui::mojom::blink::DragOperation DestinationOperation() const;
  void SetSourceOperation(DragOperationsMask);
  void SetDestinationOperation(ui::mojom::blink::DragOperation);

  DataTransferItemList* items();

  DataObject* GetDataObject() const;

  // Clip to the visible area of the visual viewport.
  static gfx::RectF ClipByVisualViewport(const gfx::RectF& rect_in_document,
                                         const LocalFrame&);

  // |layout_size| is the size of the image in layout pixels.
  // |paint_offset| is the offset from the origin of the dragged object of the
  // PaintRecordBuilder.
  static std::unique_ptr<DragImage> CreateDragImageForFrame(
      LocalFrame&,
      float,
      const gfx::SizeF& layout_size,
      const gfx::Vector2dF& paint_offset,
      PaintRecordBuilder&,
      const PropertyTreeState&);
  static std::unique_ptr<DragImage> NodeImage(LocalFrame&, Node&);

  void Trace(Visitor*) const override;

 private:
  void setDragImage(ImageResourceContent*, Node*, const gfx::Point&);

  bool HasFileOfType(const String&) const;
  bool HasStringOfType(const String&) const;

  // DataObject::Observer override.
  void OnItemListChanged() override;

  // Instead of using this member directly, prefer to use the can*() methods
  // above.
  DataTransferAccessPolicy policy_;
  AtomicString drop_effect_;
  AtomicString effect_allowed_;
  DataTransferType transfer_type_;
  Member<DataObject> data_object_;

  bool data_store_item_list_changed_;

  gfx::Point drag_loc_;
  Member<ImageResourceContent> drag_image_;
  Member<Node> drag_image_element_;
  Member<FileList> files_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CLIPBOARD_DATA_TRANSFER_H_
