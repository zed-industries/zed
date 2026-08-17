/*
 * Copyright 2008, The Android Open Source Project
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_TOUCH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_TOUCH_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/point_f.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class LocalFrame;
class TouchInit;

class CORE_EXPORT Touch final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static Touch* Create(LocalFrame* frame,
                       EventTarget* target,
                       int identifier,
                       const gfx::PointF& screen_pos,
                       const gfx::PointF& page_pos,
                       const gfx::SizeF& radius,
                       float rotation_angle,
                       float force) {
    return MakeGarbageCollected<Touch>(frame, target, identifier, screen_pos,
                                       page_pos, radius, rotation_angle, force);
  }

  static Touch* Create(const Document& document, const TouchInit* initializer) {
    return MakeGarbageCollected<Touch>(document.GetFrame(), initializer);
  }

  Touch(LocalFrame*,
        EventTarget*,
        int identifier,
        const gfx::PointF& screen_pos,
        const gfx::PointF& page_pos,
        const gfx::SizeF& radius,
        float rotation_angle,
        float force);

  Touch(EventTarget*,
        int identifier,
        const gfx::PointF& client_pos,
        const gfx::PointF& screen_pos,
        const gfx::PointF& page_pos,
        const gfx::SizeF& radius,
        float rotation_angle,
        float force,
        PhysicalOffset absolute_location);

  Touch(LocalFrame*, const TouchInit*);

  // DOM Touch implementation
  EventTarget* target() const { return target_.Get(); }
  int identifier() const { return identifier_; }
  double clientX() const { return client_pos_.x(); }
  double clientY() const { return client_pos_.y(); }
  double screenX() const { return screen_pos_.x(); }
  double screenY() const { return screen_pos_.y(); }
  double pageX() const { return page_pos_.x(); }
  double pageY() const { return page_pos_.y(); }
  float radiusX() const { return radius_.width(); }
  float radiusY() const { return radius_.height(); }
  float rotationAngle() const { return rotation_angle_; }
  float force() const { return force_; }

  // Blink-internal methods
  const PhysicalOffset& AbsoluteLocation() const { return absolute_location_; }
  const gfx::PointF& ScreenLocation() const { return screen_pos_; }
  Touch* CloneWithNewTarget(EventTarget*) const;

  void Trace(Visitor*) const override;

 private:
  Member<EventTarget> target_;
  int identifier_;
  // Position relative to the viewport in CSS px.
  gfx::PointF client_pos_;
  // Position relative to the screen in DIPs.
  gfx::PointF screen_pos_;
  // Position relative to the page in CSS px.
  gfx::PointF page_pos_;
  // Radius in CSS px.
  gfx::SizeF radius_;
  float rotation_angle_;
  float force_;
  // FIXME(rbyers): Shouldn't we be able to migrate callers to relying on
  // screenPos, pagePos or clientPos? absoluteLocation appears to be the same as
  // pagePos but without browser scale applied.
  PhysicalOffset absolute_location_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_TOUCH_H_
