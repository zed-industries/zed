/*
 * Copyright (C) 2009, 2012 Google Inc. All rights reserved.
 * Copyright (C) 2014 Opera Software ASA. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_H_

#include <vector>

#include "base/containers/span.h"
#include "cc/paint/paint_canvas.h"
#include "third_party/blink/public/common/page/drag_operation.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-shared.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/web/web_drag_status.h"
#include "third_party/blink/public/web/web_input_method_controller.h"
#include "v8/include/v8.h"

namespace gfx {
class PointF;
class Range;
class Rect;
}  // namespace gfx

namespace ui {
class Cursor;
struct ImeTextSpan;
}

namespace blink {

class WebCoalescedInputEvent;
class WebDragData;
class WebPluginContainer;
class WebURLResponse;
struct WebPrintParams;
struct WebPrintPresetOptions;
struct WebURLError;

class WebPlugin {
 public:
  // Initializes the plugin using |container| to communicate with the renderer
  // code. |container| must own this plugin. |container| must not be nullptr.
  //
  // Returns true if a plugin (not necessarily this one) has been successfully
  // initialized into |container|.
  //
  // NOTE: This method is subtle. This plugin may be marked for deletion via
  // destroy() during initialization. When this occurs, container() will
  // return nullptr. Because deletions during initialize() must be
  // asynchronous, this object is still alive immediately after initialize().
  //   1) If container() == nullptr and this method returns true, this plugin
  //      has been replaced by another during initialization. This new plugin
  //      may be accessed via container->plugin().
  //   2) If container() == nullptr and this method returns false, this plugin
  //      and the container have both been marked for deletion.
  virtual bool Initialize(WebPluginContainer*) = 0;

  // Plugins must arrange for themselves to be deleted sometime during or after
  // this method is called. This method is only called by the owning
  // WebPluginContainer.
  // The exception is if the plugin has been detached by a WebPluginContainer,
  // i.e. been replaced by another plugin. Then it must be destroyed separately.
  // Once this method has been called, container() must return nullptr.
  virtual void Destroy() = 0;

  // Returns the container that this plugin has been initialized with.
  // Must return nullptr if this plugin is scheduled for deletion.
  //
  // NOTE: This container doesn't necessarily own this plugin. For example,
  // if the container has been assigned a new plugin, then the container will
  // own the new plugin, not this old plugin.
  virtual WebPluginContainer* Container() const { return nullptr; }

  virtual v8::Local<v8::Object> V8ScriptableObject(v8::Isolate*) {
    return v8::Local<v8::Object>();
  }

  virtual bool SupportsKeyboardFocus() const { return false; }
  // Returns true if this plugin supports input method, which implements
  // setComposition(), commitText() and finishComposingText() below.
  virtual bool SupportsInputMethod() const { return false; }

  virtual bool CanProcessDrag() const { return false; }

  virtual void UpdateAllLifecyclePhases(blink::DocumentUpdateReason) = 0;
  virtual void Paint(cc::PaintCanvas*, const gfx::Rect&) = 0;

  // Coordinates are relative to the containing window.
  virtual void UpdateGeometry(const gfx::Rect& window_rect,
                              const gfx::Rect& clip_rect,
                              const gfx::Rect& unobscured_rect,
                              bool is_visible) = 0;

  virtual void UpdateFocus(bool focused, mojom::FocusType) = 0;

  virtual void UpdateVisibility(bool) = 0;

  virtual WebInputEventResult HandleInputEvent(const WebCoalescedInputEvent&,
                                               ui::Cursor*) = 0;

  virtual bool HandleDragStatusUpdate(WebDragStatus,
                                      const WebDragData&,
                                      DragOperationsMask,
                                      const gfx::PointF& position,
                                      const gfx::PointF& screen_position) {
    return false;
  }

  virtual void DidReceiveResponse(const WebURLResponse&) = 0;
  virtual void DidReceiveData(base::span<const char> data) = 0;
  virtual void DidFinishLoading() = 0;
  virtual void DidFailLoading(const WebURLError&) = 0;

  // Printing interface.
  // Whether the plugin supports its own paginated print. The other print
  // interface methods are called only if this method returns true.
  virtual bool SupportsPaginatedPrint() { return false; }
  // Returns true on success and sets the out parameter to the print preset
  // options for the document.
  virtual bool GetPrintPresetOptionsFromDocument(WebPrintPresetOptions*) {
    return false;
  }

  // Begins a print session with the given `print_params`. A call to
  // `PrintPage()` can only be made after after a successful call to
  // `PrintBegin()`. Returns the number of pages required for the print output.
  // A returned value of 0 indicates failure.
  virtual int PrintBegin(const WebPrintParams& print_params) { return 0; }

  // Prints the page specified by `page_index`, using the parameters passed to
  // `PrintBegin()`, into `canvas`.
  virtual void PrintPage(int page_index, cc::PaintCanvas* canvas) {}

  // Ends the print session. Further calls to `PrintPages()` will fail.
  virtual void PrintEnd() {}

  virtual bool HasSelection() const { return false; }
  virtual WebString SelectionAsText() const { return WebString(); }
  virtual WebString SelectionAsMarkup() const { return WebString(); }

  virtual bool CanEditText() const { return false; }
  virtual bool HasEditableText() const { return false; }

  virtual bool CanUndo() const { return false; }
  virtual bool CanRedo() const { return false; }
  virtual bool CanCopy() const { return true; }

  virtual bool ExecuteEditCommand(const WebString& name,
                                  const WebString& value) {
    return false;
  }

  // Sets composition text from input method, and returns true if the
  // composition is set successfully. If |replacementRange| is not null, the
  // text inside |replacementRange| will be replaced by |text|
  virtual bool SetComposition(
      const WebString& text,
      const std::vector<ui::ImeTextSpan>& ime_text_spans,
      const WebRange& replacement_range,
      int selection_start,
      int selection_end) {
    return false;
  }

  // Deletes the ongoing composition if any, inserts the specified text, and
  // moves the caret according to relativeCaretPosition. If |replacementRange|
  // is not null, the text inside |replacementRange| will be replaced by |text|.
  virtual bool CommitText(const WebString& text,
                          const std::vector<ui::ImeTextSpan>& ime_text_spans,
                          const WebRange& replacement_range,
                          int relative_caret_position) {
    return false;
  }

  // Confirms an ongoing composition; holds or moves selections accroding to
  // selectionBehavior.
  virtual bool FinishComposingText(
      WebInputMethodController::ConfirmCompositionBehavior selection_behavior) {
    return false;
  }

  // Deletes the current selection plus the specified number of characters
  // before and after the selection or caret.
  virtual void ExtendSelectionAndDelete(int before, int after) {}
  // Deletes text before and after the current cursor position, excluding the
  // selection. The lengths are supplied in UTF-16 Code Unit, not in code points
  // or in glyphs.
  virtual void DeleteSurroundingText(int before, int after) {}
  // Deletes text before and after the current cursor position, excluding the
  // selection. The lengths are supplied in code points, not in UTF-16 Code Unit
  // or in glyphs. Do nothing if there are one or more invalid surrogate pairs
  // in the requested range.
  virtual void DeleteSurroundingTextInCodePoints(int before, int after) {}
  // If the given position is over a link, returns the absolute url.
  // Otherwise an empty url is returned.
  virtual WebURL LinkAtPosition(const gfx::Point& position) const {
    return WebURL();
  }

  // Find interface.
  // Start a new search.  The plugin should search for a little bit at a time so
  // that it doesn't block the thread in case of a large document.  The results,
  // along with the find's identifier, should be sent asynchronously to
  // WebLocalFrameClient's reportFindInPage* methods.
  // Returns true if the search started, or false if the plugin doesn't support
  // search.
  virtual bool StartFind(const WebString& search_text,
                         bool case_sensitive,
                         int identifier) {
    return false;
  }
  // Tells the plugin to jump forward or backward in the list of find results.
  virtual void SelectFindResult(bool forward, int identifier) {}
  // Tells the plugin that the user has stopped the find operation.
  virtual void StopFind() {}

  // View rotation types.
  enum class RotationType { k90Clockwise, k90Counterclockwise };
  // Whether the plugin can rotate the view of its content.
  virtual bool CanRotateView() { return false; }
  // Rotates the plugin's view of its content.
  virtual void RotateView(RotationType type) {}
  // Check whether a plugin failed to load, with there being no possibility of
  // it loading later.
  virtual bool IsErrorPlaceholder() { return false; }

  // Indication that a current mouse lock has been lost.
  virtual void DidLoseMouseLock() {}

  // A response has been received from a previous WebPluginContainer::LockMouse
  // call.
  virtual void DidReceiveMouseLockResult(bool success) {}

  // Determines whether composition can happen inline.
  virtual bool CanComposeInline() { return false; }

  // Determines if IME events should be sent to plugin instead of processed to
  // the currently focused frame.
  virtual bool ShouldDispatchImeEventsToPlugin() { return false; }

  // Returns the current plugin text input type.
  virtual WebTextInputType GetPluginTextInputType() {
    return WebTextInputType::kWebTextInputTypeNone;
  }

  // Returns the current plugin caret bounds in blink/viewport coordinates.
  virtual gfx::Rect GetPluginCaretBounds() { return gfx::Rect(); }

  // Set the composition in plugin.
  virtual void ImeSetCompositionForPlugin(
      const WebString& text,
      const std::vector<ui::ImeTextSpan>& ime_text_spans,
      const gfx::Range& replacement_range,
      int selection_start,
      int selection_end) {}

  // Commit the text to plugin.
  virtual void ImeCommitTextForPlugin(
      const WebString& text,
      const std::vector<ui::ImeTextSpan>& ime_text_spans,
      const gfx::Range& replacement_range,
      int relative_cursor_pos) {}

  // Indicate composition is complete to plugin.
  virtual void ImeFinishComposingTextForPlugin(bool keep_selection) {}

 protected:
  virtual ~WebPlugin() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_PLUGIN_H_
