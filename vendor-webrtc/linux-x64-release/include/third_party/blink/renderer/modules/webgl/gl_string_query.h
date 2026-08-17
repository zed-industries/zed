// Copyright 2010 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_GL_STRING_QUERY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_GL_STRING_QUERY_H_

#include "base/check_op.h"
#include "base/memory/raw_ptr.h"
#include "gpu/command_buffer/client/gles2_interface.h"
#include "third_party/blink/renderer/platform/wtf/text/string_buffer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/khronos/GLES2/gl2ext.h"

namespace blink {

// Performs a query for a string on GLES2Interface and returns a WTF::String. If
// it is unable to query the string, it wil return an empty WTF::String.
class GLStringQuery {
 public:
  struct ProgramInfoLog {
    static void LengthFunction(gpu::gles2::GLES2Interface* gl,
                               GLuint id,
                               GLint* length) {
      return gl->GetProgramiv(id, GL_INFO_LOG_LENGTH, length);
    }
    static void LogFunction(gpu::gles2::GLES2Interface* gl,
                            GLuint id,
                            GLint length,
                            GLint* returned_length,
                            LChar* ptr) {
      return gl->GetProgramInfoLog(id, length, returned_length,
                                   reinterpret_cast<GLchar*>(ptr));
    }
  };

  struct ShaderInfoLog {
    static void LengthFunction(gpu::gles2::GLES2Interface* gl,
                               GLuint id,
                               GLint* length) {
      gl->GetShaderiv(id, GL_INFO_LOG_LENGTH, length);
    }
    static void LogFunction(gpu::gles2::GLES2Interface* gl,
                            GLuint id,
                            GLint length,
                            GLint* returned_length,
                            LChar* ptr) {
      gl->GetShaderInfoLog(id, length, returned_length,
                           reinterpret_cast<GLchar*>(ptr));
    }
  };

  struct TranslatedShaderSourceANGLE {
    static void LengthFunction(gpu::gles2::GLES2Interface* gl,
                               GLuint id,
                               GLint* length) {
      gl->GetShaderiv(id, GL_TRANSLATED_SHADER_SOURCE_LENGTH_ANGLE, length);
    }
    static void LogFunction(gpu::gles2::GLES2Interface* gl,
                            GLuint id,
                            GLint length,
                            GLint* returned_length,
                            LChar* ptr) {
      gl->GetTranslatedShaderSourceANGLE(id, length, returned_length,
                                         reinterpret_cast<GLchar*>(ptr));
    }
  };

  GLStringQuery(gpu::gles2::GLES2Interface* gl) : gl_(gl) {}

  template <class Traits>
  WTF::String Run(GLuint id) {
    GLint length = 0;
    Traits::LengthFunction(gl_, id, &length);
    if (!length)
      return WTF::g_empty_string;
    GLsizei returned_length = 0;
    StringBuffer<LChar> log_buffer(length);
    Traits::LogFunction(gl_, id, length, &returned_length,
                        log_buffer.Span().data());
    // The returnedLength excludes the null terminator. If this check wasn't
    // true, then we'd need to tell the returned String the real length.
    DCHECK_EQ(returned_length + 1, length);
    return String::Adopt(log_buffer);
  }

 private:
  raw_ptr<gpu::gles2::GLES2Interface> gl_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_GL_STRING_QUERY_H_
