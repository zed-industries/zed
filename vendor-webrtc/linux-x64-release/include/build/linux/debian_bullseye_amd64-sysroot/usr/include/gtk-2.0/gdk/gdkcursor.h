/* GDK - The GIMP Drawing Kit
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

#ifndef __GDK_CURSOR_H__
#define __GDK_CURSOR_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk-pixbuf/gdk-pixbuf.h>

G_BEGIN_DECLS

#define GDK_TYPE_CURSOR (gdk_cursor_get_type ())

/* Cursor types.
 */
typedef enum
{
  GDK_X_CURSOR 		  = 0,
  GDK_ARROW 		  = 2,
  GDK_BASED_ARROW_DOWN    = 4,
  GDK_BASED_ARROW_UP 	  = 6,
  GDK_BOAT 		  = 8,
  GDK_BOGOSITY 		  = 10,
  GDK_BOTTOM_LEFT_CORNER  = 12,
  GDK_BOTTOM_RIGHT_CORNER = 14,
  GDK_BOTTOM_SIDE 	  = 16,
  GDK_BOTTOM_TEE 	  = 18,
  GDK_BOX_SPIRAL 	  = 20,
  GDK_CENTER_PTR 	  = 22,
  GDK_CIRCLE 		  = 24,
  GDK_CLOCK	 	  = 26,
  GDK_COFFEE_MUG 	  = 28,
  GDK_CROSS 		  = 30,
  GDK_CROSS_REVERSE 	  = 32,
  GDK_CROSSHAIR 	  = 34,
  GDK_DIAMOND_CROSS 	  = 36,
  GDK_DOT 		  = 38,
  GDK_DOTBOX 		  = 40,
  GDK_DOUBLE_ARROW 	  = 42,
  GDK_DRAFT_LARGE 	  = 44,
  GDK_DRAFT_SMALL 	  = 46,
  GDK_DRAPED_BOX 	  = 48,
  GDK_EXCHANGE 		  = 50,
  GDK_FLEUR 		  = 52,
  GDK_GOBBLER 		  = 54,
  GDK_GUMBY 		  = 56,
  GDK_HAND1 		  = 58,
  GDK_HAND2 		  = 60,
  GDK_HEART 		  = 62,
  GDK_ICON 		  = 64,
  GDK_IRON_CROSS 	  = 66,
  GDK_LEFT_PTR 		  = 68,
  GDK_LEFT_SIDE 	  = 70,
  GDK_LEFT_TEE 		  = 72,
  GDK_LEFTBUTTON 	  = 74,
  GDK_LL_ANGLE 		  = 76,
  GDK_LR_ANGLE 	 	  = 78,
  GDK_MAN 		  = 80,
  GDK_MIDDLEBUTTON 	  = 82,
  GDK_MOUSE 		  = 84,
  GDK_PENCIL 		  = 86,
  GDK_PIRATE 		  = 88,
  GDK_PLUS 		  = 90,
  GDK_QUESTION_ARROW 	  = 92,
  GDK_RIGHT_PTR 	  = 94,
  GDK_RIGHT_SIDE 	  = 96,
  GDK_RIGHT_TEE 	  = 98,
  GDK_RIGHTBUTTON 	  = 100,
  GDK_RTL_LOGO 		  = 102,
  GDK_SAILBOAT 		  = 104,
  GDK_SB_DOWN_ARROW 	  = 106,
  GDK_SB_H_DOUBLE_ARROW   = 108,
  GDK_SB_LEFT_ARROW 	  = 110,
  GDK_SB_RIGHT_ARROW 	  = 112,
  GDK_SB_UP_ARROW 	  = 114,
  GDK_SB_V_DOUBLE_ARROW   = 116,
  GDK_SHUTTLE 		  = 118,
  GDK_SIZING 		  = 120,
  GDK_SPIDER		  = 122,
  GDK_SPRAYCAN 		  = 124,
  GDK_STAR 		  = 126,
  GDK_TARGET 		  = 128,
  GDK_TCROSS 		  = 130,
  GDK_TOP_LEFT_ARROW 	  = 132,
  GDK_TOP_LEFT_CORNER 	  = 134,
  GDK_TOP_RIGHT_CORNER 	  = 136,
  GDK_TOP_SIDE 		  = 138,
  GDK_TOP_TEE 		  = 140,
  GDK_TREK 		  = 142,
  GDK_UL_ANGLE 		  = 144,
  GDK_UMBRELLA 		  = 146,
  GDK_UR_ANGLE 		  = 148,
  GDK_WATCH 		  = 150,
  GDK_XTERM 		  = 152,
  GDK_LAST_CURSOR,
  GDK_BLANK_CURSOR        = -2,
  GDK_CURSOR_IS_PIXMAP 	  = -1
} GdkCursorType;

struct _GdkCursor
{
  GdkCursorType GSEAL (type);
  /*< private >*/
  guint GSEAL (ref_count);
};

/* Cursors
 */

GType      gdk_cursor_get_type           (void) G_GNUC_CONST;

GdkCursor* gdk_cursor_new_for_display	 (GdkDisplay      *display,
					  GdkCursorType    cursor_type);
#ifndef GDK_MULTIHEAD_SAFE
GdkCursor* gdk_cursor_new		 (GdkCursorType	   cursor_type);
#endif
GdkCursor* gdk_cursor_new_from_pixmap	 (GdkPixmap	  *source,
					  GdkPixmap	  *mask,
					  const GdkColor  *fg,
					  const GdkColor  *bg,
					  gint		   x,
					  gint		   y);
GdkCursor* gdk_cursor_new_from_pixbuf	 (GdkDisplay      *display,
					  GdkPixbuf       *pixbuf,
					  gint             x,
					  gint             y);
GdkDisplay* gdk_cursor_get_display	 (GdkCursor	  *cursor);
GdkCursor*  gdk_cursor_ref               (GdkCursor       *cursor);
void        gdk_cursor_unref             (GdkCursor       *cursor);
GdkCursor*  gdk_cursor_new_from_name	 (GdkDisplay      *display,
					  const gchar     *name);
GdkPixbuf*  gdk_cursor_get_image         (GdkCursor       *cursor);
GdkCursorType gdk_cursor_get_cursor_type (GdkCursor       *cursor);

#ifndef GDK_DISABLE_DEPRECATED
#define gdk_cursor_destroy             gdk_cursor_unref
#endif /* GDK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GDK_CURSOR_H__ */
