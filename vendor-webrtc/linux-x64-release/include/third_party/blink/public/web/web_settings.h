/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SETTINGS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SETTINGS_H_

#include <unicode/uscript.h>

#include "third_party/blink/public/common/css/navigation_controls.h"
#include "third_party/blink/public/common/web_preferences/web_preferences.h"
#include "third_party/blink/public/mojom/v8_cache_options.mojom-forward.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-forward.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_effective_connection_type.h"

namespace blink {

enum class LCDTextPreference;
class WebString;

// WebSettings is owned by the WebView and allows code to modify the settings
// for the WebView's page without any knowledge of WebCore itself.  For the most
// part, these functions have a 1:1 mapping with the methods in
// WebCore/page/Settings.h.
class WebSettings {
 public:
  // Selection strategy defines how the selection granularity changes when the
  // selection extent is moved.
  enum class SelectionStrategyType {
    // Always uses character granularity.
    kCharacter,
    // "Expand by word, shrink by character" selection strategy.
    // Uses character granularity when selection is shrinking. If the
    // selection is expanding, granularity doesn't change until a word
    // boundary is passed, after which the granularity switches to "word".
    kDirection
  };

  // Defines user preference for text track kind.
  enum class TextTrackKindUserPreference {
    // Display only tracks marked as default
    kDefault,
    // If available, display captions track in preferred language, else display
    // subtitles.
    kCaptions,
    // If available, display subtitles track in preferred language, else display
    // captions.
    kSubtitles
  };

  // Sets value of a setting by its string identifier from Settings.in and
  // string representation of value. An enum's string representation is the
  // string representation of the integer value of the enum.
  virtual void SetFromStrings(const WebString& name,
                              const WebString& value) = 0;

  virtual bool ShrinksViewportContentToFit() const = 0;
  virtual bool ViewportEnabled() const = 0;
  virtual void SetAccelerated2dCanvasMSAASampleCount(int) = 0;
  virtual void SetLCDTextPreference(LCDTextPreference) = 0;
  // Not implemented yet, see http://crbug.com/178119
  virtual void SetAcceleratedCompositingForTransitionEnabled(bool) {}
  // If set to true, allows frames with an https origin to display passive
  // contents at an insecure URL. Otherwise, disallows it. The
  // LocalFrameClient set to the frame may override the value set by this
  // method.
  virtual void SetAccessibilityPasswordValuesEnabled(bool) = 0;
  virtual void SetAllowFileAccessFromFileURLs(bool) = 0;
  virtual void SetAllowCustomScrollbarInMainFrame(bool) = 0;
  virtual void SetAllowGeolocationOnInsecureOrigins(bool) = 0;
  // If set to true, allows frames with an https origin to run active
  // contents at an insecure URL. This includes WebSockets. Otherwise,
  // disallows it. The LocalFrameClient set to the frame may override the
  // value set by this method.
  virtual void SetAllowRunningOfInsecureContent(bool) = 0;
  virtual void SetAllowScriptsToCloseWindows(bool) = 0;
  virtual void SetAllowUniversalAccessFromFileURLs(bool) = 0;
  virtual void SetAccessibilityFontWeightAdjustment(int) = 0;
  virtual void SetAlwaysShowContextMenuOnTouch(bool) = 0;
  virtual void SetAntialiased2dCanvasEnabled(bool) = 0;
  virtual void SetAntialiasedClips2dCanvasEnabled(bool) = 0;
  virtual void SetAutoplayPolicy(mojom::AutoplayPolicy) = 0;
  virtual void SetRequireTransientActivationForGetDisplayMedia(bool) = 0;
  virtual void SetRequireTransientActivationForShowFileOrDirectoryPicker(
      bool) = 0;
  virtual void SetAutoZoomFocusedEditableToLegibleScale(bool) = 0;
  virtual void SetCaretBrowsingEnabled(bool) = 0;
  virtual void SetClobberUserAgentInitialScaleQuirk(bool) = 0;
  virtual void SetCookieEnabled(bool) = 0;
  virtual void SetCursiveFontFamily(const WebString&,
                                    UScriptCode = USCRIPT_COMMON) = 0;
  virtual void SetDNSPrefetchingEnabled(bool) = 0;
  virtual void SetDOMPasteAllowed(bool) = 0;
  virtual void SetDefaultFixedFontSize(int) = 0;
  virtual void SetDefaultFontSize(int) = 0;
  virtual void SetDefaultTextEncodingName(const WebString&) = 0;
  virtual void SetDefaultVideoPosterURL(const WebString&) = 0;
  void SetDeferred2dCanvasEnabled(bool) {}  // temporary stub
  virtual void SetDeviceScaleAdjustment(float) = 0;
  virtual void SetDisableReadingFromCanvas(bool) = 0;
  virtual void SetDontSendKeyEventsToJavascript(bool) = 0;
  virtual void SetDoubleTapToZoomEnabled(bool) = 0;
  virtual void SetDownloadableBinaryFontsEnabled(bool) = 0;
  virtual void SetDynamicSafeAreaInsetsEnabled(bool) = 0;
  virtual void SetEditingBehavior(mojom::EditingBehavior) = 0;
  virtual void SetEnableScrollAnimator(bool) = 0;
  virtual void SetPrefersReducedMotion(bool) = 0;
  virtual void SetPrefersReducedTransparency(bool) = 0;
  virtual void SetInvertedColors(bool) = 0;
  virtual void SetSmoothScrollForFindEnabled(bool) = 0;
  virtual void SetWebGL1Enabled(bool) = 0;
  virtual void SetWebGL2Enabled(bool) = 0;
  virtual void SetFantasyFontFamily(const WebString&,
                                    UScriptCode = USCRIPT_COMMON) = 0;
  virtual void SetMathFontFamily(const WebString&,
                                 UScriptCode = USCRIPT_COMMON) = 0;
  // Osaka font should be represented as "Osaka", not "Osaka-Mono" in the
  // first argument.
  virtual void SetFixedFontFamily(const WebString&,
                                  UScriptCode = USCRIPT_COMMON) = 0;
  virtual void SetNetworkQuietTimeout(double timeout) = 0;
  virtual void SetForceMainWorldInitialization(bool) = 0;
  virtual void SetForceZeroLayoutHeight(bool) = 0;
  virtual void SetFullscreenSupported(bool) = 0;
  virtual void SetHideDownloadUI(bool) = 0;
  virtual void SetHighlightAds(bool) = 0;
  virtual void SetHyperlinkAuditingEnabled(bool) = 0;
  virtual void SetIgnoreMainFrameOverflowHiddenQuirk(bool) = 0;
  virtual void SetImageAnimationPolicy(mojom::ImageAnimationPolicy) = 0;
  virtual void SetImagesEnabled(bool) = 0;
  virtual void SetJavaScriptCanAccessClipboard(bool) = 0;
  virtual void SetJavaScriptEnabled(bool) = 0;
  virtual void SetLoadsImagesAutomatically(bool) = 0;
  virtual void SetLoadWithOverviewMode(bool) = 0;
  virtual void SetShouldReuseGlobalForUnownedMainFrame(bool) = 0;
  virtual void SetLocalStorageEnabled(bool) = 0;
  virtual void SetMainFrameClipsContent(bool) = 0;
  virtual void SetMainFrameResizesAreOrientationChanges(bool) = 0;
  virtual void SetMaxTouchPoints(int) = 0;
  virtual void SetPictureInPictureEnabled(bool) = 0;
  virtual void SetWebAppScope(const WebString&) = 0;
  virtual void SetPresentationRequiresUserGesture(bool) = 0;
  virtual void SetEmbeddedMediaExperienceEnabled(bool) = 0;
  virtual void SetImmersiveModeEnabled(bool) = 0;
  virtual void SetMinimumFontSize(int) = 0;
  virtual void SetMinimumLogicalFontSize(int) = 0;
  virtual void SetHideScrollbars(bool) = 0;
  virtual void SetPrefersDefaultScrollbarStyles(bool) = 0;
  virtual void SetPasswordEchoDurationInSeconds(double) = 0;
  virtual void SetPasswordEchoEnabled(bool) = 0;
  virtual void SetPluginsEnabled(bool) = 0;
  virtual void SetPresentationReceiver(bool) = 0;
  virtual void SetAvailablePointerTypes(int) = 0;
  virtual void SetPrimaryPointerType(blink::mojom::PointerType) = 0;
  virtual void SetAvailableHoverTypes(int) = 0;
  virtual void SetPrimaryHoverType(blink::mojom::HoverType) = 0;
  virtual void SetOutputDeviceUpdateAbilityType(
      blink::mojom::OutputDeviceUpdateAbilityType) = 0;
  virtual void SetPreferHiddenVolumeControls(bool) = 0;
  virtual void SetShouldProtectAgainstIpcFlooding(bool) = 0;
  virtual void SetRenderVSyncNotificationEnabled(bool) = 0;
  virtual void SetReportScreenSizeInPhysicalPixelsQuirk(bool) = 0;
  virtual void SetRubberBandingOnCompositorThread(bool) = 0;
  virtual void SetSansSerifFontFamily(const WebString&,
                                      UScriptCode = USCRIPT_COMMON) = 0;
  virtual void SetSelectTrailingWhitespaceEnabled(bool) = 0;
  virtual void SetSelectionIncludesAltImageText(bool) = 0;
  virtual void SetSelectionStrategy(SelectionStrategyType) = 0;
  virtual void SetSerifFontFamily(const WebString&,
                                  UScriptCode = USCRIPT_COMMON) = 0;
  virtual void SetShouldPrintBackgrounds(bool) = 0;
  virtual void SetShouldClearDocumentBackground(bool) = 0;
  virtual void SetShowContextMenuOnMouseUp(bool) = 0;
  virtual void SetShrinksViewportContentToFit(bool) = 0;
  virtual void SetSmartInsertDeleteEnabled(bool) = 0;
  // Spatial navigation feature, when enabled, improves the experience
  // of keyboard-controlling the web pages which originally were not designed
  // for keyboard navigation. It allows to use arrow keys to move focus between
  // the adjacent HTML elements. As a side effect, it extends the criteria for
  // elements to be focusable to include any element which has click or keyboard
  // event handlers specified. User can also trigger click handlers for such
  // elements using SPACE or ENTER keys.
  virtual void SetSpatialNavigationEnabled(bool) = 0;
  virtual void SetSpellCheckEnabledByDefault(bool) = 0;
  virtual void SetStandardFontFamily(const WebString&,
                                     UScriptCode = USCRIPT_COMMON) = 0;
  virtual void SetStrictMixedContentChecking(bool) = 0;
  virtual void SetStrictMixedContentCheckingForPlugin(bool) = 0;
  virtual void SetStrictPowerfulFeatureRestrictions(bool) = 0;
  virtual void SetStrictlyBlockBlockableMixedContent(bool) = 0;
  virtual void SetSupportDeprecatedTargetDensityDPI(bool) = 0;
  virtual void SetSupportsMultipleWindows(bool) = 0;
  virtual void SetSyncXHRInDocumentsEnabled(bool) = 0;
  // TODO(https://crbug.com/1163644): Remove once Chrome Apps are deprecated.
  virtual void SetTargetBlankImpliesNoOpenerEnabledWillBeRemoved(bool) = 0;
  // TODO(https://crbug.com/1172495): Remove once Chrome Apps are deprecated.
  virtual void SetAllowNonEmptyNavigatorPlugins(bool) = 0;
  virtual void SetTextAreasAreResizable(bool) = 0;
  virtual void SetTextAutosizingEnabled(bool) = 0;
  virtual void SetAccessibilityFontScaleFactor(float) = 0;
  virtual void SetAccessibilityTextSizeContrastFactor(int) = 0;
  virtual void SetAccessibilityAlwaysShowFocus(bool) = 0;
  virtual void SetTextTrackKindUserPreference(TextTrackKindUserPreference) = 0;
  virtual void SetTextTrackBackgroundColor(const WebString&) = 0;
  virtual void SetTextTrackFontFamily(const WebString&) = 0;
  virtual void SetTextTrackFontStyle(const WebString&) = 0;
  virtual void SetTextTrackFontVariant(const WebString&) = 0;
  virtual void SetTextTrackMarginPercentage(float) = 0;
  virtual void SetTextTrackTextColor(const WebString&) = 0;
  virtual void SetTextTrackTextShadow(const WebString&) = 0;
  virtual void SetTextTrackTextSize(const WebString&) = 0;
  virtual void SetTextTrackWindowColor(const WebString&) = 0;
  virtual void SetTextTrackWindowRadius(const WebString&) = 0;
  virtual void SetTouchDragDropEnabled(bool) = 0;
  virtual void SetTouchDragEndContextMenu(bool) = 0;
  virtual void SetBarrelButtonForDragEnabled(bool) = 0;
  virtual void SetViewportStyle(mojom::ViewportStyle) = 0;
  virtual void SetUseWideViewport(bool) = 0;
  virtual void SetV8CacheOptions(mojom::V8CacheOptions) = 0;
  virtual void SetValidationMessageTimerMagnification(int) = 0;
  virtual void SetViewportEnabled(bool) = 0;
  virtual void SetViewportMetaEnabled(bool) = 0;
  virtual void SetViewportMetaMergeContentQuirk(bool) = 0;
  virtual void SetViewportMetaNonUserScalableQuirk(bool) = 0;
  virtual void SetViewportMetaZeroValuesQuirk(bool) = 0;
  virtual void SetWebGLErrorsToConsoleEnabled(bool) = 0;
  virtual void SetWebSecurityEnabled(bool) = 0;
  virtual void SetWideViewportQuirkEnabled(bool) = 0;
  virtual void SetMediaControlsEnabled(bool) = 0;
  virtual void SetDoNotUpdateSelectionOnMutatingSelectionRange(bool) = 0;
  virtual void SetLowPriorityIframesThreshold(WebEffectiveConnectionType) = 0;
  virtual void SetLazyLoadEnabled(bool) = 0;
  virtual void SetLazyLoadingFrameMarginPxUnknown(int) = 0;
  virtual void SetLazyLoadingFrameMarginPxOffline(int) = 0;
  virtual void SetLazyLoadingFrameMarginPxSlow2G(int) = 0;
  virtual void SetLazyLoadingFrameMarginPx2G(int) = 0;
  virtual void SetLazyLoadingFrameMarginPx3G(int) = 0;
  virtual void SetLazyLoadingFrameMarginPx4G(int) = 0;
  virtual void SetLazyLoadingImageMarginPxUnknown(int) = 0;
  virtual void SetLazyLoadingImageMarginPxOffline(int) = 0;
  virtual void SetLazyLoadingImageMarginPxSlow2G(int) = 0;
  virtual void SetLazyLoadingImageMarginPx2G(int) = 0;
  virtual void SetLazyLoadingImageMarginPx3G(int) = 0;
  virtual void SetLazyLoadingImageMarginPx4G(int) = 0;
  virtual void SetForceDarkModeEnabled(bool) = 0;
  virtual void SetInForcedColors(bool) = 0;
  virtual void SetIsForcedColorsDisabled(bool) = 0;
  virtual void SetPreferredRootScrollbarColorScheme(
      blink::mojom::PreferredColorScheme) = 0;
  virtual void SetPreferredColorScheme(blink::mojom::PreferredColorScheme) = 0;
  virtual void SetPreferredContrast(mojom::PreferredContrast) = 0;
  virtual void SetNavigationControls(NavigationControls) = 0;
  virtual void SetAriaModalPrunesAXTree(bool) = 0;
  virtual void SetSelectionClipboardBufferAvailable(bool) = 0;
  virtual void SetAccessibilityIncludeSvgGElement(bool) = 0;
  virtual void SetWebXRImmersiveArAllowed(bool) = 0;
  virtual void SetModalContextMenu(bool) = 0;
  virtual void SetRequireTransientActivationAndAuthorizationForSubAppsAPIs(
      bool) = 0;
  virtual void SetRootScrollbarThemeColor(std::optional<SkColor>) = 0;

 protected:
  ~WebSettings() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SETTINGS_H_
