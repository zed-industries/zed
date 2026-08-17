/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
    Copyright (C) 2004, 2005, 2006, 2007 Apple Inc. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_H_

#include <variant>

#include "base/containers/span.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_content.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_info.h"
#include "third_party/blink/renderer/core/loader/resource/multipart_image_resource_parser.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/shared_buffer.h"

namespace blink {

class DOMWrapperWorld;
class FetchParameters;
class ImageResourceContent;
class ResourceClient;
class ResourceFetcher;

// ImageResource implements blink::Resource interface and image-specific logic
// for loading images.
// Image-related things (blink::Image and ImageResourceObserver) are handled by
// ImageResourceContent.
// Most users should use ImageResourceContent instead of ImageResource.
// https://docs.google.com/document/d/1O-fB83mrE0B_V8gzXNqHgmRLCvstTB4MMi3RnVLr8bE/edit?usp=sharing
//
// As for the lifetimes of ImageResourceContent::m_image and m_data, see this
// document:
// https://docs.google.com/document/d/1v0yTAZ6wkqX2U_M6BNIGUJpM1s0TIw1VsqpxoL7aciY/edit?usp=sharing
class CORE_EXPORT ImageResource final
    : public Resource,
      public MultipartImageResourceParser::Client {
 public:
  // Use ImageResourceContent::Fetch() unless ImageResource is required.
  // TODO(hiroshige): Make Fetch() private.
  static ImageResource* Fetch(FetchParameters&, ResourceFetcher*);

  // TODO(hiroshige): Make Create() test-only by refactoring ImageDocument.
  static ImageResource* Create(const ResourceRequest&,
                               const DOMWrapperWorld* world);
  static ImageResource* CreateForTest(const KURL&);

  ImageResource(const ResourceRequest&,
                const ResourceLoaderOptions&,
                ImageResourceContent*);
  ~ImageResource() override;

  ImageResourceContent* GetContent();
  const ImageResourceContent* GetContent() const;

  void DidAddClient(ResourceClient*) override;

  ResourceStatus GetContentStatus() const override;

  void AllClientsAndObserversRemoved() override;

  bool CanUseCacheValidator() const override;

  scoped_refptr<const SharedBuffer> ResourceBuffer() const override;
  void NotifyStartLoad() override;
  void ResponseReceived(const ResourceResponse&) override;
  void AppendData(
      std::variant<SegmentedBuffer, base::span<const char>>) override;
  void Finish(base::TimeTicks finish_time,
              base::SingleThreadTaskRunner*) override;
  void FinishAsError(const ResourceError&,
                     base::SingleThreadTaskRunner*) override;

  // For compatibility, images keep loading even if there are HTTP errors.
  bool ShouldIgnoreHTTPStatusCodeErrors() const override { return true; }

  void UpdateResourceInfoFromObservers() override;
  std::pair<ResourcePriority, ResourcePriority> PriorityFromObservers()
      const override;
  bool HasNonDegenerateSizeForDecode() const override;

  // MultipartImageResourceParser::Client
  void OnePartInMultipartReceived(const ResourceResponse&) final;
  void MultipartDataReceived(base::span<const uint8_t> bytes) final;

  // If the ImageResource came from a user agent CSS stylesheet then we should
  // flag it so that it can persist beyond navigation.
  void FlagAsUserAgentResource();

  void OnMemoryDump(WebMemoryDumpLevelOfDetail,
                    WebProcessMemoryDump*) const override;

  void Trace(Visitor*) const override;

 private:
  enum class MultipartParsingState : uint8_t {
    kWaitingForFirstPart,
    kParsingFirstPart,
    kFinishedParsingFirstPart,
  };

  class ImageResourceInfoImpl;
  class ImageResourceFactory;

  // Only for ImageResourceInfoImpl.
  void DecodeError(bool all_data_received);
  bool IsAccessAllowed(
      ImageResourceInfo::DoesCurrentFrameHaveSingleSecurityOrigin) const;

  bool HasClientsOrObservers() const override;

  void UpdateImageAndClearBuffer();
  void UpdateImage(scoped_refptr<SharedBuffer>,
                   ImageResourceContent::UpdateImageOption,
                   bool all_data_received);

  void DestroyDecodedDataIfPossible() override;
  void DestroyDecodedDataForFailedRevalidation() override;

  void FlushImageIfNeeded();

  Member<ImageResourceContent> content_;

  Member<MultipartImageResourceParser> multipart_parser_;
  base::TimeTicks last_flush_time_;

  MultipartParsingState multipart_parsing_state_ =
      MultipartParsingState::kWaitingForFirstPart;

  bool is_during_finish_as_error_ = false;

  bool is_referenced_from_ua_stylesheet_ = false;

  bool is_pending_flushing_ = false;

  V8ExternalMemoryAccounter external_memory_accounter_;
};

template <>
struct DowncastTraits<ImageResource> {
  static bool AllowFrom(const Resource& resource) {
    return resource.GetType() == ResourceType::kImage;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_IMAGE_RESOURCE_H_
