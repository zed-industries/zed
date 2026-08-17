// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_EXTENSIONS_3D_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_EXTENSIONS_3D_UTIL_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/khronos/GLES2/gl2.h"

namespace gpu {
namespace gles2 {
class GLES2Interface;
}
}

namespace blink {

class PLATFORM_EXPORT Extensions3DUtil final {
  USING_FAST_MALLOC(Extensions3DUtil);

 public:
  // Creates a new Extensions3DUtil. If the passed GLES2Interface has been
  // spontaneously lost, returns null.
  static std::unique_ptr<Extensions3DUtil> Create(gpu::gles2::GLES2Interface*);
  Extensions3DUtil(const Extensions3DUtil&) = delete;
  Extensions3DUtil& operator=(const Extensions3DUtil&) = delete;
  ~Extensions3DUtil();

  bool IsValid() { return is_valid_; }

  bool SupportsExtension(const String& name);
  bool EnsureExtensionEnabled(const String& name);
  bool IsExtensionEnabled(const String& name);

  static bool CopyTextureCHROMIUMNeedsESSL3(GLenum dest_format);
  static bool CanUseCopyTextureCHROMIUM(GLenum dest_target);

 private:
  Extensions3DUtil(gpu::gles2::GLES2Interface*);
  void InitializeExtensions();

  raw_ptr<gpu::gles2::GLES2Interface, DanglingUntriaged> gl_;
  HashSet<String> enabled_extensions_;
  HashSet<String> requestable_extensions_;
  bool is_valid_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_EXTENSIONS_3D_UTIL_H_
