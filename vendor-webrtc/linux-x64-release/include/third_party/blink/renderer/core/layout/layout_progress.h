/*
 * Copyright (C) 2009 Nokia Corporation and/or its subsidiary(-ies).
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_PROGRESS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_PROGRESS_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_block_flow.h"

namespace blink {

class HTMLProgressElement;

class CORE_EXPORT LayoutProgress : public LayoutBlockFlow {
 public:
  explicit LayoutProgress(HTMLProgressElement&);
  ~LayoutProgress() override;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(animation_timer_);
    LayoutBlockFlow::Trace(visitor);
  }

  double GetPosition() const {
    NOT_DESTROYED();
    return position_;
  }
  double AnimationProgress() const;

  bool IsDeterminate() const;
  void UpdateFromElement() override;

  HTMLProgressElement* ProgressElement() const;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutNGProgress";
  }

 protected:
  void WillBeDestroyed() override;
  bool IsProgress() const final {
    NOT_DESTROYED();
    return true;
  }

  bool IsAnimating() const;
  bool IsAnimationTimerActive() const;

 private:
  void AnimationTimerFired(TimerBase*);
  void UpdateAnimationState();

  double position_;
  base::TimeTicks animation_start_time_;
  bool animating_;
  HeapTaskRunnerTimer<LayoutProgress> animation_timer_;

  friend class LayoutProgressTest;
};

template <>
struct DowncastTraits<LayoutProgress> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsProgress();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_PROGRESS_H_
