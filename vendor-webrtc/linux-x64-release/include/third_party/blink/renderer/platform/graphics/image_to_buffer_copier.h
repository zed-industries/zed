#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_TO_BUFFER_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_TO_BUFFER_COPIER_H_

#include <memory>
#include "base/memory/raw_ptr.h"
#include "gpu/command_buffer/client/gles2_interface.h"
#include "gpu/command_buffer/client/shared_image_interface.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/gpu_memory_buffer.h"

namespace blink {

class Image;

// Supports copying an Image to a native buffer, returning a handle to the
// native buffer.
class PLATFORM_EXPORT ImageToBufferCopier {
  USING_FAST_MALLOC(ImageToBufferCopier);

 public:
  ImageToBufferCopier(gpu::gles2::GLES2Interface*,
                           gpu::SharedImageInterface*);
  ~ImageToBufferCopier();

  // SyncToken will be completed after access to the buffer is finished by
  // GPU process.
  std::pair<gfx::GpuMemoryBufferHandle, gpu::SyncToken> CopyImage(Image*);

 private:
  bool EnsureDestImage(const gfx::Size&);
  void CleanupDestImage();

  const raw_ptr<gpu::gles2::GLES2Interface> gl_;
  const raw_ptr<gpu::SharedImageInterface> sii_;
  gfx::Size dest_image_size_;
  scoped_refptr<gpu::ClientSharedImage> dest_shared_image_;

  // TODO(billorr): Add error handling for context loss or GL errors before we
  // enable this by default.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_TO_BUFFER_COPIER_H_
