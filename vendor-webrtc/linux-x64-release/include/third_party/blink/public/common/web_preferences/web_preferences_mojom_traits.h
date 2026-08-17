// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PREFERENCES_WEB_PREFERENCES_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PREFERENCES_WEB_PREFERENCES_MOJOM_TRAITS_H_

#include <optional>

#include "build/build_config.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "net/nqe/effective_connection_type.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/web_preferences/web_preferences.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom.h"
#include "third_party/skia/include/core/SkColor.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::WebPreferencesDataView,
                                        blink::web_pref::WebPreferences> {
  static const std::map<std::string, std::u16string>& standard_font_family_map(
      const blink::web_pref::WebPreferences& r) {
    return r.standard_font_family_map;
  }

  static const std::map<std::string, std::u16string>& fixed_font_family_map(
      const blink::web_pref::WebPreferences& r) {
    return r.fixed_font_family_map;
  }

  static const std::map<std::string, std::u16string>& serif_font_family_map(
      const blink::web_pref::WebPreferences& r) {
    return r.serif_font_family_map;
  }

  static const std::map<std::string, std::u16string>&
  sans_serif_font_family_map(const blink::web_pref::WebPreferences& r) {
    return r.sans_serif_font_family_map;
  }

  static const std::map<std::string, std::u16string>& cursive_font_family_map(
      const blink::web_pref::WebPreferences& r) {
    return r.cursive_font_family_map;
  }

  static const std::map<std::string, std::u16string>& fantasy_font_family_map(
      const blink::web_pref::WebPreferences& r) {
    return r.fantasy_font_family_map;
  }

  static const std::map<std::string, std::u16string>& math_font_family_map(
      const blink::web_pref::WebPreferences& r) {
    return r.math_font_family_map;
  }

  static uint32_t default_font_size(const blink::web_pref::WebPreferences& r) {
    return r.default_font_size;
  }

  static uint32_t default_fixed_font_size(
      const blink::web_pref::WebPreferences& r) {
    return r.default_fixed_font_size;
  }

  static uint32_t minimum_font_size(const blink::web_pref::WebPreferences& r) {
    return r.minimum_font_size;
  }

  static uint32_t minimum_logical_font_size(
      const blink::web_pref::WebPreferences& r) {
    return r.minimum_logical_font_size;
  }

  static std::string default_encoding(
      const blink::web_pref::WebPreferences& r) {
    return r.default_encoding;
  }

  static bool context_menu_on_mouse_up(
      const blink::web_pref::WebPreferences& r) {
    return r.context_menu_on_mouse_up;
  }

  static bool always_show_context_menu_on_touch(
      const blink::web_pref::WebPreferences& r) {
    return r.always_show_context_menu_on_touch;
  }

  static bool javascript_enabled(const blink::web_pref::WebPreferences& r) {
    return r.javascript_enabled;
  }

  static bool web_security_enabled(const blink::web_pref::WebPreferences& r) {
    return r.web_security_enabled;
  }

  static bool loads_images_automatically(
      const blink::web_pref::WebPreferences& r) {
    return r.loads_images_automatically;
  }

  static bool images_enabled(const blink::web_pref::WebPreferences& r) {
    return r.images_enabled;
  }

  static bool plugins_enabled(const blink::web_pref::WebPreferences& r) {
    return r.plugins_enabled;
  }

  static bool dom_paste_enabled(const blink::web_pref::WebPreferences& r) {
    return r.dom_paste_enabled;
  }

  static bool shrinks_standalone_images_to_fit(
      const blink::web_pref::WebPreferences& r) {
    return r.shrinks_standalone_images_to_fit;
  }

  static bool text_areas_are_resizable(
      const blink::web_pref::WebPreferences& r) {
    return r.text_areas_are_resizable;
  }

  static bool allow_scripts_to_close_windows(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_scripts_to_close_windows;
  }

  static bool remote_fonts_enabled(const blink::web_pref::WebPreferences& r) {
    return r.remote_fonts_enabled;
  }

  static bool javascript_can_access_clipboard(
      const blink::web_pref::WebPreferences& r) {
    return r.javascript_can_access_clipboard;
  }

  static bool dns_prefetching_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.dns_prefetching_enabled;
  }

  static bool data_saver_enabled(const blink::web_pref::WebPreferences& r) {
    return r.data_saver_enabled;
  }

  static bool local_storage_enabled(const blink::web_pref::WebPreferences& r) {
    return r.local_storage_enabled;
  }

  static bool databases_enabled(const blink::web_pref::WebPreferences& r) {
    return r.databases_enabled;
  }

  static bool tabs_to_links(const blink::web_pref::WebPreferences& r) {
    return r.tabs_to_links;
  }

  static bool disable_ipc_flooding_protection(
      const blink::web_pref::WebPreferences& r) {
    return r.disable_ipc_flooding_protection;
  }

  static bool hyperlink_auditing_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.hyperlink_auditing_enabled;
  }

  static bool allow_universal_access_from_file_urls(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_universal_access_from_file_urls;
  }

  static bool allow_file_access_from_file_urls(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_file_access_from_file_urls;
  }

  static bool webgl1_enabled(const blink::web_pref::WebPreferences& r) {
    return r.webgl1_enabled;
  }

  static bool webgl2_enabled(const blink::web_pref::WebPreferences& r) {
    return r.webgl2_enabled;
  }

  static bool pepper_3d_enabled(const blink::web_pref::WebPreferences& r) {
    return r.pepper_3d_enabled;
  }

  static bool privileged_webgl_extensions_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.privileged_webgl_extensions_enabled;
  }

  static bool webgl_errors_to_console_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.webgl_errors_to_console_enabled;
  }

  static bool hide_scrollbars(const blink::web_pref::WebPreferences& r) {
    return r.hide_scrollbars;
  }

  static bool prefers_default_scrollbar_styles(
      const blink::web_pref::WebPreferences& r) {
    return r.prefers_default_scrollbar_styles;
  }

  static bool accelerated_2d_canvas_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.accelerated_2d_canvas_enabled;
  }

  static bool canvas_2d_layers_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.canvas_2d_layers_enabled;
  }

  static bool antialiased_2d_canvas_disabled(
      const blink::web_pref::WebPreferences& r) {
    return r.antialiased_2d_canvas_disabled;
  }

  static bool antialiased_clips_2d_canvas_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.antialiased_clips_2d_canvas_enabled;
  }

  static bool accelerated_filters_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.accelerated_filters_enabled;
  }

  static bool deferred_filters_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.deferred_filters_enabled;
  }

  static bool container_culling_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.container_culling_enabled;
  }

  static bool allow_running_insecure_content(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_running_insecure_content;
  }

  static bool disable_reading_from_canvas(
      const blink::web_pref::WebPreferences& r) {
    return r.disable_reading_from_canvas;
  }

  static bool strict_mixed_content_checking(
      const blink::web_pref::WebPreferences& r) {
    return r.strict_mixed_content_checking;
  }

  static bool strict_powerful_feature_restrictions(
      const blink::web_pref::WebPreferences& r) {
    return r.strict_powerful_feature_restrictions;
  }

  static bool allow_geolocation_on_insecure_origins(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_geolocation_on_insecure_origins;
  }

  static bool strictly_block_blockable_mixed_content(
      const blink::web_pref::WebPreferences& r) {
    return r.strictly_block_blockable_mixed_content;
  }

  static bool block_mixed_plugin_content(
      const blink::web_pref::WebPreferences& r) {
    return r.block_mixed_plugin_content;
  }

  static bool password_echo_enabled(const blink::web_pref::WebPreferences& r) {
    return r.password_echo_enabled;
  }

  static bool should_clear_document_background(
      const blink::web_pref::WebPreferences& r) {
    return r.should_clear_document_background;
  }

  static bool enable_scroll_animator(const blink::web_pref::WebPreferences& r) {
    return r.enable_scroll_animator;
  }

  static bool prefers_reduced_motion(const blink::web_pref::WebPreferences& r) {
    return r.prefers_reduced_motion;
  }

  static bool prefers_reduced_transparency(
      const blink::web_pref::WebPreferences& r) {
    return r.prefers_reduced_transparency;
  }

  static bool inverted_colors(const blink::web_pref::WebPreferences& r) {
    return r.inverted_colors;
  }

  static bool touch_event_feature_detection_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.touch_event_feature_detection_enabled;
  }

  static uint32_t pointer_events_max_touch_points(
      const blink::web_pref::WebPreferences& r) {
    return r.pointer_events_max_touch_points;
  }

  static uint32_t available_pointer_types(
      const blink::web_pref::WebPreferences& r) {
    return r.available_pointer_types;
  }

  static blink::mojom::PointerType primary_pointer_type(
      const blink::web_pref::WebPreferences& r) {
    return r.primary_pointer_type;
  }

  static blink::mojom::OutputDeviceUpdateAbilityType
  output_device_update_ability_type(const blink::web_pref::WebPreferences& r) {
    return r.output_device_update_ability_type;
  }

  static uint32_t available_hover_types(
      const blink::web_pref::WebPreferences& r) {
    return r.available_hover_types;
  }

  static blink::mojom::HoverType primary_hover_type(
      const blink::web_pref::WebPreferences& r) {
    return r.primary_hover_type;
  }

  static bool dont_send_key_events_to_javascript(
      const blink::web_pref::WebPreferences& r) {
    return r.dont_send_key_events_to_javascript;
  }

  static bool barrel_button_for_drag_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.barrel_button_for_drag_enabled;
  }

  static bool sync_xhr_in_documents_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.sync_xhr_in_documents_enabled;
  }

  static bool target_blank_implies_no_opener_enabled_will_be_removed(
      const blink::web_pref::WebPreferences& r) {
    return r.target_blank_implies_no_opener_enabled_will_be_removed;
  }

  static bool allow_non_empty_navigator_plugins(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_non_empty_navigator_plugins;
  }

  static uint32_t number_of_cpu_cores(
      const blink::web_pref::WebPreferences& r) {
    return r.number_of_cpu_cores;
  }

  static blink::mojom::EditingBehavior editing_behavior(
      const blink::web_pref::WebPreferences& r) {
    return r.editing_behavior;
  }

  static bool supports_multiple_windows(
      const blink::web_pref::WebPreferences& r) {
    return r.supports_multiple_windows;
  }

  static bool viewport_enabled(const blink::web_pref::WebPreferences& r) {
    return r.viewport_enabled;
  }

  static bool viewport_meta_enabled(const blink::web_pref::WebPreferences& r) {
    return r.viewport_meta_enabled;
  }

  static bool auto_zoom_focused_editable_to_legible_scale(
      const blink::web_pref::WebPreferences& r) {
    return r.auto_zoom_focused_editable_to_legible_scale;
  }

  static bool shrinks_viewport_contents_to_fit(
      const blink::web_pref::WebPreferences& r) {
    return r.shrinks_viewport_contents_to_fit;
  }

  static blink::mojom::ViewportStyle viewport_style(
      const blink::web_pref::WebPreferences& r) {
    return r.viewport_style;
  }

  static bool smooth_scroll_for_find_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.smooth_scroll_for_find_enabled;
  }

  static bool main_frame_resizes_are_orientation_changes(
      const blink::web_pref::WebPreferences& r) {
    return r.main_frame_resizes_are_orientation_changes;
  }

  static bool initialize_at_minimum_page_scale(
      const blink::web_pref::WebPreferences& r) {
    return r.initialize_at_minimum_page_scale;
  }

  static bool smart_insert_delete_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.smart_insert_delete_enabled;
  }

  static bool spatial_navigation_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.spatial_navigation_enabled;
  }

  static const blink::mojom::V8CacheOptions& v8_cache_options(
      const blink::web_pref::WebPreferences& r) {
    return r.v8_cache_options;
  }

  static bool record_whole_document(const blink::web_pref::WebPreferences& r) {
    return r.record_whole_document;
  }

  static bool stylus_handwriting_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.stylus_handwriting_enabled;
  }

  static bool cookie_enabled(const blink::web_pref::WebPreferences& r) {
    return r.cookie_enabled;
  }

  static bool accelerated_video_decode_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.accelerated_video_decode_enabled;
  }

  static blink::mojom::ImageAnimationPolicy animation_policy(
      const blink::web_pref::WebPreferences& r) {
    return r.animation_policy;
  }

  static bool user_gesture_required_for_presentation(
      const blink::web_pref::WebPreferences& r) {
    return r.user_gesture_required_for_presentation;
  }

  static bool text_tracks_enabled(const blink::web_pref::WebPreferences& r) {
    return r.text_tracks_enabled;
  }

  static const std::string& text_track_background_color(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_background_color;
  }

  static const std::string& text_track_text_color(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_text_color;
  }

  static const std::string& text_track_text_size(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_text_size;
  }

  static const std::string& text_track_text_shadow(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_text_shadow;
  }

  static const std::string& text_track_font_family(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_font_family;
  }

  static const std::string& text_track_font_style(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_font_style;
  }

  static const std::string& text_track_font_variant(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_font_variant;
  }

  static const std::string& text_track_window_color(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_window_color;
  }

  static const std::string& text_track_window_radius(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_window_radius;
  }

  static float text_track_margin_percentage(
      const blink::web_pref::WebPreferences& r) {
    return r.text_track_margin_percentage;
  }

  static bool immersive_mode_enabled(const blink::web_pref::WebPreferences& r) {
    return r.immersive_mode_enabled;
  }

  static bool double_tap_to_zoom_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.double_tap_to_zoom_enabled;
  }

  static bool fullscreen_supported(const blink::web_pref::WebPreferences& r) {
    return r.fullscreen_supported;
  }

  static bool text_autosizing_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.text_autosizing_enabled;
  }

  static const GURL& web_app_scope(const ::blink::web_pref::WebPreferences& r) {
    return r.web_app_scope;
  }

#if BUILDFLAG(IS_ANDROID)
  static float font_scale_factor(const blink::web_pref::WebPreferences& r) {
    return r.font_scale_factor;
  }

  static int font_weight_adjustment(const blink::web_pref::WebPreferences& r) {
    return r.font_weight_adjustment;
  }

  static int text_size_contrast_factor(
      const blink::web_pref::WebPreferences& r) {
    return r.text_size_contrast_factor;
  }

  static float device_scale_adjustment(
      const blink::web_pref::WebPreferences& r) {
    return r.device_scale_adjustment;
  }

  static bool force_enable_zoom(const blink::web_pref::WebPreferences& r) {
    return r.force_enable_zoom;
  }

  static const GURL& default_video_poster_url(
      const blink::web_pref::WebPreferences& r) {
    return r.default_video_poster_url;
  }

  static bool support_deprecated_target_density_dpi(
      const blink::web_pref::WebPreferences& r) {
    return r.support_deprecated_target_density_dpi;
  }

  static bool wide_viewport_quirk(const blink::web_pref::WebPreferences& r) {
    return r.wide_viewport_quirk;
  }

  static bool use_wide_viewport(const blink::web_pref::WebPreferences& r) {
    return r.use_wide_viewport;
  }

  static bool force_zero_layout_height(
      const blink::web_pref::WebPreferences& r) {
    return r.force_zero_layout_height;
  }

  static bool viewport_meta_merge_content_quirk(
      const blink::web_pref::WebPreferences& r) {
    return r.viewport_meta_merge_content_quirk;
  }

  static bool viewport_meta_non_user_scalable_quirk(
      const blink::web_pref::WebPreferences& r) {
    return r.viewport_meta_non_user_scalable_quirk;
  }

  static bool viewport_meta_zero_values_quirk(
      const blink::web_pref::WebPreferences& r) {
    return r.viewport_meta_zero_values_quirk;
  }

  static bool clobber_user_agent_initial_scale_quirk(
      const blink::web_pref::WebPreferences& r) {
    return r.clobber_user_agent_initial_scale_quirk;
  }

  static bool ignore_main_frame_overflow_hidden_quirk(
      const blink::web_pref::WebPreferences& r) {
    return r.ignore_main_frame_overflow_hidden_quirk;
  }

  static bool report_screen_size_in_physical_pixels_quirk(
      const blink::web_pref::WebPreferences& r) {
    return r.report_screen_size_in_physical_pixels_quirk;
  }

  static bool reuse_global_for_unowned_main_frame(
      const blink::web_pref::WebPreferences& r) {
    return r.reuse_global_for_unowned_main_frame;
  }

  static bool spellcheck_enabled_by_default(
      const blink::web_pref::WebPreferences& r) {
    return r.spellcheck_enabled_by_default;
  }

  static bool video_fullscreen_orientation_lock_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.video_fullscreen_orientation_lock_enabled;
  }

  static bool video_rotate_to_fullscreen_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.video_rotate_to_fullscreen_enabled;
  }

  static bool embedded_media_experience_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.embedded_media_experience_enabled;
  }

  static bool css_hex_alpha_color_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.css_hex_alpha_color_enabled;
  }

  static bool scroll_top_left_interop_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.scroll_top_left_interop_enabled;
  }

  static bool disable_accelerated_small_canvases(
      const blink::web_pref::WebPreferences& r) {
    return r.disable_accelerated_small_canvases;
  }

  static bool long_press_link_select_text(
      const blink::web_pref::WebPreferences& r) {
    return r.long_press_link_select_text;
  }
#endif  // BUILDFLAG(IS_ANDROID)

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_FUCHSIA)
  static bool disable_webauthn(const blink::web_pref::WebPreferences& r) {
    return r.disable_webauthn;
  }
#endif  // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_FUCHSIA)

  static bool force_dark_mode_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.force_dark_mode_enabled;
  }

  static float default_minimum_page_scale_factor(
      const blink::web_pref::WebPreferences& r) {
    return r.default_minimum_page_scale_factor;
  }

  static float default_maximum_page_scale_factor(
      const blink::web_pref::WebPreferences& r) {
    return r.default_maximum_page_scale_factor;
  }

  static bool hide_download_ui(const blink::web_pref::WebPreferences& r) {
    return r.hide_download_ui;
  }

  static bool presentation_receiver(const blink::web_pref::WebPreferences& r) {
    return r.presentation_receiver;
  }

  static bool media_controls_enabled(const blink::web_pref::WebPreferences& r) {
    return r.media_controls_enabled;
  }

  static bool do_not_update_selection_on_mutating_selection_range(
      const blink::web_pref::WebPreferences& r) {
    return r.do_not_update_selection_on_mutating_selection_range;
  }

  static blink::mojom::AutoplayPolicy autoplay_policy(
      const blink::web_pref::WebPreferences& r) {
    return r.autoplay_policy;
  }

  static bool require_transient_activation_for_get_display_media(
      const blink::web_pref::WebPreferences& r) {
    return r.require_transient_activation_for_get_display_media;
  }

  static bool require_transient_activation_for_show_file_or_directory_picker(
      const blink::web_pref::WebPreferences& r) {
    return r.require_transient_activation_for_show_file_or_directory_picker;
  }

  static bool in_forced_colors(const blink::web_pref::WebPreferences& r) {
    return r.in_forced_colors;
  }

  static bool is_forced_colors_disabled(
      const blink::web_pref::WebPreferences& r) {
    return r.is_forced_colors_disabled;
  }

  static std::optional<SkColor> root_scrollbar_theme_color(
      const blink::web_pref::WebPreferences& r) {
    return r.root_scrollbar_theme_color;
  }

  static blink::mojom::PreferredColorScheme
  preferred_root_scrollbar_color_scheme(
      const blink::web_pref::WebPreferences& r) {
    return r.preferred_root_scrollbar_color_scheme;
  }

  static blink::mojom::PreferredColorScheme preferred_color_scheme(
      const blink::web_pref::WebPreferences& r) {
    return r.preferred_color_scheme;
  }

  static blink::mojom::PreferredContrast preferred_contrast(
      const blink::web_pref::WebPreferences& r) {
    return r.preferred_contrast;
  }

  static blink::mojom::EffectiveConnectionType low_priority_iframes_threshold(
      const blink::web_pref::WebPreferences& r) {
    return r.low_priority_iframes_threshold;
  }

  static bool picture_in_picture_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.picture_in_picture_enabled;
  }

  static bool translate_service_available(
      const blink::web_pref::WebPreferences& r) {
    return r.translate_service_available;
  }

  static blink::mojom::EffectiveConnectionType
  network_quality_estimator_web_holdback(
      const blink::web_pref::WebPreferences& r) {
    return r.network_quality_estimator_web_holdback;
  }

  static bool lazy_load_enabled(const blink::web_pref::WebPreferences& r) {
    return r.lazy_load_enabled;
  }

  static bool allow_mixed_content_upgrades(
      const blink::web_pref::WebPreferences& r) {
    return r.allow_mixed_content_upgrades;
  }

  static bool always_show_focus(const blink::web_pref::WebPreferences& r) {
    return r.always_show_focus;
  }

  static bool touch_drag_drop_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.touch_drag_drop_enabled;
  }

  static bool webxr_immersive_ar_allowed(
      const blink::web_pref::WebPreferences& r) {
    return r.webxr_immersive_ar_allowed;
  }

  static bool renderer_wide_named_frame_lookup(
      const blink::web_pref::WebPreferences& r) {
    return r.renderer_wide_named_frame_lookup;
  }

  static bool strict_mime_type_check_for_worker_scripts_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.strict_mime_type_check_for_worker_scripts_enabled;
  }

  static bool modal_context_menu(const blink::web_pref::WebPreferences& r) {
    return r.modal_context_menu;
  }

  static bool dynamic_safe_area_insets_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.dynamic_safe_area_insets_enabled;
  }

  static bool
  require_transient_activation_and_user_confirmation_for_subapps_api(
      const blink::web_pref::WebPreferences& r) {
    return r.subapps_apis_require_user_gesture_and_authorization;
  }

  static bool payment_request_enabled(
      const blink::web_pref::WebPreferences& r) {
    return r.payment_request_enabled;
  }

  static bool Read(blink::mojom::WebPreferencesDataView r,
                   blink::web_pref::WebPreferences* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PREFERENCES_WEB_PREFERENCES_MOJOM_TRAITS_H_
