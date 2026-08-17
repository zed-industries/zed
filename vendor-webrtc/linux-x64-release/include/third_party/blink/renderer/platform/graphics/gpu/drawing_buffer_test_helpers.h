// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_DRAWING_BUFFER_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_DRAWING_BUFFER_TEST_HELPERS_H_

#include "base/memory/raw_ptr.h"
#include "build/build_config.h"
#include "cc/test/stub_decode_cache.h"
#include "components/viz/test/test_context_provider.h"
#include "gpu/command_buffer/client/webgpu_interface.h"
#include "gpu/command_buffer/common/capabilities.h"
#include "gpu/config/gpu_feature_info.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/public/platform/web_graphics_context_3d_provider.h"
#include "third_party/blink/renderer/platform/graphics/gpu/drawing_buffer.h"
#include "third_party/blink/renderer/platform/graphics/gpu/extensions_3d_util.h"
#include "third_party/blink/renderer/platform/graphics/test/test_webgraphics_shared_image_interface_provider.h"
#include "ui/gl/gpu_preference.h"

namespace blink {

enum {
  kInitialWidth = 100,
  kInitialHeight = 100,
  kAlternateHeight = 50,
};

enum UseMultisampling {
  kDisableMultisampling,
  kEnableMultisampling,
};

class WebGraphicsContext3DProviderForTests
    : public WebGraphicsContext3DProvider {
 public:
  WebGraphicsContext3DProviderForTests(
      std::unique_ptr<gpu::gles2::GLES2Interface> gl)
      : gl_(std::move(gl)),
        test_shared_image_interface_(
            base::MakeRefCounted<gpu::TestSharedImageInterface>()) {}
  WebGraphicsContext3DProviderForTests(
      std::unique_ptr<gpu::webgpu::WebGPUInterface> webgpu)
      : webgpu_(std::move(webgpu)),
        test_shared_image_interface_(
            base::MakeRefCounted<gpu::TestSharedImageInterface>()) {}

  gpu::InterfaceBase* InterfaceBase() override { return gl_.get(); }
  gpu::gles2::GLES2Interface* ContextGL() override { return gl_.get(); }
  gpu::raster::RasterInterface* RasterInterface() override { return nullptr; }
  bool IsContextLost() override { return false; }
  GrDirectContext* GetGrContext() override { return nullptr; }
  gpu::webgpu::WebGPUInterface* WebGPUInterface() override {
    return webgpu_.get();
  }
  gpu::ContextSupport* ContextSupport() override { return nullptr; }

  bool BindToCurrentSequence() override { return false; }
  const gpu::Capabilities& GetCapabilities() const override {
    return capabilities_;
  }
  const gpu::GpuFeatureInfo& GetGpuFeatureInfo() const override {
    return gpu_feature_info_;
  }
  const WebglPreferences& GetWebglPreferences() const override {
    return webgl_preferences_;
  }
  gpu::GLHelper* GetGLHelper() override { return nullptr; }
  void SetLostContextCallback(base::RepeatingClosure) override {}
  void SetErrorMessageCallback(
      base::RepeatingCallback<void(const char*, int32_t id)>) override {}
  cc::ImageDecodeCache* ImageDecodeCache(SkColorType color_type) override {
    return &image_decode_cache_;
  }
  gpu::TestSharedImageInterface* SharedImageInterface() override {
    return test_shared_image_interface_.get();
  }
  viz::RasterContextProvider* RasterContextProvider() const override {
    return nullptr;
  }
  unsigned int GetGrGLTextureFormat(
      viz::SharedImageFormat format) const override {
    return 0;
  }

  gpu::GpuFeatureInfo& GetMutableGpuFeatureInfo() { return gpu_feature_info_; }

 private:
  cc::StubDecodeCache image_decode_cache_;
  std::unique_ptr<gpu::gles2::GLES2Interface> gl_;
  std::unique_ptr<gpu::webgpu::WebGPUInterface> webgpu_;
  gpu::Capabilities capabilities_;
  gpu::GpuFeatureInfo gpu_feature_info_;
  WebglPreferences webgl_preferences_;
  scoped_refptr<gpu::TestSharedImageInterface> test_shared_image_interface_;
};

class GLES2InterfaceForTests : public gpu::gles2::GLES2InterfaceStub,
                               public DrawingBuffer::Client {
 public:
  // GLES2InterfaceStub implementation:
  void BindTexture(GLenum target, GLuint texture) override {
    if (target == GL_TEXTURE_2D)
      state_.active_texture2d_binding = texture;
    if (target == GL_TEXTURE_2D_ARRAY) {
      state_.active_texture2darray_binding = texture;
    }
    if (target == GL_TEXTURE_CUBE_MAP)
      state_.active_texturecubemap_binding = texture;
    bound_textures_.insert(target, texture);
  }

  void BindFramebuffer(GLenum target, GLuint framebuffer) override {
    switch (target) {
      case GL_FRAMEBUFFER:
        state_.draw_framebuffer_binding = framebuffer;
        state_.read_framebuffer_binding = framebuffer;
        break;
      case GL_DRAW_FRAMEBUFFER:
        state_.draw_framebuffer_binding = framebuffer;
        break;
      case GL_READ_FRAMEBUFFER:
        state_.read_framebuffer_binding = framebuffer;
        break;
      default:
        break;
    }
  }

  void BindRenderbuffer(GLenum target, GLuint renderbuffer) override {
    state_.renderbuffer_binding = renderbuffer;
  }

  void Enable(GLenum cap) override {
    if (cap == GL_SCISSOR_TEST)
      state_.scissor_enabled = true;
  }

  void Disable(GLenum cap) override {
    if (cap == GL_SCISSOR_TEST)
      state_.scissor_enabled = false;
  }

  void ClearColor(GLfloat red,
                  GLfloat green,
                  GLfloat blue,
                  GLfloat alpha) override {
    state_.clear_color[0] = red;
    state_.clear_color[1] = green;
    state_.clear_color[2] = blue;
    state_.clear_color[3] = alpha;
  }

  void ClearDepthf(GLfloat depth) override { state_.clear_depth = depth; }

  void ClearStencil(GLint s) override { state_.clear_stencil = s; }

  void ColorMask(GLboolean red,
                 GLboolean green,
                 GLboolean blue,
                 GLboolean alpha) override {
    state_.color_mask[0] = red;
    state_.color_mask[1] = green;
    state_.color_mask[2] = blue;
    state_.color_mask[3] = alpha;
  }

  void DepthMask(GLboolean flag) override { state_.depth_mask = flag; }

  void StencilMask(GLuint mask) override { state_.stencil_mask = mask; }

  void StencilMaskSeparate(GLenum face, GLuint mask) override {
    if (face == GL_FRONT)
      state_.stencil_mask = mask;
  }

  void PixelStorei(GLenum pname, GLint param) override {
    if (pname == GL_PACK_ALIGNMENT)
      state_.pack_alignment = param;
  }

  void BindBuffer(GLenum target, GLuint buffer) override {
    switch (target) {
      case GL_PIXEL_UNPACK_BUFFER:
        state_.pixel_unpack_buffer_binding = buffer;
        break;
      case GL_PIXEL_PACK_BUFFER:
        state_.pixel_pack_buffer_binding = buffer;
        break;
      default:
        break;
    }
  }

  GLenum CheckFramebufferStatus(GLenum target) override {
    return GL_FRAMEBUFFER_COMPLETE;
  }

  void GetIntegerv(GLenum pname, GLint* value) override {
    switch (pname) {
      case GL_DRAW_FRAMEBUFFER_BINDING:
        *value = state_.draw_framebuffer_binding;
        break;
      case GL_READ_FRAMEBUFFER_BINDING:
        *value = state_.read_framebuffer_binding;
        break;
      case GL_MAX_TEXTURE_SIZE:
        *value = 1024;
        break;
      default:
        break;
    }
  }

  void TexImage2D(GLenum target,
                  GLint level,
                  GLint internalformat,
                  GLsizei width,
                  GLsizei height,
                  GLint border,
                  GLenum format,
                  GLenum type,
                  const void* pixels) override {
    if (target == GL_TEXTURE_2D && !level) {
      texture_sizes_.Set(bound_textures_.find(target)->value,
                         gfx::Size(width, height));
    }
  }

  void GenTextures(GLsizei n, GLuint* textures) override {
    static GLuint id = 1;
    for (GLsizei i = 0; i < n; ++i)
      textures[i] = id++;
  }

  MOCK_METHOD1(CreateAndTexStorage2DSharedImageCHROMIUMMock,
               void(const GLbyte*));
  GLuint CreateAndTexStorage2DSharedImageCHROMIUM(
      const GLbyte* mailbox) override {
    CreateAndTexStorage2DSharedImageCHROMIUMMock(mailbox);
    GLuint texture_id;
    GenTextures(1, &texture_id);
    last_imported_shared_image_.SetZero();
    last_imported_shared_image_.SetName(
        reinterpret_cast<const gpu::Mailbox*>(mailbox)->name);
    return texture_id;
  }

  // ImplementationBase implementation
  void GenSyncTokenCHROMIUM(GLbyte* sync_token) override {
    static gpu::CommandBufferId::Generator command_buffer_id_generator;
    gpu::SyncToken source(gpu::GPU_IO,
                          command_buffer_id_generator.GenerateNextId(), 2);
    memcpy(sync_token, &source, sizeof(source));
  }

  MOCK_METHOD1(WaitSyncTokenCHROMIUMMock, void(const GLbyte* sync_token));
  void WaitSyncTokenCHROMIUM(const GLbyte* sync_token) override {
    memcpy(&most_recently_waited_sync_token_, sync_token,
           sizeof(most_recently_waited_sync_token_));
    WaitSyncTokenCHROMIUMMock(sync_token);
  }

  // DrawingBuffer::Client implementation.
  bool DrawingBufferClientIsBoundForDraw() override {
    return !state_.draw_framebuffer_binding;
  }
  void DrawingBufferClientRestoreScissorTest() override {
    state_.scissor_enabled = saved_state_.scissor_enabled;
  }
  void DrawingBufferClientRestoreMaskAndClearValues() override {
    memcpy(state_.color_mask, saved_state_.color_mask,
           sizeof(state_.color_mask));
    state_.clear_depth = saved_state_.clear_depth;
    state_.clear_stencil = saved_state_.clear_stencil;

    memcpy(state_.clear_color, saved_state_.clear_color,
           sizeof(state_.clear_color));
    state_.depth_mask = saved_state_.depth_mask;
    state_.stencil_mask = saved_state_.stencil_mask;
  }
  void DrawingBufferClientRestorePixelPackParameters() override {
    // TODO(zmo): restore ES3 pack parameters?
    state_.pack_alignment = saved_state_.pack_alignment;
  }
  void DrawingBufferClientRestoreTexture2DBinding() override {
    state_.active_texture2d_binding = saved_state_.active_texture2d_binding;
  }
  void DrawingBufferClientRestoreTexture2DArrayBinding() override {
    state_.active_texture2darray_binding =
        saved_state_.active_texture2darray_binding;
  }
  void DrawingBufferClientRestoreTextureCubeMapBinding() override {
    state_.active_texturecubemap_binding =
        saved_state_.active_texturecubemap_binding;
  }
  void DrawingBufferClientRestoreRenderbufferBinding() override {
    state_.renderbuffer_binding = saved_state_.renderbuffer_binding;
  }
  void DrawingBufferClientRestoreFramebufferBinding() override {
    state_.draw_framebuffer_binding = saved_state_.draw_framebuffer_binding;
    state_.read_framebuffer_binding = saved_state_.read_framebuffer_binding;
  }
  void DrawingBufferClientRestorePixelUnpackBufferBinding() override {
    state_.pixel_unpack_buffer_binding =
        saved_state_.pixel_unpack_buffer_binding;
  }
  void DrawingBufferClientRestorePixelPackBufferBinding() override {
    state_.pixel_pack_buffer_binding = saved_state_.pixel_pack_buffer_binding;
  }
  bool DrawingBufferClientUserAllocatedMultisampledRenderbuffers() override {
    // Not unit tested yet. Tested with end-to-end tests.
    return false;
  }
  void DrawingBufferClientForceLostContextWithAutoRecovery(
      const char* reason) override {
    // Not unit tested yet. Tested with end-to-end tests.
  }
  void DrawingBufferClientInterruptPixelLocalStorage() override {
    // Not unit tested yet. Tested with end-to-end tests.
  }
  void DrawingBufferClientRestorePixelLocalStorage() override {
    // Not unit tested yet. Tested with end-to-end tests.
  }
  void DrawingBufferClientInitializeLayer(cc::Layer* layer) override {
    // Not unit tested yet. Tested with end-to-end tests.
  }

  // Testing methods.
  gpu::SyncToken MostRecentlyWaitedSyncToken() const {
    return most_recently_waited_sync_token_;
  }
  GLuint NextImageIdToBeCreated() const { return current_image_id_; }
  gfx::Size MostRecentlyProducedSize() const {
    return most_recently_produced_size_;
  }

  // Saves current GL state for later verification.
  void SaveState() { saved_state_ = state_; }
  void VerifyStateHasNotChangedSinceSave() const {
    for (size_t i = 0; i < 4; ++i) {
      EXPECT_EQ(state_.clear_color[0], saved_state_.clear_color[0]);
      EXPECT_EQ(state_.color_mask[0], saved_state_.color_mask[0]);
    }
    EXPECT_EQ(state_.clear_depth, saved_state_.clear_depth);
    EXPECT_EQ(state_.clear_stencil, saved_state_.clear_stencil);
    EXPECT_EQ(state_.depth_mask, saved_state_.depth_mask);
    EXPECT_EQ(state_.stencil_mask, saved_state_.stencil_mask);
    EXPECT_EQ(state_.pack_alignment, saved_state_.pack_alignment);
    EXPECT_EQ(state_.active_texture2d_binding,
              saved_state_.active_texture2d_binding);
    EXPECT_EQ(state_.active_texture2darray_binding,
              saved_state_.active_texture2darray_binding);
    EXPECT_EQ(state_.active_texturecubemap_binding,
              saved_state_.active_texturecubemap_binding);
    EXPECT_EQ(state_.renderbuffer_binding, saved_state_.renderbuffer_binding);
    EXPECT_EQ(state_.draw_framebuffer_binding,
              saved_state_.draw_framebuffer_binding);
    EXPECT_EQ(state_.read_framebuffer_binding,
              saved_state_.read_framebuffer_binding);
    EXPECT_EQ(state_.pixel_unpack_buffer_binding,
              saved_state_.pixel_unpack_buffer_binding);
    EXPECT_EQ(state_.pixel_pack_buffer_binding,
              saved_state_.pixel_pack_buffer_binding);
  }

  gpu::Mailbox* last_imported_shared_image() {
    return &last_imported_shared_image_;
  }

 private:
  // The target to use when binding a texture to a Chromium image.
#if BUILDFLAG(IS_MAC)
  static constexpr GLuint kImageCHROMIUMTarget = GC3D_TEXTURE_RECTANGLE_ARB;
#else
  static constexpr GLuint kImageCHROMIUMTarget = GL_TEXTURE_2D;
#endif

  HashMap<GLenum, GLuint> bound_textures_;

  // State tracked to verify that it is restored correctly.
  struct State {
    bool scissor_enabled = false;

    GLfloat clear_color[4] = {0, 0, 0, 0};
    GLfloat clear_depth = 0;
    GLint clear_stencil = 0;

    GLboolean color_mask[4] = {0, 0, 0, 0};
    GLboolean depth_mask = 0;
    GLuint stencil_mask = 0;

    GLint pack_alignment = 4;

    // The bound 2D texture for the active texture unit.
    GLuint active_texture2d_binding = 0;
    // The bound 2D array texture for the active texture unit.
    GLuint active_texture2darray_binding = 0;
    // The bound cube map texture for the active texture unit.
    GLuint active_texturecubemap_binding = 0;
    GLuint renderbuffer_binding = 0;
    GLuint draw_framebuffer_binding = 0;
    GLuint read_framebuffer_binding = 0;
    GLuint pixel_unpack_buffer_binding = 0;
    GLuint pixel_pack_buffer_binding = 0;
  };
  State state_;
  State saved_state_;

  gpu::SyncToken most_recently_waited_sync_token_;
  gfx::Size most_recently_produced_size_;
  GLuint current_image_id_ = 1;
  HashMap<GLuint, gfx::Size> texture_sizes_;
  HashMap<GLuint, gfx::Size> image_sizes_;
  HashMap<GLuint, GLuint> image_to_texture_map_;
  gpu::Mailbox last_imported_shared_image_;
};

class DrawingBufferForTests : public DrawingBuffer {
 public:
  static scoped_refptr<DrawingBufferForTests> Create(
      std::unique_ptr<WebGraphicsContext3DProvider> context_provider,
      std::unique_ptr<TestWebGraphicsSharedImageInterfaceProvider>
          shared_image_interface_provider_for_sw,
      const Platform::GraphicsInfo& graphics_info,
      DrawingBuffer::Client* client,
      const gfx::Size& size,
      PreserveDrawingBuffer preserve,
      UseMultisampling use_multisampling,
      bool desynchronized = false) {
    std::unique_ptr<Extensions3DUtil> extensions_util =
        Extensions3DUtil::Create(context_provider->ContextGL());
    scoped_refptr<DrawingBufferForTests> drawing_buffer =
        base::AdoptRef(new DrawingBufferForTests(
            std::move(context_provider), graphics_info,
            std::move(extensions_util), client, preserve, desynchronized));
    if (!drawing_buffer->Initialize(
            size, use_multisampling != kDisableMultisampling)) {
      drawing_buffer->BeginDestruction();
      return nullptr;
    }
    drawing_buffer->SetSharedImageInterfaceProviderForSoftwareRenderingTest(
        std::move(shared_image_interface_provider_for_sw));

    return drawing_buffer;
  }

  DrawingBufferForTests(
      std::unique_ptr<WebGraphicsContext3DProvider> context_provider,
      const Platform::GraphicsInfo& graphics_info,
      std::unique_ptr<Extensions3DUtil> extensions_util,
      DrawingBuffer::Client* client,
      PreserveDrawingBuffer preserve,
      bool desynchronized)
      : DrawingBuffer(
            std::move(context_provider),
            graphics_info,
            false /* usingSwapChain */,
            desynchronized,
            std::move(extensions_util),
            client,
            false /* discardFramebufferSupported */,
            false /* textureStorageEnabled */,
            true /* wantAlphaChannel */,
            true /* premultipliedAlpha */,
            preserve,
            kWebGL1,
            false /* wantDepth */,
            false /* wantStencil */,
            DrawingBuffer::kAllowChromiumImage /* ChromiumImageUsage */,
            PredefinedColorSpace::kSRGB,
            gl::GpuPreference::kHighPerformance),
        live_(nullptr) {}

  ~DrawingBufferForTests() override {
    if (live_)
      *live_ = false;
  }

  GLES2InterfaceForTests* ContextGLForTests() {
    return static_cast<GLES2InterfaceForTests*>(ContextGL());
  }

  gpu::TestSharedImageInterface* SharedImageInterfaceForTests() {
    return static_cast<gpu::TestSharedImageInterface*>(
        ContextProvider()->SharedImageInterface());
  }

  raw_ptr<bool> live_;

  int RecycledSoftwareResourceCount() {
    return recycled_software_resources_.size();
  }
};

}  // blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_DRAWING_BUFFER_TEST_HELPERS_H_
