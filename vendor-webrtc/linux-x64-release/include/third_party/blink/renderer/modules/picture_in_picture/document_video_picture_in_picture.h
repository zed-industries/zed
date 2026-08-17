// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_DOCUMENT_VIDEO_PICTURE_IN_PICTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_DOCUMENT_VIDEO_PICTURE_IN_PICTURE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;
class ExceptionState;
class ScriptState;

class DocumentVideoPictureInPicture {
  STATIC_ONLY(DocumentVideoPictureInPicture);

 public:
  static bool pictureInPictureEnabled(Document&);

  static ScriptPromise<IDLUndefined> exitPictureInPicture(ScriptState*,
                                                          Document&,
                                                          ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PICTURE_IN_PICTURE_DOCUMENT_VIDEO_PICTURE_IN_PICTURE_H_
