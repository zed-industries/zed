// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_CONTROLLER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_CONTROLLER_IMPL_H_

#include "build/build_config.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/picture_in_picture/picture_in_picture.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/frame/picture_in_picture_controller.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/picture_in_picture/picture_in_picture_window.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

#if !BUILDFLAG(TARGET_OS_IS_ANDROID)
class DocumentPictureInPictureOptions;
#endif  // !BUILDFLAG(TARGET_OS_IS_ANDROID)
class HTMLVideoElement;
class PictureInPictureWindow;
class ScriptState;
class TreeScope;

// The PictureInPictureControllerImpl is keeping the state and implementing the
// logic around the Picture-in-Picture feature. It is meant to be used as well
// by the Picture-in-Picture Web API and internally (eg. media controls). All
// consumers inside Blink modules/ should use this class to access
// Picture-in-Picture. In core/, they should use PictureInPictureController.
// PictureInPictureControllerImpl instance is associated to a Document. It is
// supplement and therefore can be lazy-initiated. Callers should consider
// whether they want to instantiate an object when they make a call.
class MODULES_EXPORT PictureInPictureControllerImpl
    : public PictureInPictureController,
      public ExecutionContextClient,
      public blink::mojom::blink::PictureInPictureSessionObserver {
 public:
  explicit PictureInPictureControllerImpl(Document&);

  PictureInPictureControllerImpl(const PictureInPictureControllerImpl&) =
      delete;
  PictureInPictureControllerImpl& operator=(
      const PictureInPictureControllerImpl&) = delete;

  ~PictureInPictureControllerImpl() override = default;

  // Gets, or creates, PictureInPictureControllerImpl supplement on Document.
  // Should be called before any other call to make sure a document is attached.
  static PictureInPictureControllerImpl& From(Document&);

  // Returns whether the document associated with the controller is allowed to
  // request Picture-in-Picture.
  Status IsDocumentAllowed(bool report_failure) const;

  // Returns the Picture-in-Picture window if there is any. This is for
  // video-only PiP.
  PictureInPictureWindow* pictureInPictureWindow() const;

#if !BUILDFLAG(TARGET_OS_IS_ANDROID)
  // Returns the Document Picture-in-Picture window if there is any.
  LocalDOMWindow* documentPictureInPictureWindow() const;

  // Creates a picture-in-picture window that can contain arbitrary HTML.
  void CreateDocumentPictureInPictureWindow(ScriptState*,
                                            LocalDOMWindow&,
                                            DocumentPictureInPictureOptions*,
                                            ScriptPromiseResolver<DOMWindow>*);
#endif  // !BUILDFLAG(TARGET_OS_IS_ANDROID)

  // Implementation of PictureInPictureController.
  void EnterPictureInPicture(
      HTMLVideoElement*,
      ScriptPromiseResolver<PictureInPictureWindow>*) override;
  void ExitPictureInPicture(HTMLVideoElement*,
                            ScriptPromiseResolver<IDLUndefined>*) override;
  bool IsPictureInPictureElement(const Element*) const override;
  void OnPictureInPictureStateChange() override;
  void OnMediaPositionStateChanged(
      const media_session::mojom::blink::MediaPositionPtr& media_position)
      override;
  Element* PictureInPictureElement() const override;
  Element* PictureInPictureElement(TreeScope&) const override;
  bool PictureInPictureEnabled() const override;
  Status IsElementAllowed(const HTMLVideoElement&,
                          bool report_failure) const override;
#if !BUILDFLAG(TARGET_OS_IS_ANDROID)
  LocalDOMWindow* GetDocumentPictureInPictureWindow() const override;
  LocalDOMWindow* GetDocumentPictureInPictureOwner() const override;

  void SetDocumentPictureInPictureOwner(LocalDOMWindow* owner);
#endif  // !BUILDFLAG(TARGET_OS_IS_ANDROID)

  // Implementation of PictureInPictureSessionObserver.
  void OnWindowSizeChanged(const gfx::Size&) override;
  void OnStopped() override;

  void Trace(Visitor*) const override;

  bool IsSessionObserverReceiverBoundForTesting() {
    return session_observer_receiver_.is_bound();
  }

 private:
  void OnEnteredPictureInPicture(
      HTMLVideoElement*,
      ScriptPromiseResolver<PictureInPictureWindow>*,
      mojo::PendingRemote<mojom::blink::PictureInPictureSession>,
      const gfx::Size&);
  void OnExitedPictureInPicture(ScriptPromiseResolver<IDLUndefined>*) override;

  // Notify viz, so we don't get throttled if the CompositorFrames generated by
  // this frame's LayerTree aren't being consumed, such as happens when the
  // tab is hidden / window minimized / etc.  If we do get throttled, then any
  // picture-in-picture window that depends on a timely update of the main page
  // will become janky.  For video-only cases, this isn't a problem; the picture
  // in picture window directly consumes the CompositorFrames from
  // VideoFrameSubmitter that contain each individual video frame.  Our
  // LayerTree won't generate any separate CompositorFrames at all, nor does it
  // really matter if it's throttled since it's not directly driving any
  // picture-in-picture content.  However, if the page is, for example, doing
  // some requestAnimationFrame- or requestVideoFrameCallback-based work to
  // generate the picture-in-picture video content, then (a) that might produce
  // as a side-effect CompositorFrames (e.g., if an on-screen canvas updates in
  // response to a new VideoFrame from the original video source) from the
  // LayerTree, resulting in (b) viz noticing that those frames are not being
  // consumed, since the page is not visible on-screen, resulting in (c) it
  // throttling requestAnimationFrame and friends, resulting in (d) the updates
  // to the pip'd video stream, which depend on those or similar callbacks, are
  // now much slower than the desired frame rate.
  //
  // https://crbug.com/1232173
  void SetMayThrottleIfUndrawnFrames(bool may_throttle);

  // Makes sure the `picture_in_picture_service_` is set. Returns whether it was
  // initialized successfully.
  bool EnsureService();

#if !BUILDFLAG(TARGET_OS_IS_ANDROID)
  // Resolves a call to |CreateDocumentPictureInPictureWindow()|.
  void ResolveOpenDocumentPictureInPicture();

  // Observer to watch a Document Picture in Picture window, so that the opener
  // can find out when it is being destroyed.
  class DocumentPictureInPictureObserver final
      : public GarbageCollected<DocumentPictureInPictureObserver>,
        public ContextLifecycleObserver {
   public:
    // Notify `controller` when our context is destroyed.
    explicit DocumentPictureInPictureObserver(
        PictureInPictureControllerImpl* controller);
    ~DocumentPictureInPictureObserver() final;

    void Trace(Visitor* visitor) const final;

   protected:
    void ContextDestroyed() final;

    Member<PictureInPictureControllerImpl> controller_;
  };

  friend class DocumentPictureInPictureObserver;

  // Called by DocumentPictureInPictureObserver.
  void OnDocumentPictureInPictureContextDestroyed();
  void OnOwnedDocumentPictureInPictureWindowContextDestroyed();
  void OnDocumentPictureInPictureOwnerWindowContextDestroyed();

  // The Document Picture-in-Picture window, if any. It shouldn't be confused
  // with `picture_in_picture_session_`, which is for video-only PiP.
  Member<LocalDOMWindow> document_picture_in_picture_window_;

  // The window that opened this document picture-in-picture window. Only set on
  // PictureInPictureControllerImpls that are owned by a document
  // picture-in-picture window.
  Member<LocalDOMWindow> document_picture_in_picture_owner_;

  // Observes for destruction for either our owned
  // `document_picture_in_picture_window_` (if this controller's Document has
  // opened a document picture-in-picture window) or our owner
  // `document_picture_in_picture_owner_` (if this controller's Document is
  // attached to a document picture-in-picture window).
  Member<DocumentPictureInPictureObserver> document_pip_context_observer_;

  // Used to force |CreateDocumentPictureInPictureWindow()| to be asynchronous.
  TaskHandle open_document_pip_task_;

  // The |ScriptPromiseResolverBase| associated with the most recent call to
  // |CreateDocumentPictureInPictureWindow()| if it has not yet been resolved.
  Member<ScriptPromiseResolver<DOMWindow>> open_document_pip_resolver_;
#endif  // !BUILDFLAG(TARGET_OS_IS_ANDROID)

  // The Picture-in-Picture element for the associated document.
  Member<HTMLVideoElement> picture_in_picture_element_;

  // The Picture-in-Picture window for the associated document. This is for
  // video-only PiP.
  Member<PictureInPictureWindow> picture_in_picture_window_;

  // Mojo bindings for the session observer interface implemented by |this|.
  HeapMojoReceiver<mojom::blink::PictureInPictureSessionObserver,
                   PictureInPictureControllerImpl>
      session_observer_receiver_;

  // Picture-in-Picture service living in the browser process.
  HeapMojoRemote<mojom::blink::PictureInPictureService>
      picture_in_picture_service_;

  // Instance of the Picture-in-Picture session sent back by the service. This
  // is for video-only PiP.
  HeapMojoRemote<mojom::blink::PictureInPictureSession>
      picture_in_picture_session_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_PICTURE_IN_PICTURE_PICTURE_IN_PICTURE_CONTROLLER_IMPL_H_
