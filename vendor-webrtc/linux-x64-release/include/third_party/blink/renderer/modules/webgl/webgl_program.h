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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_PROGRAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_PROGRAM_H_

#include "third_party/blink/renderer/modules/webgl/webgl_object.h"
#include "third_party/blink/renderer/modules/webgl/webgl_shader.h"

namespace blink {

class WebGLProgram final : public WebGLObject {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit WebGLProgram(WebGLRenderingContextBase*);
  ~WebGLProgram() override;

  bool LinkStatus(WebGLRenderingContextBase*);
  void setLinkStatus(bool link_status);

  bool CompletionStatus(WebGLRenderingContextBase*);

  unsigned LinkCount() const { return link_count_; }

  // This is to be called everytime after the program is successfully linked.
  // We don't deal with integer overflow here, assuming in reality a program
  // will never be linked so many times.
  // Also, we invalidate the cached program info.
  void IncreaseLinkCount();

  unsigned ActiveTransformFeedbackCount() const {
    return active_transform_feedback_count_;
  }
  void IncreaseActiveTransformFeedbackCount();
  void DecreaseActiveTransformFeedbackCount();

  void SetRequiredTransformFeedbackBufferCount(int count) {
    required_transform_feedback_buffer_count_after_next_link_ = count;
  }
  int GetRequiredTransformFeedbackBufferCount(
      WebGLRenderingContextBase* context) {
    CacheInfoIfNeeded(context);
    return required_transform_feedback_buffer_count_;
  }

  WebGLShader* GetAttachedShader(GLenum);
  bool AttachShader(WebGLShader*);
  bool DetachShader(WebGLShader*);

  void Trace(Visitor*) const override;

 protected:
  void DeleteObjectImpl(gpu::gles2::GLES2Interface*) override;

 private:
  void CacheInfoIfNeeded(WebGLRenderingContextBase*);

  GLint link_status_;

  // This is used to track whether a WebGLUniformLocation belongs to this
  // program or not.
  unsigned link_count_;

  // This is used to track the program being used by active transform
  // feedback objects.
  unsigned active_transform_feedback_count_;

  Member<WebGLShader> vertex_shader_;
  Member<WebGLShader> fragment_shader_;

  bool info_valid_;

  // The number of transform feedback buffers this program will write to.
  int required_transform_feedback_buffer_count_;
  int required_transform_feedback_buffer_count_after_next_link_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_PROGRAM_H_
