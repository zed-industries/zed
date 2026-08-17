/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2004, 2006, 2007, 2008, 2009, 2012 Apple Inc. All rights
 * reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PLUGIN_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PLUGIN_ELEMENT_H_

#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/create_element_flags.h"
#include "third_party/blink/renderer/core/html/html_frame_owner_element.h"
#include "v8/include/v8.h"

namespace blink {

class HTMLImageLoader;
class LayoutEmbeddedContent;
class LayoutEmbeddedObject;
enum class NamedPropertySetterResult;
class WebPluginContainerImpl;

class PluginParameters {
 public:
  PluginParameters() {}
  PluginParameters(Vector<String>& param_names, Vector<String>& param_values)
      : names_(param_names), values_(param_values) {}

  const Vector<String>& Names() const;
  const Vector<String>& Values() const;
  void AppendAttribute(const Attribute&);
  void AppendNameWithValue(const String& name, const String& value);
  void MapDataParamToSrc();

 private:
  Vector<String> names_;
  Vector<String> values_;
};

class CORE_EXPORT HTMLPlugInElement
    : public HTMLFrameOwnerElement,
      public ActiveScriptWrappable<HTMLPlugInElement> {
 public:
  ~HTMLPlugInElement() override;
  void Trace(Visitor*) const override;

  bool IsPlugin() const final { return true; }

  bool HasPendingActivity() const final;

  void SetFocused(bool, mojom::blink::FocusType) override;
  void ResetInstance();
  WebPluginContainerImpl* OwnedPlugin() const;
  bool CanProcessDrag() const;
  const String& Url() const { return url_; }

  // Public for FrameView::addPartToUpdate()
  bool NeedsPluginUpdate() const { return needs_plugin_update_; }
  void SetNeedsPluginUpdate(bool needs_plugin_update) {
    needs_plugin_update_ = needs_plugin_update;
  }
  void UpdatePlugin();

  bool ShouldAccelerate() const;

  network::ParsedPermissionsPolicy ConstructContainerPolicy() const override;

  bool IsImageType() const;
  HTMLImageLoader* ImageLoader() const { return image_loader_.Get(); }
  virtual bool UseFallbackContent() const;

  ScriptValue AnonymousNamedGetter(const AtomicString&);
  NamedPropertySetterResult AnonymousNamedSetter(const AtomicString&,
                                                 const ScriptValue&);

 protected:
  HTMLPlugInElement(const QualifiedName& tag_name,
                    Document&,
                    const CreateElementFlags);

  // Node functions:
  InsertionNotificationRequest InsertedInto(
      ContainerNode& insertion_point) override;
  void RemovedFrom(ContainerNode& insertion_point) override;
  void DidMoveToNewDocument(Document& old_document) override;
  void AttachLayoutTree(AttachContext&) override;

  // Element functions:
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;
  // HTMLFrameOwnerElement overrides:
  void DisconnectContentFrame() override;
  void NaturalSizingInfoChanged() final;

  virtual bool HasFallbackContent() const;
  // Create or update the LayoutEmbeddedContent and return it, triggering layout
  // if necessary.
  virtual LayoutEmbeddedContent* LayoutEmbeddedContentForJSBindings() const;

  LayoutEmbeddedObject* GetLayoutEmbeddedObject() const;
  bool AllowedToLoadFrameURL(const String& url);
  bool RequestObject(const PluginParameters& plugin_params);

  void DispatchErrorEvent();
  bool IsErrorplaceholder();
  void ReattachOnPluginChangeIfNeeded();

  void SetUrl(const String& url) {
    url_ = url;
    UpdateServiceTypeIfEmpty();
  }

  void SetServiceType(const String& service_type) {
    service_type_ = service_type;
    UpdateServiceTypeIfEmpty();
  }

  // Set when the current view cannot be re-used on reattach. This is the case
  // e.g. when attributes (e.g. src) change.
  void SetDisposeView() { dispose_view_ = true; }

  String service_type_;
  String url_;
  KURL loaded_url_;
  Member<HTMLImageLoader> image_loader_;
  bool is_delaying_load_event_;

 private:
  // EventTarget overrides:
  void RemoveAllEventListeners() final;

  // Node overrides:
  bool CanContainRangeEndPoint() const override { return false; }
  bool CanStartSelection() const override;
  bool WillRespondToMouseClickEvents() final;
  void DefaultEventHandler(Event&) final;
  void DetachLayoutTree(bool performing_reattach) final;
  void FinishParsingChildren() final;

  // Element overrides:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  FocusableState SupportsFocus(UpdateBehavior) const final {
    return FocusableState::kFocusable;
  }
  bool IsFocusableStyle(UpdateBehavior update_behavior =
                            UpdateBehavior::kStyleAndLayout) const final;
  bool IsKeyboardFocusableSlow(UpdateBehavior update_behavior =
                                   UpdateBehavior::kStyleAndLayout) const final;
  void DidAddUserAgentShadowRoot(ShadowRoot&) final;
  const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&) final;

  // HTMLElement overrides:
  bool HasCustomFocusLogic() const override;
  bool IsPluginElement() const final;

  // TODO(dcheng): Consider removing this, since HTMLEmbedElementLegacyCall
  // and HTMLObjectElementLegacyCall usage is extremely low.
  v8::Local<v8::Object> PluginWrapper();
  // TODO(joelhockey): Clean up PluginEmbeddedContentView and
  // OwnedEmbeddedContentView (maybe also PluginWrapper).  It would be good to
  // remove and/or rename some of these. PluginEmbeddedContentView and
  // OwnedPlugin both return the plugin that is stored as
  // HTMLFrameOwnerElement::embedded_content_view_.  However
  // PluginEmbeddedContentView will synchronously create the plugin if required
  // by calling LayoutEmbeddedContentForJSBindings.  This can cause
  // navigations, and it also means that two successive calls to
  // PluginEmbeddedContentView might not return the same result.  Possibly the
  // PluginEmbeddedContentView code can be inlined into PluginWrapper.
  WebPluginContainerImpl* PluginEmbeddedContentView() const;

  // Return any existing LayoutEmbeddedContent without triggering relayout, or 0
  // if it doesn't yet exist.
  virtual LayoutEmbeddedContent* ExistingLayoutEmbeddedContent() const = 0;
  virtual void UpdatePluginInternal() = 0;

  bool LoadPlugin(const KURL&,
                  const String& mime_type,
                  const PluginParameters& plugin_params,
                  bool use_fallback);
  // Perform checks after we have determined that a plugin will be used to
  // show the object (i.e after `AllowedToLoadObject()`).
  bool AllowedToLoadPlugin(const KURL&);
  // Perform checks based on the URL and MIME-type of the object to load.
  bool AllowedToLoadObject(const KURL&, const String& mime_type);
  void RemovePluginFromFrameView(WebPluginContainerImpl* plugin);

  enum class ObjectContentType {
    kNone,
    kImage,
    kFrame,
    kPlugin,
    kExternalPlugin,
  };
  ObjectContentType GetObjectContentType() const;

  void SetPersistedPlugin(WebPluginContainerImpl*);

  void UpdateServiceTypeIfEmpty();

  v8::Global<v8::Object> plugin_wrapper_;
  bool needs_plugin_update_;
  // Represents |layoutObject() && layoutObject()->isEmbeddedObject() &&
  // !layoutEmbeddedItem().showsUnavailablePluginIndicator()|.  We want to
  // avoid accessing |layoutObject()| in layoutObjectIsFocusable().
  bool plugin_is_available_ = false;

  // Normally the plugin is stored in
  // HTMLFrameOwnerElement::embedded_content_view. However, plugins can persist
  // even when not rendered. In order to prevent confusing code which may assume
  // that OwnedEmbeddedContentView() != null means the frame is active, we save
  // off embedded_content_view_ here while the plugin is persisting but not
  // being displayed.
  Member<WebPluginContainerImpl> persisted_plugin_;

  // True when the element has changed in such a way (new URL, for instance)
  // that we cannot re-use the old view when re-attaching.
  bool dispose_view_ = false;
};

template <>
struct DowncastTraits<HTMLPlugInElement> {
  static bool AllowFrom(const Node& node) {
    auto* html_element = DynamicTo<HTMLElement>(node);
    return html_element && AllowFrom(*html_element);
  }
  static bool AllowFrom(const HTMLElement& html_element) {
    return html_element.IsPluginElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PLUGIN_ELEMENT_H_
