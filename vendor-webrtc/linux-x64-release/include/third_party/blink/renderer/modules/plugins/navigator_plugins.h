// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PLUGINS_NAVIGATOR_PLUGINS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PLUGINS_NAVIGATOR_PLUGINS_H_

#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class DOMMimeTypeArray;
class DOMPluginArray;
class LocalDOMWindow;
class Navigator;

class NavigatorPlugins final : public GarbageCollected<NavigatorPlugins>,
                               public Supplement<Navigator> {
 public:
  static const char kSupplementName[];

  static NavigatorPlugins& From(Navigator&);
  static NavigatorPlugins* ToNavigatorPlugins(Navigator&);

  static DOMPluginArray* plugins(Navigator&);
  static DOMMimeTypeArray* mimeTypes(Navigator&);
  static bool pdfViewerEnabled(Navigator&);

  static bool javaEnabled(Navigator&);

  explicit NavigatorPlugins(Navigator&);

  void Trace(Visitor*) const override;

 private:
  DOMPluginArray* plugins(LocalDOMWindow*) const;
  DOMMimeTypeArray* mimeTypes(LocalDOMWindow*) const;
  bool pdfViewerEnabled(LocalDOMWindow* window) const;

  const bool should_return_fixed_plugin_data_;

  mutable Member<DOMPluginArray> plugins_;
  mutable Member<DOMMimeTypeArray> mime_types_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PLUGINS_NAVIGATOR_PLUGINS_H_
