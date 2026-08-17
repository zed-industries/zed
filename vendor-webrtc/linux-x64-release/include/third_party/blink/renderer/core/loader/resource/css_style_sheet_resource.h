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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_CSS_STYLE_SHEET_RESOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_CSS_STYLE_SHEET_RESOURCE_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/resource/text_resource.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/text_resource_decoder_options.h"
#include "third_party/blink/renderer/platform/wtf/text/text_encoding.h"

namespace blink {

class CSSParserContext;
class FetchParameters;
class KURL;
class ResourceFetcher;
class StyleSheetContents;

class CORE_EXPORT CSSStyleSheetResource final : public TextResource {
 public:
  enum class MIMETypeCheck { kStrict, kLax };

  static CSSStyleSheetResource* Fetch(FetchParameters&,
                                      ResourceFetcher*,
                                      ResourceClient*);
  static CSSStyleSheetResource* CreateForTest(const KURL&,
                                              const WTF::TextEncoding&);

  CSSStyleSheetResource(const ResourceRequest&,
                        const ResourceLoaderOptions&,
                        const TextResourceDecoderOptions&);

  ~CSSStyleSheetResource() override;
  void Trace(Visitor*) const override;
  void OnMemoryDump(WebMemoryDumpLevelOfDetail,
                    WebProcessMemoryDump*) const override;

  const String SheetText(const CSSParserContext*,
                         MIMETypeCheck = MIMETypeCheck::kStrict) const;
  StyleSheetContents* CreateParsedStyleSheetFromCache(const CSSParserContext*);
  void SaveParsedStyleSheet(StyleSheetContents*);
  network::mojom::ReferrerPolicy GetReferrerPolicy() const;

 private:
  class CSSStyleSheetResourceFactory : public ResourceFactory {
   public:
    CSSStyleSheetResourceFactory()
        : ResourceFactory(ResourceType::kCSSStyleSheet,
                          TextResourceDecoderOptions::kCSSContent) {}

    Resource* Create(
        const ResourceRequest& request,
        const ResourceLoaderOptions& options,
        const TextResourceDecoderOptions& decoder_options) const override {
      return MakeGarbageCollected<CSSStyleSheetResource>(request, options,
                                                         decoder_options);
    }
  };

  bool CanUseSheet(const CSSParserContext*, MIMETypeCheck) const;

  void SetParsedStyleSheetCache(StyleSheetContents*);
  void SetDecodedSheetText(const String&);

  void NotifyFinished() override;
  void DestroyDecodedDataIfPossible() override;
  void DestroyDecodedDataForFailedRevalidation() override;
  void UpdateDecodedSize();

  // Decoded sheet text cache is available iff loading this CSS resource is
  // successfully complete.
  String decoded_sheet_text_;

  Member<StyleSheetContents> parsed_style_sheet_cache_;
};

template <>
struct DowncastTraits<CSSStyleSheetResource> {
  static bool AllowFrom(const Resource& resource) {
    return resource.GetType() == ResourceType::kCSSStyleSheet;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_CSS_STYLE_SHEET_RESOURCE_H_
