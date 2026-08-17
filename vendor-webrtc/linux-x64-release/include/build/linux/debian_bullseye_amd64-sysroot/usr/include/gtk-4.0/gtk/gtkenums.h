/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_ENUMS_H__
#define __GTK_ENUMS_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>


G_BEGIN_DECLS

/**
 * GtkAlign:
 * @GTK_ALIGN_FILL: stretch to fill all space if possible, center if
 *   no meaningful way to stretch
 * @GTK_ALIGN_START: snap to left or top side, leaving space on right or bottom
 * @GTK_ALIGN_END: snap to right or bottom side, leaving space on left or top
 * @GTK_ALIGN_CENTER: center natural width of widget inside the allocation
 * @GTK_ALIGN_BASELINE: align the widget according to the baseline.
 *   See [class@Gtk.Widget].
 *
 * Controls how a widget deals with extra space in a single dimension.
 *
 * Alignment only matters if the widget receives a “too large” allocation,
 * for example if you packed the widget with the [property@Gtk.Widget:hexpand]
 * property inside a [class@Box], then the widget might get extra space.
 * If you have for example a 16x16 icon inside a 32x32 space, the icon
 * could be scaled and stretched, it could be centered, or it could be
 * positioned to one side of the space.
 *
 * Note that in horizontal context %GTK_ALIGN_START and %GTK_ALIGN_END
 * are interpreted relative to text direction.
 *
 * %GTK_ALIGN_BASELINE support is optional for containers and widgets, and
 * it is only supported for vertical alignment.  When it's not supported by
 * a child or a container it is treated as %GTK_ALIGN_FILL.
 */
typedef enum
{
  GTK_ALIGN_FILL,
  GTK_ALIGN_START,
  GTK_ALIGN_END,
  GTK_ALIGN_CENTER,
  GTK_ALIGN_BASELINE
} GtkAlign;

/**
 * GtkArrowType:
 * @GTK_ARROW_UP: Represents an upward pointing arrow.
 * @GTK_ARROW_DOWN: Represents a downward pointing arrow.
 * @GTK_ARROW_LEFT: Represents a left pointing arrow.
 * @GTK_ARROW_RIGHT: Represents a right pointing arrow.
 * @GTK_ARROW_NONE: No arrow.
 *
 * Used to indicate the direction in which an arrow should point.
 */
typedef enum
{
  GTK_ARROW_UP,
  GTK_ARROW_DOWN,
  GTK_ARROW_LEFT,
  GTK_ARROW_RIGHT,
  GTK_ARROW_NONE
} GtkArrowType;

/**
 * GtkBaselinePosition:
 * @GTK_BASELINE_POSITION_TOP: Align the baseline at the top
 * @GTK_BASELINE_POSITION_CENTER: Center the baseline
 * @GTK_BASELINE_POSITION_BOTTOM: Align the baseline at the bottom
 *
 * Baseline position in a row of widgets.
 *
 * Whenever a container has some form of natural row it may align
 * children in that row along a common typographical baseline. If
 * the amount of vertical space in the row is taller than the total
 * requested height of the baseline-aligned children then it can use a
 * `GtkBaselinePosition` to select where to put the baseline inside the
 * extra available space.
 */
typedef enum
{
  GTK_BASELINE_POSITION_TOP,
  GTK_BASELINE_POSITION_CENTER,
  GTK_BASELINE_POSITION_BOTTOM
} GtkBaselinePosition;

/**
 * GtkContentFit:
 * @GTK_CONTENT_FIT_FILL: Make the content fill the entire allocation,
 *   without taking its aspect ratio in consideration. The resulting
 *   content will appear as stretched if its aspect ratio is different
 *   from the allocation aspect ratio.
 * @GTK_CONTENT_FIT_CONTAIN: Scale the content to fit the allocation,
 *   while taking its aspect ratio in consideration. The resulting
 *   content will appear as letterboxed if its aspect ratio is different
 *   from the allocation aspect ratio.
 * @GTK_CONTENT_FIT_COVER: Cover the entire allocation, while taking
 *   the content aspect ratio in consideration. The resulting content
 *   will appear as clipped if its aspect ratio is different from the
 *   allocation aspect ratio.
 * @GTK_CONTENT_FIT_SCALE_DOWN: The content is scaled down to fit the
 *   allocation, if needed, otherwise its original size is used.
 *
 * Controls how a content should be made to fit inside an allocation.
 *
 * Since: 4.8
 */
typedef enum
{
  GTK_CONTENT_FIT_FILL,
  GTK_CONTENT_FIT_CONTAIN,
  GTK_CONTENT_FIT_COVER,
  GTK_CONTENT_FIT_SCALE_DOWN,
} GtkContentFit;

/**
 * GtkDeleteType:
 * @GTK_DELETE_CHARS: Delete characters.
 * @GTK_DELETE_WORD_ENDS: Delete only the portion of the word to the
 *   left/right of cursor if we’re in the middle of a word.
 * @GTK_DELETE_WORDS: Delete words.
 * @GTK_DELETE_DISPLAY_LINES: Delete display-lines. Display-lines
 *   refers to the visible lines, with respect to the current line
 *   breaks. As opposed to paragraphs, which are defined by line
 *   breaks in the input.
 * @GTK_DELETE_DISPLAY_LINE_ENDS: Delete only the portion of the
 *   display-line to the left/right of cursor.
 * @GTK_DELETE_PARAGRAPH_ENDS: Delete to the end of the
 *   paragraph. Like C-k in Emacs (or its reverse).
 * @GTK_DELETE_PARAGRAPHS: Delete entire line. Like C-k in pico.
 * @GTK_DELETE_WHITESPACE: Delete only whitespace. Like M-\ in Emacs.
 *
 * Passed to various keybinding signals for deleting text.
 */
typedef enum
{
  GTK_DELETE_CHARS,
  GTK_DELETE_WORD_ENDS,
  GTK_DELETE_WORDS,
  GTK_DELETE_DISPLAY_LINES,
  GTK_DELETE_DISPLAY_LINE_ENDS,
  GTK_DELETE_PARAGRAPH_ENDS,
  GTK_DELETE_PARAGRAPHS,
  GTK_DELETE_WHITESPACE
} GtkDeleteType;

/* Focus movement types */
/**
 * GtkDirectionType:
 * @GTK_DIR_TAB_FORWARD: Move forward.
 * @GTK_DIR_TAB_BACKWARD: Move backward.
 * @GTK_DIR_UP: Move up.
 * @GTK_DIR_DOWN: Move down.
 * @GTK_DIR_LEFT: Move left.
 * @GTK_DIR_RIGHT: Move right.
 *
 * Focus movement types.
 */
typedef enum
{
  GTK_DIR_TAB_FORWARD,
  GTK_DIR_TAB_BACKWARD,
  GTK_DIR_UP,
  GTK_DIR_DOWN,
  GTK_DIR_LEFT,
  GTK_DIR_RIGHT
} GtkDirectionType;

/**
 * GtkIconSize:
 * @GTK_ICON_SIZE_INHERIT: Keep the size of the parent element
 * @GTK_ICON_SIZE_NORMAL: Size similar to text size
 * @GTK_ICON_SIZE_LARGE: Large size, for example in an icon view
 *
 * Built-in icon sizes.
 *
 * Icon sizes default to being inherited. Where they cannot be
 * inherited, text size is the default.
 *
 * All widgets which use `GtkIconSize` set the normal-icons or
 * large-icons style classes correspondingly, and let themes
 * determine the actual size to be used with the
 * `-gtk-icon-size` CSS property.
 */
typedef enum
{
  GTK_ICON_SIZE_INHERIT,
  GTK_ICON_SIZE_NORMAL,
  GTK_ICON_SIZE_LARGE
} GtkIconSize;

/**
 * GtkSensitivityType:
 * @GTK_SENSITIVITY_AUTO: The control is made insensitive if no
 *   action can be triggered
 * @GTK_SENSITIVITY_ON: The control is always sensitive
 * @GTK_SENSITIVITY_OFF: The control is always insensitive
 *
 * Determines how GTK handles the sensitivity of various controls,
 * such as combo box buttons.
 */
typedef enum
{
  GTK_SENSITIVITY_AUTO,
  GTK_SENSITIVITY_ON,
  GTK_SENSITIVITY_OFF
} GtkSensitivityType;

/* Reading directions for text */
/**
 * GtkTextDirection:
 * @GTK_TEXT_DIR_NONE: No direction.
 * @GTK_TEXT_DIR_LTR: Left to right text direction.
 * @GTK_TEXT_DIR_RTL: Right to left text direction.
 *
 * Reading directions for text.
 */
typedef enum
{
  GTK_TEXT_DIR_NONE,
  GTK_TEXT_DIR_LTR,
  GTK_TEXT_DIR_RTL
} GtkTextDirection;

/**
 * GtkJustification:
 * @GTK_JUSTIFY_LEFT: The text is placed at the left edge of the label.
 * @GTK_JUSTIFY_RIGHT: The text is placed at the right edge of the label.
 * @GTK_JUSTIFY_CENTER: The text is placed in the center of the label.
 * @GTK_JUSTIFY_FILL: The text is placed is distributed across the label.
 *
 * Used for justifying the text inside a [class@Label] widget.
 */
typedef enum
{
  GTK_JUSTIFY_LEFT,
  GTK_JUSTIFY_RIGHT,
  GTK_JUSTIFY_CENTER,
  GTK_JUSTIFY_FILL
} GtkJustification;

/**
 * GtkMessageType:
 * @GTK_MESSAGE_INFO: Informational message
 * @GTK_MESSAGE_WARNING: Non-fatal warning message
 * @GTK_MESSAGE_QUESTION: Question requiring a choice
 * @GTK_MESSAGE_ERROR: Fatal error message
 * @GTK_MESSAGE_OTHER: None of the above
 *
 * The type of message being displayed in a [class@MessageDialog].
 */
typedef enum
{
  GTK_MESSAGE_INFO,
  GTK_MESSAGE_WARNING,
  GTK_MESSAGE_QUESTION,
  GTK_MESSAGE_ERROR,
  GTK_MESSAGE_OTHER
} GtkMessageType;

/**
 * GtkMovementStep:
 * @GTK_MOVEMENT_LOGICAL_POSITIONS: Move forward or back by graphemes
 * @GTK_MOVEMENT_VISUAL_POSITIONS:  Move left or right by graphemes
 * @GTK_MOVEMENT_WORDS:             Move forward or back by words
 * @GTK_MOVEMENT_DISPLAY_LINES:     Move up or down lines (wrapped lines)
 * @GTK_MOVEMENT_DISPLAY_LINE_ENDS: Move to either end of a line
 * @GTK_MOVEMENT_PARAGRAPHS:        Move up or down paragraphs (newline-ended lines)
 * @GTK_MOVEMENT_PARAGRAPH_ENDS:    Move to either end of a paragraph
 * @GTK_MOVEMENT_PAGES:             Move by pages
 * @GTK_MOVEMENT_BUFFER_ENDS:       Move to ends of the buffer
 * @GTK_MOVEMENT_HORIZONTAL_PAGES:  Move horizontally by pages
 *
 * Passed as argument to various keybinding signals for moving the
 * cursor position.
 */
typedef enum
{
  GTK_MOVEMENT_LOGICAL_POSITIONS,
  GTK_MOVEMENT_VISUAL_POSITIONS,
  GTK_MOVEMENT_WORDS,
  GTK_MOVEMENT_DISPLAY_LINES,
  GTK_MOVEMENT_DISPLAY_LINE_ENDS,
  GTK_MOVEMENT_PARAGRAPHS,
  GTK_MOVEMENT_PARAGRAPH_ENDS,
  GTK_MOVEMENT_PAGES,
  GTK_MOVEMENT_BUFFER_ENDS,
  GTK_MOVEMENT_HORIZONTAL_PAGES
} GtkMovementStep;

/**
 * GtkNaturalWrapMode:
 * @GTK_NATURAL_WRAP_INHERIT: Inherit the minimum size request.
 *   In particular, this should be used with %PANGO_WRAP_CHAR.
 * @GTK_NATURAL_WRAP_NONE: Try not to wrap the text. This mode is the
 *   closest to GTK3's behavior but can lead to a wide label leaving
 *   lots of empty space below the text.
 * @GTK_NATURAL_WRAP_WORD: Attempt to wrap at word boundaries. This
 *   is useful in particular when using %PANGO_WRAP_WORD_CHAR as the
 *   wrap mode.
 *
 * Options for selecting a different wrap mode for natural size
 * requests.
 *
 * See for example the [property@Gtk.Label:natural-wrap-mode] property.
 *
 * Since: 4.6
 */
typedef enum
{
  GTK_NATURAL_WRAP_INHERIT,
  GTK_NATURAL_WRAP_NONE,
  GTK_NATURAL_WRAP_WORD
} GtkNaturalWrapMode;

/**
 * GtkScrollStep:
 * @GTK_SCROLL_STEPS: Scroll in steps.
 * @GTK_SCROLL_PAGES: Scroll by pages.
 * @GTK_SCROLL_ENDS: Scroll to ends.
 * @GTK_SCROLL_HORIZONTAL_STEPS: Scroll in horizontal steps.
 * @GTK_SCROLL_HORIZONTAL_PAGES: Scroll by horizontal pages.
 * @GTK_SCROLL_HORIZONTAL_ENDS: Scroll to the horizontal ends.
 *
 * Passed as argument to various keybinding signals.
 */
typedef enum
{
  GTK_SCROLL_STEPS,
  GTK_SCROLL_PAGES,
  GTK_SCROLL_ENDS,
  GTK_SCROLL_HORIZONTAL_STEPS,
  GTK_SCROLL_HORIZONTAL_PAGES,
  GTK_SCROLL_HORIZONTAL_ENDS
} GtkScrollStep;

/**
 * GtkOrientation:
 * @GTK_ORIENTATION_HORIZONTAL: The element is in horizontal orientation.
 * @GTK_ORIENTATION_VERTICAL: The element is in vertical orientation.
 *
 * Represents the orientation of widgets and other objects.
 *
 * Typical examples are [class@Box] or [class@GesturePan].
 */
typedef enum
{
  GTK_ORIENTATION_HORIZONTAL,
  GTK_ORIENTATION_VERTICAL
} GtkOrientation;

/**
 * GtkOverflow:
 * @GTK_OVERFLOW_VISIBLE: No change is applied. Content is drawn at the specified
 *   position.
 * @GTK_OVERFLOW_HIDDEN: Content is clipped to the bounds of the area. Content
 *   outside the area is not drawn and cannot be interacted with.
 *
 * Defines how content overflowing a given area should be handled.
 *
 * This is used in [method@Gtk.Widget.set_overflow]. The
 * [property@Gtk.Widget:overflow] property is modeled after the
 * CSS overflow property, but implements it only partially.
 */
typedef enum
{
  GTK_OVERFLOW_VISIBLE,
  GTK_OVERFLOW_HIDDEN
} GtkOverflow;

/**
 * GtkPackType:
 * @GTK_PACK_START: The child is packed into the start of the widget
 * @GTK_PACK_END: The child is packed into the end of the widget
 *
 * Represents the packing location of a children in its parent.
 *
 * See [class@WindowControls] for example.
 */
typedef enum
{
  GTK_PACK_START,
  GTK_PACK_END
} GtkPackType;

/**
 * GtkPositionType:
 * @GTK_POS_LEFT: The feature is at the left edge.
 * @GTK_POS_RIGHT: The feature is at the right edge.
 * @GTK_POS_TOP: The feature is at the top edge.
 * @GTK_POS_BOTTOM: The feature is at the bottom edge.
 *
 * Describes which edge of a widget a certain feature is positioned at.
 *
 * For examples, see the tabs of a [class@Notebook], or the label
 * of a [class@Scale].
 */
typedef enum
{
  GTK_POS_LEFT,
  GTK_POS_RIGHT,
  GTK_POS_TOP,
  GTK_POS_BOTTOM
} GtkPositionType;

/**
 * GtkScrollType:
 * @GTK_SCROLL_NONE: No scrolling.
 * @GTK_SCROLL_JUMP: Jump to new location.
 * @GTK_SCROLL_STEP_BACKWARD: Step backward.
 * @GTK_SCROLL_STEP_FORWARD: Step forward.
 * @GTK_SCROLL_PAGE_BACKWARD: Page backward.
 * @GTK_SCROLL_PAGE_FORWARD: Page forward.
 * @GTK_SCROLL_STEP_UP: Step up.
 * @GTK_SCROLL_STEP_DOWN: Step down.
 * @GTK_SCROLL_PAGE_UP: Page up.
 * @GTK_SCROLL_PAGE_DOWN: Page down.
 * @GTK_SCROLL_STEP_LEFT: Step to the left.
 * @GTK_SCROLL_STEP_RIGHT: Step to the right.
 * @GTK_SCROLL_PAGE_LEFT: Page to the left.
 * @GTK_SCROLL_PAGE_RIGHT: Page to the right.
 * @GTK_SCROLL_START: Scroll to start.
 * @GTK_SCROLL_END: Scroll to end.
 *
 * Scrolling types.
 */
typedef enum
{
  GTK_SCROLL_NONE,
  GTK_SCROLL_JUMP,
  GTK_SCROLL_STEP_BACKWARD,
  GTK_SCROLL_STEP_FORWARD,
  GTK_SCROLL_PAGE_BACKWARD,
  GTK_SCROLL_PAGE_FORWARD,
  GTK_SCROLL_STEP_UP,
  GTK_SCROLL_STEP_DOWN,
  GTK_SCROLL_PAGE_UP,
  GTK_SCROLL_PAGE_DOWN,
  GTK_SCROLL_STEP_LEFT,
  GTK_SCROLL_STEP_RIGHT,
  GTK_SCROLL_PAGE_LEFT,
  GTK_SCROLL_PAGE_RIGHT,
  GTK_SCROLL_START,
  GTK_SCROLL_END
} GtkScrollType;

/**
 * GtkSelectionMode:
 * @GTK_SELECTION_NONE: No selection is possible.
 * @GTK_SELECTION_SINGLE: Zero or one element may be selected.
 * @GTK_SELECTION_BROWSE: Exactly one element is selected.
 *   In some circumstances, such as initially or during a search
 *   operation, it’s possible for no element to be selected with
 *   %GTK_SELECTION_BROWSE. What is really enforced is that the user
 *   can’t deselect a currently selected element except by selecting
 *   another element.
 * @GTK_SELECTION_MULTIPLE: Any number of elements may be selected.
 *   The Ctrl key may be used to enlarge the selection, and Shift
 *   key to select between the focus and the child pointed to.
 *   Some widgets may also allow Click-drag to select a range of elements.
 *
 * Used to control what selections users are allowed to make.
 */
typedef enum
{
  GTK_SELECTION_NONE,
  GTK_SELECTION_SINGLE,
  GTK_SELECTION_BROWSE,
  GTK_SELECTION_MULTIPLE
} GtkSelectionMode;

/* Widget states */

/**
 * GtkWrapMode:
 * @GTK_WRAP_NONE: do not wrap lines; just make the text area wider
 * @GTK_WRAP_CHAR: wrap text, breaking lines anywhere the cursor can
 *   appear (between characters, usually - if you want to be technical,
 *   between graphemes, see pango_get_log_attrs())
 * @GTK_WRAP_WORD: wrap text, breaking lines in between words
 * @GTK_WRAP_WORD_CHAR: wrap text, breaking lines in between words, or if
 *   that is not enough, also between graphemes
 *
 * Describes a type of line wrapping.
 */
typedef enum
{
  GTK_WRAP_NONE,
  GTK_WRAP_CHAR,
  GTK_WRAP_WORD,
  GTK_WRAP_WORD_CHAR
} GtkWrapMode;

/**
 * GtkSortType:
 * @GTK_SORT_ASCENDING: Sorting is in ascending order.
 * @GTK_SORT_DESCENDING: Sorting is in descending order.
 *
 * Determines the direction of a sort.
 */
typedef enum
{
  GTK_SORT_ASCENDING,
  GTK_SORT_DESCENDING
} GtkSortType;

/**
 * GtkPrintPages:
 * @GTK_PRINT_PAGES_ALL: All pages.
 * @GTK_PRINT_PAGES_CURRENT: Current page.
 * @GTK_PRINT_PAGES_RANGES: Range of pages.
 * @GTK_PRINT_PAGES_SELECTION: Selected pages.
 *
 * See also gtk_print_job_set_pages()
 */
typedef enum
{
  GTK_PRINT_PAGES_ALL,
  GTK_PRINT_PAGES_CURRENT,
  GTK_PRINT_PAGES_RANGES,
  GTK_PRINT_PAGES_SELECTION
} GtkPrintPages;

/**
 * GtkPageSet:
 * @GTK_PAGE_SET_ALL: All pages.
 * @GTK_PAGE_SET_EVEN: Even pages.
 * @GTK_PAGE_SET_ODD: Odd pages.
 *
 * See also gtk_print_job_set_page_set().
 */
typedef enum
{
  GTK_PAGE_SET_ALL,
  GTK_PAGE_SET_EVEN,
  GTK_PAGE_SET_ODD
} GtkPageSet;

/**
 * GtkNumberUpLayout:
 * @GTK_NUMBER_UP_LAYOUT_LEFT_TO_RIGHT_TOP_TO_BOTTOM: ![](layout-lrtb.png)
 * @GTK_NUMBER_UP_LAYOUT_LEFT_TO_RIGHT_BOTTOM_TO_TOP: ![](layout-lrbt.png)
 * @GTK_NUMBER_UP_LAYOUT_RIGHT_TO_LEFT_TOP_TO_BOTTOM: ![](layout-rltb.png)
 * @GTK_NUMBER_UP_LAYOUT_RIGHT_TO_LEFT_BOTTOM_TO_TOP: ![](layout-rlbt.png)
 * @GTK_NUMBER_UP_LAYOUT_TOP_TO_BOTTOM_LEFT_TO_RIGHT: ![](layout-tblr.png)
 * @GTK_NUMBER_UP_LAYOUT_TOP_TO_BOTTOM_RIGHT_TO_LEFT: ![](layout-tbrl.png)
 * @GTK_NUMBER_UP_LAYOUT_BOTTOM_TO_TOP_LEFT_TO_RIGHT: ![](layout-btlr.png)
 * @GTK_NUMBER_UP_LAYOUT_BOTTOM_TO_TOP_RIGHT_TO_LEFT: ![](layout-btrl.png)
 *
 * Used to determine the layout of pages on a sheet when printing
 * multiple pages per sheet.
 */
typedef enum
{
  GTK_NUMBER_UP_LAYOUT_LEFT_TO_RIGHT_TOP_TO_BOTTOM, /*< nick=lrtb >*/
  GTK_NUMBER_UP_LAYOUT_LEFT_TO_RIGHT_BOTTOM_TO_TOP, /*< nick=lrbt >*/
  GTK_NUMBER_UP_LAYOUT_RIGHT_TO_LEFT_TOP_TO_BOTTOM, /*< nick=rltb >*/
  GTK_NUMBER_UP_LAYOUT_RIGHT_TO_LEFT_BOTTOM_TO_TOP, /*< nick=rlbt >*/
  GTK_NUMBER_UP_LAYOUT_TOP_TO_BOTTOM_LEFT_TO_RIGHT, /*< nick=tblr >*/
  GTK_NUMBER_UP_LAYOUT_TOP_TO_BOTTOM_RIGHT_TO_LEFT, /*< nick=tbrl >*/
  GTK_NUMBER_UP_LAYOUT_BOTTOM_TO_TOP_LEFT_TO_RIGHT, /*< nick=btlr >*/
  GTK_NUMBER_UP_LAYOUT_BOTTOM_TO_TOP_RIGHT_TO_LEFT  /*< nick=btrl >*/
} GtkNumberUpLayout;

/**
 * GtkOrdering:
 * @GTK_ORDERING_SMALLER: the first value is smaller than the second
 * @GTK_ORDERING_EQUAL: the two values are equal
 * @GTK_ORDERING_LARGER: the first value is larger than the second
 *
 * Describes the way two values can be compared.
 *
 * These values can be used with a [callback@GLib.CompareFunc]. However,
 * a `GCompareFunc` is allowed to return any integer values.
 * For converting such a value to a `GtkOrdering` value, use
 * [func@Gtk.Ordering.from_cmpfunc].
 */
typedef enum {
  GTK_ORDERING_SMALLER = -1,
  GTK_ORDERING_EQUAL = 0,
  GTK_ORDERING_LARGER = 1
} GtkOrdering;

/* The GI scanner does not handle static inline functions, because
 * of the `static` keyword; so we clip this out when parsing the
 * header, and we replace it with a real function in gtksorter.c
 * that only exists when parsing the source for introspection.
 */
#ifdef __GI_SCANNER__
GtkOrdering     gtk_ordering_from_cmpfunc       (int cmpfunc_result);
#else
/**
 * gtk_ordering_from_cmpfunc:
 * @cmpfunc_result: Result of a comparison function
 *
 * Converts the result of a `GCompareFunc` like strcmp() to a
 * `GtkOrdering` value.
 *
 * Returns: the corresponding `GtkOrdering`
 **/
static inline GtkOrdering
gtk_ordering_from_cmpfunc (int cmpfunc_result)
{
  return (GtkOrdering) ((cmpfunc_result > 0) - (cmpfunc_result < 0));
}
#endif

/**
 * GtkPageOrientation:
 * @GTK_PAGE_ORIENTATION_PORTRAIT: Portrait mode.
 * @GTK_PAGE_ORIENTATION_LANDSCAPE: Landscape mode.
 * @GTK_PAGE_ORIENTATION_REVERSE_PORTRAIT: Reverse portrait mode.
 * @GTK_PAGE_ORIENTATION_REVERSE_LANDSCAPE: Reverse landscape mode.
 *
 * See also gtk_print_settings_set_orientation().
 */
typedef enum
{
  GTK_PAGE_ORIENTATION_PORTRAIT,
  GTK_PAGE_ORIENTATION_LANDSCAPE,
  GTK_PAGE_ORIENTATION_REVERSE_PORTRAIT,
  GTK_PAGE_ORIENTATION_REVERSE_LANDSCAPE
} GtkPageOrientation;

/**
 * GtkPrintQuality:
 * @GTK_PRINT_QUALITY_LOW: Low quality.
 * @GTK_PRINT_QUALITY_NORMAL: Normal quality.
 * @GTK_PRINT_QUALITY_HIGH: High quality.
 * @GTK_PRINT_QUALITY_DRAFT: Draft quality.
 *
 * See also gtk_print_settings_set_quality().
 */
typedef enum
{
  GTK_PRINT_QUALITY_LOW,
  GTK_PRINT_QUALITY_NORMAL,
  GTK_PRINT_QUALITY_HIGH,
  GTK_PRINT_QUALITY_DRAFT
} GtkPrintQuality;

/**
 * GtkPrintDuplex:
 * @GTK_PRINT_DUPLEX_SIMPLEX: No duplex.
 * @GTK_PRINT_DUPLEX_HORIZONTAL: Horizontal duplex.
 * @GTK_PRINT_DUPLEX_VERTICAL: Vertical duplex.
 *
 * See also gtk_print_settings_set_duplex().
 */
typedef enum
{
  GTK_PRINT_DUPLEX_SIMPLEX,
  GTK_PRINT_DUPLEX_HORIZONTAL,
  GTK_PRINT_DUPLEX_VERTICAL
} GtkPrintDuplex;


/**
 * GtkUnit:
 * @GTK_UNIT_NONE: No units.
 * @GTK_UNIT_POINTS: Dimensions in points.
 * @GTK_UNIT_INCH: Dimensions in inches.
 * @GTK_UNIT_MM: Dimensions in millimeters
 *
 * See also gtk_print_settings_set_paper_width().
 */
typedef enum
{
  GTK_UNIT_NONE,
  GTK_UNIT_POINTS,
  GTK_UNIT_INCH,
  GTK_UNIT_MM
} GtkUnit;

#define GTK_UNIT_PIXEL GTK_UNIT_NONE

/**
 * GtkTreeViewGridLines:
 * @GTK_TREE_VIEW_GRID_LINES_NONE: No grid lines.
 * @GTK_TREE_VIEW_GRID_LINES_HORIZONTAL: Horizontal grid lines.
 * @GTK_TREE_VIEW_GRID_LINES_VERTICAL: Vertical grid lines.
 * @GTK_TREE_VIEW_GRID_LINES_BOTH: Horizontal and vertical grid lines.
 *
 * Used to indicate which grid lines to draw in a tree view.
 */
typedef enum
{
  GTK_TREE_VIEW_GRID_LINES_NONE,
  GTK_TREE_VIEW_GRID_LINES_HORIZONTAL,
  GTK_TREE_VIEW_GRID_LINES_VERTICAL,
  GTK_TREE_VIEW_GRID_LINES_BOTH
} GtkTreeViewGridLines;

/**
 * GtkSizeGroupMode:
 * @GTK_SIZE_GROUP_NONE: group has no effect
 * @GTK_SIZE_GROUP_HORIZONTAL: group affects horizontal requisition
 * @GTK_SIZE_GROUP_VERTICAL: group affects vertical requisition
 * @GTK_SIZE_GROUP_BOTH: group affects both horizontal and vertical requisition
 *
 * The mode of the size group determines the directions in which the size
 * group affects the requested sizes of its component widgets.
 **/
typedef enum {
  GTK_SIZE_GROUP_NONE,
  GTK_SIZE_GROUP_HORIZONTAL,
  GTK_SIZE_GROUP_VERTICAL,
  GTK_SIZE_GROUP_BOTH
} GtkSizeGroupMode;

/**
 * GtkSizeRequestMode:
 * @GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH: Prefer height-for-width geometry management
 * @GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT: Prefer width-for-height geometry management
 * @GTK_SIZE_REQUEST_CONSTANT_SIZE: Don’t trade height-for-width or width-for-height
 *
 * Specifies a preference for height-for-width or
 * width-for-height geometry management.
 */
typedef enum
{
  GTK_SIZE_REQUEST_HEIGHT_FOR_WIDTH = 0,
  GTK_SIZE_REQUEST_WIDTH_FOR_HEIGHT,
  GTK_SIZE_REQUEST_CONSTANT_SIZE
} GtkSizeRequestMode;

/**
 * GtkScrollablePolicy:
 * @GTK_SCROLL_MINIMUM: Scrollable adjustments are based on the minimum size
 * @GTK_SCROLL_NATURAL: Scrollable adjustments are based on the natural size
 *
 * Defines the policy to be used in a scrollable widget when updating
 * the scrolled window adjustments in a given orientation.
 */
typedef enum
{
  GTK_SCROLL_MINIMUM = 0,
  GTK_SCROLL_NATURAL
} GtkScrollablePolicy;

/**
 * GtkStateFlags:
 * @GTK_STATE_FLAG_NORMAL: State during normal operation
 * @GTK_STATE_FLAG_ACTIVE: Widget is active
 * @GTK_STATE_FLAG_PRELIGHT: Widget has a mouse pointer over it
 * @GTK_STATE_FLAG_SELECTED: Widget is selected
 * @GTK_STATE_FLAG_INSENSITIVE: Widget is insensitive
 * @GTK_STATE_FLAG_INCONSISTENT: Widget is inconsistent
 * @GTK_STATE_FLAG_FOCUSED: Widget has the keyboard focus
 * @GTK_STATE_FLAG_BACKDROP: Widget is in a background toplevel window
 * @GTK_STATE_FLAG_DIR_LTR: Widget is in left-to-right text direction
 * @GTK_STATE_FLAG_DIR_RTL: Widget is in right-to-left text direction
 * @GTK_STATE_FLAG_LINK: Widget is a link
 * @GTK_STATE_FLAG_VISITED: The location the widget points to has already been visited
 * @GTK_STATE_FLAG_CHECKED: Widget is checked
 * @GTK_STATE_FLAG_DROP_ACTIVE: Widget is highlighted as a drop target for DND
 * @GTK_STATE_FLAG_FOCUS_VISIBLE: Widget has the visible focus
 * @GTK_STATE_FLAG_FOCUS_WITHIN: Widget contains the keyboard focus
 *
 * Describes a widget state.
 *
 * Widget states are used to match the widget against CSS pseudo-classes.
 * Note that GTK extends the regular CSS classes and sometimes uses
 * different names.
 */
typedef enum
{
  GTK_STATE_FLAG_NORMAL        = 0,
  GTK_STATE_FLAG_ACTIVE        = 1 << 0,
  GTK_STATE_FLAG_PRELIGHT      = 1 << 1,
  GTK_STATE_FLAG_SELECTED      = 1 << 2,
  GTK_STATE_FLAG_INSENSITIVE   = 1 << 3,
  GTK_STATE_FLAG_INCONSISTENT  = 1 << 4,
  GTK_STATE_FLAG_FOCUSED       = 1 << 5,
  GTK_STATE_FLAG_BACKDROP      = 1 << 6,
  GTK_STATE_FLAG_DIR_LTR       = 1 << 7,
  GTK_STATE_FLAG_DIR_RTL       = 1 << 8,
  GTK_STATE_FLAG_LINK          = 1 << 9,
  GTK_STATE_FLAG_VISITED       = 1 << 10,
  GTK_STATE_FLAG_CHECKED       = 1 << 11,
  GTK_STATE_FLAG_DROP_ACTIVE   = 1 << 12,
  GTK_STATE_FLAG_FOCUS_VISIBLE = 1 << 13,
  GTK_STATE_FLAG_FOCUS_WITHIN  = 1 << 14
} GtkStateFlags;

/**
 * GtkBorderStyle:
 * @GTK_BORDER_STYLE_NONE: No visible border
 * @GTK_BORDER_STYLE_HIDDEN: Same as %GTK_BORDER_STYLE_NONE
 * @GTK_BORDER_STYLE_SOLID: A single line segment
 * @GTK_BORDER_STYLE_INSET: Looks as if the content is sunken into the canvas
 * @GTK_BORDER_STYLE_OUTSET: Looks as if the content is coming out of the canvas
 * @GTK_BORDER_STYLE_DOTTED: A series of round dots
 * @GTK_BORDER_STYLE_DASHED: A series of square-ended dashes
 * @GTK_BORDER_STYLE_DOUBLE: Two parallel lines with some space between them
 * @GTK_BORDER_STYLE_GROOVE: Looks as if it were carved in the canvas
 * @GTK_BORDER_STYLE_RIDGE: Looks as if it were coming out of the canvas
 *
 * Describes how the border of a UI element should be rendered.
 */
typedef enum {
  GTK_BORDER_STYLE_NONE,
  GTK_BORDER_STYLE_HIDDEN,
  GTK_BORDER_STYLE_SOLID,
  GTK_BORDER_STYLE_INSET,
  GTK_BORDER_STYLE_OUTSET,
  GTK_BORDER_STYLE_DOTTED,
  GTK_BORDER_STYLE_DASHED,
  GTK_BORDER_STYLE_DOUBLE,
  GTK_BORDER_STYLE_GROOVE,
  GTK_BORDER_STYLE_RIDGE
} GtkBorderStyle;

/**
 * GtkLevelBarMode:
 * @GTK_LEVEL_BAR_MODE_CONTINUOUS: the bar has a continuous mode
 * @GTK_LEVEL_BAR_MODE_DISCRETE: the bar has a discrete mode
 *
 * Describes how [class@LevelBar] contents should be rendered.
 *
 * Note that this enumeration could be extended with additional modes
 * in the future.
 */
typedef enum {
  GTK_LEVEL_BAR_MODE_CONTINUOUS,
  GTK_LEVEL_BAR_MODE_DISCRETE
} GtkLevelBarMode;

G_END_DECLS

/**
 * GtkInputPurpose:
 * @GTK_INPUT_PURPOSE_FREE_FORM: Allow any character
 * @GTK_INPUT_PURPOSE_ALPHA: Allow only alphabetic characters
 * @GTK_INPUT_PURPOSE_DIGITS: Allow only digits
 * @GTK_INPUT_PURPOSE_NUMBER: Edited field expects numbers
 * @GTK_INPUT_PURPOSE_PHONE: Edited field expects phone number
 * @GTK_INPUT_PURPOSE_URL: Edited field expects URL
 * @GTK_INPUT_PURPOSE_EMAIL: Edited field expects email address
 * @GTK_INPUT_PURPOSE_NAME: Edited field expects the name of a person
 * @GTK_INPUT_PURPOSE_PASSWORD: Like %GTK_INPUT_PURPOSE_FREE_FORM, but characters are hidden
 * @GTK_INPUT_PURPOSE_PIN: Like %GTK_INPUT_PURPOSE_DIGITS, but characters are hidden
 * @GTK_INPUT_PURPOSE_TERMINAL: Allow any character, in addition to control codes
 *
 * Describes primary purpose of the input widget.
 *
 * This information is useful for on-screen keyboards and similar input
 * methods to decide which keys should be presented to the user.
 *
 * Note that the purpose is not meant to impose a totally strict rule
 * about allowed characters, and does not replace input validation.
 * It is fine for an on-screen keyboard to let the user override the
 * character set restriction that is expressed by the purpose. The
 * application is expected to validate the entry contents, even if
 * it specified a purpose.
 *
 * The difference between %GTK_INPUT_PURPOSE_DIGITS and
 * %GTK_INPUT_PURPOSE_NUMBER is that the former accepts only digits
 * while the latter also some punctuation (like commas or points, plus,
 * minus) and “e” or “E” as in 3.14E+000.
 *
 * This enumeration may be extended in the future; input methods should
 * interpret unknown values as “free form”.
 */
typedef enum
{
  GTK_INPUT_PURPOSE_FREE_FORM,
  GTK_INPUT_PURPOSE_ALPHA,
  GTK_INPUT_PURPOSE_DIGITS,
  GTK_INPUT_PURPOSE_NUMBER,
  GTK_INPUT_PURPOSE_PHONE,
  GTK_INPUT_PURPOSE_URL,
  GTK_INPUT_PURPOSE_EMAIL,
  GTK_INPUT_PURPOSE_NAME,
  GTK_INPUT_PURPOSE_PASSWORD,
  GTK_INPUT_PURPOSE_PIN,
  GTK_INPUT_PURPOSE_TERMINAL,
} GtkInputPurpose;

/**
 * GtkInputHints:
 * @GTK_INPUT_HINT_NONE: No special behaviour suggested
 * @GTK_INPUT_HINT_SPELLCHECK: Suggest checking for typos
 * @GTK_INPUT_HINT_NO_SPELLCHECK: Suggest not checking for typos
 * @GTK_INPUT_HINT_WORD_COMPLETION: Suggest word completion
 * @GTK_INPUT_HINT_LOWERCASE: Suggest to convert all text to lowercase
 * @GTK_INPUT_HINT_UPPERCASE_CHARS: Suggest to capitalize all text
 * @GTK_INPUT_HINT_UPPERCASE_WORDS: Suggest to capitalize the first
 *   character of each word
 * @GTK_INPUT_HINT_UPPERCASE_SENTENCES: Suggest to capitalize the
 *   first word of each sentence
 * @GTK_INPUT_HINT_INHIBIT_OSK: Suggest to not show an onscreen keyboard
 *   (e.g for a calculator that already has all the keys).
 * @GTK_INPUT_HINT_VERTICAL_WRITING: The text is vertical
 * @GTK_INPUT_HINT_EMOJI: Suggest offering Emoji support
 * @GTK_INPUT_HINT_NO_EMOJI: Suggest not offering Emoji support
 * @GTK_INPUT_HINT_PRIVATE: Request that the input method should not
 *    update personalized data (like typing history)
 *
 * Describes hints that might be taken into account by input methods
 * or applications.
 *
 * Note that input methods may already tailor their behaviour according
 * to the [enum@InputPurpose] of the entry.
 *
 * Some common sense is expected when using these flags - mixing
 * %GTK_INPUT_HINT_LOWERCASE with any of the uppercase hints makes no sense.
 *
 * This enumeration may be extended in the future; input methods should
 * ignore unknown values.
 */
typedef enum
{
  GTK_INPUT_HINT_NONE                = 0,
  GTK_INPUT_HINT_SPELLCHECK          = 1 << 0,
  GTK_INPUT_HINT_NO_SPELLCHECK       = 1 << 1,
  GTK_INPUT_HINT_WORD_COMPLETION     = 1 << 2,
  GTK_INPUT_HINT_LOWERCASE           = 1 << 3,
  GTK_INPUT_HINT_UPPERCASE_CHARS     = 1 << 4,
  GTK_INPUT_HINT_UPPERCASE_WORDS     = 1 << 5,
  GTK_INPUT_HINT_UPPERCASE_SENTENCES = 1 << 6,
  GTK_INPUT_HINT_INHIBIT_OSK         = 1 << 7,
  GTK_INPUT_HINT_VERTICAL_WRITING    = 1 << 8,
  GTK_INPUT_HINT_EMOJI               = 1 << 9,
  GTK_INPUT_HINT_NO_EMOJI            = 1 << 10,
  GTK_INPUT_HINT_PRIVATE             = 1 << 11,
} GtkInputHints;

/**
 * GtkPropagationPhase:
 * @GTK_PHASE_NONE: Events are not delivered.
 * @GTK_PHASE_CAPTURE: Events are delivered in the capture phase. The
 *   capture phase happens before the bubble phase, runs from the toplevel down
 *   to the event widget. This option should only be used on containers that
 *   might possibly handle events before their children do.
 * @GTK_PHASE_BUBBLE: Events are delivered in the bubble phase. The bubble
 *   phase happens after the capture phase, and before the default handlers
 *   are run. This phase runs from the event widget, up to the toplevel.
 * @GTK_PHASE_TARGET: Events are delivered in the default widget event handlers,
 *   note that widget implementations must chain up on button, motion, touch and
 *   grab broken handlers for controllers in this phase to be run.
 *
 * Describes the stage at which events are fed into a [class@EventController].
 */
typedef enum
{
  GTK_PHASE_NONE,
  GTK_PHASE_CAPTURE,
  GTK_PHASE_BUBBLE,
  GTK_PHASE_TARGET
} GtkPropagationPhase;

/**
 * GtkPropagationLimit:
 * @GTK_LIMIT_NONE: Events are handled regardless of what their
 *   target is.
 * @GTK_LIMIT_SAME_NATIVE: Events are only handled if their target
 *   is in the same [iface@Native] as the event controllers widget. Note
 *   that some event types have two targets (origin and destination).
 *
 * Describes limits of a [class@EventController] for handling events
 * targeting other widgets.
 */
typedef enum
{
  GTK_LIMIT_NONE,
  GTK_LIMIT_SAME_NATIVE
} GtkPropagationLimit;

/**
 * GtkEventSequenceState:
 * @GTK_EVENT_SEQUENCE_NONE: The sequence is handled, but not grabbed.
 * @GTK_EVENT_SEQUENCE_CLAIMED: The sequence is handled and grabbed.
 * @GTK_EVENT_SEQUENCE_DENIED: The sequence is denied.
 *
 * Describes the state of a [struct@Gdk.EventSequence] in a [class@Gesture].
 */
typedef enum
{
  GTK_EVENT_SEQUENCE_NONE,
  GTK_EVENT_SEQUENCE_CLAIMED,
  GTK_EVENT_SEQUENCE_DENIED
} GtkEventSequenceState;

/**
 * GtkPanDirection:
 * @GTK_PAN_DIRECTION_LEFT: panned towards the left
 * @GTK_PAN_DIRECTION_RIGHT: panned towards the right
 * @GTK_PAN_DIRECTION_UP: panned upwards
 * @GTK_PAN_DIRECTION_DOWN: panned downwards
 *
 * Describes the panning direction of a [class@GesturePan].
 */
typedef enum
{
  GTK_PAN_DIRECTION_LEFT,
  GTK_PAN_DIRECTION_RIGHT,
  GTK_PAN_DIRECTION_UP,
  GTK_PAN_DIRECTION_DOWN
} GtkPanDirection;

/**
 * GtkShortcutScope:
 * @GTK_SHORTCUT_SCOPE_LOCAL: Shortcuts are handled inside
 *   the widget the controller belongs to.
 * @GTK_SHORTCUT_SCOPE_MANAGED: Shortcuts are handled by
 *   the first ancestor that is a [iface@ShortcutManager]
 * @GTK_SHORTCUT_SCOPE_GLOBAL: Shortcuts are handled by
 *   the root widget.
 *
 * Describes where [class@Shortcut]s added to a
 * [class@ShortcutController] get handled.
 */
typedef enum
{
  GTK_SHORTCUT_SCOPE_LOCAL,
  GTK_SHORTCUT_SCOPE_MANAGED,
  GTK_SHORTCUT_SCOPE_GLOBAL
} GtkShortcutScope;

/**
 * GtkPickFlags:
 * @GTK_PICK_DEFAULT: The default behavior, include widgets that are receiving events
 * @GTK_PICK_INSENSITIVE: Include widgets that are insensitive
 * @GTK_PICK_NON_TARGETABLE: Include widgets that are marked as non-targetable. See [property@Widget:can-target]
 *
 * Flags that influence the behavior of [method@Widget.pick].
 */
typedef enum {
  GTK_PICK_DEFAULT        = 0,
  GTK_PICK_INSENSITIVE    = 1 << 0,
  GTK_PICK_NON_TARGETABLE = 1 << 1
} GtkPickFlags;

/**
 * GtkConstraintRelation:
 * @GTK_CONSTRAINT_RELATION_EQ: Equal
 * @GTK_CONSTRAINT_RELATION_LE: Less than, or equal
 * @GTK_CONSTRAINT_RELATION_GE: Greater than, or equal
 *
 * The relation between two terms of a constraint.
 */
typedef enum {
  GTK_CONSTRAINT_RELATION_LE = -1,
  GTK_CONSTRAINT_RELATION_EQ = 0,
  GTK_CONSTRAINT_RELATION_GE = 1
} GtkConstraintRelation;

/**
 * GtkConstraintStrength:
 * @GTK_CONSTRAINT_STRENGTH_REQUIRED: The constraint is required towards solving the layout
 * @GTK_CONSTRAINT_STRENGTH_STRONG: A strong constraint
 * @GTK_CONSTRAINT_STRENGTH_MEDIUM: A medium constraint
 * @GTK_CONSTRAINT_STRENGTH_WEAK: A weak constraint
 *
 * The strength of a constraint, expressed as a symbolic constant.
 *
 * The strength of a [class@Constraint] can be expressed with any positive
 * integer; the values of this enumeration can be used for readability.
 */
typedef enum {
  GTK_CONSTRAINT_STRENGTH_REQUIRED = 1001001000,
  GTK_CONSTRAINT_STRENGTH_STRONG   = 1000000000,
  GTK_CONSTRAINT_STRENGTH_MEDIUM   = 1000,
  GTK_CONSTRAINT_STRENGTH_WEAK     = 1
} GtkConstraintStrength;

/**
 * GtkConstraintAttribute:
 * @GTK_CONSTRAINT_ATTRIBUTE_NONE: No attribute, used for constant
 *   relations
 * @GTK_CONSTRAINT_ATTRIBUTE_LEFT: The left edge of a widget, regardless of
 *   text direction
 * @GTK_CONSTRAINT_ATTRIBUTE_RIGHT: The right edge of a widget, regardless
 *   of text direction
 * @GTK_CONSTRAINT_ATTRIBUTE_TOP: The top edge of a widget
 * @GTK_CONSTRAINT_ATTRIBUTE_BOTTOM: The bottom edge of a widget
 * @GTK_CONSTRAINT_ATTRIBUTE_START: The leading edge of a widget, depending
 *   on text direction; equivalent to %GTK_CONSTRAINT_ATTRIBUTE_LEFT for LTR
 *   languages, and %GTK_CONSTRAINT_ATTRIBUTE_RIGHT for RTL ones
 * @GTK_CONSTRAINT_ATTRIBUTE_END: The trailing edge of a widget, depending
 *   on text direction; equivalent to %GTK_CONSTRAINT_ATTRIBUTE_RIGHT for LTR
 *   languages, and %GTK_CONSTRAINT_ATTRIBUTE_LEFT for RTL ones
 * @GTK_CONSTRAINT_ATTRIBUTE_WIDTH: The width of a widget
 * @GTK_CONSTRAINT_ATTRIBUTE_HEIGHT: The height of a widget
 * @GTK_CONSTRAINT_ATTRIBUTE_CENTER_X: The center of a widget, on the
 *   horizontal axis
 * @GTK_CONSTRAINT_ATTRIBUTE_CENTER_Y: The center of a widget, on the
 *   vertical axis
 * @GTK_CONSTRAINT_ATTRIBUTE_BASELINE: The baseline of a widget
 *
 * The widget attributes that can be used when creating a [class@Constraint].
 */
typedef enum {
  GTK_CONSTRAINT_ATTRIBUTE_NONE,
  GTK_CONSTRAINT_ATTRIBUTE_LEFT,
  GTK_CONSTRAINT_ATTRIBUTE_RIGHT,
  GTK_CONSTRAINT_ATTRIBUTE_TOP,
  GTK_CONSTRAINT_ATTRIBUTE_BOTTOM,
  GTK_CONSTRAINT_ATTRIBUTE_START,
  GTK_CONSTRAINT_ATTRIBUTE_END,
  GTK_CONSTRAINT_ATTRIBUTE_WIDTH,
  GTK_CONSTRAINT_ATTRIBUTE_HEIGHT,
  GTK_CONSTRAINT_ATTRIBUTE_CENTER_X,
  GTK_CONSTRAINT_ATTRIBUTE_CENTER_Y,
  GTK_CONSTRAINT_ATTRIBUTE_BASELINE
} GtkConstraintAttribute;

/**
 * GtkConstraintVflParserError:
 * @GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_SYMBOL: Invalid or unknown symbol
 * @GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_ATTRIBUTE: Invalid or unknown attribute
 * @GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_VIEW: Invalid or unknown view
 * @GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_METRIC: Invalid or unknown metric
 * @GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_PRIORITY: Invalid or unknown priority
 * @GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_RELATION: Invalid or unknown relation
 *
 * Domain for VFL parsing errors.
 */
typedef enum {
  GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_SYMBOL,
  GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_ATTRIBUTE,
  GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_VIEW,
  GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_METRIC,
  GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_PRIORITY,
  GTK_CONSTRAINT_VFL_PARSER_ERROR_INVALID_RELATION
} GtkConstraintVflParserError;

/**
 * GtkSystemSetting:
 * @GTK_SYSTEM_SETTING_DPI: the [property@Gtk.Settings:gtk-xft-dpi] setting has changed
 * @GTK_SYSTEM_SETTING_FONT_NAME: The [property@Gtk.Settings:gtk-font-name] setting has changed
 * @GTK_SYSTEM_SETTING_FONT_CONFIG: The font configuration has changed in a way that
 *   requires text to be redrawn. This can be any of the
 *   [property@Gtk.Settings:gtk-xft-antialias],
 *   [property@Gtk.Settings:gtk-xft-hinting],
 *   [property@Gtk.Settings:gtk-xft-hintstyle],
 *   [property@Gtk.Settings:gtk-xft-rgba] or
 *   [property@Gtk.Settings:gtk-fontconfig-timestamp] settings
 * @GTK_SYSTEM_SETTING_DISPLAY: The display has changed
 * @GTK_SYSTEM_SETTING_ICON_THEME: The icon theme has changed in a way that requires
 *   icons to be looked up again
 *
 * Values that can be passed to the [vfunc@Gtk.Widget.system_setting_changed]
 * vfunc.
 *
 * The values indicate which system setting has changed.
 * Widgets may need to drop caches, or react otherwise.
 *
 * Most of the values correspond to [class@Settings] properties.
 *
 * More values may be added over time.
 */
typedef enum {
  GTK_SYSTEM_SETTING_DPI,
  GTK_SYSTEM_SETTING_FONT_NAME,
  GTK_SYSTEM_SETTING_FONT_CONFIG,
  GTK_SYSTEM_SETTING_DISPLAY,
  GTK_SYSTEM_SETTING_ICON_THEME
} GtkSystemSetting;

/**
 * GtkSymbolicColor:
 * @GTK_SYMBOLIC_COLOR_FOREGROUND: The default foreground color
 * @GTK_SYMBOLIC_COLOR_ERROR: Indication color for errors
 * @GTK_SYMBOLIC_COLOR_WARNING: Indication color for warnings
 * @GTK_SYMBOLIC_COLOR_SUCCESS: Indication color for success
 *
 * The indexes of colors passed to symbolic color rendering, such as
 * [vfunc@Gtk.SymbolicPaintable.snapshot_symbolic].
 *
 * More values may be added over time.
 *
 * Since: 4.6
 */
typedef enum {
  GTK_SYMBOLIC_COLOR_FOREGROUND = 0,
  GTK_SYMBOLIC_COLOR_ERROR = 1,
  GTK_SYMBOLIC_COLOR_WARNING = 2,
  GTK_SYMBOLIC_COLOR_SUCCESS = 3
} GtkSymbolicColor;

/**
 * GtkAccessibleRole:
 * @GTK_ACCESSIBLE_ROLE_ALERT: An element with important, and usually
 *   time-sensitive, information
 * @GTK_ACCESSIBLE_ROLE_ALERT_DIALOG: A type of dialog that contains an
 *   alert message
 * @GTK_ACCESSIBLE_ROLE_BANNER: Unused
 * @GTK_ACCESSIBLE_ROLE_BUTTON: An input element that allows for
 *   user-triggered actions when clicked or pressed
 * @GTK_ACCESSIBLE_ROLE_CAPTION: Unused
 * @GTK_ACCESSIBLE_ROLE_CELL: Unused
 * @GTK_ACCESSIBLE_ROLE_CHECKBOX: A checkable input element that has
 *   three possible values: `true`, `false`, or `mixed`
 * @GTK_ACCESSIBLE_ROLE_COLUMN_HEADER: A header in a columned list.
 * @GTK_ACCESSIBLE_ROLE_COMBO_BOX: An input that controls another element,
 *   such as a list or a grid, that can dynamically pop up to help the user
 *   set the value of the input
 * @GTK_ACCESSIBLE_ROLE_COMMAND: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_COMPOSITE: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_DIALOG: A dialog is a window that is designed to interrupt
 *    the current processing of an application in order to prompt the user to enter
 *    information or require a response.
 * @GTK_ACCESSIBLE_ROLE_DOCUMENT: Unused
 * @GTK_ACCESSIBLE_ROLE_FEED: Unused
 * @GTK_ACCESSIBLE_ROLE_FORM: Unused
 * @GTK_ACCESSIBLE_ROLE_GENERIC: Unused
 * @GTK_ACCESSIBLE_ROLE_GRID: A grid of items.
 * @GTK_ACCESSIBLE_ROLE_GRID_CELL: An item in a grid or tree grid.
 * @GTK_ACCESSIBLE_ROLE_GROUP: An element that groups multiple widgets. GTK uses
 *   this role for various containers, like [class@Box], [class@Viewport], and [class@HeaderBar].
 * @GTK_ACCESSIBLE_ROLE_HEADING: Unused
 * @GTK_ACCESSIBLE_ROLE_IMG: An image.
 * @GTK_ACCESSIBLE_ROLE_INPUT: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_LABEL: A visible name or caption for a user interface component.
 * @GTK_ACCESSIBLE_ROLE_LANDMARK: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_LEGEND: Unused
 * @GTK_ACCESSIBLE_ROLE_LINK: A clickable link.
 * @GTK_ACCESSIBLE_ROLE_LIST: A list of items.
 * @GTK_ACCESSIBLE_ROLE_LIST_BOX: Unused.
 * @GTK_ACCESSIBLE_ROLE_LIST_ITEM: An item in a list.
 * @GTK_ACCESSIBLE_ROLE_LOG: Unused
 * @GTK_ACCESSIBLE_ROLE_MAIN: Unused
 * @GTK_ACCESSIBLE_ROLE_MARQUEE: Unused
 * @GTK_ACCESSIBLE_ROLE_MATH: Unused
 * @GTK_ACCESSIBLE_ROLE_METER: An element that represents a value within a known range.
 * @GTK_ACCESSIBLE_ROLE_MENU: A menu.
 * @GTK_ACCESSIBLE_ROLE_MENU_BAR: A menubar.
 * @GTK_ACCESSIBLE_ROLE_MENU_ITEM: An item in a menu.
 * @GTK_ACCESSIBLE_ROLE_MENU_ITEM_CHECKBOX: A check item in a menu.
 * @GTK_ACCESSIBLE_ROLE_MENU_ITEM_RADIO: A radio item in a menu.
 * @GTK_ACCESSIBLE_ROLE_NAVIGATION: Unused
 * @GTK_ACCESSIBLE_ROLE_NONE: An element that is not represented to accessibility technologies.
 * @GTK_ACCESSIBLE_ROLE_NOTE: Unused
 * @GTK_ACCESSIBLE_ROLE_OPTION: Unused
 * @GTK_ACCESSIBLE_ROLE_PRESENTATION: An element that is not represented to accessibility technologies.
 * @GTK_ACCESSIBLE_ROLE_PROGRESS_BAR: An element that displays the progress
 *    status for tasks that take a long time.
 * @GTK_ACCESSIBLE_ROLE_RADIO: A checkable input in a group of radio roles,
 *    only one of which can be checked at a time.
 * @GTK_ACCESSIBLE_ROLE_RADIO_GROUP: Unused
 * @GTK_ACCESSIBLE_ROLE_RANGE: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_REGION: Unused
 * @GTK_ACCESSIBLE_ROLE_ROW: A row in a columned list.
 * @GTK_ACCESSIBLE_ROLE_ROW_GROUP: Unused
 * @GTK_ACCESSIBLE_ROLE_ROW_HEADER: Unused
 * @GTK_ACCESSIBLE_ROLE_SCROLLBAR: A graphical object that controls the scrolling
 *    of content within a viewing area, regardless of whether the content is fully
 *    displayed within the viewing area.
 * @GTK_ACCESSIBLE_ROLE_SEARCH: Unused
 * @GTK_ACCESSIBLE_ROLE_SEARCH_BOX: A type of textbox intended for specifying
 *    search criteria.
 * @GTK_ACCESSIBLE_ROLE_SECTION: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_SECTION_HEAD: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_SELECT: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_SEPARATOR: A divider that separates and distinguishes
 *    sections of content or groups of menuitems.
 * @GTK_ACCESSIBLE_ROLE_SLIDER: A user input where the user selects a value
 *    from within a given range.
 * @GTK_ACCESSIBLE_ROLE_SPIN_BUTTON: A form of range that expects the user to
 *    select from among discrete choices.
 * @GTK_ACCESSIBLE_ROLE_STATUS: Unused
 * @GTK_ACCESSIBLE_ROLE_STRUCTURE: Abstract role.
 * @GTK_ACCESSIBLE_ROLE_SWITCH: A type of checkbox that represents on/off values,
 *    as opposed to checked/unchecked values.
 * @GTK_ACCESSIBLE_ROLE_TAB: An item in a list of tab used for switching pages.
 * @GTK_ACCESSIBLE_ROLE_TABLE: Unused
 * @GTK_ACCESSIBLE_ROLE_TAB_LIST: A list of tabs for switching pages.
 * @GTK_ACCESSIBLE_ROLE_TAB_PANEL: A page in a notebook or stack.
 * @GTK_ACCESSIBLE_ROLE_TEXT_BOX: A type of input that allows free-form text
 *    as its value.
 * @GTK_ACCESSIBLE_ROLE_TIME: Unused
 * @GTK_ACCESSIBLE_ROLE_TIMER: Unused
 * @GTK_ACCESSIBLE_ROLE_TOOLBAR: Unused
 * @GTK_ACCESSIBLE_ROLE_TOOLTIP: Unused
 * @GTK_ACCESSIBLE_ROLE_TREE: Unused
 * @GTK_ACCESSIBLE_ROLE_TREE_GRID: A treeview-like, columned list.
 * @GTK_ACCESSIBLE_ROLE_TREE_ITEM: Unused
 * @GTK_ACCESSIBLE_ROLE_WIDGET: An interactive component of a graphical user
 *    interface. This is the role that GTK uses by default for widgets.
 * @GTK_ACCESSIBLE_ROLE_WINDOW: An application window.
 *
 * The accessible role for a [iface@Accessible] implementation.
 *
 * Abstract roles are only used as part of the ontology; application
 * developers must not use abstract roles in their code.
 */
typedef enum {
  GTK_ACCESSIBLE_ROLE_ALERT,
  GTK_ACCESSIBLE_ROLE_ALERT_DIALOG,
  GTK_ACCESSIBLE_ROLE_BANNER,
  GTK_ACCESSIBLE_ROLE_BUTTON,
  GTK_ACCESSIBLE_ROLE_CAPTION,
  GTK_ACCESSIBLE_ROLE_CELL,
  GTK_ACCESSIBLE_ROLE_CHECKBOX,
  GTK_ACCESSIBLE_ROLE_COLUMN_HEADER,
  GTK_ACCESSIBLE_ROLE_COMBO_BOX,
  GTK_ACCESSIBLE_ROLE_COMMAND,
  GTK_ACCESSIBLE_ROLE_COMPOSITE,
  GTK_ACCESSIBLE_ROLE_DIALOG,
  GTK_ACCESSIBLE_ROLE_DOCUMENT,
  GTK_ACCESSIBLE_ROLE_FEED,
  GTK_ACCESSIBLE_ROLE_FORM,
  GTK_ACCESSIBLE_ROLE_GENERIC,
  GTK_ACCESSIBLE_ROLE_GRID,
  GTK_ACCESSIBLE_ROLE_GRID_CELL,
  GTK_ACCESSIBLE_ROLE_GROUP,
  GTK_ACCESSIBLE_ROLE_HEADING,
  GTK_ACCESSIBLE_ROLE_IMG,
  GTK_ACCESSIBLE_ROLE_INPUT,
  GTK_ACCESSIBLE_ROLE_LABEL,
  GTK_ACCESSIBLE_ROLE_LANDMARK,
  GTK_ACCESSIBLE_ROLE_LEGEND,
  GTK_ACCESSIBLE_ROLE_LINK,
  GTK_ACCESSIBLE_ROLE_LIST,
  GTK_ACCESSIBLE_ROLE_LIST_BOX,
  GTK_ACCESSIBLE_ROLE_LIST_ITEM,
  GTK_ACCESSIBLE_ROLE_LOG,
  GTK_ACCESSIBLE_ROLE_MAIN,
  GTK_ACCESSIBLE_ROLE_MARQUEE,
  GTK_ACCESSIBLE_ROLE_MATH,
  GTK_ACCESSIBLE_ROLE_METER,
  GTK_ACCESSIBLE_ROLE_MENU,
  GTK_ACCESSIBLE_ROLE_MENU_BAR,
  GTK_ACCESSIBLE_ROLE_MENU_ITEM,
  GTK_ACCESSIBLE_ROLE_MENU_ITEM_CHECKBOX,
  GTK_ACCESSIBLE_ROLE_MENU_ITEM_RADIO,
  GTK_ACCESSIBLE_ROLE_NAVIGATION,
  GTK_ACCESSIBLE_ROLE_NONE,
  GTK_ACCESSIBLE_ROLE_NOTE,
  GTK_ACCESSIBLE_ROLE_OPTION,
  GTK_ACCESSIBLE_ROLE_PRESENTATION,
  GTK_ACCESSIBLE_ROLE_PROGRESS_BAR,
  GTK_ACCESSIBLE_ROLE_RADIO,
  GTK_ACCESSIBLE_ROLE_RADIO_GROUP,
  GTK_ACCESSIBLE_ROLE_RANGE,
  GTK_ACCESSIBLE_ROLE_REGION,
  GTK_ACCESSIBLE_ROLE_ROW,
  GTK_ACCESSIBLE_ROLE_ROW_GROUP,
  GTK_ACCESSIBLE_ROLE_ROW_HEADER,
  GTK_ACCESSIBLE_ROLE_SCROLLBAR,
  GTK_ACCESSIBLE_ROLE_SEARCH,
  GTK_ACCESSIBLE_ROLE_SEARCH_BOX,
  GTK_ACCESSIBLE_ROLE_SECTION,
  GTK_ACCESSIBLE_ROLE_SECTION_HEAD,
  GTK_ACCESSIBLE_ROLE_SELECT,
  GTK_ACCESSIBLE_ROLE_SEPARATOR,
  GTK_ACCESSIBLE_ROLE_SLIDER,
  GTK_ACCESSIBLE_ROLE_SPIN_BUTTON,
  GTK_ACCESSIBLE_ROLE_STATUS,
  GTK_ACCESSIBLE_ROLE_STRUCTURE,
  GTK_ACCESSIBLE_ROLE_SWITCH,
  GTK_ACCESSIBLE_ROLE_TAB,
  GTK_ACCESSIBLE_ROLE_TABLE,
  GTK_ACCESSIBLE_ROLE_TAB_LIST,
  GTK_ACCESSIBLE_ROLE_TAB_PANEL,
  GTK_ACCESSIBLE_ROLE_TEXT_BOX,
  GTK_ACCESSIBLE_ROLE_TIME,
  GTK_ACCESSIBLE_ROLE_TIMER,
  GTK_ACCESSIBLE_ROLE_TOOLBAR,
  GTK_ACCESSIBLE_ROLE_TOOLTIP,
  GTK_ACCESSIBLE_ROLE_TREE,
  GTK_ACCESSIBLE_ROLE_TREE_GRID,
  GTK_ACCESSIBLE_ROLE_TREE_ITEM,
  GTK_ACCESSIBLE_ROLE_WIDGET,
  GTK_ACCESSIBLE_ROLE_WINDOW
} GtkAccessibleRole;

/**
 * GtkAccessibleState:
 * @GTK_ACCESSIBLE_STATE_BUSY: A “busy” state. This state has boolean values
 * @GTK_ACCESSIBLE_STATE_CHECKED: A “checked” state; indicates the current
 *   state of a [class@CheckButton]. Value type: [enum@AccessibleTristate]
 * @GTK_ACCESSIBLE_STATE_DISABLED: A “disabled” state; corresponds to the
 *   [property@Widget:sensitive] property. It indicates a UI element
 *   that is perceivable, but not editable or operable. Value type: boolean
 * @GTK_ACCESSIBLE_STATE_EXPANDED: An “expanded” state; corresponds to the
 *   [property@Expander:expanded] property. Value type: boolean
 *   or undefined
 * @GTK_ACCESSIBLE_STATE_HIDDEN: A “hidden” state; corresponds to the
 *   [property@Widget:visible] property. You can use this state
 *   explicitly on UI elements that should not be exposed to an assistive
 *   technology. Value type: boolean
 *   See also: %GTK_ACCESSIBLE_STATE_DISABLED
 * @GTK_ACCESSIBLE_STATE_INVALID: An “invalid” state; set when a widget
 *   is showing an error. Value type: [enum@AccessibleInvalidState]
 * @GTK_ACCESSIBLE_STATE_PRESSED: A “pressed” state; indicates the current
 *   state of a [class@ToggleButton]. Value type: [enum@AccessibleTristate]
 *   enumeration
 * @GTK_ACCESSIBLE_STATE_SELECTED: A “selected” state; set when a widget
 *   is selected. Value type: boolean or undefined
 *
 * The possible accessible states of a [iface@Accessible].
 */
typedef enum {
  GTK_ACCESSIBLE_STATE_BUSY,
  GTK_ACCESSIBLE_STATE_CHECKED,
  GTK_ACCESSIBLE_STATE_DISABLED,
  GTK_ACCESSIBLE_STATE_EXPANDED,
  GTK_ACCESSIBLE_STATE_HIDDEN,
  GTK_ACCESSIBLE_STATE_INVALID,
  GTK_ACCESSIBLE_STATE_PRESSED,
  GTK_ACCESSIBLE_STATE_SELECTED
} GtkAccessibleState;

/**
 * GTK_ACCESSIBLE_VALUE_UNDEFINED:
 *
 * An undefined value. The accessible attribute is either unset, or its
 * value is undefined.
 */
#define GTK_ACCESSIBLE_VALUE_UNDEFINED  (-1)

/**
 * GtkAccessibleProperty:
 * @GTK_ACCESSIBLE_PROPERTY_AUTOCOMPLETE: Indicates whether inputting text
 *    could trigger display of one or more predictions of the user's intended
 *    value for a combobox, searchbox, or textbox and specifies how predictions
 *    would be presented if they were made. Value type: [enum@AccessibleAutocomplete]
 * @GTK_ACCESSIBLE_PROPERTY_DESCRIPTION: Defines a string value that describes
 *    or annotates the current element. Value type: string
 * @GTK_ACCESSIBLE_PROPERTY_HAS_POPUP: Indicates the availability and type of
 *    interactive popup element, such as menu or dialog, that can be triggered
 *    by an element.
 * @GTK_ACCESSIBLE_PROPERTY_KEY_SHORTCUTS: Indicates keyboard shortcuts that an
 *    author has implemented to activate or give focus to an element. Value type:
 *    string
 * @GTK_ACCESSIBLE_PROPERTY_LABEL: Defines a string value that labels the current
 *    element. Value type: string
 * @GTK_ACCESSIBLE_PROPERTY_LEVEL: Defines the hierarchical level of an element
 *    within a structure. Value type: integer
 * @GTK_ACCESSIBLE_PROPERTY_MODAL: Indicates whether an element is modal when
 *    displayed. Value type: boolean
 * @GTK_ACCESSIBLE_PROPERTY_MULTI_LINE: Indicates whether a text box accepts
 *    multiple lines of input or only a single line. Value type: boolean
 * @GTK_ACCESSIBLE_PROPERTY_MULTI_SELECTABLE: Indicates that the user may select
 *    more than one item from the current selectable descendants. Value type:
 *    boolean
 * @GTK_ACCESSIBLE_PROPERTY_ORIENTATION: Indicates whether the element's
 *    orientation is horizontal, vertical, or unknown/ambiguous. Value type:
 *    [enum@Orientation]
 * @GTK_ACCESSIBLE_PROPERTY_PLACEHOLDER: Defines a short hint (a word or short
 *    phrase) intended to aid the user with data entry when the control has no
 *    value. A hint could be a sample value or a brief description of the expected
 *    format. Value type: string
 * @GTK_ACCESSIBLE_PROPERTY_READ_ONLY: Indicates that the element is not editable,
 *    but is otherwise operable. Value type: boolean
 * @GTK_ACCESSIBLE_PROPERTY_REQUIRED: Indicates that user input is required on
 *    the element before a form may be submitted. Value type: boolean
 * @GTK_ACCESSIBLE_PROPERTY_ROLE_DESCRIPTION: Defines a human-readable,
 *    author-localized description for the role of an element. Value type: string
 * @GTK_ACCESSIBLE_PROPERTY_SORT: Indicates if items in a table or grid are
 *    sorted in ascending or descending order. Value type: [enum@AccessibleSort]
 * @GTK_ACCESSIBLE_PROPERTY_VALUE_MAX: Defines the maximum allowed value for a
 *    range widget. Value type: double
 * @GTK_ACCESSIBLE_PROPERTY_VALUE_MIN: Defines the minimum allowed value for a
 *    range widget. Value type: double
 * @GTK_ACCESSIBLE_PROPERTY_VALUE_NOW: Defines the current value for a range widget.
 *    Value type: double
 * @GTK_ACCESSIBLE_PROPERTY_VALUE_TEXT: Defines the human readable text alternative
 *    of aria-valuenow for a range widget. Value type: string
 *
 * The possible accessible properties of a [iface@Accessible].
 */
typedef enum {
  GTK_ACCESSIBLE_PROPERTY_AUTOCOMPLETE,
  GTK_ACCESSIBLE_PROPERTY_DESCRIPTION,
  GTK_ACCESSIBLE_PROPERTY_HAS_POPUP,
  GTK_ACCESSIBLE_PROPERTY_KEY_SHORTCUTS,
  GTK_ACCESSIBLE_PROPERTY_LABEL,
  GTK_ACCESSIBLE_PROPERTY_LEVEL,
  GTK_ACCESSIBLE_PROPERTY_MODAL,
  GTK_ACCESSIBLE_PROPERTY_MULTI_LINE,
  GTK_ACCESSIBLE_PROPERTY_MULTI_SELECTABLE,
  GTK_ACCESSIBLE_PROPERTY_ORIENTATION,
  GTK_ACCESSIBLE_PROPERTY_PLACEHOLDER,
  GTK_ACCESSIBLE_PROPERTY_READ_ONLY,
  GTK_ACCESSIBLE_PROPERTY_REQUIRED,
  GTK_ACCESSIBLE_PROPERTY_ROLE_DESCRIPTION,
  GTK_ACCESSIBLE_PROPERTY_SORT,
  GTK_ACCESSIBLE_PROPERTY_VALUE_MAX,
  GTK_ACCESSIBLE_PROPERTY_VALUE_MIN,
  GTK_ACCESSIBLE_PROPERTY_VALUE_NOW,
  GTK_ACCESSIBLE_PROPERTY_VALUE_TEXT
} GtkAccessibleProperty;

/**
 * GtkAccessibleRelation:
 * @GTK_ACCESSIBLE_RELATION_ACTIVE_DESCENDANT: Identifies the currently active
 *    element when focus is on a composite widget, combobox, textbox, group,
 *    or application. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_COL_COUNT: Defines the total number of columns
 *    in a table, grid, or treegrid. Value type: integer
 * @GTK_ACCESSIBLE_RELATION_COL_INDEX: Defines an element's column index or
 *    position with respect to the total number of columns within a table,
 *    grid, or treegrid. Value type: integer
 * @GTK_ACCESSIBLE_RELATION_COL_INDEX_TEXT: Defines a human readable text
 *   alternative of %GTK_ACCESSIBLE_RELATION_COL_INDEX. Value type: string
 * @GTK_ACCESSIBLE_RELATION_COL_SPAN: Defines the number of columns spanned
 *   by a cell or gridcell within a table, grid, or treegrid. Value type: integer
 * @GTK_ACCESSIBLE_RELATION_CONTROLS: Identifies the element (or elements) whose
 *    contents or presence are controlled by the current element. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_DESCRIBED_BY: Identifies the element (or elements)
 *    that describes the object. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_DETAILS: Identifies the element (or elements) that
 *    provide additional information related to the object. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_ERROR_MESSAGE: Identifies the element that provides
 *    an error message for an object. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_FLOW_TO: Identifies the next element (or elements)
 *    in an alternate reading order of content which, at the user's discretion,
 *    allows assistive technology to override the general default of reading in
 *    document source order. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_LABELLED_BY: Identifies the element (or elements)
 *    that labels the current element. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_OWNS: Identifies an element (or elements) in order
 *    to define a visual, functional, or contextual parent/child relationship
 *    between elements where the widget hierarchy cannot be used to represent
 *    the relationship. Value type: reference
 * @GTK_ACCESSIBLE_RELATION_POS_IN_SET: Defines an element's number or position
 *    in the current set of listitems or treeitems. Value type: integer
 * @GTK_ACCESSIBLE_RELATION_ROW_COUNT: Defines the total number of rows in a table,
 *    grid, or treegrid. Value type: integer
 * @GTK_ACCESSIBLE_RELATION_ROW_INDEX: Defines an element's row index or position
 *    with respect to the total number of rows within a table, grid, or treegrid.
 *    Value type: integer
 * @GTK_ACCESSIBLE_RELATION_ROW_INDEX_TEXT: Defines a human readable text
 *    alternative of aria-rowindex. Value type: string
 * @GTK_ACCESSIBLE_RELATION_ROW_SPAN: Defines the number of rows spanned by a
 *    cell or gridcell within a table, grid, or treegrid. Value type: integer
 * @GTK_ACCESSIBLE_RELATION_SET_SIZE: Defines the number of items in the current
 *    set of listitems or treeitems. Value type: integer
 *
 * The possible accessible relations of a [iface@Accessible].
 *
 * Accessible relations can be references to other widgets,
 * integers or strings.
 */
typedef enum {
  GTK_ACCESSIBLE_RELATION_ACTIVE_DESCENDANT,
  GTK_ACCESSIBLE_RELATION_COL_COUNT,
  GTK_ACCESSIBLE_RELATION_COL_INDEX,
  GTK_ACCESSIBLE_RELATION_COL_INDEX_TEXT,
  GTK_ACCESSIBLE_RELATION_COL_SPAN,
  GTK_ACCESSIBLE_RELATION_CONTROLS,
  GTK_ACCESSIBLE_RELATION_DESCRIBED_BY,
  GTK_ACCESSIBLE_RELATION_DETAILS,
  GTK_ACCESSIBLE_RELATION_ERROR_MESSAGE,
  GTK_ACCESSIBLE_RELATION_FLOW_TO,
  GTK_ACCESSIBLE_RELATION_LABELLED_BY,
  GTK_ACCESSIBLE_RELATION_OWNS,
  GTK_ACCESSIBLE_RELATION_POS_IN_SET,
  GTK_ACCESSIBLE_RELATION_ROW_COUNT,
  GTK_ACCESSIBLE_RELATION_ROW_INDEX,
  GTK_ACCESSIBLE_RELATION_ROW_INDEX_TEXT,
  GTK_ACCESSIBLE_RELATION_ROW_SPAN,
  GTK_ACCESSIBLE_RELATION_SET_SIZE
} GtkAccessibleRelation;

/**
 * GtkAccessibleTristate:
 * @GTK_ACCESSIBLE_TRISTATE_FALSE: The state is `false`
 * @GTK_ACCESSIBLE_TRISTATE_TRUE: The state is `true`
 * @GTK_ACCESSIBLE_TRISTATE_MIXED: The state is `mixed`
 *
 * The possible values for the %GTK_ACCESSIBLE_STATE_PRESSED
 * accessible state.
 *
 * Note that the %GTK_ACCESSIBLE_TRISTATE_FALSE and
 * %GTK_ACCESSIBLE_TRISTATE_TRUE have the same values
 * as %FALSE and %TRUE.
 */
typedef enum {
  GTK_ACCESSIBLE_TRISTATE_FALSE,
  GTK_ACCESSIBLE_TRISTATE_TRUE,
  GTK_ACCESSIBLE_TRISTATE_MIXED
} GtkAccessibleTristate;

/**
 * GtkAccessibleInvalidState:
 * @GTK_ACCESSIBLE_INVALID_FALSE: There are no detected errors in the value
 * @GTK_ACCESSIBLE_INVALID_TRUE: The value entered by the user has failed validation
 * @GTK_ACCESSIBLE_INVALID_GRAMMAR: A grammatical error was detected
 * @GTK_ACCESSIBLE_INVALID_SPELLING: A spelling error was detected
 *
 * The possible values for the %GTK_ACCESSIBLE_STATE_INVALID
 * accessible state.
 *
 * Note that the %GTK_ACCESSIBLE_INVALID_FALSE and
 * %GTK_ACCESSIBLE_INVALID_TRUE have the same values
 * as %FALSE and %TRUE.
 */
typedef enum { /*< prefix=GTK_ACCESSIBLE_INVALID >*/
  GTK_ACCESSIBLE_INVALID_FALSE,
  GTK_ACCESSIBLE_INVALID_TRUE,
  GTK_ACCESSIBLE_INVALID_GRAMMAR,
  GTK_ACCESSIBLE_INVALID_SPELLING,
} GtkAccessibleInvalidState;

/**
 * GtkAccessibleAutocomplete:
 * @GTK_ACCESSIBLE_AUTOCOMPLETE_NONE: Automatic suggestions are not displayed.
 * @GTK_ACCESSIBLE_AUTOCOMPLETE_INLINE: When a user is providing input, text
 *    suggesting one way to complete the provided input may be dynamically
 *    inserted after the caret.
 * @GTK_ACCESSIBLE_AUTOCOMPLETE_LIST: When a user is providing input, an element
 *    containing a collection of values that could complete the provided input
 *    may be displayed.
 * @GTK_ACCESSIBLE_AUTOCOMPLETE_BOTH: When a user is providing input, an element
 *    containing a collection of values that could complete the provided input
 *    may be displayed. If displayed, one value in the collection is automatically
 *    selected, and the text needed to complete the automatically selected value
 *    appears after the caret in the input.
 *
 * The possible values for the %GTK_ACCESSIBLE_PROPERTY_AUTOCOMPLETE
 * accessible property.
 */
typedef enum { /*< prefix=GTK_ACCESSIBLE_AUTOCOMPLETE >*/
  GTK_ACCESSIBLE_AUTOCOMPLETE_NONE,
  GTK_ACCESSIBLE_AUTOCOMPLETE_INLINE,
  GTK_ACCESSIBLE_AUTOCOMPLETE_LIST,
  GTK_ACCESSIBLE_AUTOCOMPLETE_BOTH
} GtkAccessibleAutocomplete;

/**
 * GtkAccessibleSort:
 * @GTK_ACCESSIBLE_SORT_NONE: There is no defined sort applied to the column.
 * @GTK_ACCESSIBLE_SORT_ASCENDING: Items are sorted in ascending order by this column.
 * @GTK_ACCESSIBLE_SORT_DESCENDING: Items are sorted in descending order by this column.
 * @GTK_ACCESSIBLE_SORT_OTHER: A sort algorithm other than ascending or
 *    descending has been applied.
 *
 * The possible values for the %GTK_ACCESSIBLE_PROPERTY_SORT
 * accessible property.
 */
typedef enum { /*< prefix=GTK_ACCESSIBLE_SORT >*/
  GTK_ACCESSIBLE_SORT_NONE,
  GTK_ACCESSIBLE_SORT_ASCENDING,
  GTK_ACCESSIBLE_SORT_DESCENDING,
  GTK_ACCESSIBLE_SORT_OTHER
} GtkAccessibleSort;

#endif /* __GTK_ENUMS_H__ */
