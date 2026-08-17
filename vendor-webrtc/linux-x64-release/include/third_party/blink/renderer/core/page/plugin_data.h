
/*
    Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)

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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PLUGIN_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PLUGIN_DATA_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PluginInfo;

class CORE_EXPORT MimeClassInfo final : public GarbageCollected<MimeClassInfo> {
 public:
  void Trace(Visitor*) const;

  MimeClassInfo(const String& type,
                const String& description,
                PluginInfo& plugin,
                const Vector<String> extensions);

  const String& Type() const { return type_; }
  const String& Description() const { return description_; }
  const Vector<String>& Extensions() const { return extensions_; }
  const PluginInfo* Plugin() const { return plugin_.Get(); }

 private:
  friend class PluginData;

  String type_;
  String description_;
  Vector<String> extensions_;
  Member<PluginInfo> plugin_;
};

class CORE_EXPORT PluginInfo final : public GarbageCollected<PluginInfo> {
 public:
  void Trace(Visitor*) const;

  PluginInfo(const String& name,
             const String& filename,
             const String& desc,
             Color background_color,
             bool may_use_mime_handler_view);

  void AddMimeType(MimeClassInfo*);

  const HeapVector<Member<MimeClassInfo>>& Mimes() const { return mimes_; }
  const MimeClassInfo* GetMimeClassInfo(wtf_size_t index) const;
  const MimeClassInfo* GetMimeClassInfo(const String& type) const;
  wtf_size_t GetMimeClassInfoSize() const;

  const String& Name() const { return name_; }
  const String& Filename() const { return filename_; }
  const String& Description() const { return description_; }
  Color BackgroundColor() const { return background_color_; }
  bool MayUseExternalHandler() const { return may_use_external_handler_; }

 private:
  friend class MimeClassInfo;
  friend class PluginData;

  String name_;
  String filename_;
  String description_;
  Color background_color_;
  bool may_use_external_handler_;
  HeapVector<Member<MimeClassInfo>> mimes_;
};

class CORE_EXPORT PluginData final : public GarbageCollected<PluginData> {
 public:
  void Trace(Visitor*) const;

  PluginData() = default;
  PluginData(const PluginData&) = delete;
  PluginData& operator=(const PluginData&) = delete;

  const HeapVector<Member<PluginInfo>>& Plugins() const { return plugins_; }
  const HeapVector<Member<MimeClassInfo>>& Mimes() const { return mimes_; }
  void UpdatePluginList();
  void ResetPluginData();

  bool SupportsMimeType(const String& mime_type) const;
  Color PluginBackgroundColorForMimeType(const String& mime_type) const;
  bool IsExternalPluginMimeType(const String& mime_type) const;

  // refreshBrowserSidePluginCache doesn't update existent instances of
  // PluginData.
  static void RefreshBrowserSidePluginCache();

 private:
  HeapVector<Member<PluginInfo>> plugins_;
  HeapVector<Member<MimeClassInfo>> mimes_;
  bool updated_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_PLUGIN_DATA_H_
