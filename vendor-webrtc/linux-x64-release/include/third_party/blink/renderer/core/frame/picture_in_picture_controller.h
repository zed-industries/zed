// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PICTURE_IN_PICTURE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PICTURE_IN_PICTURE_CONTROLLER_H_

#include "services/media_session/public/mojom/media_session.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/buildflags.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class Element;
class HTMLVideoElement;
class LocalDOMWindow;
class PictureInPictureWindow;
class TreeScope;

// PictureInPictureController allows to know if Picture-in-Picture is allowed
// for a video element in Blink outside of modules/ module. It
// is an interface that the module will implement and add a provider for.
class CORE_EXPORT PictureInPictureController
    : public GarbageCollected<PictureInPictureController>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];

  PictureInPictureController(const PictureInPictureController&) = delete;
  PictureInPictureController& operator=(const PictureInPictureController&) =
      delete;
  virtual ~PictureInPictureController() = default;

  // Should be called before any other call to make sure a document is attached.
  static PictureInPictureController& From(Document&);

  // Returns whether the given element is currently in Picture-in-Picture. It
  // returns false if PictureInPictureController is not attached to a document.
  static bool IsElementInPictureInPicture(const Element*);

  // Returns whether the given element is currently in a document
  // Picture-in-Picture window.
  static bool IsInDocumentPictureInPicture(const Element* element);

  // Returns the document picture-in-picture window opened by the Document. It
  // returns null if there is no open document picture-in-picture window for the
  // Document or if PictureInPictureController is not attached to the Document.
  static LocalDOMWindow* GetDocumentPictureInPictureWindow(const Document&);

  // If the provided document is attached to a document picture-in-picture
  // window, then this returns the LocalDOMWindow that owns and originally
  // opened the document picture-in-picture window. Returns null if the provided
  // document is not attached to a document picture-in-picture window.
  static LocalDOMWindow* GetDocumentPictureInPictureOwner(const Document&);

  // List of Picture-in-Picture support statuses. If status is kEnabled,
  // Picture-in-Picture is enabled for a document or element, otherwise it is
  // not supported.
  enum class Status {
    kEnabled,
    kFrameDetached,
    kMetadataNotLoaded,
    kVideoTrackNotAvailable,
    kDisabledBySystem,
    kDisabledByPermissionsPolicy,
    kDisabledByAttribute,
    kAutoPipAndroid,

    // An active document that's already a picture-in-picture document may not
    // re-enter picture-in-picture mode.
    kDocumentPip,
  };

  // Enter Picture-in-Picture for a video element and resolve promise if any.
  virtual void EnterPictureInPicture(
      HTMLVideoElement*,
      ScriptPromiseResolver<PictureInPictureWindow>*) = 0;

  // Exit Picture-in-Picture for a video element and resolve promise if any.
  virtual void ExitPictureInPicture(HTMLVideoElement*,
                                    ScriptPromiseResolver<IDLUndefined>*) = 0;

  // Returns whether a given video element in a document associated with the
  // controller is allowed to request Picture-in-Picture.
  virtual Status IsElementAllowed(const HTMLVideoElement&,
                                  bool report_failure = false) const = 0;

  // Should be called when an element has exited Picture-in-Picture.
  virtual void OnExitedPictureInPicture(
      ScriptPromiseResolver<IDLUndefined>*) = 0;

  // Notifies that one of the states used by Picture-in-Picture has changed.
  virtual void OnPictureInPictureStateChange() = 0;

  // Notifies that the media position has changed for the player in
  // Picture-in-Picture.
  virtual void OnMediaPositionStateChanged(
      const media_session::mojom::blink::MediaPositionPtr& media_position) = 0;

  // Returns element currently in Picture-in-Picture if any. Null otherwise.
  virtual Element* PictureInPictureElement() const = 0;
  virtual Element* PictureInPictureElement(TreeScope&) const = 0;

  // Returns whether system allows Picture-in-Picture feature or not for
  // the associated document.
  virtual bool PictureInPictureEnabled() const = 0;

  void Trace(Visitor*) const override;

 protected:
  explicit PictureInPictureController(Document&);

  // Returns whether the given element is currently in Picture-in-Picture.
  // It is protected so that clients use the static method
  // IsElementInPictureInPicture() that avoids creating the controller.
  virtual bool IsPictureInPictureElement(const Element*) const = 0;

#if !BUILDFLAG(TARGET_OS_IS_ANDROID)
  // Returns the document picture-in-picture window opened by the Document. It
  // returns null if there is no open document picture-in-picture window for the
  // Document or if PictureInPictureController is not attached to the Document.
  // It is protected so that clients use the static method
  // GetDocumentPictureInPictureWindow() that avoids creating the controller.
  virtual LocalDOMWindow* GetDocumentPictureInPictureWindow() const = 0;

  // If this is attached to a document picture-in-picture window, then this
  // returns the LocalDOMWindow that owns and originally opened the document
  // picture-in-picture window. Returns null if the this is not attached to a
  // document picture-in-picture window.
  virtual LocalDOMWindow* GetDocumentPictureInPictureOwner() const = 0;
#endif  // !BUILDFLAG(TARGET_OS_IS_ANDROID)
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PICTURE_IN_PICTURE_CONTROLLER_H_
