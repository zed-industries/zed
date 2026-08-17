/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_EXTENSION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_EXTENSION_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension_name.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class WebGLExtension;
class WebGLRenderingContextBase;

class WebGLExtensionScopedContext final {
  STACK_ALLOCATED();

 public:
  explicit WebGLExtensionScopedContext(WebGLExtension*);

  WebGLExtensionScopedContext(const WebGLExtensionScopedContext&) = delete;
  WebGLExtensionScopedContext& operator=(const WebGLExtensionScopedContext&) =
      delete;

  bool IsLost() const { return !context_; }
  WebGLRenderingContextBase* Context() const { return context_; }

 private:
  WebGLRenderingContextBase* context_;
};

class WebGLExtension : public ScriptWrappable {
 public:
  WebGLExtension(const WebGLExtension&) = delete;
  WebGLExtension& operator=(const WebGLExtension&) = delete;

  virtual WebGLExtensionName GetName() const = 0;

  // Lose this extension. Passing true = force loss. Some extensions
  // like WEBGL_lose_context are not normally lost when the context
  // is lost but must be lost when destroying their WebGLRenderingContextBase.
  virtual void Lose(bool) { context_ = nullptr; }

  bool IsLost() { return !context_; }

  void Trace(Visitor*) const override;

 protected:
  explicit WebGLExtension(WebGLRenderingContextBase*);

 private:
  friend WebGLExtensionScopedContext;

  WeakMember<WebGLRenderingContextBase> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_EXTENSION_H_
