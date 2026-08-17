/*
 * Copyright (C) 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_TEXTURE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_TEXTURE_H_

#include "base/time/time.h"
#include "media/base/video_frame.h"
#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/renderer/modules/webgl/webgl_object.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class WebGLTexture : public WebGLObject {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit WebGLTexture(WebGLRenderingContextBase*);

  ~WebGLTexture() override;

  void SetTarget(GLenum);

  GLenum GetTarget() const { return target_; }

  bool HasEverBeenBound() const { return Object() && target_; }

  static GLint ComputeLevelCount(GLsizei width, GLsizei height, GLsizei depth);

  // See https://www.w3.org/TR/webxrlayers-1/#opaque-texture.
  virtual bool IsOpaqueTexture() const { return false; }

 protected:
  // Constructor for WebGLUnownedTexture.
  explicit WebGLTexture(WebGLRenderingContextBase* ctx,
                        GLuint texture,
                        GLenum target);

 private:
  void DeleteObjectImpl(gpu::gles2::GLES2Interface*) override;

  bool IsTexture() const override { return true; }

  int MapTargetToIndex(GLenum) const;

  GLenum target_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_TEXTURE_H_
