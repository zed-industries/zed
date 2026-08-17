// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_EMULATION_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_EMULATION_AGENT_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/platform/web_theme_engine.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/parser/parser_synchronization_policy.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/emulation.h"
#include "third_party/blink/renderer/core/loader/frame_loader_types.h"
#include "third_party/blink/renderer/core/timezone/timezone_controller.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/scheduler/public/page_scheduler.h"

namespace blink {

class DocumentLoader;
class ResourceRequest;
class WebLocalFrameImpl;
class WebViewImpl;
enum class ResourceType : uint8_t;

namespace protocol {
namespace DOM {
class RGBA;
}  // namespace DOM
}  // namespace protocol

class CORE_EXPORT InspectorEmulationAgent final
    : public InspectorBaseAgent<protocol::Emulation::Metainfo> {
 public:
  InspectorEmulationAgent(WebLocalFrameImpl*, VirtualTimeController&);
  InspectorEmulationAgent(const InspectorEmulationAgent&) = delete;
  InspectorEmulationAgent& operator=(const InspectorEmulationAgent&) = delete;
  ~InspectorEmulationAgent() override;

  // protocol::Dispatcher::EmulationCommandHandler implementation.
  protocol::Response resetPageScaleFactor() override;
  protocol::Response setPageScaleFactor(double) override;
  protocol::Response setScriptExecutionDisabled(bool value) override;
  protocol::Response setScrollbarsHidden(bool hidden) override;
  protocol::Response setDocumentCookieDisabled(bool disabled) override;
  protocol::Response setTouchEmulationEnabled(
      bool enabled,
      std::optional<int> max_touch_points) override;
  protocol::Response setEmulatedMedia(
      std::optional<String> media,
      std::unique_ptr<protocol::Array<protocol::Emulation::MediaFeature>>
          features) override;
  protocol::Response setEmulatedVisionDeficiency(const String&) override;
  protocol::Response setCPUThrottlingRate(double) override;
  protocol::Response setFocusEmulationEnabled(bool) override;
  protocol::Response setAutoDarkModeOverride(std::optional<bool>) override;
  protocol::Response setVirtualTimePolicy(
      const String& policy,
      std::optional<double> virtual_time_budget_ms,
      std::optional<int> max_virtual_time_task_starvation_count,
      std::optional<double> initial_virtual_time,
      double* virtual_time_ticks_base_ms) override;
  protocol::Response setTimezoneOverride(const String& timezone_id) override;
  protocol::Response setNavigatorOverrides(const String& platform) override;
  protocol::Response setDefaultBackgroundColorOverride(
      std::unique_ptr<protocol::DOM::RGBA>) override;
  protocol::Response setSafeAreaInsetsOverride(
      std::unique_ptr<protocol::Emulation::SafeAreaInsets> insets) override;
  protocol::Response setDeviceMetricsOverride(
      int width,
      int height,
      double device_scale_factor,
      bool mobile,
      std::optional<double> scale,
      std::optional<int> screen_width,
      std::optional<int> screen_height,
      std::optional<int> position_x,
      std::optional<int> position_y,
      std::optional<bool> dont_set_visible_size,
      std::unique_ptr<protocol::Emulation::ScreenOrientation>,
      std::unique_ptr<protocol::Page::Viewport>,
      std::unique_ptr<protocol::Emulation::DisplayFeature>,
      std::unique_ptr<protocol::Emulation::DevicePosture>) override;
  protocol::Response clearDeviceMetricsOverride() override;
  protocol::Response setHardwareConcurrencyOverride(
      int hardware_concurrency) override;
  protocol::Response setUserAgentOverride(
      const String& user_agent,
      std::optional<String> accept_language,
      std::optional<String> platform,
      std::unique_ptr<protocol::Emulation::UserAgentMetadata>
          ua_metadata_override) override;
  protocol::Response setLocaleOverride(std::optional<String>) override;
  protocol::Response setDisabledImageTypes(
      std::unique_ptr<protocol::Array<protocol::Emulation::DisabledImageType>>)
      override;
  protocol::Response setAutomationOverride(bool enabled) override;
  protocol::Response setSmallViewportHeightDifferenceOverride(
      int difference) override;

  // Automation Emulation API
  void ApplyAutomationOverride(bool& enabled) const;

  // InspectorInstrumentation API
  void ApplyAcceptLanguageOverride(String* accept_lang);
  void ApplyHardwareConcurrencyOverride(unsigned int& hardware_concurrency);
  void ApplyUserAgentOverride(String* user_agent);
  void ApplyUserAgentMetadataOverride(
      std::optional<blink::UserAgentMetadata>* ua_metadata);
  void PrepareRequest(DocumentLoader*,
                      ResourceRequest&,
                      ResourceLoaderOptions&,
                      ResourceType);
  void GetDisabledImageTypes(HashSet<String>* result);
  void WillCommitLoad(LocalFrame*, DocumentLoader*);
  void WillCreateDocumentParser(bool& force_sync_parsing);

  // InspectorBaseAgent overrides.
  protocol::Response disable() override;
  void Restore() override;
  void DidCommitLoadForLocalFrame(LocalFrame*) override;

  void Trace(Visitor*) const override;

  static AtomicString OverrideAcceptImageHeader(const HashSet<String>*);

 private:
  WebViewImpl* GetWebViewImpl();
  protocol::Response AssertPage();
  void VirtualTimeBudgetExpired();
  void InnerEnable();
  void SetSystemThemeState();

  Member<WebLocalFrameImpl> web_local_frame_;
  VirtualTimeController& virtual_time_controller_;
  base::TimeTicks virtual_time_base_ticks_;
  HeapVector<Member<DocumentLoader>> pending_document_loaders_;

  std::unique_ptr<TimeZoneController::TimeZoneOverride> timezone_override_;

  bool initial_system_forced_colors_state_;

  // Unlike other media features `forced-colors` state must be tracked outside
  // the document.
  bool forced_colors_override_ = false;

  bool enabled_ = false;

  InspectorAgentState::Bytes default_background_color_override_rgba_;
  InspectorAgentState::Boolean script_execution_disabled_;
  InspectorAgentState::Boolean scrollbars_hidden_;
  InspectorAgentState::Boolean document_cookie_disabled_;
  InspectorAgentState::Boolean touch_event_emulation_enabled_;
  InspectorAgentState::Integer max_touch_points_;
  InspectorAgentState::String emulated_media_;
  InspectorAgentState::StringMap emulated_media_features_;
  InspectorAgentState::String emulated_vision_deficiency_;
  InspectorAgentState::String navigator_platform_override_;
  InspectorAgentState::Integer hardware_concurrency_override_;
  InspectorAgentState::String user_agent_override_;
  InspectorAgentState::Bytes serialized_ua_metadata_override_;
  std::optional<blink::UserAgentMetadata> ua_metadata_override_;
  InspectorAgentState::String accept_language_override_;
  InspectorAgentState::String locale_override_;
  InspectorAgentState::Double virtual_time_budget_;
  InspectorAgentState::Double initial_virtual_time_;
  InspectorAgentState::String virtual_time_policy_;
  InspectorAgentState::Integer virtual_time_task_starvation_count_;
  InspectorAgentState::Boolean emulate_focus_;
  InspectorAgentState::Boolean emulate_auto_dark_mode_;
  InspectorAgentState::Boolean auto_dark_mode_override_;
  InspectorAgentState::String timezone_id_override_;
  InspectorAgentState::BooleanMap disabled_image_types_;
  InspectorAgentState::Double cpu_throttling_rate_;
  InspectorAgentState::Boolean automation_override_;
  InspectorAgentState::Bytes safe_area_insets_override_;
  InspectorAgentState::Double small_viewport_height_difference_override_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_EMULATION_AGENT_H_
