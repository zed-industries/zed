// Copyright (C) 2013 Google Inc. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_STATE_SAVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_STATE_SAVER_H_

#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class PLATFORM_EXPORT GraphicsContextStateSaver final {
  STACK_ALLOCATED();

 public:
  GraphicsContextStateSaver(GraphicsContext& context,
                            bool save_and_restore = true)
      : context_(context), save_and_restore_(save_and_restore) {
    if (save_and_restore_) {
      context_.Save();
    }
  }

  GraphicsContextStateSaver(const GraphicsContextStateSaver&) = delete;
  GraphicsContextStateSaver& operator=(const GraphicsContextStateSaver&) =
      delete;

  ~GraphicsContextStateSaver() {
    if (save_and_restore_) {
      context_.Restore();
    }
  }

  void Save() {
    DCHECK(!save_and_restore_);
    context_.Save();
    save_and_restore_ = true;
  }

  void SaveIfNeeded() {
    if (Saved())
      return;
    Save();
  }

  void Restore() {
    DCHECK(save_and_restore_);
    context_.Restore();
    save_and_restore_ = false;
  }

  GraphicsContext& Context() const { return context_; }
  bool Saved() const { return save_and_restore_; }

 private:
  GraphicsContext& context_;
  bool save_and_restore_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GRAPHICS_CONTEXT_STATE_SAVER_H_
