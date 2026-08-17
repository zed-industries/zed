// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_ANIMATED_ARROW_CONTAINER_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_ANIMATED_ARROW_CONTAINER_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_animation_event_listener.h"
#include "third_party/blink/renderer/modules/media_controls/elements/media_control_div_element.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace WTF {
class AtomicString;
}

namespace blink {

class MediaControlsImpl;

class MODULES_EXPORT MediaControlAnimatedArrowContainerElement final
    : public MediaControlDivElement {
 public:
  enum class ArrowDirection { kLeft, kRight };

  explicit MediaControlAnimatedArrowContainerElement(MediaControlsImpl&);

  void ShowArrowAnimation(ArrowDirection);

  void Trace(Visitor*) const override;

 private:
  friend class MediaControlAnimatedArrowContainerElementTest;

  // This class is responible for displaying the arrow animation when a jump is
  // triggered by the user.
  class MODULES_EXPORT AnimatedArrow final
      : public HTMLDivElement,
        public MediaControlAnimationEventListener::Observer {
   public:
    AnimatedArrow(const AtomicString& id, Document& document);

    // MediaControlAnimationEventListener::Observer overrides
    void OnAnimationIteration() override;
    void OnAnimationEnd() override {}
    Element& WatchedAnimationElement() const override;

    // Shows the animated arrows for a single animation iteration. If the
    // arrows are already shown it will show them for another animation
    // iteration.
    void Show();

    void Trace(Visitor*) const override;

   private:
    void HideInternal();
    void ShowInternal();

    int counter_ = 0;
    bool hidden_ = true;

    Member<Element> last_arrow_;
    Member<Element> svg_container_;
    Member<MediaControlAnimationEventListener> event_listener_;
  };

  Member<AnimatedArrow> left_jump_arrow_;
  Member<AnimatedArrow> right_jump_arrow_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_ELEMENTS_MEDIA_CONTROL_ANIMATED_ARROW_CONTAINER_ELEMENT_H_
