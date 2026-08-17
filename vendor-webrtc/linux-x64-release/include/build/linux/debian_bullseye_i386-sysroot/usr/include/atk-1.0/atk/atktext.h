/* ATK - The Accessibility Toolkit for GTK+
 * Copyright 2001 Sun Microsystems Inc.
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
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __ATK_TEXT_H__
#define __ATK_TEXT_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <glib-object.h>
#include <atk/atkobject.h>
#include <atk/atkutil.h>
#include <atk/atkcomponent.h>

G_BEGIN_DECLS

/**
 *AtkTextAttribute:
 *@ATK_TEXT_ATTR_INVALID: Invalid attribute, like bad spelling or grammar.
 *@ATK_TEXT_ATTR_LEFT_MARGIN: The pixel width of the left margin
 *@ATK_TEXT_ATTR_RIGHT_MARGIN: The pixel width of the right margin
 *@ATK_TEXT_ATTR_INDENT: The number of pixels that the text is indented
 *@ATK_TEXT_ATTR_INVISIBLE: Either "true" or "false" indicating whether text is visible or not
 *@ATK_TEXT_ATTR_EDITABLE: Either "true" or "false" indicating whether text is editable or not
 *@ATK_TEXT_ATTR_PIXELS_ABOVE_LINES: Pixels of blank space to leave above each newline-terminated line. 
 *@ATK_TEXT_ATTR_PIXELS_BELOW_LINES: Pixels of blank space to leave below each newline-terminated line.
 *@ATK_TEXT_ATTR_PIXELS_INSIDE_WRAP: Pixels of blank space to leave between wrapped lines inside the same newline-terminated line (paragraph).
 *@ATK_TEXT_ATTR_BG_FULL_HEIGHT: "true" or "false" whether to make the background color for each character the height of the highest font used on the current line, or the height of the font used for the current character.
 *@ATK_TEXT_ATTR_RISE: Number of pixels that the characters are risen above the baseline. See also ATK_TEXT_ATTR_TEXT_POSITION.
 *@ATK_TEXT_ATTR_UNDERLINE: "none", "single", "double", "low", or "error"
 *@ATK_TEXT_ATTR_STRIKETHROUGH: "true" or "false" whether the text is strikethrough 
 *@ATK_TEXT_ATTR_SIZE: The size of the characters in points. eg: 10
 *@ATK_TEXT_ATTR_SCALE: The scale of the characters. The value is a string representation of a double 
 *@ATK_TEXT_ATTR_WEIGHT: The weight of the characters.
 *@ATK_TEXT_ATTR_LANGUAGE: The language used
 *@ATK_TEXT_ATTR_FAMILY_NAME: The font family name
 *@ATK_TEXT_ATTR_BG_COLOR: The background color. The value is an RGB value of the format "%u,%u,%u"
 *@ATK_TEXT_ATTR_FG_COLOR:The foreground color. The value is an RGB value of the format "%u,%u,%u"
 *@ATK_TEXT_ATTR_BG_STIPPLE: "true" if a #GdkBitmap is set for stippling the background color.
 *@ATK_TEXT_ATTR_FG_STIPPLE: "true" if a #GdkBitmap is set for stippling the foreground color.
 *@ATK_TEXT_ATTR_WRAP_MODE: The wrap mode of the text, if any. Values are "none", "char", "word", or "word_char".
 *@ATK_TEXT_ATTR_DIRECTION: The direction of the text, if set. Values are "none", "ltr" or "rtl" 
 *@ATK_TEXT_ATTR_JUSTIFICATION: The justification of the text, if set. Values are "left", "right", "center" or "fill" 
 *@ATK_TEXT_ATTR_STRETCH: The stretch of the text, if set. Values are "ultra_condensed", "extra_condensed", "condensed", "semi_condensed", "normal", "semi_expanded", "expanded", "extra_expanded" or "ultra_expanded"
 *@ATK_TEXT_ATTR_VARIANT: The capitalization variant of the text, if set. Values are "normal" or "small_caps"
 *@ATK_TEXT_ATTR_STYLE: The slant style of the text, if set. Values are "normal", "oblique" or "italic"
 *@ATK_TEXT_ATTR_TEXT_POSITION: The vertical position with respect to the baseline. Values are "baseline", "super", or "sub". Note that a super or sub text attribute refers to position with respect to the baseline of the prior character.
 *@ATK_TEXT_ATTR_LAST_DEFINED: not a valid text attribute, used for finding end of enumeration
 *
 * Describes the text attributes supported
 **/
typedef enum
{
  ATK_TEXT_ATTR_INVALID = 0,
  ATK_TEXT_ATTR_LEFT_MARGIN,
  ATK_TEXT_ATTR_RIGHT_MARGIN,
  ATK_TEXT_ATTR_INDENT,
  ATK_TEXT_ATTR_INVISIBLE,
  ATK_TEXT_ATTR_EDITABLE,
  ATK_TEXT_ATTR_PIXELS_ABOVE_LINES,
  ATK_TEXT_ATTR_PIXELS_BELOW_LINES,
  ATK_TEXT_ATTR_PIXELS_INSIDE_WRAP,
  ATK_TEXT_ATTR_BG_FULL_HEIGHT,
  ATK_TEXT_ATTR_RISE,
  ATK_TEXT_ATTR_UNDERLINE,
  ATK_TEXT_ATTR_STRIKETHROUGH,
  ATK_TEXT_ATTR_SIZE,
  ATK_TEXT_ATTR_SCALE,
  ATK_TEXT_ATTR_WEIGHT,
  ATK_TEXT_ATTR_LANGUAGE,
  ATK_TEXT_ATTR_FAMILY_NAME,
  ATK_TEXT_ATTR_BG_COLOR,
  ATK_TEXT_ATTR_FG_COLOR,
  ATK_TEXT_ATTR_BG_STIPPLE,
  ATK_TEXT_ATTR_FG_STIPPLE,
  ATK_TEXT_ATTR_WRAP_MODE,
  ATK_TEXT_ATTR_DIRECTION,
  ATK_TEXT_ATTR_JUSTIFICATION,
  ATK_TEXT_ATTR_STRETCH,
  ATK_TEXT_ATTR_VARIANT,
  ATK_TEXT_ATTR_STYLE,
  ATK_TEXT_ATTR_TEXT_POSITION,
  ATK_TEXT_ATTR_LAST_DEFINED
} AtkTextAttribute;

ATK_AVAILABLE_IN_ALL
AtkTextAttribute         atk_text_attribute_register   (const gchar *name);


#define ATK_TYPE_TEXT                    (atk_text_get_type ())
#define ATK_IS_TEXT(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_TEXT)
#define ATK_TEXT(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_TEXT, AtkText)
#define ATK_TEXT_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_TEXT, AtkTextIface))

#ifndef _TYPEDEF_ATK_TEXT_
#define _TYPEDEF_ATK_TEXT_
typedef struct _AtkText AtkText;
#endif
typedef struct _AtkTextIface AtkTextIface;


/**
 *AtkTextBoundary:
 *@ATK_TEXT_BOUNDARY_CHAR: Boundary is the boundary between characters
 * (including non-printing characters)
 *@ATK_TEXT_BOUNDARY_WORD_START: Boundary is the start (i.e. first character) of a word.
 *@ATK_TEXT_BOUNDARY_WORD_END: Boundary is the end (i.e. last
 * character) of a word.
 *@ATK_TEXT_BOUNDARY_SENTENCE_START: Boundary is the first character in a sentence.
 *@ATK_TEXT_BOUNDARY_SENTENCE_END: Boundary is the last (terminal)
 * character in a sentence; in languages which use "sentence stop"
 * punctuation such as English, the boundary is thus the '.', '?', or
 * similar terminal punctuation character.
 *@ATK_TEXT_BOUNDARY_LINE_START: Boundary is the initial character of the content or a
 * character immediately following a newline, linefeed, or return character.
 *@ATK_TEXT_BOUNDARY_LINE_END: Boundary is the linefeed, or return
 * character.
 *
 * Text boundary types used for specifying boundaries for regions of text.
 * This enumeration is deprecated since 2.9.4 and should not be used. Use
 * AtkTextGranularity with #atk_text_get_string_at_offset instead.
 **/
typedef enum {
  ATK_TEXT_BOUNDARY_CHAR,
  ATK_TEXT_BOUNDARY_WORD_START,
  ATK_TEXT_BOUNDARY_WORD_END,
  ATK_TEXT_BOUNDARY_SENTENCE_START,
  ATK_TEXT_BOUNDARY_SENTENCE_END,
  ATK_TEXT_BOUNDARY_LINE_START,
  ATK_TEXT_BOUNDARY_LINE_END
} AtkTextBoundary;

/**
 *AtkTextGranularity:
 *@ATK_TEXT_GRANULARITY_CHAR: Granularity is defined by the boundaries between characters
 * (including non-printing characters)
 *@ATK_TEXT_GRANULARITY_WORD: Granularity is defined by the boundaries of a word,
 * starting at the beginning of the current word and finishing at the beginning of
 * the following one, if present.
 *@ATK_TEXT_GRANULARITY_SENTENCE: Granularity is defined by the boundaries of a sentence,
 * starting at the beginning of the current sentence and finishing at the beginning of
 * the following one, if present.
 *@ATK_TEXT_GRANULARITY_LINE: Granularity is defined by the boundaries of a line,
 * starting at the beginning of the current line and finishing at the beginning of
 * the following one, if present.
 *@ATK_TEXT_GRANULARITY_PARAGRAPH: Granularity is defined by the boundaries of a paragraph,
 * starting at the beginning of the current paragraph and finishing at the beginning of
 * the following one, if present.
 *
 * Text granularity types used for specifying the granularity of the region of
 * text we are interested in.
 **/
typedef enum {
  ATK_TEXT_GRANULARITY_CHAR,
  ATK_TEXT_GRANULARITY_WORD,
  ATK_TEXT_GRANULARITY_SENTENCE,
  ATK_TEXT_GRANULARITY_LINE,
  ATK_TEXT_GRANULARITY_PARAGRAPH
} AtkTextGranularity;

/**
 * AtkTextRectangle:
 * @x: The horizontal coordinate of a rectangle
 * @y: The vertical coordinate of a rectangle
 * @width: The width of a rectangle
 * @height: The height of a rectangle
 *
 * A structure used to store a rectangle used by AtkText.
 **/

typedef struct _AtkTextRectangle AtkTextRectangle;

struct _AtkTextRectangle {
  gint x;
  gint y;
  gint width;
  gint height;
};

/**
 * AtkTextRange:
 * @bounds: A rectangle giving the bounds of the text range
 * @start_offset: The start offset of a AtkTextRange
 * @end_offset: The end offset of a AtkTextRange
 * @content: The text in the text range
 *
 * A structure used to describe a text range.
 **/
typedef struct _AtkTextRange AtkTextRange;

struct _AtkTextRange {
  AtkTextRectangle bounds;
  gint start_offset;
  gint end_offset;
  gchar* content;
};

ATK_AVAILABLE_IN_ALL
GType atk_text_range_get_type (void);

/**
 *AtkTextClipType:
 *@ATK_TEXT_CLIP_NONE: No clipping to be done
 *@ATK_TEXT_CLIP_MIN: Text clipped by min coordinate is omitted
 *@ATK_TEXT_CLIP_MAX: Text clipped by max coordinate is omitted
 *@ATK_TEXT_CLIP_BOTH: Only text fully within mix/max bound is retained
 *
 *Describes the type of clipping required.
 **/
typedef enum {
    ATK_TEXT_CLIP_NONE,
    ATK_TEXT_CLIP_MIN,
    ATK_TEXT_CLIP_MAX,
    ATK_TEXT_CLIP_BOTH
} AtkTextClipType;

/**
 * AtkTextIface:
 * @get_text_after_offset: Gets specified text. This virtual function
 *   is deprecated and it should not be overridden.
 * @get_text_at_offset: Gets specified text. This virtual function
 *   is deprecated and it should not be overridden.
 * @get_text_before_offset: Gets specified text. This virtual function
 *   is deprecated and it should not be overridden.
 * @get_string_at_offset: Gets a portion of the text exposed through
 *   an AtkText according to a given offset and a specific
 *   granularity, along with the start and end offsets defining the
 *   boundaries of such a portion of text.
 * @text_changed: the signal handler which is executed when there is a
 *   text change. This virtual function is deprecated sice 2.9.4 and
 *   it should not be overriden.
 */
struct _AtkTextIface
{
  GTypeInterface parent;

  gchar*         (* get_text)                     (AtkText          *text,
                                                   gint             start_offset,
                                                   gint             end_offset);
  gchar*         (* get_text_after_offset)        (AtkText          *text,
                                                   gint             offset,
                                                   AtkTextBoundary  boundary_type,
						   gint             *start_offset,
						   gint             *end_offset);
  gchar*         (* get_text_at_offset)           (AtkText          *text,
                                                   gint             offset,
                                                   AtkTextBoundary  boundary_type,
						   gint             *start_offset,
						   gint             *end_offset);
  gunichar       (* get_character_at_offset)      (AtkText          *text,
                                                   gint             offset);
  gchar*         (* get_text_before_offset)       (AtkText          *text,
                                                   gint             offset,
                                                   AtkTextBoundary  boundary_type,
 						   gint             *start_offset,
						   gint             *end_offset);
  gint           (* get_caret_offset)             (AtkText          *text);
  AtkAttributeSet* (* get_run_attributes)         (AtkText	    *text,
						   gint	  	    offset,
						   gint             *start_offset,
						   gint	 	    *end_offset);
  AtkAttributeSet* (* get_default_attributes)     (AtkText	    *text);
  void           (* get_character_extents)        (AtkText          *text,
                                                   gint             offset,
                                                   gint             *x,
                                                   gint             *y,
                                                   gint             *width,
                                                   gint             *height,
                                                   AtkCoordType	    coords);
  gint           (* get_character_count)          (AtkText          *text);
  gint           (* get_offset_at_point)          (AtkText          *text,
                                                   gint             x,
                                                   gint             y,
                                                   AtkCoordType	    coords);
  gint		 (* get_n_selections)		  (AtkText          *text);
  gchar*         (* get_selection)	          (AtkText          *text,
						   gint		    selection_num,
						   gint		    *start_offset,
						   gint		    *end_offset);
  gboolean       (* add_selection)		  (AtkText          *text,
						   gint		    start_offset,
						   gint		    end_offset);
  gboolean       (* remove_selection)		  (AtkText          *text,
						   gint             selection_num);
  gboolean       (* set_selection)		  (AtkText          *text,
						   gint		    selection_num,
						   gint		    start_offset,
						   gint		    end_offset);
  gboolean       (* set_caret_offset)             (AtkText          *text,
                                                   gint             offset);

  /*
   * signal handlers
   */
  void		 (* text_changed)                 (AtkText          *text,
                                                   gint             position,
                                                   gint             length);
  void           (* text_caret_moved)             (AtkText          *text,
                                                   gint             location);
  void           (* text_selection_changed)       (AtkText          *text);

  void           (* text_attributes_changed)      (AtkText          *text);


  void           (* get_range_extents)            (AtkText          *text,
                                                   gint             start_offset,
                                                   gint             end_offset,
                                                   AtkCoordType     coord_type,
                                                   AtkTextRectangle *rect);

  AtkTextRange** (* get_bounded_ranges)           (AtkText          *text,
                                                   AtkTextRectangle *rect,
                                                   AtkCoordType     coord_type,
                                                   AtkTextClipType  x_clip_type,
                                                   AtkTextClipType  y_clip_type);

  gchar*         (* get_string_at_offset)         (AtkText            *text,
                                                   gint               offset,
                                                   AtkTextGranularity granularity,
                                                   gint               *start_offset,
                                                   gint               *end_offset);
  /*
   * Scrolls this text range so it becomes visible on the screen.
   *
   * scroll_substring_to lets the implementation compute an appropriate target
   * position on the screen, with type used as a positioning hint.
   *
   * scroll_substring_to_point lets the client specify a precise target position
   * on the screen for the top-left of the substring.
   *
   * Since ATK 2.32
   */
  gboolean       (* scroll_substring_to)          (AtkText          *text,
                                                   gint             start_offset,
                                                   gint             end_offset,
                                                   AtkScrollType    type);
  gboolean       (* scroll_substring_to_point)    (AtkText          *text,
                                                   gint             start_offset,
                                                   gint             end_offset,
                                                   AtkCoordType     coords,
                                                   gint             x,
                                                   gint             y);
};

ATK_AVAILABLE_IN_ALL
GType            atk_text_get_type (void);


/*
 * Additional AtkObject properties used by AtkText:
 *    "accessible_text" (accessible text has changed)
 *    "accessible_caret" (accessible text cursor position changed:
 *                         editable text only)
 */

ATK_AVAILABLE_IN_ALL
gchar*        atk_text_get_text                           (AtkText          *text,
                                                           gint             start_offset,
                                                           gint             end_offset);
ATK_AVAILABLE_IN_ALL
gunichar      atk_text_get_character_at_offset            (AtkText          *text,
                                                           gint             offset);
ATK_DEPRECATED_IN_2_10_FOR(atk_text_get_string_at_offset)
gchar*        atk_text_get_text_after_offset              (AtkText          *text,
                                                           gint             offset,
                                                           AtkTextBoundary  boundary_type,
							   gint             *start_offset,
							   gint	            *end_offset);
ATK_DEPRECATED_IN_2_10_FOR(atk_text_get_string_at_offset)
gchar*        atk_text_get_text_at_offset                 (AtkText          *text,
                                                           gint             offset,
                                                           AtkTextBoundary  boundary_type,
							   gint             *start_offset,
							   gint             *end_offset);
ATK_DEPRECATED_IN_2_10_FOR(atk_text_get_string_at_offset)
gchar*        atk_text_get_text_before_offset             (AtkText          *text,
                                                           gint             offset,
                                                           AtkTextBoundary  boundary_type,
							   gint             *start_offset,
							   gint	            *end_offset);
ATK_AVAILABLE_IN_2_10
gchar*        atk_text_get_string_at_offset               (AtkText            *text,
                                                           gint               offset,
                                                           AtkTextGranularity granularity,
                                                           gint               *start_offset,
                                                           gint               *end_offset);
ATK_AVAILABLE_IN_ALL
gint          atk_text_get_caret_offset                   (AtkText          *text);
ATK_AVAILABLE_IN_ALL
void          atk_text_get_character_extents              (AtkText          *text,
                                                           gint             offset,
                                                           gint             *x,
                                                           gint             *y,
                                                           gint             *width,
                                                           gint             *height,
                                                           AtkCoordType	    coords);
ATK_AVAILABLE_IN_ALL
AtkAttributeSet* atk_text_get_run_attributes              (AtkText	    *text,
						           gint	  	    offset,
						           gint             *start_offset,
						           gint	 	    *end_offset);
ATK_AVAILABLE_IN_ALL
AtkAttributeSet* atk_text_get_default_attributes          (AtkText	    *text);
ATK_AVAILABLE_IN_ALL
gint          atk_text_get_character_count                (AtkText          *text);
ATK_AVAILABLE_IN_ALL
gint          atk_text_get_offset_at_point                (AtkText          *text,
                                                           gint             x,
                                                           gint             y,
                                                           AtkCoordType	    coords);
ATK_AVAILABLE_IN_ALL
gint          atk_text_get_n_selections			  (AtkText          *text);
ATK_AVAILABLE_IN_ALL
gchar*        atk_text_get_selection			  (AtkText          *text,
							   gint		    selection_num,
							   gint             *start_offset,
							   gint             *end_offset);
ATK_AVAILABLE_IN_ALL
gboolean      atk_text_add_selection                      (AtkText          *text,
							   gint             start_offset,
							   gint             end_offset);
ATK_AVAILABLE_IN_ALL
gboolean      atk_text_remove_selection                   (AtkText          *text,
							   gint		    selection_num);
ATK_AVAILABLE_IN_ALL
gboolean      atk_text_set_selection                      (AtkText          *text,
							   gint		    selection_num,
							   gint             start_offset,
							   gint             end_offset);
ATK_AVAILABLE_IN_ALL
gboolean      atk_text_set_caret_offset                   (AtkText          *text,
                                                           gint             offset);
ATK_AVAILABLE_IN_ALL
void          atk_text_get_range_extents                  (AtkText          *text,

                                                           gint             start_offset,
                                                           gint             end_offset,
                                                           AtkCoordType     coord_type,
                                                           AtkTextRectangle *rect);
ATK_AVAILABLE_IN_ALL
AtkTextRange**  atk_text_get_bounded_ranges               (AtkText          *text,
                                                           AtkTextRectangle *rect,
                                                           AtkCoordType     coord_type,
                                                           AtkTextClipType  x_clip_type,
                                                           AtkTextClipType  y_clip_type);
ATK_AVAILABLE_IN_ALL
void          atk_text_free_ranges                        (AtkTextRange     **ranges);
ATK_AVAILABLE_IN_ALL
void 	      atk_attribute_set_free                      (AtkAttributeSet  *attrib_set);
ATK_AVAILABLE_IN_ALL
const gchar*  atk_text_attribute_get_name                 (AtkTextAttribute attr);
ATK_AVAILABLE_IN_ALL
AtkTextAttribute       atk_text_attribute_for_name        (const gchar      *name);
ATK_AVAILABLE_IN_ALL
const gchar*  atk_text_attribute_get_value                (AtkTextAttribute attr,
                                                           gint             index_);

ATK_AVAILABLE_IN_ALL
gboolean      atk_text_scroll_substring_to                (AtkText          *text,
                                                           gint             start_offset,
                                                           gint             end_offset,
                                                           AtkScrollType    type);

ATK_AVAILABLE_IN_ALL
gboolean      atk_text_scroll_substring_to_point          (AtkText          *text,
                                                           gint             start_offset,
                                                           gint             end_offset,
                                                           AtkCoordType     coords,
                                                           gint             x,
                                                           gint             y);

G_END_DECLS

#endif /* __ATK_TEXT_H__ */
