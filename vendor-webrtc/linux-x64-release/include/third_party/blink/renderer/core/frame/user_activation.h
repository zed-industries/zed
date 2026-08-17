// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_USER_ACTIVATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_USER_ACTIVATION_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class LocalDOMWindow;

class UserActivation final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Creates an instance that is a snapshot of the current state of this
  // LocalDOMWindow.
  static UserActivation* CreateSnapshot(LocalDOMWindow* window);

  explicit UserActivation(LocalDOMWindow* window);
  UserActivation(bool has_been_active, bool is_active);
  ~UserActivation() override;

  void Trace(Visitor*) const override;

  bool hasBeenActive() const;
  bool isActive() const;

 private:
  Member<LocalDOMWindow> window_;
  bool has_been_active_ = false;
  bool is_active_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_USER_ACTIVATION_H_
