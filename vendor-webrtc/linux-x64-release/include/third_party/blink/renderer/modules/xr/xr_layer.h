// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LAYER_H_

#include <optional>

#include "gpu/command_buffer/common/mailbox_holder.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"

namespace blink {

class XRSession;
struct XRLayerSharedImages;

class XRLayer : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit XRLayer(XRSession*);
  ~XRLayer() override = default;

  XRSession* session() const { return session_.Get(); }

  virtual void OnFrameStart() = 0;
  virtual void OnFrameEnd() = 0;
  virtual void OnResize() = 0;

  // EventTarget overrides.
  ExecutionContext* GetExecutionContext() const override;
  const AtomicString& InterfaceName() const override;

  uint32_t layer_id() const { return layer_id_; }
  const XRLayerSharedImages& GetSharedImages() const;

  void Trace(Visitor*) const override;

 private:
  const Member<XRSession> session_;
  const uint32_t layer_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_LAYER_H_
