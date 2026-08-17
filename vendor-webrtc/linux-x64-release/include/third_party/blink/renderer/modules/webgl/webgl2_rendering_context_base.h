// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL2_RENDERING_CONTEXT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL2_RENDERING_CONTEXT_BASE_H_

#include <memory>
#include <optional>

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.h"

namespace blink {

class WebGLTexture;

class WebGLActiveInfo;
class WebGLBuffer;
class WebGLProgram;
class WebGLQuery;
class WebGLSampler;
class WebGLSync;
class WebGLTransformFeedback;
class WebGLUniformLocation;
class WebGLVertexArrayObject;

class WebGL2RenderingContextBase : public WebGLRenderingContextBase {
 public:
  void DestroyContext() override;

  /* Buffer objects */
  void bufferData(GLenum,
                  MaybeShared<DOMArrayBufferView>,
                  GLenum,
                  int64_t,
                  GLuint);
  void bufferSubData(GLenum,
                     int64_t offset,
                     MaybeShared<DOMArrayBufferView>,
                     int64_t,
                     GLuint);
  // Have to re-declare/re-define the following buffer{Sub}Data functions from
  // base class.  This is because the above buffer{Sub}Data() hides the name
  // from base class.
  void bufferData(GLenum target, int64_t size, GLenum usage);
  void bufferData(GLenum target, DOMArrayBufferBase* data, GLenum usage);
  void bufferData(GLenum target,
                  MaybeShared<DOMArrayBufferView> data,
                  GLenum usage);
  void bufferSubData(GLenum target,
                     int64_t offset,
                     base::span<const uint8_t> data);

  void copyBufferSubData(GLenum, GLenum, int64_t, int64_t, int64_t);
  void getBufferSubData(GLenum,
                        int64_t,
                        MaybeShared<DOMArrayBufferView>,
                        int64_t,
                        GLuint);

  /* Framebuffer objects */
  bool ValidateTexFuncLayer(const char*, GLenum tex_target, GLint layer);
  void blitFramebuffer(GLint,
                       GLint,
                       GLint,
                       GLint,
                       GLint,
                       GLint,
                       GLint,
                       GLint,
                       GLbitfield,
                       GLenum);
  void framebufferTextureLayer(GLenum, GLenum, WebGLTexture*, GLint, GLint);
  ScriptValue getInternalformatParameter(ScriptState*, GLenum, GLenum, GLenum);
  void invalidateFramebuffer(GLenum, const Vector<GLenum>&);
  void invalidateSubFramebuffer(GLenum,
                                const Vector<GLenum>&,
                                GLint,
                                GLint,
                                GLsizei,
                                GLsizei);
  void readBuffer(GLenum);

  /* Renderbuffer objects */
  void renderbufferStorageMultisample(GLenum,
                                      GLsizei,
                                      GLenum,
                                      GLsizei,
                                      GLsizei);

  /* Texture objects */
  void texImage2D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  int64_t);
  void texImage2D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  ImageData*);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  HTMLImageElement*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  CanvasRenderingContextHost*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  HTMLVideoElement*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  VideoFrame*,
                  ExceptionState&);
  void texImage2D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  ImageBitmap*,
                  ExceptionState&);
  void texImage2D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  MaybeShared<DOMArrayBufferView>,
                  int64_t);

  void texSubImage2D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     int64_t);
  void texSubImage2D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     ImageData*);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     HTMLImageElement*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     CanvasRenderingContextHost*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     HTMLVideoElement*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     VideoFrame*,
                     ExceptionState&);
  void texSubImage2D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     ImageBitmap*,
                     ExceptionState&);
  void texSubImage2D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     MaybeShared<DOMArrayBufferView>,
                     int64_t);

  // Have to re-declare/re-define the following tex{Sub}Image2D functions from
  // base class.  This is because the above tex{Sub}Image2D() hides the name
  // from base class.
  void texImage2D(GLenum, GLint, GLint, GLenum, GLenum, ImageData*);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLenum,
                  GLenum,
                  HTMLImageElement*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLenum,
                  GLenum,
                  CanvasRenderingContextHost*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLenum,
                  GLenum,
                  HTMLVideoElement*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLenum,
                  GLenum,
                  VideoFrame*,
                  ExceptionState&);
  void texImage2D(GLenum,
                  GLint,
                  GLint,
                  GLenum,
                  GLenum,
                  ImageBitmap*,
                  ExceptionState&);
  void texSubImage2D(GLenum, GLint, GLint, GLint, GLenum, GLenum, ImageData*);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLenum,
                     GLenum,
                     HTMLImageElement*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLenum,
                     GLenum,
                     CanvasRenderingContextHost*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLenum,
                     GLenum,
                     HTMLVideoElement*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLenum,
                     GLenum,
                     VideoFrame*,
                     ExceptionState&);
  void texSubImage2D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLenum,
                     GLenum,
                     ImageBitmap*,
                     ExceptionState&);

  void texStorage2D(GLenum, GLsizei, GLenum, GLsizei, GLsizei);
  void texStorage3D(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei);
  void texImage3D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  MaybeShared<DOMArrayBufferView>);
  void texImage3D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  MaybeShared<DOMArrayBufferView>,
                  GLuint);
  void texImage3D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  ImageData*);
  void texImage3D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  HTMLImageElement*,
                  ExceptionState&);
  void texImage3D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  CanvasRenderingContextHost*,
                  ExceptionState&);
  void texImage3D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  HTMLVideoElement*,
                  ExceptionState&);
  void texImage3D(ScriptState*,
                  GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  VideoFrame*,
                  ExceptionState&);
  void texImage3D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  ImageBitmap*,
                  ExceptionState&);
  void texImage3D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  int64_t);
  void texSubImage3D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     MaybeShared<DOMArrayBufferView>,
                     GLuint);
  void texSubImage3D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     int64_t);
  void texSubImage3D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     ImageData*);
  void texSubImage3D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     HTMLImageElement*,
                     ExceptionState&);
  void texSubImage3D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     CanvasRenderingContextHost*,
                     ExceptionState&);
  void texSubImage3D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     HTMLVideoElement*,
                     ExceptionState&);
  void texSubImage3D(ScriptState*,
                     GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     VideoFrame*,
                     ExceptionState&);
  void texSubImage3D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     ImageBitmap*,
                     ExceptionState&);

  void texImage2D(GLenum,
                  GLint,
                  GLint,
                  GLsizei,
                  GLsizei,
                  GLint,
                  GLenum,
                  GLenum,
                  MaybeShared<DOMArrayBufferView>);
  void texSubImage2D(GLenum,
                     GLint,
                     GLint,
                     GLint,
                     GLsizei,
                     GLsizei,
                     GLenum,
                     GLenum,
                     MaybeShared<DOMArrayBufferView>);

  void copyTexSubImage3D(GLenum,
                         GLint,
                         GLint,
                         GLint,
                         GLint,
                         GLint,
                         GLint,
                         GLsizei,
                         GLsizei);

  void compressedTexImage2D(GLenum target,
                            GLint level,
                            GLenum internalformat,
                            GLsizei width,
                            GLsizei height,
                            GLint border,
                            MaybeShared<DOMArrayBufferView> data,
                            GLuint src_offset,
                            GLuint src_length_override);
  void compressedTexSubImage2D(GLenum target,
                               GLint level,
                               GLint xoffset,
                               GLint yoffset,
                               GLsizei width,
                               GLsizei height,
                               GLenum format,
                               MaybeShared<DOMArrayBufferView> data,
                               GLuint src_offset,
                               GLuint src_length_override);
  void compressedTexImage3D(GLenum,
                            GLint,
                            GLenum,
                            GLsizei,
                            GLsizei,
                            GLsizei,
                            GLint,
                            MaybeShared<DOMArrayBufferView>,
                            GLuint,
                            GLuint);
  void compressedTexSubImage3D(GLenum,
                               GLint,
                               GLint,
                               GLint,
                               GLint,
                               GLsizei,
                               GLsizei,
                               GLsizei,
                               GLenum,
                               MaybeShared<DOMArrayBufferView>,
                               GLuint,
                               GLuint);
  void compressedTexImage2D(GLenum target,
                            GLint level,
                            GLenum internalformat,
                            GLsizei width,
                            GLsizei height,
                            GLint border,
                            GLsizei image_size,
                            int64_t offset);
  void compressedTexSubImage2D(GLenum target,
                               GLint level,
                               GLint xoffset,
                               GLint yoffset,
                               GLsizei width,
                               GLsizei height,
                               GLenum format,
                               GLsizei image_size,
                               int64_t offset);
  void compressedTexImage3D(GLenum target,
                            GLint level,
                            GLenum internalformat,
                            GLsizei width,
                            GLsizei height,
                            GLsizei depth,
                            GLint border,
                            GLsizei image_size,
                            int64_t offset);
  void compressedTexSubImage3D(GLenum target,
                               GLint level,
                               GLint xoffset,
                               GLint yoffset,
                               GLint zoffset,
                               GLsizei width,
                               GLsizei height,
                               GLsizei depth,
                               GLenum format,
                               GLsizei image_size,
                               int64_t offset);

  // Have to re-declare/re-define the following compressedTex{Sub}Image2D
  // functions from the base class. This is because the above
  // compressedTex{Sub}Image2D() hide the name from base class.
  void compressedTexImage2D(GLenum target,
                            GLint level,
                            GLenum internalformat,
                            GLsizei width,
                            GLsizei height,
                            GLint border,
                            MaybeShared<DOMArrayBufferView> data);
  void compressedTexSubImage2D(GLenum target,
                               GLint level,
                               GLint xoffset,
                               GLint yoffset,
                               GLsizei width,
                               GLsizei height,
                               GLenum format,
                               MaybeShared<DOMArrayBufferView> data);

  /* Programs and shaders */
  GLint getFragDataLocation(WebGLProgram*, const String&);

  /* Uniforms and attributes */
  void uniform1ui(const WebGLUniformLocation*, GLuint);
  void uniform2ui(const WebGLUniformLocation*, GLuint, GLuint);
  void uniform3ui(const WebGLUniformLocation*, GLuint, GLuint, GLuint);
  void uniform4ui(const WebGLUniformLocation*, GLuint, GLuint, GLuint, GLuint);
  void uniform1fv(const WebGLUniformLocation*,
                  base::span<const GLfloat>,
                  GLuint,
                  GLuint);
  void uniform2fv(const WebGLUniformLocation*,
                  base::span<const GLfloat>,
                  GLuint,
                  GLuint);
  void uniform3fv(const WebGLUniformLocation*,
                  base::span<const GLfloat>,
                  GLuint,
                  GLuint);
  void uniform4fv(const WebGLUniformLocation*,
                  base::span<const GLfloat>,
                  GLuint,
                  GLuint);
  void uniform1iv(const WebGLUniformLocation*,
                  base::span<const GLint>,
                  GLuint,
                  GLuint);
  void uniform2iv(const WebGLUniformLocation*,
                  base::span<const GLint>,
                  GLuint,
                  GLuint);
  void uniform3iv(const WebGLUniformLocation*,
                  base::span<const GLint>,
                  GLuint,
                  GLuint);
  void uniform4iv(const WebGLUniformLocation*,
                  base::span<const GLint>,
                  GLuint,
                  GLuint);
  void uniform1uiv(const WebGLUniformLocation*,
                   base::span<const GLuint>,
                   GLuint,
                   GLuint);
  void uniform2uiv(const WebGLUniformLocation*,
                   base::span<const GLuint>,
                   GLuint,
                   GLuint);
  void uniform3uiv(const WebGLUniformLocation*,
                   base::span<const GLuint>,
                   GLuint,
                   GLuint);
  void uniform4uiv(const WebGLUniformLocation*,
                   base::span<const GLuint>,
                   GLuint,
                   GLuint);
  void uniformMatrix2fv(const WebGLUniformLocation*,
                        GLboolean,
                        base::span<const GLfloat>,
                        GLuint,
                        GLuint);
  void uniformMatrix3fv(const WebGLUniformLocation*,
                        GLboolean,
                        base::span<const GLfloat>,
                        GLuint,
                        GLuint);
  void uniformMatrix4fv(const WebGLUniformLocation*,
                        GLboolean,
                        base::span<const GLfloat>,
                        GLuint,
                        GLuint);
  void uniformMatrix2x3fv(const WebGLUniformLocation*,
                          GLboolean,
                          base::span<const GLfloat>,
                          GLuint,
                          GLuint);
  void uniformMatrix3x2fv(const WebGLUniformLocation*,
                          GLboolean,
                          base::span<const GLfloat>,
                          GLuint,
                          GLuint);
  void uniformMatrix2x4fv(const WebGLUniformLocation*,
                          GLboolean,
                          base::span<const GLfloat>,
                          GLuint,
                          GLuint);
  void uniformMatrix4x2fv(const WebGLUniformLocation*,
                          GLboolean,
                          base::span<const GLfloat>,
                          GLuint,
                          GLuint);
  void uniformMatrix3x4fv(const WebGLUniformLocation*,
                          GLboolean,
                          base::span<const GLfloat>,
                          GLuint,
                          GLuint);
  void uniformMatrix4x3fv(const WebGLUniformLocation*,
                          GLboolean,
                          base::span<const GLfloat>,
                          GLuint,
                          GLuint);
  // Have to re-declare/re-define the following uniform*()
  // functions from the base class. This is because the above
  // uniform*() hide the name from base class.
  void uniform1fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform2fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform3fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform4fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform1iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniform2iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniform3iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniform4iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniformMatrix2fv(const WebGLUniformLocation*,
                        GLboolean transpose,
                        base::span<const GLfloat> value);
  void uniformMatrix3fv(const WebGLUniformLocation*,
                        GLboolean transpose,
                        base::span<const GLfloat> value);
  void uniformMatrix4fv(const WebGLUniformLocation*,
                        GLboolean transpose,
                        base::span<const GLfloat> value);

  void vertexAttribI4i(GLuint, GLint, GLint, GLint, GLint);
  void vertexAttribI4iv(GLuint, base::span<const GLint>);
  void vertexAttribI4ui(GLuint, GLuint, GLuint, GLuint, GLuint);
  void vertexAttribI4uiv(GLuint, base::span<const GLuint>);
  void vertexAttribIPointer(GLuint index,
                            GLint size,
                            GLenum type,
                            GLsizei stride,
                            int64_t offset);

  /* Writing to the drawing buffer */
  void vertexAttribDivisor(GLuint, GLuint);
  void drawArraysInstanced(GLenum, GLint, GLsizei, GLsizei);
  void drawElementsInstanced(GLenum, GLsizei, GLenum, int64_t, GLsizei);
  void drawRangeElements(GLenum mode,
                         GLuint start,
                         GLuint end,
                         GLsizei count,
                         GLenum type,
                         int64_t offset);

  /* Multiple Render Targets */
  void drawBuffers(const Vector<GLenum>&);
  void clearBufferiv(GLenum, GLint, base::span<const GLint>, GLuint);
  void clearBufferuiv(GLenum, GLint, base::span<const GLuint>, GLuint);
  void clearBufferfv(GLenum, GLint, base::span<const GLfloat>, GLuint);
  void clearBufferfi(GLenum, GLint, GLfloat, GLint);

  /* Query Objects */
  WebGLQuery* createQuery();
  void deleteQuery(WebGLQuery*);
  bool isQuery(WebGLQuery*);
  void beginQuery(GLenum, WebGLQuery*);
  void endQuery(GLenum);
  ScriptValue getQuery(ScriptState*, GLenum, GLenum);
  ScriptValue getQueryParameter(ScriptState*, WebGLQuery*, GLenum);

  /* Sampler Objects */
  WebGLSampler* createSampler();
  void deleteSampler(WebGLSampler*);
  bool isSampler(WebGLSampler*);
  void bindSampler(GLuint, WebGLSampler*);
  void samplerParameteri(WebGLSampler*, GLenum, GLint);
  void samplerParameterf(WebGLSampler*, GLenum, GLfloat);
  ScriptValue getSamplerParameter(ScriptState*, WebGLSampler*, GLenum);

  /* Sync objects */
  WebGLSync* fenceSync(GLenum, GLbitfield);
  bool isSync(WebGLSync*);
  void deleteSync(WebGLSync*);
  GLenum clientWaitSync(WebGLSync*, GLbitfield, GLuint64);
  void waitSync(WebGLSync*, GLbitfield, GLint64);

  ScriptValue getSyncParameter(ScriptState*, WebGLSync*, GLenum);

  /* Transform Feedback */
  WebGLTransformFeedback* createTransformFeedback();
  void deleteTransformFeedback(WebGLTransformFeedback*);
  bool isTransformFeedback(WebGLTransformFeedback*);
  void bindTransformFeedback(GLenum, WebGLTransformFeedback*);
  void beginTransformFeedback(GLenum);
  void endTransformFeedback();
  void transformFeedbackVaryings(WebGLProgram*, const Vector<String>&, GLenum);
  WebGLActiveInfo* getTransformFeedbackVarying(WebGLProgram*, GLuint);
  void pauseTransformFeedback();
  void resumeTransformFeedback();
  bool ValidateTransformFeedbackPrimitiveMode(const char* function_name,
                                              GLenum primitive_mode);

  void OnBeforeDrawCall(CanvasPerformanceMonitor::DrawType) override;

  /* Uniform Buffer Objects and Transform Feedback Buffers */
  void bindBufferBase(GLenum, GLuint, WebGLBuffer*);
  void bindBufferRange(GLenum, GLuint, WebGLBuffer*, int64_t, int64_t);
  virtual ScriptValue getIndexedParameter(ScriptState*, GLenum, GLuint);
  std::optional<Vector<GLuint>> getUniformIndices(WebGLProgram*,
                                                  const Vector<String>&);
  ScriptValue getActiveUniforms(ScriptState*,
                                WebGLProgram*,
                                const Vector<GLuint>&,
                                GLenum);
  GLuint getUniformBlockIndex(WebGLProgram*, const String&);
  ScriptValue getActiveUniformBlockParameter(ScriptState*,
                                             WebGLProgram*,
                                             GLuint,
                                             GLenum);
  String getActiveUniformBlockName(WebGLProgram*, GLuint);
  void uniformBlockBinding(WebGLProgram*, GLuint, GLuint);

  /* Vertex Array Objects */
  WebGLVertexArrayObject* createVertexArray();
  void deleteVertexArray(WebGLVertexArrayObject*);
  bool isVertexArray(WebGLVertexArrayObject*);
  void bindVertexArray(WebGLVertexArrayObject*);

  /* Reading */
  void readPixels(GLint x,
                  GLint y,
                  GLsizei width,
                  GLsizei height,
                  GLenum format,
                  GLenum type,
                  MaybeShared<DOMArrayBufferView> pixels,
                  int64_t offset);
  void readPixels(GLint x,
                  GLint y,
                  GLsizei width,
                  GLsizei height,
                  GLenum format,
                  GLenum type,
                  int64_t offset);

  /* WebGLRenderingContextBase overrides */
  void InitializeNewContext() override;
  void bindFramebuffer(GLenum target, WebGLFramebuffer*) override;
  void deleteFramebuffer(WebGLFramebuffer*) override;
  ScriptValue getParameter(ScriptState*, GLenum pname) override;
  ScriptValue getTexParameter(ScriptState*,
                              GLenum target,
                              GLenum pname) override;
  ScriptValue getFramebufferAttachmentParameter(ScriptState*,
                                                GLenum target,
                                                GLenum attachment,
                                                GLenum pname) override;
  void pixelStorei(GLenum pname, GLint param) override;
  void readPixels(GLint x,
                  GLint y,
                  GLsizei width,
                  GLsizei height,
                  GLenum format,
                  GLenum type,
                  MaybeShared<DOMArrayBufferView> pixels) override;
  void RestoreCurrentFramebuffer() override;
  void useProgram(WebGLProgram*) override;

  void Trace(Visitor*) const override;

 protected:
  friend class V8WebGL2RenderingContext;
  friend class WebGLSync;

  WebGL2RenderingContextBase(
      CanvasRenderingContextHost*,
      std::unique_ptr<WebGraphicsContext3DProvider>,
      const Platform::GraphicsInfo&,
      const CanvasContextCreationAttributesCore& requested_attributes,
      Platform::ContextType context_type);

  // DrawingBuffer::Client implementation.
  void DrawingBufferClientRestorePixelUnpackBufferBinding() override;
  void DrawingBufferClientRestorePixelPackBufferBinding() override;
  void DrawingBufferClientRestorePixelPackParameters() override;

  // Helper function to validate target and the attachment combination for
  // getFramebufferAttachmentParameters.  Generate GL error and return false if
  // parameters are illegal.
  bool ValidateGetFramebufferAttachmentParameterFunc(const char* function_name,
                                                     GLenum target,
                                                     GLenum attachment);

  bool ValidateClearBuffer(const char* function_name,
                           GLenum buffer,
                           size_t length,
                           GLuint src_offset);

  enum TexStorageType {
    kTexStorageType2D,
    kTexStorageType3D,
  };

  bool ValidateUniformBlockIndex(const char*, WebGLProgram*, GLuint);

  ScriptValue GetInt64Parameter(ScriptState*, GLenum);

  void SamplerParameter(WebGLSampler*, GLenum, GLfloat, GLint, bool);

  bool IsBufferBoundToTransformFeedback(WebGLBuffer*);
  bool IsBufferBoundToNonTransformFeedback(WebGLBuffer*);
  virtual bool ValidateBufferTargetCompatibility(const char*,
                                                 GLenum,
                                                 WebGLBuffer*);

  virtual bool ValidateBufferBaseTarget(const char* function_name,
                                        GLenum target);
  virtual bool ValidateAndUpdateBufferBindBaseTarget(const char* function_name,
                                                     GLenum,
                                                     GLuint,
                                                     WebGLBuffer*);

  WebGLImageConversion::PixelStoreParams GetPackPixelStoreParams() override;
  WebGLImageConversion::PixelStoreParams GetUnpackPixelStoreParams(
      TexImageDimension) override;

  bool CheckAndTranslateAttachments(const char* function_name,
                                    GLenum,
                                    Vector<GLenum>&);

  gfx::Rect GetTextureSourceSubRectangle(GLsizei width, GLsizei height);

  /* WebGLRenderingContextBase overrides */
  unsigned GetMaxWebGLLocationLength() const override { return 1024; }
  bool ValidateCapability(const char* function_name, GLenum) override;
  bool ValidateBufferTarget(const char* function_name, GLenum target) override;
  bool ValidateAndUpdateBufferBindTarget(const char* function_name,
                                         GLenum,
                                         WebGLBuffer*) override;
  bool ValidateFramebufferTarget(GLenum target) override;

  bool ValidateReadPixelsFormatAndType(GLenum format,
                                       GLenum type,
                                       DOMArrayBufferView*) override;
  WebGLFramebuffer* GetFramebufferBinding(GLenum target) override;
  WebGLFramebuffer* GetReadFramebufferBinding() override;
  GLint GetMaxTextureLevelForTarget(GLenum target) override;
  void RenderbufferStorageImpl(GLenum target,
                               GLsizei samples,
                               GLenum internalformat,
                               GLsizei width,
                               GLsizei height,
                               const char* function_name) override;

  void GetCurrentUnpackState(TexImageParams& params) override;
  WebGLTexture* ValidateTexImageBinding(const TexImageParams& params) override;
  // Helper function to check texture 3D target and texture bound to the target.
  // Generates GL errors and returns 0 if target is invalid or texture bound is
  // null.  Otherwise, returns the texture bound to the target.
  // If |validate_opaque_textures| is true, the helper will also generate a GL
  // error when the texture bound to the target is opaque.
  // See https://www.w3.org/TR/webxrlayers-1/#opaque-texture for details about
  // opaque textures.
  WebGLTexture* ValidateTexture3DBinding(const char* function_name,
                                         GLenum target,
                                         bool validate_opaque_textures = false);

  WebGLBuffer* ValidateBufferDataTarget(const char* function_name,
                                        GLenum target) override;
  bool ValidateBufferDataUsage(const char* function_name,
                               GLenum usage) override;

  const char* ValidateGetBufferSubData(const char* function_name,
                                       GLenum target,
                                       int64_t source_byte_offset,
                                       DOMArrayBufferView*,
                                       int64_t destination_offset,
                                       GLuint length,
                                       WebGLBuffer**,
                                       void** out_destination_data_ptr,
                                       int64_t* out_destination_byte_length);
  const char* ValidateGetBufferSubDataBounds(const char* function_name,
                                             WebGLBuffer*,
                                             GLintptr source_byte_offset,
                                             int64_t destination_byte_length);

  void RemoveBoundBuffer(WebGLBuffer*) override;

  void ResetUnpackParameters() override;
  void RestoreUnpackParameters() override;

  void RenderbufferStorageHelper(GLenum target,
                                 GLsizei samples,
                                 GLenum internalformat,
                                 GLsizei width,
                                 GLsizei height,
                                 const char* function_name);

  Member<WebGLFramebuffer> read_framebuffer_binding_;
  Member<WebGLTransformFeedback> transform_feedback_binding_;
  // This instance isn't exposed to JavaScript, which is why it's a
  // Member rather than Member.
  Member<WebGLTransformFeedback> default_transform_feedback_;

  GLenumHashSet supported_internal_formats_storage_;

  Member<WebGLBuffer> bound_copy_read_buffer_;
  Member<WebGLBuffer> bound_copy_write_buffer_;
  Member<WebGLBuffer> bound_pixel_pack_buffer_;
  Member<WebGLBuffer> bound_pixel_unpack_buffer_;
  Member<WebGLBuffer> bound_transform_feedback_buffer_;
  Member<WebGLBuffer> bound_uniform_buffer_;

  HeapVector<Member<WebGLBuffer>> bound_indexed_uniform_buffers_;
  GLint max_transform_feedback_separate_attribs_;

  Member<WebGLQuery> current_boolean_occlusion_query_;
  Member<WebGLQuery> current_transform_feedback_primitives_written_query_;
  Member<WebGLQuery> current_elapsed_query_;
  HeapVector<Member<WebGLSampler>> sampler_units_;

  GLint pack_row_length_;
  GLint pack_skip_pixels_;
  GLint pack_skip_rows_;
  GLint unpack_image_height_;
  GLint unpack_skip_images_;

 private:
  void RecordInternalFormatParameter(GLenum internalformat,
                                     GLint* values,
                                     GLint length);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL2_RENDERING_CONTEXT_BASE_H_
