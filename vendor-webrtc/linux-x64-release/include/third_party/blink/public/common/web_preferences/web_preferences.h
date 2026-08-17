// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PREFERENCES_WEB_PREFERENCES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PREFERENCES_WEB_PREFERENCES_H_

#include <map>
#include <string>
#include <vector>

#include "build/build_config.h"
#include "net/nqe/effective_connection_type.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-shared.h"
#include "third_party/blink/public/mojom/css/preferred_contrast.mojom-shared.h"
#include "third_party/blink/public/mojom/v8_cache_options.mojom-forward.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-shared.h"
#include "third_party/skia/include/core/SkColor.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace blink {

namespace web_pref {

using blink::mojom::EffectiveConnectionType;

// Map of ISO 15924 four-letter script code to font family.  For example,
// "Arab" to "My Arabic Font".
typedef std::map<std::string, std::u16string> ScriptFontFamilyMap;

// The ISO 15924 script code for undetermined script aka Common. It's the
// default used on WebKit's side to get/set a font setting when no script is
// specified.
BLINK_COMMON_EXPORT extern const char kCommonScript[];

// A struct for managing blink's settings.
//
// Adding new values to this class probably involves updating
// blink::WebSettings,
// browser/profiles/profile.cc, and
// content/public/common/common_param_traits_macros.h
struct BLINK_COMMON_EXPORT WebPreferences {
  ScriptFontFamilyMap standard_font_family_map;
  // The value for Osaka font should be "Osaka", not "Osaka-Mono".
  ScriptFontFamilyMap fixed_font_family_map;
  ScriptFontFamilyMap serif_font_family_map;
  ScriptFontFamilyMap sans_serif_font_family_map;
  ScriptFontFamilyMap cursive_font_family_map;
  ScriptFontFamilyMap fantasy_font_family_map;
  ScriptFontFamilyMap math_font_family_map;
  int default_font_size = 16;
  int default_fixed_font_size = 13;
  int minimum_font_size = 0;
  int minimum_logical_font_size = 6;
  std::string default_encoding = "ISO-8859-1";
  bool context_menu_on_mouse_up = BUILDFLAG(IS_WIN);
  bool javascript_enabled = true;
  bool web_security_enabled = true;
  bool loads_images_automatically = true;
  bool images_enabled = true;
  bool plugins_enabled = true;
  bool dom_paste_enabled = false;
  bool shrinks_standalone_images_to_fit = true;
  bool text_areas_are_resizable = true;
  bool allow_scripts_to_close_windows = false;
  bool remote_fonts_enabled = true;
  bool javascript_can_access_clipboard = false;
  // We don't use dns_prefetching_enabled to disable DNS prefetching.  Instead,
  // we disable the feature at a lower layer so that we catch non-WebKit uses of
  // DNS prefetch as well.
  bool dns_prefetching_enabled = true;
  // Preference to save data. When enabled, requests will contain the header
  // 'Save-Data: on'.
  bool data_saver_enabled = false;
  bool local_storage_enabled = false;
  bool databases_enabled = false;
  bool tabs_to_links = true;
  bool disable_ipc_flooding_protection = false;
  bool hyperlink_auditing_enabled = true;
  bool allow_universal_access_from_file_urls = false;
  bool allow_file_access_from_file_urls = false;
  bool webgl1_enabled = true;
  bool webgl2_enabled = true;
  bool pepper_3d_enabled = false;
  bool privileged_webgl_extensions_enabled = false;
  bool webgl_errors_to_console_enabled = true;
  bool hide_scrollbars = false;
  // If true, ignore ::-webkit-scrollbar-* CSS pseudo-elements in stylesheets
  // and use default values for `ScrollbarWidth` and `ScrollbarColor`
  // CSS properties.
  bool prefers_default_scrollbar_styles = false;
  bool accelerated_2d_canvas_enabled = false;
  bool canvas_2d_layers_enabled = false;
  bool antialiased_2d_canvas_disabled = false;
  bool antialiased_clips_2d_canvas_enabled = true;
  bool accelerated_filters_enabled = false;
  bool deferred_filters_enabled = false;
  bool container_culling_enabled = false;
  bool allow_running_insecure_content = false;
  // If true, taints all <canvas> elements, regardless of origin.
  bool disable_reading_from_canvas = false;
  // Strict mixed content checking disables both displaying and running insecure
  // mixed content, and disables embedder notifications that such content was
  // requested (thereby preventing user override).
  bool strict_mixed_content_checking = false;
  // Strict powerful feature restrictions block insecure usage of powerful
  // features (like device orientation) that we haven't yet disabled for the web
  // at large.
  bool strict_powerful_feature_restrictions = false;
  // TODO(jww): Remove when WebView no longer needs this exception.
  bool allow_geolocation_on_insecure_origins = false;
  // Disallow user opt-in for blockable mixed content.
  bool strictly_block_blockable_mixed_content = false;
  bool block_mixed_plugin_content = false;
  bool password_echo_enabled = false;
  bool should_print_backgrounds = false;
  bool should_clear_document_background = true;
  bool enable_scroll_animator = false;
  bool prefers_reduced_motion = false;
  bool prefers_reduced_transparency = false;
  bool inverted_colors = false;
  bool touch_event_feature_detection_enabled = false;
  int pointer_events_max_touch_points = 0;
  int available_pointer_types = 0;
  blink::mojom::PointerType primary_pointer_type =
      blink::mojom::PointerType::kPointerNone;
  int available_hover_types = 0;
  blink::mojom::HoverType primary_hover_type =
      blink::mojom::HoverType::kHoverNone;
  blink::mojom::OutputDeviceUpdateAbilityType
      output_device_update_ability_type =
          blink::mojom::OutputDeviceUpdateAbilityType::kFastType;
  bool dont_send_key_events_to_javascript = false;
  bool barrel_button_for_drag_enabled = false;
  bool sync_xhr_in_documents_enabled = true;
  // TODO(https://crbug.com/1163644): Remove once Chrome Apps are deprecated.
  bool target_blank_implies_no_opener_enabled_will_be_removed = true;
  // TODO(https://crbug.com/1172495): Remove once Chrome Apps are deprecated.
  bool allow_non_empty_navigator_plugins = false;
  int number_of_cpu_cores = 1;
  blink::mojom::EditingBehavior editing_behavior =
#if BUILDFLAG(IS_APPLE)
      mojom::EditingBehavior::kEditingMacBehavior;
#elif BUILDFLAG(IS_WIN)
      mojom::EditingBehavior::kEditingWindowsBehavior;
#elif BUILDFLAG(IS_ANDROID)
      mojom::EditingBehavior::kEditingAndroidBehavior;
#elif BUILDFLAG(IS_CHROMEOS)
      mojom::EditingBehavior::kEditingChromeOSBehavior;
#elif BUILDFLAG(IS_POSIX)
      mojom::EditingBehavior::kEditingUnixBehavior;
#else
      mojom::EditingBehavior::kEditingMacBehavior;
#endif

  bool supports_multiple_windows = true;
  bool viewport_enabled = false;
  bool viewport_meta_enabled = BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS);
  bool auto_zoom_focused_editable_to_legible_scale =
      BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS);

  // If true - Blink will clamp the minimum scale factor to the content width,
  // preventing zoom beyond the visible content. This is really only needed if
  // `viewport_enabled` is on.
  bool shrinks_viewport_contents_to_fit =
      BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS);

  blink::mojom::ViewportStyle viewport_style =
#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS)
      mojom::ViewportStyle::kMobile;
#else
      mojom::ViewportStyle::kDefault;
#endif
  bool always_show_context_menu_on_touch =
      !(BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS));
  bool smooth_scroll_for_find_enabled =
      BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS);
  bool main_frame_resizes_are_orientation_changes =
      BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS);
  bool initialize_at_minimum_page_scale = true;
  bool smart_insert_delete_enabled = BUILDFLAG(IS_MAC);
  bool spatial_navigation_enabled = false;
  blink::mojom::V8CacheOptions v8_cache_options =
      blink::mojom::V8CacheOptions::kDefault;
  bool record_whole_document = false;

  // If true, stylus handwriting recognition to text input will be available in
  // editable input fields which are non-password type.
  bool stylus_handwriting_enabled = false;

  // This flags corresponds to a Page's Settings' setCookieEnabled state. It
  // only controls whether or not the "document.cookie" field is properly
  // connected to the backing store, for instance if you wanted to be able to
  // define custom getters and setters from within a unique security content
  // without raising a DOM security exception.
  bool cookie_enabled = true;

  // This flag indicates whether H/W accelerated video decode is enabled.
  // Defaults to false.
  bool accelerated_video_decode_enabled = false;

  blink::mojom::ImageAnimationPolicy animation_policy =
      blink::mojom::ImageAnimationPolicy::kImageAnimationPolicyAllowed;

  bool user_gesture_required_for_presentation = true;

  bool text_tracks_enabled = false;

  // These fields specify the foreground and background color for WebVTT text
  // tracks. Their values can be any legal CSS color descriptor.
  std::string text_track_background_color;
  std::string text_track_text_color;

  // These fields specify values for CSS properties used to style WebVTT text
  // tracks.
  // Specifies CSS font-size property in percentage.
  std::string text_track_text_size;
  std::string text_track_text_shadow;
  std::string text_track_font_family;
  std::string text_track_font_style;
  // Specifies the value for CSS font-variant property.
  std::string text_track_font_variant;

  // These fields specify values for CSS properties used to style the window
  // around WebVTT text tracks.
  // Window color can be any legal CSS color descriptor.
  std::string text_track_window_color;
  // Window radius is in pixels.
  std::string text_track_window_radius;

  // Specifies the margin for WebVTT text tracks as a percentage of media
  // element height/width (for horizontal/vertical text respectively).
  // Cues will not be placed in this margin area.
  float text_track_margin_percentage = 0.0f;

  bool immersive_mode_enabled = false;

  bool double_tap_to_zoom_enabled =
      BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_APPLE);

  bool fullscreen_supported = true;

  bool text_autosizing_enabled = BUILDFLAG(IS_ANDROID);

  // Representation of the Web App Manifest scope if any.
  GURL web_app_scope;

#if BUILDFLAG(IS_ANDROID)
  float font_scale_factor = 1.0f;
  int font_weight_adjustment = 0;
  int text_size_contrast_factor = 0;
  float device_scale_adjustment = 1.0f;
  bool force_enable_zoom = false;
  GURL default_video_poster_url;
  bool support_deprecated_target_density_dpi = false;
  bool wide_viewport_quirk = false;
  bool use_wide_viewport = true;
  bool force_zero_layout_height = false;
  bool viewport_meta_merge_content_quirk = false;
  bool viewport_meta_non_user_scalable_quirk = false;
  bool viewport_meta_zero_values_quirk = false;
  bool clobber_user_agent_initial_scale_quirk = false;
  bool ignore_main_frame_overflow_hidden_quirk = false;
  bool report_screen_size_in_physical_pixels_quirk = false;
  // Used by Android_WebView only to support legacy apps that inject script into
  // a top-level initial empty document and expect it to persist on navigation.
  bool reuse_global_for_unowned_main_frame = false;
  // Specifies default setting for spellcheck when the spellcheck attribute is
  // not explicitly specified.
  bool spellcheck_enabled_by_default = true;
  // If enabled, when a video goes fullscreen, the orientation should be locked.
  bool video_fullscreen_orientation_lock_enabled = false;
  // If enabled, fullscreen should be entered/exited when the device is rotated
  // to/from the orientation of the video.
  bool video_rotate_to_fullscreen_enabled = false;
  bool embedded_media_experience_enabled = false;
  // Enable 8 (#RRGGBBAA) and 4 (#RGBA) value hex colors in CSS Android
  // WebView quirk (http://crbug.com/618472).
  bool css_hex_alpha_color_enabled = true;
  // Enable support for document.scrollingElement
  // WebView sets this to false to retain old documentElement behaviour
  // (http://crbug.com/761016).
  bool scroll_top_left_interop_enabled = true;

  // Don't accelerate small canvases to avoid crashes TODO(crbug.com/1004304)
  bool disable_accelerated_small_canvases = false;

  // Long press on links selects text instead of triggering context menu.
  bool long_press_link_select_text = false;
#endif  // BUILDFLAG(IS_ANDROID)

// TODO(crbug.com/1284805): Remove IS_ANDROID once WebView supports WebAuthn.
// TODO(crbug.com/1382970): Remove IS_FUCHSIA and merge with the block above
// once all Content embedders on Fuchsia support WebAuthn.
#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_FUCHSIA)
  // Disable the Web Authentication API.
  bool disable_webauthn = false;
#endif  // BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_FUCHSIA)

  // Enable forcibly modifying content rendering to result in a light on dark
  // color scheme.
  bool force_dark_mode_enabled = false;

  // Default (used if the page or UA doesn't override these) values for page
  // scale limits. These are set directly on the WebView so there's no analogue
  // in WebSettings.
#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_IOS)
  float default_minimum_page_scale_factor = 0.25f;
  float default_maximum_page_scale_factor = 5.f;
#elif BUILDFLAG(IS_MAC)
  float default_minimum_page_scale_factor = 1.f;
  float default_maximum_page_scale_factor = 3.f;
#else
  float default_minimum_page_scale_factor = 1.f;
  float default_maximum_page_scale_factor = 4.f;
#endif

  // Whether download UI should be hidden on this page.
  bool hide_download_ui = false;

  // Whether it is a presentation receiver.
  bool presentation_receiver = false;

  // If disabled, media controls should never be used.
  bool media_controls_enabled = true;

  // Whether we want to disable updating selection on mutating selection range.
  // This is to work around Samsung's email app issue. See
  // https://crbug.com/699943 for details.
  // TODO(changwan): remove this once we no longer support Android N.
  bool do_not_update_selection_on_mutating_selection_range = false;

  // Defines the current autoplay policy.
  blink::mojom::AutoplayPolicy autoplay_policy =
      blink::mojom::AutoplayPolicy::kDocumentUserActivationRequired;

  // `getDisplayMedia()`'s transient activation requirement can be bypassed via
  // `ScreenCaptureWithoutGestureAllowedForOrigins` policy.
  bool require_transient_activation_for_get_display_media = true;

  // `show{OpenFile|SaveFile|Directory}Picker()`'s transient activation
  // requirement can be bypassed via
  // `FileOrDirectoryPickerWithoutGestureAllowedForOrigins` policy.
  bool require_transient_activation_for_show_file_or_directory_picker = true;

  // `navigator.subApps.{add|remove|list}()`'s user gesture and authorization
  // can be bypassed via
  // `SubAppsAPIsAllowedWithoutGestureAndAuthorizationForOrigins` policy.
  bool subapps_apis_require_user_gesture_and_authorization = true;

  // The forced colors state for the web content. The forced colors state
  // is used to evaluate the forced-colors media query, as well as determining
  // when to apply system color overrides to author specified styles.
  bool in_forced_colors = false;

  // Indicates if Forced Colors mode should be disabled for this page.
  // This allows users opt out of forced colors on specific sites.
  // Forced colors are disabled for sites in the `kPageColorsBlockList` pref.
  bool is_forced_colors_disabled = false;

  // Holds the browser's theme color to be used to render root non-overlay
  // Fluent scrollbars. Stored from an SkColor as ARGB.
  std::optional<SkColor> root_scrollbar_theme_color;

  // The preferred color scheme set by the user's browser settings. The variable
  // follows the browser's color mode setting unless a browser theme (custom or
  // not) is defined, in which case the color scheme is set to the default
  // value. This value is used to evaluate the used color scheme in non overlay
  // root scrollbars.
  blink::mojom::PreferredColorScheme preferred_root_scrollbar_color_scheme =
      blink::mojom::PreferredColorScheme::kLight;

  // The preferred color scheme for the web content. The scheme is used to
  // evaluate the prefers-color-scheme media query and resolve UA color scheme
  // to be used based on the supported-color-schemes META tag and CSS property.
  blink::mojom::PreferredColorScheme preferred_color_scheme =
      blink::mojom::PreferredColorScheme::kLight;

  // The preferred contrast for the web content. The contrast is used to
  // evaluate the prefers-contrast media query.
  blink::mojom::PreferredContrast preferred_contrast =
      blink::mojom::PreferredContrast::kNoPreference;

  // Network quality threshold below which resources from iframes are assigned
  // either kVeryLow or kVeryLow Blink priority.
  EffectiveConnectionType low_priority_iframes_threshold =
      EffectiveConnectionType::kEffectiveConnectionUnknownType;

  // Whether Picture-in-Picture is enabled.
  bool picture_in_picture_enabled = true;

  // Whether a translate service is available.
  // blink's hrefTranslate attribute existence relies on the result.
  // See https://github.com/dtapuska/html-translate
  bool translate_service_available = false;

  // A value other than
  // mojom::EffectiveConnectionType::kEffectiveConnectionUnknownType implies
  // that the network quality estimate related Web APIs are in the holdback
  // mode. When the holdback is enabled, the related Web APIs return network
  // quality estimate corresponding to |network_quality_estimator_web_holdback|
  // regardless of the actual quality.
  EffectiveConnectionType network_quality_estimator_web_holdback =
      EffectiveConnectionType::kEffectiveConnectionUnknownType;

  // Whether lazy loading of frames and images is enabled.
  bool lazy_load_enabled = true;

  // Setting to false disables upgrades to HTTPS for HTTP resources in HTTPS
  // sites.
  bool allow_mixed_content_upgrades = true;

  // Whether the focused element should always be indicated (for example, by
  // forcing :focus-visible to match regardless of focus method).
  bool always_show_focus = false;

  // Whether touch input can trigger HTML drag-and-drop operations. The
  // default value depends on the platform.
  bool touch_drag_drop_enabled;  // Set in web_preferences.cc

  // Whether the end of a drag fires a contextmenu event and possibly shows a
  // context-menu (depends on how the event is handled).  Currently touch-drags
  // cannot show context menus, see crbug.com/1096189.
  bool touch_dragend_context_menu = false;

  // By default, WebXR's immersive-ar session creation is allowed, but this can
  // change depending on the enterprise policy if the platform supports it.
  bool webxr_immersive_ar_allowed = true;

  // Whether lookup of frames in the associated WebView (e.g. lookup via
  // window.open or via <a target=...>) should be renderer-wide (i.e. going
  // beyond the usual opener-relationship-based BrowsingInstance boundaries).
  bool renderer_wide_named_frame_lookup = false;

  // Whether MIME type checking for worker scripts is strict (true) or lax
  // (false). Used by StrictMimetypeCheckForWorkerScriptsEnabled policy.
  bool strict_mime_type_check_for_worker_scripts_enabled = true;

  // Whether modal context menu is used. A modal context menu meaning it is
  // blocking user's access to the background web content.
  bool modal_context_menu = true;

  // Whether the safe-area-insets should be changed dynamically based on
  // browser controls shown ratio. This value is used in web settings only
  // when feature DynamicSafeAreaInsets is enabled.
  bool dynamic_safe_area_insets_enabled = false;

  // Whether PaymentRequest is enabled. Controlled by WebView settings on
  // WebView and by `kWebPayments` feature flag everywhere.
  bool payment_request_enabled = false;

  // We try to keep the default values the same as the default values in
  // chrome, except for the cases where it would require lots of extra work for
  // the embedder to use the same default value.
  WebPreferences();
  WebPreferences(const WebPreferences& other);
  WebPreferences(WebPreferences&& other);
  ~WebPreferences();
  WebPreferences& operator=(const WebPreferences& other);
  WebPreferences& operator=(WebPreferences&& other);
};

}  // namespace web_pref

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PREFERENCES_WEB_PREFERENCES_H_
