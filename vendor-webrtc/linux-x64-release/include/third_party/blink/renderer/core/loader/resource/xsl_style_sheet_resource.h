/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2006 Samuel Weinig (sam.weinig@gmail.com)
    Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.

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

    This class provides all functionality needed for loading images, style
    sheets and html pages from the web. It has a memory cache for these objects.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_XSL_STYLE_SHEET_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_XSL_STYLE_SHEET_RESOURCE_H_

#include "third_party/blink/renderer/core/loader/resource/text_resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/text_resource_decoder_options.h"

namespace blink {

class FetchParameters;
class ResourceFetcher;

class XSLStyleSheetResource final : public TextResource {
 public:
  static XSLStyleSheetResource* FetchSynchronously(FetchParameters&,
                                                   ResourceFetcher*);
  static XSLStyleSheetResource* Fetch(FetchParameters&,
                                      ResourceFetcher*,
                                      ResourceClient*);

  XSLStyleSheetResource(const ResourceRequest&,
                        const ResourceLoaderOptions&,
                        const TextResourceDecoderOptions&);

  const String& Sheet() const { return sheet_; }

 private:
  class XSLStyleSheetResourceFactory : public ResourceFactory {
   public:
    XSLStyleSheetResourceFactory()
        : ResourceFactory(ResourceType::kXSLStyleSheet,
                          TextResourceDecoderOptions::kXMLContent) {}

    Resource* Create(
        const ResourceRequest& request,
        const ResourceLoaderOptions& options,
        const TextResourceDecoderOptions& decoder_options) const override {
      return MakeGarbageCollected<XSLStyleSheetResource>(request, options,
                                                         decoder_options);
    }
  };

  void NotifyFinished() override;

  String sheet_;
};

template <>
struct DowncastTraits<XSLStyleSheetResource> {
  static bool AllowFrom(const Resource& resource) {
    return resource.GetType() == ResourceType::kXSLStyleSheet;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_XSL_STYLE_SHEET_RESOURCE_H_
