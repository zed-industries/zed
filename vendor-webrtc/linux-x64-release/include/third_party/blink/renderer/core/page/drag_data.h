/*
 * Copyright (C) 2007 Apple Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_DATA_H_

#include "third_party/blink/public/common/page/drag_operation.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/drag_actions.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/gfx/geometry/point_f.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class DataObject;
class DocumentFragment;
class LocalFrame;

class CORE_EXPORT DragData {
  STACK_ALLOCATED();

 public:
  enum FilenameConversionPolicy { kDoNotConvertFilenames, kConvertFilenames };

  // clientPosition is taken to be the position of the drag event within the
  // target window, with (0,0) at the top left.
  // TODO(crbug.com/331733541): Update `DragData` to store the viewport
  // coordinates and scale.
  DragData(DataObject*,
           const gfx::PointF& client_position,
           const gfx::PointF& global_position,
           DragOperationsMask,
           bool force_default_action);
  const gfx::PointF& ClientPosition() const { return client_position_; }
  const gfx::PointF& GlobalPosition() const { return global_position_; }
  DataObject* PlatformData() const { return platform_drag_data_; }
  DragOperationsMask DraggingSourceOperationMask() const {
    return dragging_source_operation_mask_;
  }
  bool ContainsURL(
      FilenameConversionPolicy filename_policy = kConvertFilenames) const;
  bool ContainsPlainText() const;
  bool ContainsCompatibleContent() const;
  WTF::String AsURL(
      FilenameConversionPolicy filename_policy = kConvertFilenames,
      WTF::String* title = nullptr) const;
  Vector<WTF::String> AsURLs(
      FilenameConversionPolicy filename_policy = kConvertFilenames) const;
  WTF::String AsPlainText() const;
  void AsFilePaths(Vector<WTF::String>&) const;
  unsigned NumberOfFiles() const;
  DocumentFragment* AsFragment(LocalFrame*) const;
  bool CanSmartReplace() const;
  bool ContainsFiles() const;
  bool ForceDefaultAction() const;
  int GetModifiers() const;

  WTF::String DroppedFileSystemId() const;

 private:
  const gfx::PointF client_position_;
  const gfx::PointF global_position_;
  DataObject* const platform_drag_data_;
  const DragOperationsMask dragging_source_operation_mask_;
  bool force_default_action_;

  bool ContainsHTML() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_DRAG_DATA_H_
