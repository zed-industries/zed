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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_RENDERING_CONTEXT_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_RENDERING_CONTEXT_BASE_H_

#include <array>
#include <memory>
#include <optional>

#include "base/check_op.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/numerics/checked_math.h"
#include "base/task/single_thread_task_runner.h"
#include "device/vr/public/mojom/vr_service.mojom-blink-forward.h"
#include "third_party/blink/public/platform/platform.h"
#include "third_party/blink/public/platform/web_graphics_context_3d_provider.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_webgl_context_attributes.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_context_creation_attributes_core.h"
#include "third_party/blink/renderer/core/html/canvas/canvas_rendering_context.h"
#include "third_party/blink/renderer/core/html/canvas/ukm_parameters.h"
#include "third_party/blink/renderer/core/layout/content_change_type.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/webgl/webgl_extension_name.h"
#include "third_party/blink/renderer/modules/webgl/webgl_texture.h"
#include "third_party/blink/renderer/modules/webgl/webgl_uniform_location.h"
#include "third_party/blink/renderer/modules/webgl/webgl_vertex_array_object_base.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/graphics/gpu/drawing_buffer.h"
#include "third_party/blink/renderer/platform/graphics/gpu/extensions_3d_util.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgl_image_conversion.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/khronos/GLES2/gl2.h"
#include "third_party/khronos/GLES3/gl31.h"
#include "third_party/skia/include/core/SkData.h"

namespace cc {
class Layer;
}

namespace gpu {
namespace gles2 {
class GLES2Interface;
}
}  // namespace gpu

namespace media {
class PaintCanvasVideoRenderer;
}

namespace blink {

class AcceleratedStaticBitmapImage;
class CanvasResourceProvider;
class EXTDisjointTimerQuery;
class EXTDisjointTimerQueryWebGL2;
class ExceptionState;
class HTMLCanvasElement;
class HTMLImageElement;
class HTMLVideoElement;
class ImageBitmap;
class ImageData;
class OESVertexArrayObject;
class ScriptState;
class V8PredefinedColorSpace;
class V8UnionHTMLCanvasElementOrOffscreenCanvas;
class VideoFrame;
class WebGLActiveInfo;
class WebGLBuffer;
class WebGLCompressedTextureASTC;
class WebGLCompressedTextureETC;
class WebGLCompressedTextureETC1;
class WebGLCompressedTexturePVRTC;
class WebGLCompressedTextureS3TC;
class WebGLCompressedTextureS3TCsRGB;
class WebGLContextObject;
class WebGLDebugShaders;
class WebGLDrawBuffers;
class WebGLExtension;
class WebGLFramebuffer;
class WebGLObject;
class WebGLProgram;
class WebGLRenderbuffer;
class WebGLShader;
class WebGLShaderPrecisionFormat;
class WebGLVertexArrayObjectBase;
class XRSystem;

using GLenumHashSet = HashSet<GLenum, AlreadyHashedWithZeroKeyTraits>;

// This class uses the color mask to prevent drawing to the alpha channel, if
// the DrawingBuffer requires RGB emulation.
class ScopedRGBEmulationColorMask {
  STACK_ALLOCATED();

 public:
  ScopedRGBEmulationColorMask(WebGLRenderingContextBase*,
                              GLboolean* color_mask,
                              DrawingBuffer*);
  ~ScopedRGBEmulationColorMask();

 private:
  WebGLRenderingContextBase* context_;
  std::array<GLboolean, 4> color_mask_;
  const bool requires_emulation_;
};

class MODULES_EXPORT WebGLRenderingContextBase : public ScriptWrappable,
                                                 public CanvasRenderingContext,
                                                 public DrawingBuffer::Client {
 public:
  WebGLRenderingContextBase(const WebGLRenderingContextBase&) = delete;
  WebGLRenderingContextBase& operator=(const WebGLRenderingContextBase&) =
      delete;

  ~WebGLRenderingContextBase() override;

  HTMLCanvasElement* canvas() const;

  const UkmParameters GetUkmParameters() const {
    return Host()->GetUkmParameters();
  }

  virtual String ContextName() const = 0;
  virtual void RegisterContextExtensions() = 0;

  virtual void InitializeNewContext();

  static unsigned GetWebGLVersion(const CanvasRenderingContext*);

  static std::unique_ptr<WebGraphicsContext3DProvider>
  CreateWebGraphicsContext3DProvider(CanvasRenderingContextHost*,
                                     const CanvasContextCreationAttributesCore&,
                                     Platform::ContextType context_type,
                                     Platform::GraphicsInfo* graphics_info);
  static void ForceNextWebGLContextCreationToFail();

  Platform::ContextType ContextType() const { return context_type_; }

  int drawingBufferWidth() const;
  int drawingBufferHeight() const;
  GLenum drawingBufferFormat() const;
  V8PredefinedColorSpace drawingBufferColorSpace() const;
  void setDrawingBufferColorSpace(const V8PredefinedColorSpace& color_space,
                                  ExceptionState&);

  V8PredefinedColorSpace unpackColorSpace() const;
  void setUnpackColorSpace(const V8PredefinedColorSpace& color_space,
                           ExceptionState&);

  void activeTexture(GLenum texture);
  void attachShader(WebGLProgram*, WebGLShader*);
  void bindAttribLocation(WebGLProgram*, GLuint index, const String& name);
  void bindBuffer(GLenum target, WebGLBuffer* buffer);
  virtual void bindFramebuffer(GLenum target, WebGLFramebuffer*);
  void bindRenderbuffer(GLenum target, WebGLRenderbuffer*);
  void bindTexture(GLenum target, WebGLTexture*);
  void blendColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
  void blendEquation(GLenum mode);
  void blendEquationSeparate(GLenum mode_rgb, GLenum mode_alpha);
  void blendFunc(GLenum sfactor, GLenum dfactor);
  void blendFuncSeparate(GLenum src_rgb,
                         GLenum dst_rgb,
                         GLenum src_alpha,
                         GLenum dst_alpha);

  void bufferData(GLenum target, int64_t size, GLenum usage);
  void bufferData(GLenum target, DOMArrayBufferBase* data, GLenum usage);
  void bufferData(GLenum target,
                  MaybeShared<DOMArrayBufferView> data,
                  GLenum usage);
  void bufferSubData(GLenum target,
                     int64_t offset,
                     base::span<const uint8_t> data);

  GLenum checkFramebufferStatus(GLenum target);
  void clear(GLbitfield mask);
  void clearColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
  void clearDepth(GLfloat);
  void clearStencil(GLint);
  void colorMask(GLboolean red,
                 GLboolean green,
                 GLboolean blue,
                 GLboolean alpha);
  void compileShader(WebGLShader*);

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

  void copyTexImage2D(GLenum target,
                      GLint level,
                      GLenum internalformat,
                      GLint x,
                      GLint y,
                      GLsizei width,
                      GLsizei height,
                      GLint border);
  void copyTexSubImage2D(GLenum target,
                         GLint level,
                         GLint xoffset,
                         GLint yoffset,
                         GLint x,
                         GLint y,
                         GLsizei width,
                         GLsizei height);

  WebGLBuffer* createBuffer();
  WebGLFramebuffer* createFramebuffer();
  WebGLProgram* createProgram();
  WebGLRenderbuffer* createRenderbuffer();
  WebGLShader* createShader(GLenum type);
  WebGLTexture* createTexture();

  void cullFace(GLenum mode);

  void deleteBuffer(WebGLBuffer*);
  virtual void deleteFramebuffer(WebGLFramebuffer*);
  void deleteProgram(WebGLProgram*);
  void deleteRenderbuffer(WebGLRenderbuffer*);
  void deleteShader(WebGLShader*);
  void deleteTexture(WebGLTexture*);

  void depthFunc(GLenum);
  void depthMask(GLboolean);
  void depthRange(GLfloat z_near, GLfloat z_far);
  void detachShader(WebGLProgram*, WebGLShader*);
  void disable(GLenum cap);
  void disableVertexAttribArray(GLuint index);

  void drawArrays(GLenum mode, GLint first, GLsizei count);

  void drawElements(GLenum mode, GLsizei count, GLenum type, int64_t offset);

  void DrawArraysInstancedANGLE(GLenum mode,
                                GLint first,
                                GLsizei count,
                                GLsizei primcount);
  void DrawElementsInstancedANGLE(GLenum mode,
                                  GLsizei count,
                                  GLenum type,
                                  int64_t offset,
                                  GLsizei primcount);

  void enable(GLenum cap);
  void enableVertexAttribArray(GLuint index);
  void finish();
  void flush();
  void framebufferRenderbuffer(GLenum target,
                               GLenum attachment,
                               GLenum renderbuffertarget,
                               WebGLRenderbuffer*);
  void framebufferTexture2D(GLenum target,
                            GLenum attachment,
                            GLenum textarget,
                            WebGLTexture*,
                            GLint level);
  void frontFace(GLenum mode);
  void generateMipmap(GLenum target);

  WebGLActiveInfo* getActiveAttrib(WebGLProgram*, GLuint index);
  WebGLActiveInfo* getActiveUniform(WebGLProgram*, GLuint index);
  bool getAttachedShaders(WebGLProgram*, HeapVector<Member<WebGLShader>>&);
  std::optional<HeapVector<Member<WebGLShader>>> getAttachedShaders(
      WebGLProgram*);
  GLint getAttribLocation(WebGLProgram*, const String& name);
  ScriptValue getBufferParameter(ScriptState*, GLenum target, GLenum pname);
  WebGLContextAttributes* getContextAttributes() const;
  GLenum getError();
  ScriptObject getExtension(ScriptState*, const String& name);
  virtual ScriptValue getFramebufferAttachmentParameter(ScriptState*,
                                                        GLenum target,
                                                        GLenum attachment,
                                                        GLenum pname);
  virtual ScriptValue getParameter(ScriptState*, GLenum pname);
  ScriptValue getProgramParameter(ScriptState*, WebGLProgram*, GLenum pname);
  String getProgramInfoLog(WebGLProgram*);
  ScriptValue getRenderbufferParameter(ScriptState*,
                                       GLenum target,
                                       GLenum pname);
  ScriptValue getShaderParameter(ScriptState*, WebGLShader*, GLenum pname);
  String getShaderInfoLog(WebGLShader*);
  WebGLShaderPrecisionFormat* getShaderPrecisionFormat(GLenum shader_type,
                                                       GLenum precision_type);
  String getShaderSource(WebGLShader*);
  std::optional<Vector<String>> getSupportedExtensions();
  virtual ScriptValue getTexParameter(ScriptState*,
                                      GLenum target,
                                      GLenum pname);
  ScriptValue getUniform(ScriptState*,
                         WebGLProgram*,
                         const WebGLUniformLocation*);
  WebGLUniformLocation* getUniformLocation(WebGLProgram*, const String&);
  ScriptValue getVertexAttrib(ScriptState*, GLuint index, GLenum pname);
  int64_t getVertexAttribOffset(GLuint index, GLenum pname);

  void hint(GLenum target, GLenum mode);
  bool isBuffer(WebGLBuffer*);
  bool isContextLost() const override;
  bool isEnabled(GLenum cap);
  bool isFramebuffer(WebGLFramebuffer*);
  bool isProgram(WebGLProgram*);
  bool isRenderbuffer(WebGLRenderbuffer*);
  bool isShader(WebGLShader*);
  bool isTexture(WebGLTexture*);

  void lineWidth(GLfloat);
  void linkProgram(WebGLProgram*);
  virtual void pixelStorei(GLenum pname, GLint param);
  void polygonOffset(GLfloat factor, GLfloat units);
  virtual void readPixels(GLint x,
                          GLint y,
                          GLsizei width,
                          GLsizei height,
                          GLenum format,
                          GLenum type,
                          MaybeShared<DOMArrayBufferView> pixels);
  void renderbufferStorage(GLenum target,
                           GLenum internalformat,
                           GLsizei width,
                           GLsizei height);
  void sampleCoverage(GLfloat value, GLboolean invert);
  void scissor(GLint x, GLint y, GLsizei width, GLsizei height);
  void shaderSource(WebGLShader*, const String&);
  void stencilFunc(GLenum func, GLint ref, GLuint mask);
  void stencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
  void stencilMask(GLuint);
  void stencilMaskSeparate(GLenum face, GLuint mask);
  void stencilOp(GLenum fail, GLenum zfail, GLenum zpass);
  void stencilOpSeparate(GLenum face, GLenum fail, GLenum zfail, GLenum zpass);

  void texImage2D(GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLsizei width,
                  GLsizei height,
                  GLint border,
                  GLenum format,
                  GLenum type,
                  MaybeShared<DOMArrayBufferView>);
  void texImage2D(GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLenum format,
                  GLenum type,
                  ImageData*);
  void texImage2D(ScriptState*,
                  GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLenum format,
                  GLenum type,
                  HTMLImageElement*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLenum format,
                  GLenum type,
                  CanvasRenderingContextHost*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLenum format,
                  GLenum type,
                  HTMLVideoElement*,
                  ExceptionState&);
  void texImage2D(ScriptState*,
                  GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLenum format,
                  GLenum type,
                  VideoFrame*,
                  ExceptionState&);
  void texImage2D(GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLenum format,
                  GLenum type,
                  ImageBitmap*,
                  ExceptionState&);

  void texParameterf(GLenum target, GLenum pname, GLfloat param);
  void texParameteri(GLenum target, GLenum pname, GLint param);

  void texSubImage2D(GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLsizei width,
                     GLsizei height,
                     GLenum format,
                     GLenum type,
                     MaybeShared<DOMArrayBufferView>);
  void texSubImage2D(GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLenum format,
                     GLenum type,
                     ImageData*);
  void texSubImage2D(ScriptState*,
                     GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLenum format,
                     GLenum type,
                     HTMLImageElement*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLenum format,
                     GLenum type,
                     CanvasRenderingContextHost*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLenum format,
                     GLenum type,
                     HTMLVideoElement*,
                     ExceptionState&);
  void texSubImage2D(ScriptState*,
                     GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLenum format,
                     GLenum type,
                     VideoFrame*,
                     ExceptionState&);
  void texSubImage2D(GLenum target,
                     GLint level,
                     GLint xoffset,
                     GLint yoffset,
                     GLenum format,
                     GLenum type,
                     ImageBitmap*,
                     ExceptionState&);

  void uniform1f(const WebGLUniformLocation*, GLfloat x);
  void uniform1fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform1i(const WebGLUniformLocation*, GLint x);
  void uniform1iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniform2f(const WebGLUniformLocation*, GLfloat x, GLfloat y);
  void uniform2fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform2i(const WebGLUniformLocation*, GLint x, GLint y);
  void uniform2iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniform3f(const WebGLUniformLocation*, GLfloat x, GLfloat y, GLfloat z);
  void uniform3fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform3i(const WebGLUniformLocation*, GLint x, GLint y, GLint z);
  void uniform3iv(const WebGLUniformLocation*, base::span<const GLint>);
  void uniform4f(const WebGLUniformLocation*,
                 GLfloat x,
                 GLfloat y,
                 GLfloat z,
                 GLfloat w);
  void uniform4fv(const WebGLUniformLocation*, base::span<const GLfloat>);
  void uniform4i(const WebGLUniformLocation*,
                 GLint x,
                 GLint y,
                 GLint z,
                 GLint w);
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

  virtual void useProgram(WebGLProgram*);
  void validateProgram(WebGLProgram*);

  void vertexAttrib1f(GLuint index, GLfloat x);
  void vertexAttrib1fv(GLuint index, base::span<const GLfloat> values);
  void vertexAttrib2f(GLuint index, GLfloat x, GLfloat y);
  void vertexAttrib2fv(GLuint index, base::span<const GLfloat> values);
  void vertexAttrib3f(GLuint index, GLfloat x, GLfloat y, GLfloat z);
  void vertexAttrib3fv(GLuint index, base::span<const GLfloat> values);
  void vertexAttrib4f(GLuint index, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
  void vertexAttrib4fv(GLuint index, base::span<const GLfloat> values);
  void vertexAttribPointer(GLuint index,
                           GLint size,
                           GLenum type,
                           GLboolean normalized,
                           GLsizei stride,
                           int64_t offset);

  void VertexAttribDivisorANGLE(GLuint index, GLuint divisor);

  void viewport(GLint x, GLint y, GLsizei width, GLsizei height);

  // WEBGL_lose_context support
  enum AutoRecoveryMethod {
    // Don't restore automatically.
    kManual,

    // Restore when resources are available.
    kWhenAvailable,

    // Restore as soon as possible, but only when
    // the canvas is visible.
    kAuto
  };
  void LoseContext(LostContextMode) override;
  void ForceLostContext(LostContextMode, AutoRecoveryMethod);
  void ForceRestoreContext();
  void LoseContextImpl(LostContextMode, AutoRecoveryMethod);
  uint32_t NumberOfContextLosses() const;

  // Utilities to restore GL state to match the rendering context's
  // saved state. Use these after contextGL()-based state changes that
  // bypass the rendering context.
  void RestoreScissorEnabled();
  void RestoreScissorBox();
  void RestoreClearColor();
  void RestoreColorMask();
  void RestoreVertexArrayObjectBinding();
  void RestoreProgram();
  void RestoreActiveTexture();

  gpu::gles2::GLES2Interface* ContextGL() const {
    DrawingBuffer* d = GetDrawingBuffer();
    if (!d)
      return nullptr;
    return d->ContextGL();
  }
  const gpu::Capabilities& ContextGLCapabilities() const {
    // This should only be called in contexts where ContextGL() is guaranteed
    // to exist.
    CHECK(ContextGL());
    // Note: DrawingBuffer::ContextGL() comes from
    // DrawingBuffer::ContextProvider::ContextGL().
    return GetDrawingBuffer()->ContextProvider()->GetCapabilities();
  }
  gpu::SharedImageInterface* SharedImageInterface() const {
    DrawingBuffer* d = GetDrawingBuffer();
    if (!d)
      return nullptr;
    return d->ContextProvider()->SharedImageInterface();
  }

  Extensions3DUtil* ExtensionsUtil();

  void Reshape(int width, int height) override;

  void MarkLayerComposited() override;

  sk_sp<SkData> PaintRenderingResultsToRGBADataArray(
      SourceDrawingBuffer) override;

  unsigned MaxVertexAttribs() const { return max_vertex_attribs_; }

  void Trace(Visitor*) const override;

  // Returns approximate gpu memory allocated per pixel.
  int ExternallyAllocatedBufferCountPerPixel() override;

  // Returns the drawing buffer size after it is, probably, has scaled down
  // to the maximum supported canvas size.
  gfx::Size DrawingBufferSize() const override;
  DrawingBuffer* GetDrawingBuffer() const;

  class TextureUnitState {
    DISALLOW_NEW();

   public:
    Member<WebGLTexture> texture2d_binding_;
    Member<WebGLTexture> texture_cube_map_binding_;
    Member<WebGLTexture> texture3d_binding_;
    Member<WebGLTexture> texture2d_array_binding_;
    Member<WebGLTexture> texture_video_image_binding_;
    Member<WebGLTexture> texture_external_oes_binding_;
    Member<WebGLTexture> texture_rectangle_arb_binding_;

    void Trace(Visitor*) const;
  };

  SkAlphaType GetAlphaType() const override;
  viz::SharedImageFormat GetSharedImageFormat() const override;
  gfx::ColorSpace GetColorSpace() const override;
  scoped_refptr<StaticBitmapImage> GetImage(FlushReason) override;
  void SetHdrMetadata(const gfx::HDRMetadata& hdr_metadata) override;

  V8UnionHTMLCanvasElementOrOffscreenCanvas* getHTMLOrOffscreenCanvas() const;

  void drawingBufferStorage(GLenum sizedformat, GLsizei width, GLsizei height);

  void commit();

  ScriptPromise<IDLUndefined> makeXRCompatible(ScriptState*, ExceptionState&);
  bool IsXRCompatible() const;

  void UpdateNumberOfUserAllocatedMultisampledRenderbuffers(int delta);

  // The maximum supported size of an ArrayBuffer is the maximum size that can
  // be allocated in JavaScript. This maximum is defined by the maximum size
  // PartitionAlloc can allocate.
  // We limit the maximum size of ArrayBuffers we support to avoid integer
  // overflows in the WebGL implementation. WebGL stores the data size as
  // uint32_t, so if sizes just below uint32_t::max() were passed in, integer
  // overflows could happen. The limit defined here is (2GB-2MB), which should
  // be enough buffer to avoid integer overflow.
  // This limit should restrict the usability of WebGL2 only insignificantly, as
  // JavaScript cannot allocate bigger ArrayBuffers anyways. Only with
  // WebAssembly it is possible to allocate bigger ArrayBuffers.
  static constexpr size_t kMaximumSupportedArrayBufferSize =
      ::partition_alloc::internal::MaxDirectMapped();

 protected:
  // WebGL object types.
  friend class WebGLContextObject;
  friend class WebGLObject;
  friend class WebGLQuery;
  friend class WebGLTimerQueryEXT;
  friend class WebGLVertexArrayObjectBase;

  // Implementation helpers.
  friend class ScopedPixelLocalStorageInterrupt;
  friend class ScopedDrawingBufferBinder;
  friend class ScopedFramebufferRestorer;
  friend class ScopedTexture2DRestorer;
  friend class ScopedUnpackParametersResetRestore;

  // WebGL extensions.
  friend class EXTColorBufferFloat;
  friend class EXTColorBufferHalfFloat;
  friend class EXTDisjointTimerQuery;
  friend class EXTDisjointTimerQueryWebGL2;
  friend class EXTTextureCompressionBPTC;
  friend class EXTTextureCompressionRGTC;
  friend class OESDrawBuffersIndexed;
  friend class OESTextureFloat;
  friend class OESVertexArrayObject;
  friend class OVRMultiview2;
  friend class WebGLColorBufferFloat;
  friend class WebGLCompressedTextureASTC;
  friend class WebGLCompressedTextureETC;
  friend class WebGLCompressedTextureETC1;
  friend class WebGLCompressedTexturePVRTC;
  friend class WebGLCompressedTextureS3TC;
  friend class WebGLCompressedTextureS3TCsRGB;
  friend class WebGLDebugShaders;
  friend class WebGLDrawBuffers;
  friend class WebGLDrawInstancedBaseVertexBaseInstance;
  friend class WebGLFramebuffer;
  friend class WebGLMultiDraw;
  friend class WebGLMultiDrawCommon;
  friend class WebGLMultiDrawInstancedBaseVertexBaseInstance;
  friend class WebGLPolygonMode;
  friend class WebGLShaderPixelLocalStorage;

  WebGLRenderingContextBase(CanvasRenderingContextHost*,
                            std::unique_ptr<WebGraphicsContext3DProvider>,
                            const Platform::GraphicsInfo& graphics_info,
                            const CanvasContextCreationAttributesCore&,
                            Platform::ContextType);
  scoped_refptr<DrawingBuffer> CreateDrawingBuffer(
      std::unique_ptr<WebGraphicsContext3DProvider>,
      const Platform::GraphicsInfo& graphics_info);
  void SetupFlags();
  bool CopyRenderingResultsFromDrawingBuffer(CanvasResourceProvider*,
                                             SourceDrawingBuffer);

  // CanvasRenderingContext implementation.
  bool IsComposited() const override { return true; }
  bool UsingSwapChain() const override;
  void PageVisibilityChanged() override;
  bool PaintRenderingResultsToCanvas(SourceDrawingBuffer) override;
  bool CopyRenderingResultsToVideoFrame(
      WebGraphicsContext3DVideoFramePool*,
      SourceDrawingBuffer,
      const gfx::ColorSpace&,
      VideoFrameCopyCompletedCallback) override;

  cc::Layer* CcLayer() const override;
  void Stop() override;
  void FinalizeFrame(FlushReason) override;
  bool PushFrame() override;

  // DrawingBuffer::Client implementation.
  bool DrawingBufferClientIsBoundForDraw() override;
  void DrawingBufferClientInterruptPixelLocalStorage() override;
  void DrawingBufferClientRestorePixelLocalStorage() override;
  void DrawingBufferClientRestoreScissorTest() override;
  void DrawingBufferClientRestoreMaskAndClearValues() override;
  void DrawingBufferClientRestorePixelPackParameters() override;
  void DrawingBufferClientRestoreTexture2DBinding() override;
  void DrawingBufferClientRestoreTexture2DArrayBinding() override;
  void DrawingBufferClientRestoreTextureCubeMapBinding() override;
  void DrawingBufferClientRestoreRenderbufferBinding() override;
  void DrawingBufferClientRestoreFramebufferBinding() override;
  void DrawingBufferClientRestorePixelUnpackBufferBinding() override;
  void DrawingBufferClientRestorePixelPackBufferBinding() override;
  bool DrawingBufferClientUserAllocatedMultisampledRenderbuffers() override;
  void DrawingBufferClientForceLostContextWithAutoRecovery(
      const char* reason) override;
  void DrawingBufferClientInitializeLayer(cc::Layer* layer) override;

  // All draw calls should go through this wrapper so that various
  // bookkeeping related to compositing and preserveDrawingBuffer
  // can happen.
  template <typename Func>
  void DrawWrapper(const char* func_name,
                   CanvasPerformanceMonitor::DrawType draw_type,
                   Func draw_func) {
    if (!bound_vertex_array_object_->IsAllEnabledAttribBufferBound()) {
      SynthesizeGLError(GL_INVALID_OPERATION, func_name,
                        "no buffer is bound to enabled attribute");
      return;
    }

    ScopedRGBEmulationColorMask emulation_color_mask(this, color_mask_.data(),
                                                     drawing_buffer_.get());
    OnBeforeDrawCall(draw_type);
    draw_func();
    if (!has_been_drawn_to_) {
      // At first draw call, record
      // Canvas/OffscreenCanvas.RenderingContextDrawnTo and what the ANGLE
      // implementation is.
      has_been_drawn_to_ = true;
      RecordUKMCanvasDrawnToRenderingAPI();
      RecordANGLEImplementation();
    }
  }

  virtual void DestroyContext();
  void MarkContextChanged(ContentChangeType,
                          CanvasPerformanceMonitor::DrawType);

  void OnErrorMessage(const char*, int32_t id);

  scoped_refptr<base::SingleThreadTaskRunner> GetContextTaskRunner();

  // Query if depth_stencil buffer is supported.
  bool IsDepthStencilSupported() { return is_depth_stencil_supported_; }

  // Check if each enabled vertex attribute is bound to a buffer.
  bool ValidateRenderingState(const char*);

  // Helper function for APIs which can legally receive null objects, including
  // the bind* calls (bindBuffer, bindTexture, etc.) and useProgram. Checks that
  // the object belongs to this context and that it's not marked for deletion.
  // Returns false if the caller should return without further processing.
  // Performs a context loss check internally.
  // This returns true for null WebGLObject arguments!
  bool ValidateNullableWebGLObject(const char* function_name, WebGLObject*);

  // Validates the incoming WebGL object, which is assumed to be non-null.
  // Checks that the object belongs to this context and that it's not marked for
  // deletion. Performs a context loss check internally.
  bool ValidateWebGLObject(const char* function_name, WebGLObject*);

  // Validates the incoming WebGL program or shader, which is assumed to be
  // non-null. OpenGL ES's validation rules differ for these types of objetcts
  // compared to others. Performs a context loss check internally.
  bool ValidateWebGLProgramOrShader(const char* function_name, WebGLObject*);

  // Adds a compressed texture format.
  void AddCompressedTextureFormat(GLenum);
  void RemoveAllCompressedTextureFormats();

  // Set UNPACK_ALIGNMENT to 1, all other parameters to 0.
  virtual void ResetUnpackParameters();
  // Restore the client unpack parameters.
  virtual void RestoreUnpackParameters();

  // Draw the specified image into a new image. Used for a workaround when
  // uploading SVG images (see the caller).
  scoped_refptr<Image> DrawImageIntoBufferForTexImage(
      scoped_refptr<Image>,
      int width,
      int height,
      const char* function_name);

  // Structure for rendering to a DrawingBuffer, instead of directly
  // to the back-buffer of m_context.
  scoped_refptr<DrawingBuffer> drawing_buffer_;

  LostContextMode context_lost_mode_ = kNotLostContext;
  AutoRecoveryMethod auto_recovery_method_ = kManual;
  // Dispatches a context lost event once it is determined that one is needed.
  // This is used for synthetic, WEBGL_lose_context and real context losses. For
  // real ones, it's likely that there's no JavaScript on the stack, but that
  // might be dependent on how exactly the platform discovers that the context
  // was lost. For better portability we always defer the dispatch of the event.
  HeapTaskRunnerTimer<WebGLRenderingContextBase>
      dispatch_context_lost_event_timer_;
  bool restore_allowed_ = false;
  HeapTaskRunnerTimer<WebGLRenderingContextBase> restore_timer_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  bool destruction_in_progress_ = false;
  bool marked_canvas_dirty_;
  // For performance reasons we must separately track whether we've
  // copied WebGL's drawing buffer to the canvas's backing store, for
  // example for printing.
  bool must_paint_to_canvas_;

  // List of bound VBO's. Used to maintain info about sizes for ARRAY_BUFFER and
  // stored values for ELEMENT_ARRAY_BUFFER
  Member<WebGLBuffer> bound_array_buffer_;

  Member<WebGLVertexArrayObjectBase> default_vertex_array_object_;
  Member<WebGLVertexArrayObjectBase> bound_vertex_array_object_;
  void SetBoundVertexArrayObject(WebGLVertexArrayObjectBase*);

  enum VertexAttribValueType {
    kFloat32ArrayType,
    kInt32ArrayType,
    kUint32ArrayType,
  };

  Vector<VertexAttribValueType> vertex_attrib_type_;
  unsigned max_vertex_attribs_;
  void SetVertexAttribType(GLuint index, VertexAttribValueType);

  Member<WebGLProgram> current_program_;
  Member<WebGLFramebuffer> framebuffer_binding_;
  Member<WebGLRenderbuffer> renderbuffer_binding_;

  static bool MakeXrCompatibleSync(CanvasRenderingContextHost* host);
  static bool IsXrCompatibleFromResult(
      device::mojom::blink::XrCompatibleResult result);
  static bool DidGpuRestart(device::mojom::blink::XrCompatibleResult result);
  static XRSystem* GetXrSystemFromHost(CanvasRenderingContextHost* host);
  void MakeXrCompatibleAsync();
  void OnMakeXrCompatibleFinished(
      device::mojom::blink::XrCompatibleResult xr_compatible_result);
  void CompleteXrCompatiblePromiseIfPending(DOMExceptionCode exception_code);
  bool xr_compatible_;
  Member<ScriptPromiseResolver<IDLUndefined>> make_xr_compatible_resolver_;

  HeapVector<TextureUnitState> texture_units_;
  wtf_size_t active_texture_unit_;

  Vector<GLenum> compressed_texture_formats_;

  // Fixed-size cache of reusable resource providers for image and video
  // texImage2D calls.
  class LRUCanvasResourceProviderCache {
   public:
    enum class CacheType { kImage, kVideo };
    LRUCanvasResourceProviderCache(wtf_size_t capacity, CacheType type);
    // The pointer returned is owned by the image buffer map.
    CanvasResourceProvider* GetCanvasResourceProvider(
        gfx::Size size,
        viz::SharedImageFormat format,
        SkAlphaType alpha_type,
        const gfx::ColorSpace& color_space);

   private:
    void BubbleToFront(wtf_size_t idx);
    const wtf_size_t capacity_;
    const CacheType type_;
    Vector<std::unique_ptr<CanvasResourceProvider>> resource_providers_;
    // The returned CanvasResourceProvider may have a different format from the
    // one requested (e.g, BGRA vs RGBA). Ensure this doesn't cause cache
    // misses by recording also the requested format.
    Vector<viz::SharedImageFormat> requested_formats_;
  };
  LRUCanvasResourceProviderCache generated_image_cache_{
      4, LRUCanvasResourceProviderCache::CacheType::kImage};
  LRUCanvasResourceProviderCache generated_video_cache_{
      4, LRUCanvasResourceProviderCache::CacheType::kVideo};

  GLint max_texture_size_;
  GLint max_cube_map_texture_size_;
  GLint max3d_texture_size_;
  GLint max_array_texture_layers_;
  GLint max_renderbuffer_size_;
  std::array<GLint, 2> max_viewport_dims_;
  GLint max_texture_level_;
  GLint max_cube_map_texture_level_;
  GLint max3d_texture_level_;

  GLint max_draw_buffers_;
  GLint max_color_attachments_;
  GLenum back_draw_buffer_;
  bool draw_buffers_web_gl_requirements_checked_;
  bool draw_buffers_supported_;

  GLenum read_buffer_of_default_framebuffer_;

  GLint pack_alignment_ = 4;
  GLint unpack_alignment_ = 4;
  bool unpack_flip_y_ = false;
  bool unpack_premultiply_alpha_ = false;
  GLenum unpack_colorspace_conversion_ = GC3D_BROWSER_DEFAULT_WEBGL;
  // The following three unpack params belong to WebGL2 only.
  GLint unpack_skip_pixels_ = 0;
  GLint unpack_skip_rows_ = 0;
  GLint unpack_row_length_ = 0;

  std::array<GLfloat, 4> clear_color_;
  bool scissor_enabled_;
  std::array<GLint, 4> scissor_box_;
  GLfloat clear_depth_;
  GLint clear_stencil_;
  // State of the color mask - or the zeroth indexed color mask, if
  // OES_draw_buffers_indexed is enabled.
  std::array<GLboolean, 4> color_mask_;
  GLboolean depth_mask_;

  bool depth_enabled_;
  bool stencil_enabled_;
  GLuint stencil_mask_, stencil_mask_back_;
  GLint stencil_func_ref_,
      stencil_func_ref_back_;  // Note that these are the user specified values,
                               // not the internal clamped value.
  GLuint stencil_func_mask_, stencil_func_mask_back_;

  // WebGL 2.0 only, but putting it here saves multiple virtual functions.
  bool rasterizer_discard_enabled_;

  bool is_depth_stencil_supported_;

  bool synthesized_errors_to_console_ = true;
  int num_gl_errors_to_console_allowed_;

  wtf_size_t one_plus_max_non_default_texture_unit_ = 0;

  std::unique_ptr<Extensions3DUtil> extensions_util_;

  enum ExtensionFlags {
    kApprovedExtension = 0x00,
    // Extension that is behind the draft extensions runtime flag:
    kDraftExtension = 0x01,
    // Extension that is intended for development rather than
    // deployment time.
    kDeveloperExtension = 0x02,
  };

  class ExtensionTracker : public GarbageCollected<ExtensionTracker>,
                           public NameClient {
   public:
    explicit ExtensionTracker(ExtensionFlags flags)
        : draft_(flags & kDraftExtension),
          developer_(flags & kDeveloperExtension) {}
    ~ExtensionTracker() override = default;

    bool Draft() const { return draft_; }
    bool Developer() const { return developer_; }

    bool MatchesName(const String&) const;

    virtual WebGLExtension* GetExtension(WebGLRenderingContextBase*) = 0;
    virtual bool Supported(WebGLRenderingContextBase*) const = 0;
    virtual const char* ExtensionName() const = 0;
    virtual void LoseExtension(bool) = 0;

    // This is only used for keeping the JS wrappers of extensions alive.
    virtual WebGLExtension* GetExtensionObjectIfAlreadyEnabled() = 0;

    virtual void Trace(Visitor* visitor) const {}
    const char* NameInHeapSnapshot() const override {
      return "ExtensionTracker";
    }

   private:
    bool draft_;
    bool developer_;
  };

  template <typename T>
  class TypedExtensionTracker final : public ExtensionTracker {
   public:
    explicit TypedExtensionTracker(ExtensionFlags flags)
        : ExtensionTracker(flags) {}

    WebGLExtension* GetExtension(WebGLRenderingContextBase* context) override {
      if (!extension_) {
        extension_ = MakeGarbageCollected<T>(context);
      }

      return extension_.Get();
    }

    bool Supported(WebGLRenderingContextBase* context) const override {
      return T::Supported(context);
    }

    const char* ExtensionName() const override { return T::ExtensionName(); }

    void LoseExtension(bool force) override {
      if (extension_) {
        extension_->Lose(force);
        if (extension_->IsLost())
          extension_ = nullptr;
      }
    }

    WebGLExtension* GetExtensionObjectIfAlreadyEnabled() override {
      return extension_.Get();
    }

    void Trace(Visitor* visitor) const override {
      visitor->Trace(extension_);
      ExtensionTracker::Trace(visitor);
    }

   private:
    // ExtensionTracker holds it's own reference to the extension to ensure
    // that it is not deleted before this object's destructor is called
    Member<T> extension_;
  };

  std::array<bool, kWebGLExtensionNameCount> extension_enabled_;
  HeapVector<Member<ExtensionTracker>> extensions_;
  HashSet<String> disabled_extensions_;

  template <typename T>
  void RegisterExtension(ExtensionFlags flags = kApprovedExtension) {
    extensions_.push_back(
        MakeGarbageCollected<TypedExtensionTracker<T>>(flags));
  }

  bool ExtensionSupportedAndAllowed(const ExtensionTracker*);
  WebGLExtension* EnableExtensionIfSupported(const String& name);

  inline bool ExtensionEnabled(WebGLExtensionName name) {
    return extension_enabled_[name];
  }

  bool TimerQueryExtensionsEnabled();

  // ScopedDrawingBufferBinder is used for
  // ReadPixels/CopyTexImage2D/CopySubImage2D to read from a multisampled
  // DrawingBuffer. In this situation, we need to blit to a single sampled
  // buffer for reading, during which the bindings could be changed and need to
  // be recovered.
  //
  // It is possible for the binding operation to fail, in which case
  // the context will have been lost. Users must check the Succeeded()
  // status before proceeding.
  class ScopedDrawingBufferBinder {
    STACK_ALLOCATED();

   public:
    ScopedDrawingBufferBinder(DrawingBuffer* drawing_buffer,
                              WebGLFramebuffer* framebuffer_binding)
        : drawing_buffer_(drawing_buffer),
          read_framebuffer_binding_(framebuffer_binding),
          succeeded_(true) {
      // Commit DrawingBuffer if needed (e.g., for multisampling)
      if (!read_framebuffer_binding_ && drawing_buffer_)
        succeeded_ = drawing_buffer_->ResolveAndBindForReadAndDraw();
    }

    // Users must check this before proceeding with their logic.
    [[nodiscard]] bool Succeeded() { return succeeded_; }

    ~ScopedDrawingBufferBinder() {
      // Restore DrawingBuffer if needed
      if (!read_framebuffer_binding_ && drawing_buffer_ && succeeded_) {
        drawing_buffer_->RestoreFramebufferBindings();
      }
    }

   private:
    DrawingBuffer* drawing_buffer_;
    WebGLFramebuffer* read_framebuffer_binding_;
    bool succeeded_;
  };

  // Errors raised by synthesizeGLError() while the context is lost.
  Vector<GLenum> lost_context_errors_;
  // Other errors raised by synthesizeGLError().
  Vector<GLenum> synthetic_errors_;

  bool is_web_gl2_formats_types_added_ = false;
  bool is_web_gl2_tex_image_source_formats_types_added_ = false;
  bool is_web_gl2_internal_formats_copy_tex_image_added_ = false;
  bool is_oes_texture_float_formats_types_added_ = false;
  bool is_oes_texture_half_float_formats_types_added_ = false;
  bool is_web_gl_depth_texture_formats_types_added_ = false;
  bool is_ext_srgb_formats_types_added_ = false;
  bool is_ext_color_buffer_float_formats_added_ = false;
  bool is_ext_color_buffer_half_float_formats_added_ = false;
  bool is_ext_texture_norm16_added_ = false;

  GLenumHashSet supported_internal_formats_;
  GLenumHashSet supported_tex_image_source_internal_formats_;
  GLenumHashSet supported_internal_formats_copy_tex_image_;
  GLenumHashSet supported_formats_;
  GLenumHashSet supported_tex_image_source_formats_;
  GLenumHashSet supported_types_;
  GLenumHashSet supported_tex_image_source_types_;

  // Helpers for getParameter and others
  ScriptValue GetBooleanParameter(ScriptState*, GLenum);
  ScriptValue GetBooleanArrayParameter(ScriptState*, GLenum);
  ScriptValue GetFloatParameter(ScriptState*, GLenum);
  ScriptValue GetIntParameter(ScriptState*, GLenum);
  ScriptValue GetInt64Parameter(ScriptState*, GLenum);
  ScriptValue GetUnsignedIntParameter(ScriptState*, GLenum);
  ScriptValue GetWebGLFloatArrayParameter(ScriptState*, GLenum);
  ScriptValue GetWebGLIntArrayParameter(ScriptState*, GLenum);

  // Clear the backbuffer if it was composited since the last operation.
  // clearMask is set to the bitfield of any clear that would happen anyway at
  // this time and the function returns |CombinedClear| if that clear is now
  // unnecessary.
  enum HowToClear {
    // Skip clearing the backbuffer.
    kSkipped,
    // Clear the backbuffer.
    kJustClear,
    // Combine webgl.clear() API with the backbuffer clear, so webgl.clear()
    // doesn't have to call glClear() again.
    kCombinedClear
  };
  enum ClearCaller {
    // Caller of ClearIfComposited is a user-level draw or clear call.
    kClearCallerDrawOrClear,
    // Caller of ClearIfComposited is anything else, including
    // readbacks or copies.
    kClearCallerOther,
  };

  HowToClear ClearIfComposited(ClearCaller caller, GLbitfield clear_mask = 0);

  // Convert texture internal format.
  GLenum ConvertTexInternalFormat(GLenum internalformat, GLenum type);

  enum TexImageSourceType {
    kSourceArrayBufferView,
    kSourceImageData,
    kSourceHTMLImageElement,
    kSourceHTMLCanvasElement,
    kSourceHTMLVideoElement,
    kSourceImageBitmap,
    kSourceUnpackBuffer,
    kSourceVideoFrame,
  };

  enum TexImageFunctionType {
    kTexImage,
    kTexSubImage,
    kCopyTexImage,
    kCompressedTexImage
  };

  // This must stay in sync with WebMediaPlayer::TexImageFunctionID.
  enum TexImageFunctionID {
    kTexImage2D,
    kTexSubImage2D,
    kTexImage3D,
    kTexSubImage3D
  };

  enum TexImageDimension { kTex2D, kTex3D };

  // Parameters for all TexImage functions.
  struct TexImageParams {
    TexImageSourceType source_type = kSourceArrayBufferView;
    TexImageFunctionID function_id = kTexImage2D;
    GLenum target = 0;
    GLint level = 0;

    // The internal format for the texture to create, only applies to
    // TexImage calls.
    GLint internalformat = 0;

    // The offset into the destination in which to do the copy, only applies
    // to TexSubImage calls.
    GLint xoffset = 0;
    GLint yoffset = 0;
    GLint zoffset = 0;

    // The volume to copy. This is always specified by TexSubImage calls, and
    // is sometimes specified by TexImage calls. For TexImage calls where this
    // is not specified, it must be populated with the native size of the source
    // of the texture upload.
    std::optional<GLsizei> width;
    std::optional<GLsizei> height;
    std::optional<GLsizei> depth;

    // The border parameter, only applies to TexImage calls.
    GLint border = 0;

    // The format and type of the source texel data.
    GLenum format = 0;
    GLenum type = 0;

    // If true, then the input should be converted to premultiplied before
    // being uploaded (and if false, then the input should be converted to
    // unpremultiplied). For ImageBitmap sources, this must be set in such a way
    // that it will have no effect.
    bool unpack_premultiply_alpha = false;

    // If true, then the input should be flipped vertically before being
    // uploaded. For ImageBitmap sources, this must be set in such a way that it
    // will have no effect.
    bool unpack_flip_y = false;

    // The offset into the source from which to do the copy (the terminology
    // used here is pixels,rows,images instead of x,y,z).
    GLint unpack_skip_pixels = 0;
    GLint unpack_skip_rows = 0;
    GLint unpack_skip_images = 0;

    // Returns sub rect of the source based on `unpack_*` params above. This
    // rect is always in top-left coordinate space.
    gfx::Rect GetSourceRect(gfx::Size source_size) const;

    // Returns GrSurfaceOrigin of the destination texture for this operation.
    // Note, that textures don't have persistent orientation (e.g unlike
    // SharedImages), so this value doesn't affect the data in the texture
    // before the operation, only operation itself. This is helper because
    // everything else operates in terms of GrSurfaceOrigin.
    GrSurfaceOrigin GetDestinationOrigin() const;

    // Returns desired alpha type of the uploaded data.
    SkAlphaType GetDestinationAlphaType() const;

    // The source's height for 3D copies. If we are doing a 3D copy, then we
    // interpret the 2D source as 3D by treating it as vertical sequence of
    // images with this height.
    GLint unpack_image_height = 0;

    // If true, then the source should be converted to the unpack color space.
    bool unpack_colorspace_conversion = true;
  };

  // Populate the unpack state based on the context's current state. This is
  // virtual because some state is only tracked in WebGL 2.
  virtual void GetCurrentUnpackState(TexImageParams& params);

  // Upload `image` to the specified texture.
  void TexImageSkImage(TexImageParams params, sk_sp<SkImage> image);

  // Call the underlying Tex[Sub]Image{2D|3D} function. Always replace
  // `params.internalformat` with the result from ConvertTexInternalFormat.
  void TexImageBase(const TexImageParams& params, const void* pixels);

  // Upload `image` to the specified texture. If `allow_copy_via_gpu` is
  // true and `image` is an AcceleratedBitmapImage, then the copy may be
  // performed using TexImageViaGPU. Otherwise, the copy will be be performed
  // using TexImageSkImage.
  void TexImageStaticBitmapImage(TexImageParams params,
                                 StaticBitmapImage* image,
                                 bool allow_copy_via_gpu);
  template <typename T>
  gfx::Rect GetTextureSourceSize(T* texture_source) {
    return gfx::Rect(0, 0, texture_source->width(), texture_source->height());
  }

  template <typename T>
  bool ValidateTexImageSubRectangle(const TexImageParams& params,
                                    T* image,
                                    bool* selecting_sub_rectangle) {
    const char* function_name = GetTexImageFunctionName(params.function_id);
    DCHECK(function_name);
    DCHECK(selecting_sub_rectangle);
    if (!image) {
      // Probably indicates a failure to allocate the image.
      SynthesizeGLError(GL_OUT_OF_MEMORY, function_name, "out of memory");
      return false;
    }

    const int image_width = static_cast<int>(image->width());
    const int image_height = static_cast<int>(image->height());
    const gfx::Rect sub_rect(params.unpack_skip_pixels, params.unpack_skip_rows,
                             params.width.value_or(image_width),
                             params.height.value_or(image_height));
    const GLsizei depth = params.depth.value_or(1);
    *selecting_sub_rectangle =
        !(sub_rect.x() == 0 && sub_rect.y() == 0 &&
          sub_rect.width() == image_width && sub_rect.height() == image_height);
    // If the source image rect selects anything except the entire
    // contents of the image, assert that we're running WebGL 2.0 or
    // higher, since this should never happen for WebGL 1.0 (even though
    // the code could support it). If the image is null, that will be
    // signaled as an error later.
    DCHECK(!*selecting_sub_rectangle || IsWebGL2())
        << "subRect = (" << sub_rect.width() << " x " << sub_rect.height()
        << ") @ (" << sub_rect.x() << ", " << sub_rect.y() << "), image = ("
        << image_width << " x " << image_height << ")";

    if (sub_rect.x() < 0 || sub_rect.y() < 0 ||
        sub_rect.right() > image_width || sub_rect.bottom() > image_height ||
        sub_rect.width() < 0 || sub_rect.height() < 0) {
      SynthesizeGLError(GL_INVALID_OPERATION, function_name,
                        "source sub-rectangle specified via pixel unpack "
                        "parameters is invalid");
      return false;
    }

    if (params.function_id == kTexImage3D ||
        params.function_id == kTexSubImage3D) {
      DCHECK_GE(params.unpack_image_height, 0);

      if (depth < 1) {
        SynthesizeGLError(GL_INVALID_OPERATION, function_name,
                          "Can't define a 3D texture with depth < 1");
        return false;
      }

      // According to the WebGL 2.0 spec, specifying depth > 1 means
      // to select multiple rectangles stacked vertically.
      base::CheckedNumeric<GLint> max_y_accessed;
      if (params.unpack_image_height) {
        max_y_accessed = params.unpack_image_height;
      } else {
        max_y_accessed = sub_rect.height();
      }
      max_y_accessed *= depth - 1;
      max_y_accessed += sub_rect.height();
      max_y_accessed += sub_rect.y();

      if (!max_y_accessed.IsValid()) {
        SynthesizeGLError(GL_INVALID_OPERATION, function_name,
                          "Out-of-range parameters passed for 3D texture "
                          "upload");
        return false;
      }

      if (max_y_accessed.ValueOrDie() > image_height) {
        SynthesizeGLError(GL_INVALID_OPERATION, function_name,
                          "Not enough data supplied to upload to a 3D texture "
                          "with depth > 1");
        return false;
      }
    } else {
      DCHECK_EQ(depth, 1);
      DCHECK_EQ(params.unpack_image_height, 0);
    }
    return true;
  }

  virtual WebGLImageConversion::PixelStoreParams GetPackPixelStoreParams();
  virtual WebGLImageConversion::PixelStoreParams GetUnpackPixelStoreParams(
      TexImageDimension);

  // Helper function for copyTex{Sub}Image, check whether the internalformat
  // and the color buffer format of the current bound framebuffer combination
  // is valid.
  bool IsTexInternalFormatColorBufferCombinationValid(
      GLenum tex_internal_format,
      GLenum color_buffer_format);

  // Helper function to verify limits on the length of uniform and attribute
  // locations.
  virtual unsigned GetMaxWebGLLocationLength() const { return 256; }
  bool ValidateLocationLength(const char* function_name, const String&);

  // Helper function to check if size is non-negative.
  // Generate GL error and return false for negative inputs; otherwise, return
  // true.
  bool ValidateSize(const char* function_name, GLint x, GLint y, GLint z = 0);

  // Helper function to check if a character belongs to the ASCII subset as
  // defined in GLSL ES 1.0 spec section 3.1.
  bool ValidateCharacter(unsigned char c);

  // Helper function to check if all characters in the string belong to the
  // ASCII subset as defined in GLSL ES 1.0 spec section 3.1.
  bool ValidateString(const char* function_name, const String&);

  // Helper function to check if an identifier starts with reserved prefixes.
  bool IsPrefixReserved(const String& name);

  virtual bool ValidateShaderType(const char* function_name,
                                  GLenum shader_type);

  // Helper function to check texture binding target and texture bound to the
  // target.  Generates GL errors and returns 0 if target is invalid or texture
  // bound is null.  Otherwise, returns the texture bound to the target.
  WebGLTexture* ValidateTextureBinding(const char* function_name,
                                       GLenum target);

  // Wrapper function for validateTexture2D(3D)Binding, used in TexImageHelper
  // functions.
  virtual WebGLTexture* ValidateTexImageBinding(const TexImageParams& params);

  // Helper function to check texture 2D target and texture bound to the target.
  // Generate GL errors and return 0 if target is invalid or texture bound is
  // null.  Otherwise, return the texture bound to the target.
  // If |validate_opaque_textures| is true, the helper will also generate a GL
  // error when the texture bound to the target is opaque.
  // See https://www.w3.org/TR/webxrlayers-1/#opaque-texture for details about
  // opaque textures.
  WebGLTexture* ValidateTexture2DBinding(const char* function_name,
                                         GLenum target,
                                         bool validate_opaque_textures = false);

  void AddExtensionSupportedFormatsTypes();
  void AddExtensionSupportedFormatsTypesWebGL2();

  // Helper function to check input internalformat/format/type for functions
  // Tex{Sub}Image taking TexImageSource source data.  Generates GL error and
  // returns false if parameters are invalid.
  bool ValidateTexImageSourceFormatAndType(const TexImageParams& params);

  // Helper function to check input internalformat/format/type for functions
  // Tex{Sub}Image.  Generates GL error and returns false if parameters are
  // invalid.
  bool ValidateTexFuncFormatAndType(const TexImageParams& params);

  // Helper function to check readbuffer validity for copyTex{Sub}Image.
  // If yes, obtains the bound read framebuffer, returns true.
  // If not, generates a GL error, returns false.
  bool ValidateReadBufferAndGetInfo(
      const char* function_name,
      WebGLFramebuffer*& read_framebuffer_binding);

  // Helper function to check format/type and ArrayBuffer view type for
  // readPixels.
  // Generates INVALID_ENUM and returns false if parameters are invalid.
  // Generates INVALID_OPERATION if ArrayBuffer view type is incompatible with
  // type.
  virtual bool ValidateReadPixelsFormatAndType(GLenum format,
                                               GLenum type,
                                               DOMArrayBufferView*);

  // Helper function to check parameters of readPixels. Returns true if all
  // parameters are valid. Otherwise, generates appropriate error and returns
  // false.
  bool ValidateReadPixelsFuncParameters(GLsizei width,
                                        GLsizei height,
                                        GLenum format,
                                        GLenum type,
                                        DOMArrayBufferView*,
                                        int64_t buffer_size);

  virtual GLint GetMaxTextureLevelForTarget(GLenum target);

  // Helper function to check input level for functions {copy}Tex{Sub}Image.
  // Generates GL error and returns false if level is invalid.
  bool ValidateTexFuncLevel(const char* function_name,
                            GLenum target,
                            GLint level);

  // Helper function to check if a 64-bit value is non-negative and can fit into
  // a 32-bit integer.  Generates GL error and returns false if not.
  bool ValidateValueFitNonNegInt32(const char* function_name,
                                   const char* param_name,
                                   int64_t value);

  // Helper function for tex{Sub}Image{2|3}D to check if the input params'
  // format/type/level/target/width/height/depth/border/xoffset/yoffset/zoffset
  // are valid.  Otherwise, it would return quickly without doing other work.
  // If `source_width` and `source_height` are specified, then overwrite
  // params.width and params.height with those values before performing
  // validation.
  bool ValidateTexFunc(TexImageParams params,
                       std::optional<GLsizei> source_width,
                       std::optional<GLsizei> source_height);

  // Helper function to check input width and height for functions {copy,
  // compressed}Tex{Sub}Image.  Generates GL error and returns false if width,
  // height, or depth is invalid.
  bool ValidateTexFuncDimensions(const char* function_name,
                                 TexImageFunctionType,
                                 GLenum target,
                                 GLint level,
                                 GLsizei width,
                                 GLsizei height,
                                 GLsizei depth);

  // Helper function to check input parameters for functions
  // {copy}Tex{Sub}Image.  Generates GL error and returns false if parameters
  // are invalid.
  bool ValidateTexFuncParameters(const TexImageParams& params);

  enum NullDisposition { kNullAllowed, kNullNotAllowed, kNullNotReachable };

  // Helper function to validate that the given ArrayBufferView
  // is of the correct type and contains enough data for the texImage call.
  // Generates GL error and returns false if parameters are invalid.
  bool ValidateTexFuncData(const TexImageParams& params,
                           DOMArrayBufferView* pixels,
                           NullDisposition,
                           int64_t src_offset);

  // Helper function to validate a given texture format is settable as in
  // you can supply data to texImage2D, or call texImage2D, copyTexImage2D and
  // copyTexSubImage2D.
  // Generates GL error and returns false if the format is not settable.
  bool ValidateSettableTexFormat(const char* function_name, GLenum format);

  // Helper function to validate format for CopyTexImage.
  bool ValidateCopyTexFormat(const char* function_name, GLenum format);

  // Helper function for validating compressed texture formats.
  bool ValidateCompressedTexFormat(const char* function_name, GLenum format);

  // Helper function to validate stencil or depth func.
  bool ValidateStencilOrDepthFunc(const char* function_name, GLenum);

  // Helper function for texParameterf and texParameteri.
  void TexParameter(GLenum target,
                    GLenum pname,
                    GLfloat paramf,
                    GLint parami,
                    bool is_float);

  // Helper function to print GL errors to console.
  void PrintGLErrorToConsole(const String&);

  // Helper function to print warnings to console. Currently
  // used only to warn about use of obsolete functions.
  void PrintWarningToConsole(const String&);

  // Wrap probe::DidFireWebGLErrorOrWarning and friends, but defer inside a
  // FastCall.
  void NotifyWebGLErrorOrWarning(const String& message);
  void NotifyWebGLError(const String& error_type);
  void NotifyWebGLWarning();

  // Helper function to validate the target for checkFramebufferStatus and
  // validateFramebufferFuncParameters.
  virtual bool ValidateFramebufferTarget(GLenum target);

  // Get the framebuffer bound to given target
  virtual WebGLFramebuffer* GetFramebufferBinding(GLenum target);

  virtual WebGLFramebuffer* GetReadFramebufferBinding();

  // Helper function to validate input parameters for framebuffer functions.
  // Generate GL error if parameters are illegal.
  bool ValidateFramebufferFuncParameters(const char* function_name,
                                         GLenum target,
                                         GLenum attachment);

  // Helper function to validate blend equation mode.
  bool ValidateBlendEquation(const char* function_name, GLenum);

  // Helper function to validate blend func factors.
  bool ValidateBlendFuncFactors(const char* function_name,
                                GLenum src,
                                GLenum dst);

  // Helper function to validate WEBGL_blend_func_extended
  // factors. Needed only to make negative tests pass on the
  // validating command decoder.
  bool ValidateBlendFuncExtendedFactors(const char* function_name,
                                        GLenum src,
                                        GLenum dst);

  // Helper function to validate a GL capability.
  virtual bool ValidateCapability(const char* function_name, GLenum);

  bool ValidateUniformLocation(const char* function_name,
                               const WebGLUniformLocation* location,
                               const WebGLProgram* program) {
    const WebGLProgram* loc_program = location->Program();
    if (!loc_program) {
      SynthesizeGLError(GL_INVALID_OPERATION, function_name,
                        "location has been invalidated");
      return false;
    }
    if (loc_program != program) {
      SynthesizeGLError(GL_INVALID_OPERATION, function_name,
                        "location is not from the associated program");
      return false;
    }
    return true;
  }

  // Helper function to validate input parameters for uniform functions.
  template <typename T>
  bool ValidateUniformMatrixParameters(const char* function_name,
                                       const WebGLUniformLocation* location,
                                       GLboolean transpose,
                                       base::span<const GLfloat> v,
                                       GLsizei required_min_size,
                                       GLuint src_offset,
                                       size_t src_length,
                                       const T** out_data,
                                       GLuint* out_length) {
    *out_data = nullptr;
    *out_length = 0;
    if (v.empty()) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "no array");
      return false;
    }
    if (!base::CheckedNumeric<GLuint>(src_length).IsValid()) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name,
                        "src_length exceeds the maximum supported length");
      return false;
    }
    return ValidateUniformMatrixParameters(
        function_name, location, transpose, v.data(), v.size(),
        required_min_size, src_offset, static_cast<GLuint>(src_length),
        out_data, out_length);
  }

  template <typename T>
  bool ValidateUniformMatrixParameters(const char* function_name,
                                       const WebGLUniformLocation* location,
                                       GLboolean transpose,
                                       const T* v,
                                       size_t size,
                                       GLsizei required_min_size,
                                       GLuint src_offset,
                                       GLuint src_length,
                                       const T** out_data,
                                       GLuint* out_length) {
    *out_data = nullptr;
    *out_length = 0;
    DCHECK(size >= 0 && required_min_size > 0);
    if (!location)
      return false;
    if (!ValidateUniformLocation(function_name, location, current_program_)) {
      return false;
    }
    if (!v) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "no array");
      return false;
    }
    if (!base::CheckedNumeric<GLsizei>(size).IsValid()) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name,
                        "array exceeds the maximum supported size");
      return false;
    }
    if (transpose && !IsWebGL2()) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "transpose not FALSE");
      return false;
    }
    if (src_offset >= static_cast<GLuint>(size)) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "invalid srcOffset");
      return false;
    }
    GLsizei actual_size = static_cast<GLsizei>(size) - src_offset;
    if (src_length > 0) {
      if (src_length > static_cast<GLuint>(actual_size)) {
        SynthesizeGLError(GL_INVALID_VALUE, function_name,
                          "invalid srcOffset + srcLength");
        return false;
      }
      actual_size = src_length;
    }
    if (actual_size < required_min_size || (actual_size % required_min_size)) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "invalid size");
      return false;
    }
    // By design the command buffer has an internal (signed) 32-bit
    // limit, so ensure that the amount of data passed down to it
    // doesn't exceed what it can handle. Only integer or float typed
    // arrays can be passed into the uniform*v or uniformMatrix*v
    // functions; each has 4-byte elements.
    base::CheckedNumeric<int32_t> total_size(actual_size);
    total_size *= 4;
    // Add on a fixed constant to account for internal metadata in the
    // command buffer.
    constexpr int32_t kExtraCommandSize = 1024;
    total_size += kExtraCommandSize;
    int32_t total_size_val;
    if (!total_size.AssignIfValid(&total_size_val) ||
        static_cast<size_t>(total_size_val) >
            kMaximumSupportedArrayBufferSize) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name,
                        "size * elementSize, plus a constant, is too large");
      return false;
    }
    *out_data = UNSAFE_TODO(v + src_offset);
    *out_length = actual_size / required_min_size;
    return true;
  }

  template <typename T>
  bool ValidateUniformParameters(const char* function_name,
                                 const WebGLUniformLocation* location,
                                 base::span<const T> v,
                                 GLsizei required_min_size,
                                 GLuint src_offset,
                                 size_t src_length,
                                 const T** out_data,
                                 GLuint* out_length) {
    GLuint length;
    if (!base::CheckedNumeric<GLuint>(src_length).AssignIfValid(&length)) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name,
                        "src_length is too big");
      return false;
    }
    GLuint array_length;
    if (!base::CheckedNumeric<GLuint>(v.size()).AssignIfValid(&array_length)) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "array is too big");
      return false;
    }
    if (v.empty()) {
      SynthesizeGLError(GL_INVALID_VALUE, function_name, "no array");
      return false;
    }
    return ValidateUniformMatrixParameters(
        function_name, location, false, v.data(), array_length,
        required_min_size, src_offset, length, out_data, out_length);
  }

  // Helper function to validate the target for bufferData and
  // getBufferParameter.
  virtual bool ValidateBufferTarget(const char* function_name, GLenum target);

  // Helper function to validate the target for bufferData.
  // Return the current bound buffer to target, or 0 if the target is invalid.
  virtual WebGLBuffer* ValidateBufferDataTarget(const char* function_name,
                                                GLenum target);
  // Helper function to validate the usage for bufferData.
  virtual bool ValidateBufferDataUsage(const char* function_name, GLenum usage);

  virtual bool ValidateAndUpdateBufferBindTarget(const char* function_name,
                                                 GLenum target,
                                                 WebGLBuffer*);

  virtual void RemoveBoundBuffer(WebGLBuffer*);

  // Helper function for tex{Sub}Image2D to make sure image is ready and
  // wouldn't taint Origin.

  bool ValidateHTMLImageElement(const SecurityOrigin*,
                                const char* function_name,
                                HTMLImageElement*,
                                ExceptionState&);

  // Helper function for tex{Sub}Image2D to make sure canvas or OffscreenCanvas
  // is ready and wouldn't taint Origin.
  bool ValidateCanvasRenderingContextHost(const SecurityOrigin*,
                                          const char* function_name,
                                          CanvasRenderingContextHost*,
                                          ExceptionState&);

  // Helper function for tex{Sub}Image2D to make sure video is ready wouldn't
  // taint Origin.
  bool ValidateHTMLVideoElement(const SecurityOrigin*,
                                const char* function_name,
                                HTMLVideoElement*,
                                ExceptionState&);

  // Helper function for tex{Sub}Image2D to make sure imagebitmap is ready and
  // wouldn't taint Origin.
  bool ValidateImageBitmap(const char* function_name,
                           ImageBitmap*,
                           ExceptionState&);

  // Helper function to validate drawArrays(Instanced) calls
  bool ValidateDrawArrays(const char* function_name);

  // Helper function to validate drawElements(Instanced) calls
  bool ValidateDrawElements(const char* function_name,
                            GLenum type,
                            int64_t offset);

  // Helper function to check if the byte length of {data} fits into an integer
  // of type {T.} If so, the byte length is stored in {data_length}.
  template <typename T>
  bool ExtractDataLengthIfValid(const char* function_name,
                                MaybeShared<DOMArrayBufferView> data,
                                T* data_length) {
    if (base::CheckedNumeric<T>(data->byteLength())
            .AssignIfValid(data_length)) {
      return true;
    }
    SynthesizeGLError(GL_INVALID_VALUE, function_name,
                      "provided data exceeds the maximum supported length");
    return false;
  }

  // State updates and operations necessary before or at draw call time.
  virtual void OnBeforeDrawCall(CanvasPerformanceMonitor::DrawType);

  // Helper functions to bufferData() and bufferSubData().
  bool ValidateBufferDataBufferSize(const char* function_name, int64_t size);

  void BufferDataImpl(GLenum target,
                      int64_t size,
                      const void* data,
                      GLenum usage);
  void BufferSubDataImpl(GLenum target,
                         int64_t offset,
                         int64_t size,
                         const void* data);

  // Helper function for delete* (deleteBuffer, deleteProgram, etc) functions.
  // Return false if caller should return without further processing.
  bool DeleteObject(WebGLObject*);

  void DispatchContextLostEvent(TimerBase*);
  // Helper for restoration after context lost.
  void MaybeRestoreContext(TimerBase*);

  enum ConsoleDisplayPreference { kDisplayInConsole, kDontDisplayInConsole };

  // Reports an error to glGetError, sends a message to the JavaScript
  // console.
  void SynthesizeGLError(GLenum,
                         const char* function_name,
                         const char* description,
                         ConsoleDisplayPreference = kDisplayInConsole);
  void EmitGLWarning(const char* function, const char* reason);

  String EnsureNotNull(const String&) const;

  // Enable or disable the depth and stencil test based on the user's
  // setting and whether the current FBO has a depth and stencil
  // buffer.
  void ApplyDepthAndStencilTest();

  // Helper for enabling or disabling a capability.
  void EnableOrDisable(GLenum capability, bool enable);

  // Clamp the width and height to GL_MAX_VIEWPORT_DIMS.
  gfx::Size ClampedCanvasSize() const;

  // First time called, if EXT_draw_buffers is supported, query the value;
  // otherwise return 0.  Later, return the cached value.
  GLint MaxDrawBuffers();
  GLint MaxColorAttachments();

  void SetBackDrawBuffer(GLenum);
  void SetFramebuffer(GLenum, WebGLFramebuffer*);

  virtual void RestoreCurrentFramebuffer();
  void RestoreCurrentTexture2D();
  void RestoreCurrentTexture2DArray();
  void RestoreCurrentTextureCubeMap();

  void FindNewMaxNonDefaultTextureUnit();

  virtual void RenderbufferStorageImpl(GLenum target,
                                       GLsizei samples,
                                       GLenum internalformat,
                                       GLsizei width,
                                       GLsizei height,
                                       const char* function_name);

  friend class WebGLStateRestorer;
  friend class WebGLRenderingContextEvictionManager;

  static void ActivateContext(WebGLRenderingContextBase*);
  static void DeactivateContext(WebGLRenderingContextBase*);
  static void AddToEvictedList(WebGLRenderingContextBase*);
  static void RemoveFromEvictedList(WebGLRenderingContextBase*);
  static void RestoreEvictedContext(WebGLRenderingContextBase*);
  static void ForciblyLoseOldestContext(const String& reason);
  // Return the least recently used context's position in the active context
  // vector.  If the vector is empty, return the maximum allowed active context
  // number.
  static WebGLRenderingContextBase* OldestContext();
  static WebGLRenderingContextBase* OldestEvictedContext();

  friend class ScopedRGBEmulationColorMask;
  unsigned active_scoped_rgb_emulation_color_masks_;

  ImageBitmap* TransferToImageBitmapBase(ScriptState*, ExceptionState&);

  // Helper functions for tex(Sub)Image2D && texSubImage3D
  void TexImageHelperDOMArrayBufferView(TexImageParams params,
                                        DOMArrayBufferView*,
                                        NullDisposition,
                                        int64_t src_offset);
  void TexImageHelperImageData(TexImageParams, ImageData*);

  void TexImageHelperHTMLImageElement(const SecurityOrigin*,
                                      const TexImageParams& params,
                                      HTMLImageElement*,
                                      ExceptionState&);

  void TexImageHelperCanvasRenderingContextHost(const SecurityOrigin*,
                                                TexImageParams params,
                                                CanvasRenderingContextHost*,
                                                ExceptionState&);

  void TexImageHelperHTMLVideoElement(const SecurityOrigin*,
                                      TexImageParams,
                                      HTMLVideoElement*,
                                      ExceptionState&);

  void TexImageHelperVideoFrame(const SecurityOrigin*,
                                TexImageParams,
                                VideoFrame*,
                                ExceptionState&);

  void TexImageHelperImageBitmap(TexImageParams params,
                                 ImageBitmap*,
                                 ExceptionState&);
  static const char* GetTexImageFunctionName(TexImageFunctionID);
  static TexImageFunctionType GetTexImageFunctionType(TexImageFunctionID);
  gfx::Rect SafeGetImageSize(Image*);
  gfx::Rect GetImageDataSize(ImageData*);

  // Helper implementing readPixels for WebGL 1.0 and 2.0.
  void ReadPixelsHelper(GLint x,
                        GLint y,
                        GLsizei width,
                        GLsizei height,
                        GLenum format,
                        GLenum type,
                        DOMArrayBufferView* pixels,
                        int64_t offset);

  void RecordANGLEImplementation();

 private:
  WebGLRenderingContextBase(CanvasRenderingContextHost*,
                            scoped_refptr<base::SingleThreadTaskRunner>,
                            std::unique_ptr<WebGraphicsContext3DProvider>,
                            const Platform::GraphicsInfo& graphics_info,
                            const CanvasContextCreationAttributesCore&,
                            Platform::ContextType);
  static std::unique_ptr<WebGraphicsContext3DProvider>
  CreateContextProviderInternal(CanvasRenderingContextHost*,
                                const CanvasContextCreationAttributesCore&,
                                Platform::ContextType context_type,
                                Platform::GraphicsInfo* graphics_info);

  void TexImageHelperMediaVideoFrame(
      TexImageParams,
      WebGLTexture*,
      scoped_refptr<media::VideoFrame> media_video_frame,
      media::PaintCanvasVideoRenderer* video_renderer);

  // Copy from the source directly to texture target specified by `params` via
  // the gpu, without a read-back to system memory. Source can be an
  // AcceleratedStaticBitmapImage or WebGLRenderingContextBase.
  void TexImageViaGPU(TexImageParams,
                      AcceleratedStaticBitmapImage*,
                      WebGLRenderingContextBase*);
  bool CanUseTexImageViaGPU(const TexImageParams&);

  const Platform::ContextType context_type_;

  bool IsPaintable() const final { return GetDrawingBuffer(); }

  void HoldReferenceToDrawingBuffer(DrawingBuffer*);

  static void InitializeWebGLContextLimits(
      WebGraphicsContext3DProvider* context_provider);
  static unsigned CurrentMaxGLContexts();

  void RecordIdentifiableGLParameterDigest(GLenum pname,
                                           IdentifiableToken value);

  void RecordShaderPrecisionFormatForStudy(GLenum shader_type,
                                           GLenum precision_type,
                                           WebGLShaderPrecisionFormat* format);

  // PushFrameWithCopy will make a potential copy if the resource is accelerated
  // or a drawImage if the resource is non accelerated.
  bool PushFrameWithCopy();
  // PushFrameNoCopy will try and export the content of the DrawingBuffer as a
  // ExtenralCanvasResource.
  bool PushFrameNoCopy();

  static bool webgl_context_limits_initialized_;
  static unsigned max_active_webgl_contexts_;
  static unsigned max_active_webgl_contexts_on_worker_;

  void addProgramCompletionQuery(WebGLProgram* program, GLuint query);
  void clearProgramCompletionQueries();
  bool checkProgramCompletionQueryAvailable(WebGLProgram* program,
                                            bool* completed);
  static constexpr unsigned int kMaxProgramCompletionQueries = 128u;

  // Support for KHR_parallel_shader_compile.
  //
  // TODO(crbug.com/1474141): once a HeapLinkedHashMap is available,
  // convert these two fields to use that instead.
  HeapVector<Member<WebGLProgram>> program_completion_query_list_;
  HeapHashMap<Member<WebGLProgram>, GLuint> program_completion_query_map_;

  int number_of_user_allocated_multisampled_renderbuffers_;

  bool has_been_drawn_to_ = false;

  uint32_t number_of_context_losses_ = 0;

  // Tracks if the context has ever called glBeginPixelLocalStorageANGLE. If it
  // has, we need to start using the pixel local storage interrupt mechanism
  // when we take over the client's context.
  bool has_activated_pixel_local_storage_ = false;

  PredefinedColorSpace drawing_buffer_color_space_ =
      PredefinedColorSpace::kSRGB;
  PredefinedColorSpace unpack_color_space_ = PredefinedColorSpace::kSRGB;
};

template <>
struct DowncastTraits<WebGLRenderingContextBase> {
  static bool AllowFrom(const CanvasRenderingContext& context) {
    return context.IsWebGL();
  }
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::WebGLRenderingContextBase::TextureUnitState)

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_RENDERING_CONTEXT_BASE_H_
