/*
 * Copyright (c) 2013, Opera Software ASA. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Opera Software ASA nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_EVENT_HANDLERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_EVENT_HANDLERS_H_

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class WindowEventHandlers {
 public:
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(afterprint, kAfterprint)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(beforeprint, kBeforeprint)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(beforeunload, kBeforeunload)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(hashchange, kHashchange)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(languagechange, kLanguagechange)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(message, kMessage)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(move, kMove)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(messageerror, kMessageerror)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(offline, kOffline)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(online, kOnline)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(pagehide, kPagehide)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(pageshow, kPageshow)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(popstate, kPopstate)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(rejectionhandled, kRejectionhandled)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(storage, kStorage)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(timezonechange, kTimezonechange)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(unhandledrejection,
                                         kUnhandledrejection)
  DEFINE_WINDOW_ATTRIBUTE_EVENT_LISTENER(unload, kUnload)

 protected:
  virtual Document& GetDocumentForWindowEventHandler() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_WINDOW_EVENT_HANDLERS_H_
