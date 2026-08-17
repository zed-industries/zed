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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_ENUMS_H__
#define __GTK_ENUMS_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>

G_BEGIN_DECLS

/* Anchor types */
typedef enum
{
  GTK_ANCHOR_CENTER,
  GTK_ANCHOR_NORTH,
  GTK_ANCHOR_NORTH_WEST,
  GTK_ANCHOR_NORTH_EAST,
  GTK_ANCHOR_SOUTH,
  GTK_ANCHOR_SOUTH_WEST,
  GTK_ANCHOR_SOUTH_EAST,
  GTK_ANCHOR_WEST,
  GTK_ANCHOR_EAST,
  GTK_ANCHOR_N		= GTK_ANCHOR_NORTH,
  GTK_ANCHOR_NW		= GTK_ANCHOR_NORTH_WEST,
  GTK_ANCHOR_NE		= GTK_ANCHOR_NORTH_EAST,
  GTK_ANCHOR_S		= GTK_ANCHOR_SOUTH,
  GTK_ANCHOR_SW		= GTK_ANCHOR_SOUTH_WEST,
  GTK_ANCHOR_SE		= GTK_ANCHOR_SOUTH_EAST,
  GTK_ANCHOR_W		= GTK_ANCHOR_WEST,
  GTK_ANCHOR_E		= GTK_ANCHOR_EAST
} GtkAnchorType;

/* Arrow placement */
typedef enum
{
  GTK_ARROWS_BOTH,
  GTK_ARROWS_START,
  GTK_ARROWS_END
} GtkArrowPlacement;

/* Arrow types */
typedef enum
{
  GTK_ARROW_UP,
  GTK_ARROW_DOWN,
  GTK_ARROW_LEFT,
  GTK_ARROW_RIGHT,
  GTK_ARROW_NONE
} GtkArrowType;

/* Attach options (for tables) */
typedef enum
{
  GTK_EXPAND = 1 << 0,
  GTK_SHRINK = 1 << 1,
  GTK_FILL   = 1 << 2
} GtkAttachOptions;

/* Button box styles */
typedef enum
{
  GTK_BUTTONBOX_DEFAULT_STYLE,
  GTK_BUTTONBOX_SPREAD,
  GTK_BUTTONBOX_EDGE,
  GTK_BUTTONBOX_START,
  GTK_BUTTONBOX_END,
  GTK_BUTTONBOX_CENTER
} GtkButtonBoxStyle;

#ifndef GTK_DISABLE_DEPRECATED
/* Curve types */
typedef enum
{
  GTK_CURVE_TYPE_LINEAR,       /* linear interpolation */
  GTK_CURVE_TYPE_SPLINE,       /* spline interpolation */
  GTK_CURVE_TYPE_FREE          /* free form curve */
} GtkCurveType;
#endif

typedef enum
{
  GTK_DELETE_CHARS,
  GTK_DELETE_WORD_ENDS,           /* delete only the portion of the word to the
                                   * left/right of cursor if we're in the middle
                                   * of a word */
  GTK_DELETE_WORDS,
  GTK_DELETE_DISPLAY_LINES,
  GTK_DELETE_DISPLAY_LINE_ENDS,
  GTK_DELETE_PARAGRAPH_ENDS,      /* like C-k in Emacs (or its reverse) */
  GTK_DELETE_PARAGRAPHS,          /* C-k in pico, kill whole line */
  GTK_DELETE_WHITESPACE           /* M-\ in Emacs */
} GtkDeleteType;

/* Focus movement types */
typedef enum
{
  GTK_DIR_TAB_FORWARD,
  GTK_DIR_TAB_BACKWARD,
  GTK_DIR_UP,
  GTK_DIR_DOWN,
  GTK_DIR_LEFT,
  GTK_DIR_RIGHT
} GtkDirectionType;

/* Expander styles */
typedef enum
{
  GTK_EXPANDER_COLLAPSED,
  GTK_EXPANDER_SEMI_COLLAPSED,
  GTK_EXPANDER_SEMI_EXPANDED,
  GTK_EXPANDER_EXPANDED
} GtkExpanderStyle;

/* Built-in stock icon sizes */
typedef enum
{
  GTK_ICON_SIZE_INVALID,
  GTK_ICON_SIZE_MENU,
  GTK_ICON_SIZE_SMALL_TOOLBAR,
  GTK_ICON_SIZE_LARGE_TOOLBAR,
  GTK_ICON_SIZE_BUTTON,
  GTK_ICON_SIZE_DND,
  GTK_ICON_SIZE_DIALOG
} GtkIconSize;

/* automatic sensitivity */
typedef enum
{
  GTK_SENSITIVITY_AUTO,
  GTK_SENSITIVITY_ON,
  GTK_SENSITIVITY_OFF
} GtkSensitivityType;

#ifndef GTK_DISABLE_DEPRECATED
/* side types */
typedef enum
{
  GTK_SIDE_TOP,
  GTK_SIDE_BOTTOM,
  GTK_SIDE_LEFT,
  GTK_SIDE_RIGHT
} GtkSideType;
#endif /* GTK_DISABLE_DEPRECATED */

/* Reading directions for text */
typedef enum
{
  GTK_TEXT_DIR_NONE,
  GTK_TEXT_DIR_LTR,
  GTK_TEXT_DIR_RTL
} GtkTextDirection;

/* justification for label and maybe other widgets (text?) */
typedef enum
{
  GTK_JUSTIFY_LEFT,
  GTK_JUSTIFY_RIGHT,
  GTK_JUSTIFY_CENTER,
  GTK_JUSTIFY_FILL
} GtkJustification;

#ifndef GTK_DISABLE_DEPRECATED
/* GtkPatternSpec match types */
typedef enum
{
  GTK_MATCH_ALL,       /* "*A?A*" */
  GTK_MATCH_ALL_TAIL,  /* "*A?AA" */
  GTK_MATCH_HEAD,      /* "AAAA*" */
  GTK_MATCH_TAIL,      /* "*AAAA" */
  GTK_MATCH_EXACT,     /* "AAAAA" */
  GTK_MATCH_LAST
} GtkMatchType;
#endif /* GTK_DISABLE_DEPRECATED */

/* Menu keyboard movement types */
typedef enum
{
  GTK_MENU_DIR_PARENT,
  GTK_MENU_DIR_CHILD,
  GTK_MENU_DIR_NEXT,
  GTK_MENU_DIR_PREV
} GtkMenuDirectionType;

/**
 * GtkMessageType:
 * @GTK_MESSAGE_INFO: Informational message
 * @GTK_MESSAGE_WARNING: Nonfatal warning message
 * @GTK_MESSAGE_QUESTION: Question requiring a choice
 * @GTK_MESSAGE_ERROR: Fatal error message
 * @GTK_MESSAGE_OTHER: None of the above, doesn't get an icon
 *
 * The type of message being displayed in the dialog.
 */
typedef enum
{
  GTK_MESSAGE_INFO,
  GTK_MESSAGE_WARNING,
  GTK_MESSAGE_QUESTION,
  GTK_MESSAGE_ERROR,
  GTK_MESSAGE_OTHER
} GtkMessageType;

typedef enum
{
  GTK_PIXELS,
  GTK_INCHES,
  GTK_CENTIMETERS
} GtkMetricType;

typedef enum
{
  GTK_MOVEMENT_LOGICAL_POSITIONS, /* move by forw/back graphemes */
  GTK_MOVEMENT_VISUAL_POSITIONS,  /* move by left/right graphemes */
  GTK_MOVEMENT_WORDS,             /* move by forward/back words */
  GTK_MOVEMENT_DISPLAY_LINES,     /* move up/down lines (wrapped lines) */
  GTK_MOVEMENT_DISPLAY_LINE_ENDS, /* move to either end of a line */
  GTK_MOVEMENT_PARAGRAPHS,        /* move up/down paragraphs (newline-ended lines) */
  GTK_MOVEMENT_PARAGRAPH_ENDS,    /* move to either end of a paragraph */
  GTK_MOVEMENT_PAGES,	          /* move by pages */
  GTK_MOVEMENT_BUFFER_ENDS,       /* move to ends of the buffer */
  GTK_MOVEMENT_HORIZONTAL_PAGES   /* move horizontally by pages */
} GtkMovementStep;

typedef enum
{
  GTK_SCROLL_STEPS,
  GTK_SCROLL_PAGES,
  GTK_SCROLL_ENDS,
  GTK_SCROLL_HORIZONTAL_STEPS,
  GTK_SCROLL_HORIZONTAL_PAGES,
  GTK_SCROLL_HORIZONTAL_ENDS
} GtkScrollStep;

/* Orientation for toolbars, etc. */
typedef enum
{
  GTK_ORIENTATION_HORIZONTAL,
  GTK_ORIENTATION_VERTICAL
} GtkOrientation;

/* Placement type for scrolled window */
typedef enum
{
  GTK_CORNER_TOP_LEFT,
  GTK_CORNER_BOTTOM_LEFT,
  GTK_CORNER_TOP_RIGHT,
  GTK_CORNER_BOTTOM_RIGHT
} GtkCornerType;

/* Packing types (for boxes) */
typedef enum
{
  GTK_PACK_START,
  GTK_PACK_END
} GtkPackType;

/* priorities for path lookups */
typedef enum
{
  GTK_PATH_PRIO_LOWEST      = 0,
  GTK_PATH_PRIO_GTK	    = 4,
  GTK_PATH_PRIO_APPLICATION = 8,
  GTK_PATH_PRIO_THEME       = 10,
  GTK_PATH_PRIO_RC          = 12,
  GTK_PATH_PRIO_HIGHEST     = 15
} GtkPathPriorityType;
#define GTK_PATH_PRIO_MASK 0x0f

/* widget path types */
typedef enum
{
  GTK_PATH_WIDGET,
  GTK_PATH_WIDGET_CLASS,
  GTK_PATH_CLASS
} GtkPathType;

/* Scrollbar policy types (for scrolled windows) */
typedef enum
{
  GTK_POLICY_ALWAYS,
  GTK_POLICY_AUTOMATIC,
  GTK_POLICY_NEVER
} GtkPolicyType;

typedef enum
{
  GTK_POS_LEFT,
  GTK_POS_RIGHT,
  GTK_POS_TOP,
  GTK_POS_BOTTOM
} GtkPositionType;

#ifndef GTK_DISABLE_DEPRECATED
typedef enum
{
  GTK_PREVIEW_COLOR,
  GTK_PREVIEW_GRAYSCALE
} GtkPreviewType;
#endif /* GTK_DISABLE_DEPRECATED */

/* Style for buttons */
typedef enum
{
  GTK_RELIEF_NORMAL,
  GTK_RELIEF_HALF,
  GTK_RELIEF_NONE
} GtkReliefStyle;

/* Resize type */
typedef enum
{
  GTK_RESIZE_PARENT,		/* Pass resize request to the parent */
  GTK_RESIZE_QUEUE,		/* Queue resizes on this widget */
  GTK_RESIZE_IMMEDIATE		/* Perform the resizes now */
} GtkResizeMode;

#ifndef GTK_DISABLE_DEPRECATED
/* signal run types */
typedef enum			/*< flags >*/
{
  GTK_RUN_FIRST      = G_SIGNAL_RUN_FIRST,
  GTK_RUN_LAST       = G_SIGNAL_RUN_LAST,
  GTK_RUN_BOTH       = (GTK_RUN_FIRST | GTK_RUN_LAST),
  GTK_RUN_NO_RECURSE = G_SIGNAL_NO_RECURSE,
  GTK_RUN_ACTION     = G_SIGNAL_ACTION,
  GTK_RUN_NO_HOOKS   = G_SIGNAL_NO_HOOKS
} GtkSignalRunType;
#endif /* GTK_DISABLE_DEPRECATED */

/* scrolling types */
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

/* list selection modes */
typedef enum
{
  GTK_SELECTION_NONE,                             /* Nothing can be selected */
  GTK_SELECTION_SINGLE,
  GTK_SELECTION_BROWSE,
  GTK_SELECTION_MULTIPLE,
  GTK_SELECTION_EXTENDED = GTK_SELECTION_MULTIPLE /* Deprecated */
} GtkSelectionMode;

/* Shadow types */
typedef enum
{
  GTK_SHADOW_NONE,
  GTK_SHADOW_IN,
  GTK_SHADOW_OUT,
  GTK_SHADOW_ETCHED_IN,
  GTK_SHADOW_ETCHED_OUT
} GtkShadowType;

/* Widget states */
typedef enum
{
  GTK_STATE_NORMAL,
  GTK_STATE_ACTIVE,
  GTK_STATE_PRELIGHT,
  GTK_STATE_SELECTED,
  GTK_STATE_INSENSITIVE
} GtkStateType;

#if !defined(GTK_DISABLE_DEPRECATED) || defined (GTK_MENU_INTERNALS)
/* Directions for submenus */
typedef enum
{
  GTK_DIRECTION_LEFT,
  GTK_DIRECTION_RIGHT
} GtkSubmenuDirection;

/* Placement of submenus */
typedef enum
{
  GTK_TOP_BOTTOM,
  GTK_LEFT_RIGHT
} GtkSubmenuPlacement;
#endif /* GTK_DISABLE_DEPRECATED */

/* Style for toolbars */
typedef enum
{
  GTK_TOOLBAR_ICONS,
  GTK_TOOLBAR_TEXT,
  GTK_TOOLBAR_BOTH,
  GTK_TOOLBAR_BOTH_HORIZ
} GtkToolbarStyle;

/* Data update types (for ranges) */
typedef enum
{
  GTK_UPDATE_CONTINUOUS,
  GTK_UPDATE_DISCONTINUOUS,
  GTK_UPDATE_DELAYED
} GtkUpdateType;

/* Generic visibility flags */
typedef enum
{
  GTK_VISIBILITY_NONE,
  GTK_VISIBILITY_PARTIAL,
  GTK_VISIBILITY_FULL
} GtkVisibility;

/* Window position types */
typedef enum
{
  GTK_WIN_POS_NONE,
  GTK_WIN_POS_CENTER,
  GTK_WIN_POS_MOUSE,
  GTK_WIN_POS_CENTER_ALWAYS,
  GTK_WIN_POS_CENTER_ON_PARENT
} GtkWindowPosition;

/* Window types */
typedef enum
{
  GTK_WINDOW_TOPLEVEL,
  GTK_WINDOW_POPUP
} GtkWindowType;

/* Text wrap */
typedef enum
{
  GTK_WRAP_NONE,
  GTK_WRAP_CHAR,
  GTK_WRAP_WORD,
  GTK_WRAP_WORD_CHAR
} GtkWrapMode;

/* How to sort */
typedef enum
{
  GTK_SORT_ASCENDING,
  GTK_SORT_DESCENDING
} GtkSortType;

/* Style for gtk input method preedit/status */
typedef enum
{
  GTK_IM_PREEDIT_NOTHING,
  GTK_IM_PREEDIT_CALLBACK,
  GTK_IM_PREEDIT_NONE
} GtkIMPreeditStyle;

typedef enum
{
  GTK_IM_STATUS_NOTHING,
  GTK_IM_STATUS_CALLBACK,
  GTK_IM_STATUS_NONE
} GtkIMStatusStyle;

typedef enum
{
  GTK_PACK_DIRECTION_LTR,
  GTK_PACK_DIRECTION_RTL,
  GTK_PACK_DIRECTION_TTB,
  GTK_PACK_DIRECTION_BTT
} GtkPackDirection;

typedef enum
{
  GTK_PRINT_PAGES_ALL,
  GTK_PRINT_PAGES_CURRENT,
  GTK_PRINT_PAGES_RANGES,
  GTK_PRINT_PAGES_SELECTION
} GtkPrintPages;

typedef enum
{
  GTK_PAGE_SET_ALL,
  GTK_PAGE_SET_EVEN,
  GTK_PAGE_SET_ODD
} GtkPageSet;

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

typedef enum
{
  GTK_PAGE_ORIENTATION_PORTRAIT,
  GTK_PAGE_ORIENTATION_LANDSCAPE,
  GTK_PAGE_ORIENTATION_REVERSE_PORTRAIT,
  GTK_PAGE_ORIENTATION_REVERSE_LANDSCAPE
} GtkPageOrientation;

typedef enum
{
  GTK_PRINT_QUALITY_LOW,
  GTK_PRINT_QUALITY_NORMAL,
  GTK_PRINT_QUALITY_HIGH,
  GTK_PRINT_QUALITY_DRAFT
} GtkPrintQuality;

typedef enum
{
  GTK_PRINT_DUPLEX_SIMPLEX,
  GTK_PRINT_DUPLEX_HORIZONTAL,
  GTK_PRINT_DUPLEX_VERTICAL
} GtkPrintDuplex;


typedef enum
{
  GTK_UNIT_PIXEL,
  GTK_UNIT_POINTS,
  GTK_UNIT_INCH,
  GTK_UNIT_MM
} GtkUnit;

typedef enum
{
  GTK_TREE_VIEW_GRID_LINES_NONE,
  GTK_TREE_VIEW_GRID_LINES_HORIZONTAL,
  GTK_TREE_VIEW_GRID_LINES_VERTICAL,
  GTK_TREE_VIEW_GRID_LINES_BOTH
} GtkTreeViewGridLines;

typedef enum
{
  GTK_DRAG_RESULT_SUCCESS,
  GTK_DRAG_RESULT_NO_TARGET,
  GTK_DRAG_RESULT_USER_CANCELLED,
  GTK_DRAG_RESULT_TIMEOUT_EXPIRED,
  GTK_DRAG_RESULT_GRAB_BROKEN,
  GTK_DRAG_RESULT_ERROR
} GtkDragResult;

G_END_DECLS

#endif /* __GTK_ENUMS_H__ */
