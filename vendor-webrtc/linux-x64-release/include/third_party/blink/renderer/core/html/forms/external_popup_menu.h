/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EXTERNAL_POPUP_MENU_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EXTERNAL_POPUP_MENU_H_

#include <memory>
#include "cc/paint/paint_canvas.h"
#include "third_party/blink/public/mojom/choosers/popup_menu.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/popup_menu.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class HTMLSelectElement;
class LocalFrame;
class WebMouseEvent;

// The ExternalPopupMenu is a PopupMenu implementation for macOS and Android.
// It uses a OS-native menu implementation.
class CORE_EXPORT ExternalPopupMenu final
    : public PopupMenu,
      public mojom::blink::PopupMenuClient {
 public:
  ExternalPopupMenu(LocalFrame&, HTMLSelectElement&);
  ~ExternalPopupMenu() override;

  // Fills |info| with the popup menu information contained in the
  // PopupMenuClient associated with this ExternalPopupMenu.
  // FIXME: public only for test access. Need to revert once gtest
  // helpers from chromium are available for blink.
  static void GetPopupMenuInfo(HTMLSelectElement&,
                               double* font_size,
                               int32_t* selected_item,
                               Vector<mojom::blink::MenuItemPtr>* menu_items,
                               bool* right_aligned,
                               bool* allow_multiple_selection);
  static int ToPopupMenuItemIndex(int index, HTMLSelectElement&);
  static int ToExternalPopupMenuItemIndex(int index, HTMLSelectElement&);

  void Trace(Visitor*) const override;

 private:
  // PopupMenu methods:
  void Show(ShowEventType type) override;
  void Hide() override;
  void UpdateFromElement(UpdateReason) override;
  void DisconnectClient() override;

  // mojom::blink::PopupMenuClient methods:
  void DidAcceptIndices(const Vector<int32_t>& indices) override;
  void DidCancel() override;

  void Reset();
  bool ShowInternal();
  void DispatchEvent(TimerBase*);
  void Update();

  Member<HTMLSelectElement> owner_element_;
  Member<LocalFrame> local_frame_;
  std::unique_ptr<WebMouseEvent> synthetic_event_;
  HeapTaskRunnerTimer<ExternalPopupMenu> dispatch_event_timer_;
  // The actual implementor of the show menu.
  HeapMojoReceiver<mojom::blink::PopupMenuClient, ExternalPopupMenu> receiver_;
  bool needs_update_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_EXTERNAL_POPUP_MENU_H_
