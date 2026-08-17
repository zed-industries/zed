// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_GPU_GL_CALCULATOR_HELPER_H_
#define MEDIAPIPE_GPU_GL_CALCULATOR_HELPER_H_

#include <memory>
#include <type_traits>

#include "absl/base/attributes.h"
#include "absl/memory/memory.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gl_context.h"
#include "mediapipe/gpu/gpu_buffer.h"
#include "mediapipe/gpu/graph_support.h"

namespace mediapipe {

class GlTexture;
class GpuResources;
struct GpuSharedData;

using ImageFrameSharedPtr = std::shared_ptr<ImageFrame>;

// TODO: remove this and Process below, or make Process available
// on Android.
typedef std::function<void(const GlTexture& src, const GlTexture& dst)>
    RenderFunction;

// Helper class that manages OpenGL contexts and operations.
// Calculators that implement an image filter, taking one input stream of
// frames and producing one output stream of frame, should subclass
// GlSimpleCalculatorBase instead of using GlCalculatorHelper directly.
// Direct use of this class is recommended for calculators that do not fit
// that mold (e.g. calculators that combine two video streams).
class GlCalculatorHelper {
 public:
  GlCalculatorHelper();
  ~GlCalculatorHelper();

  // Call Open from the Open method of a calculator to initialize the helper.
  absl::Status Open(CalculatorContext* cc);

  // Can be used to initialize the helper outside of a calculator. Useful for
  // testing.
  void InitializeForTest(GpuResources* gpu_resources);
  ABSL_DEPRECATED("Use InitializeForTest(GpuResources)")
  void InitializeForTest(GpuSharedData* gpu_shared);

  // This method can be called from GetContract to set up the needed GPU
  // resources.
  static absl::Status UpdateContract(CalculatorContract* cc,
                                     bool request_gpu_as_optional = false);

  // This method can be called from FillExpectations to set the correct types
  // for the shared GL input side packet(s).
  ABSL_DEPRECATED("Use UpdateContract")
  static absl::Status SetupInputSidePackets(PacketTypeSet* input_side_packets);

  // Execute the provided function within the helper's GL context. On some
  // platforms, this may be run on a different thread; however, this method
  // will still wait for the function to finish executing before returning.
  // The status result from the function is passed on to the caller.
  absl::Status RunInGlContext(std::function<absl::Status(void)> gl_func);

  // Convenience version of RunInGlContext for arguments with a void result
  // type. As with the absl::Status version, this also waits for the
  // function to finish executing before returning.
  //
  // Implementation note: we cannot use a std::function<void(void)> argument
  // here, because that would break passing in a lambda that returns a status;
  // e.g.:
  //   RunInGlContext([]() -> absl::Status { ... });
  //
  // The reason is that std::function<void(...)> allows the implicit conversion
  // of a callable with any result type, as long as the argument types match.
  // As a result, the above lambda would be implicitly convertible to both
  // std::function<absl::Status(void)> and std::function<void(void)>, and
  // the invocation would be ambiguous.
  //
  // Therefore, instead of using std::function<void(void)>, we use a template
  // that only accepts arguments with a void result type.
  template <typename T,
            typename = typename std::enable_if<std::is_void<
                typename std::invoke_result<T>::type>::value>::type>
  void RunInGlContext(T f) {
    RunInGlContext([f] {
      f();
      return absl::OkStatus();
    }).IgnoreError();
  }

  // Use CreateSourceTexture and CreateDestinationTexture to set up textures
  // for input and output frames. They are not just a convenience; on platforms
  // where it is supported (iOS, for now) they take advantage of memory sharing
  // between the CPU and GPU, avoiding memory copies.

  // Gives access to an input frame as an OpenGL texture for reading (sampling).
  //
  // IMPORTANT: the returned GlTexture should be treated as a short-term view
  // into the frame (typically for the duration of a Process call). Do not store
  // it as a member in your calculator. If you need to keep a frame around,
  // store the GpuBuffer instead, and call CreateSourceTexture again on each
  // Process call.
  //
  // TODO: rename this; the use of "Create" makes this sound more expensive than
  // it is.
  GlTexture CreateSourceTexture(const GpuBuffer& pixel_buffer);
  GlTexture CreateSourceTexture(const mediapipe::Image& image);

  // Gives read access to a plane of a planar buffer.
  // The plane index is zero-based. The number of planes depends on the
  // internal format of the buffer.
  // Note: multi-plane support is not available on all platforms.
  GlTexture CreateSourceTexture(const GpuBuffer& pixel_buffer, int plane);

  // Convenience function for converting an ImageFrame to GpuBuffer and then
  // accessing it as a texture.
  // This is deprecated because: 1) it encourages the use of GlTexture as a
  // long-lived object; 2) it requires copying the ImageFrame's contents,
  // which may not always be necessary.
  //
  // WARNING: do NOT use as a destination texture which will be sent to
  // downstream calculators as it may lead to synchronization issues. The result
  // is meant to be a short-lived object, local to a single calculator and
  // single GL thread. Use `CreateDestinationTexture` instead, if you need a
  // destination texture.
  ABSL_DEPRECATED("Use `GpuBufferWithImageFrame`.")
  GlTexture CreateSourceTexture(const ImageFrame& image_frame);

  // Creates a GpuBuffer sharing ownership of image_frame. The contents of
  // image_frame should not be modified after calling this.
  GpuBuffer GpuBufferWithImageFrame(std::shared_ptr<ImageFrame> image_frame);

  // Creates a GpuBuffer copying the contents of image_frame.
  GpuBuffer GpuBufferCopyingImageFrame(const ImageFrame& image_frame);

  // Extracts GpuBuffer dimensions without creating a texture.
  ABSL_DEPRECATED("Use width and height methods on GpuBuffer instead")
  void GetGpuBufferDimensions(const GpuBuffer& pixel_buffer, int* width,
                              int* height);

  // Gives access to an OpenGL texture for writing (rendering) a new frame.
  // TODO: This should either return errors or a status.
  GlTexture CreateDestinationTexture(
      int output_width, int output_height,
      GpuBufferFormat format = GpuBufferFormat::kBGRA32);

  // Allows user provided buffers to be used as rendering destinations.
  GlTexture CreateDestinationTexture(GpuBuffer& buffer);

  // Creates a destination texture copying and uploading passed image frame.
  //
  // WARNING: mind that this functions creates a new texture every time and
  // doesn't use MediaPipe's gpu buffer pool.
  // TODO: ensure buffer pool is used when creating textures out of
  // ImageFrame.
  GlTexture CreateDestinationTexture(const ImageFrame& image_frame);

  // Creates the framebuffer for rendering. Use this when the calculator
  // needs a managed framebuffer but manages its own textures.
  void CreateFramebuffer();

  // The OpenGL name of the output framebuffer.
  GLuint framebuffer() const;

  // Binds the rendering framebuffer to a destination texture.
  // TODO: do we need an unbind method too?
  void BindFramebuffer(const GlTexture& dst);

  GlContext& GetGlContext() const { return *gl_context_; }

  std::shared_ptr<GlContext> GetSharedGlContext() const { return gl_context_; }

  GlVersion GetGlVersion() const { return gl_context_->GetGlVersion(); }

  // Check if the calculator helper has been previously initialized.
  bool Initialized() { return gpu_resources_ != nullptr; }

 private:
  void InitializeInternal(CalculatorContext* cc, GpuResources* gpu_resources);

  absl::Status RunInGlContext(std::function<absl::Status(void)> gl_func,
                              CalculatorContext* calculator_context);

  // Makes a GpuBuffer accessible as a texture in the GL context.
  GlTexture MapGpuBuffer(const GpuBuffer& gpu_buffer, GlTextureView view);

  std::shared_ptr<GlContext> gl_context_;

  GLuint framebuffer_ = 0;

  GpuResources* gpu_resources_ = nullptr;
};

// Represents an OpenGL texture, and is a 'view' into the memory pool.
// It's more like a GlTextureLock, because its main purpose (in conjunction
// with the helper) is: to manage GL sync points in the gl command queue.
//
// This class should be the main way to interface with GL memory within a single
// calculator. This is the preferred way to utilize the memory pool inside of
// the helper, because GlTexture manages efficiently releasing memory back into
// the pool. A GPU backed Image can be extracted from the underlying
// memory.
class GlTexture {
 public:
  GlTexture() : view_(std::make_shared<GlTextureView>()) {}
  ~GlTexture() = default;

  int width() const { return view_->width(); }
  int height() const { return view_->height(); }
  GLenum target() const { return view_->target(); }
  GLuint name() const { return view_->name(); }

  // Returns a buffer that can be sent to another calculator.
  // & manages sync token
  // Can be used with GpuBuffer or ImageFrame or Image
  template <typename T>
  std::unique_ptr<T> GetFrame() const;

  // Releases texture memory & manages sync token
  void Release() { view_ = std::make_shared<GlTextureView>(); }

 private:
  explicit GlTexture(GlTextureView view, GpuBuffer gpu_buffer)
      : gpu_buffer_(std::move(gpu_buffer)),
        view_(std::make_shared<GlTextureView>(std::move(view))) {}
  friend class GlCalculatorHelper;
  // We store the GpuBuffer to support GetFrame, and to ensure that the storage
  // outlives the view.
  GpuBuffer gpu_buffer_;
  std::shared_ptr<GlTextureView> view_;
};

// Returns the entry with the given tag if the collection uses tags, with the
// given index otherwise. Can be used with PacketTypeSet*, PacketSet,
// OutputStreamSet, InputStreamSet, etc.
// It would be possible to have a single version of this if we could use
// non-const references. Unfortunately, they are not allowed by the style guide.
// The const-reference version cannot work with PacketTypeSet because the Set
// method is (naturally) non-const. We could add a const_cast, but I figure
// it is better to keep const-safety and accept having two versions of the
// same thing.
template <typename T>
ABSL_DEPRECATED("Only for legacy calculators")
auto TagOrIndex(const T& collection, const std::string& tag, int index)
    -> decltype(collection.Tag(tag)) {
  return collection.UsesTags() ? collection.Tag(tag) : collection.Index(index);
}

template <typename T>
ABSL_DEPRECATED("Only for legacy calculators")
auto TagOrIndex(T* collection, const std::string& tag, int index)
    -> decltype(collection->Tag(tag)) {
  return collection->UsesTags() ? collection->Tag(tag)
                                : collection->Index(index);
}

template <typename T>
ABSL_DEPRECATED("Only for legacy calculators")
bool HasTagOrIndex(const T& collection, const std::string& tag, int index) {
  return collection.UsesTags() ? collection.HasTag(tag)
                               : index < collection.NumEntries();
}

template <typename T>
ABSL_DEPRECATED("Only for legacy calculators")
bool HasTagOrIndex(T* collection, const std::string& tag, int index) {
  return collection->UsesTags() ? collection->HasTag(tag)
                                : index < collection->NumEntries();
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_CALCULATOR_HELPER_H_
