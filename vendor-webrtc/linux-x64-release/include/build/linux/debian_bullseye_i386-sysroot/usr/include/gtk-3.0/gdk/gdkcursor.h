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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_CURSOR_H__
#define __GDK_CURSOR_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk-pixbuf/gdk-pixbuf.h>

G_BEGIN_DECLS

#define GDK_TYPE_CURSOR              (gdk_cursor_get_type ())
#define GDK_CURSOR(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_CURSOR, GdkCursor))
#define GDK_IS_CURSOR(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_CURSOR))


/**
 * GdkCursorType:
 * @GDK_X_CURSOR: ![](X_cursor.png)
 * @GDK_ARROW: ![](arrow.png)
 * @GDK_BASED_ARROW_DOWN: ![](based_arrow_down.png)
 * @GDK_BASED_ARROW_UP: ![](based_arrow_up.png)
 * @GDK_BOAT: ![](boat.png)
 * @GDK_BOGOSITY: ![](bogosity.png)
 * @GDK_BOTTOM_LEFT_CORNER: ![](bottom_left_corner.png)
 * @GDK_BOTTOM_RIGHT_CORNER: ![](bottom_right_corner.png)
 * @GDK_BOTTOM_SIDE: ![](bottom_side.png)
 * @GDK_BOTTOM_TEE: ![](bottom_tee.png)
 * @GDK_BOX_SPIRAL: ![](box_spiral.png)
 * @GDK_CENTER_PTR: ![](center_ptr.png)
 * @GDK_CIRCLE: ![](circle.png)
 * @GDK_CLOCK: ![](clock.png)
 * @GDK_COFFEE_MUG: ![](coffee_mug.png)
 * @GDK_CROSS: ![](cross.png)
 * @GDK_CROSS_REVERSE: ![](cross_reverse.png)
 * @GDK_CROSSHAIR: ![](crosshair.png)
 * @GDK_DIAMOND_CROSS: ![](diamond_cross.png)
 * @GDK_DOT: ![](dot.png)
 * @GDK_DOTBOX: ![](dotbox.png)
 * @GDK_DOUBLE_ARROW: ![](double_arrow.png)
 * @GDK_DRAFT_LARGE: ![](draft_large.png)
 * @GDK_DRAFT_SMALL: ![](draft_small.png)
 * @GDK_DRAPED_BOX: ![](draped_box.png)
 * @GDK_EXCHANGE: ![](exchange.png)
 * @GDK_FLEUR: ![](fleur.png)
 * @GDK_GOBBLER: ![](gobbler.png)
 * @GDK_GUMBY: ![](gumby.png)
 * @GDK_HAND1: ![](hand1.png)
 * @GDK_HAND2: ![](hand2.png)
 * @GDK_HEART: ![](heart.png)
 * @GDK_ICON: ![](icon.png)
 * @GDK_IRON_CROSS: ![](iron_cross.png)
 * @GDK_LEFT_PTR: ![](left_ptr.png)
 * @GDK_LEFT_SIDE: ![](left_side.png)
 * @GDK_LEFT_TEE: ![](left_tee.png)
 * @GDK_LEFTBUTTON: ![](leftbutton.png)
 * @GDK_LL_ANGLE: ![](ll_angle.png)
 * @GDK_LR_ANGLE: ![](lr_angle.png)
 * @GDK_MAN: ![](man.png)
 * @GDK_MIDDLEBUTTON: ![](middlebutton.png)
 * @GDK_MOUSE: ![](mouse.png)
 * @GDK_PENCIL: ![](pencil.png)
 * @GDK_PIRATE: ![](pirate.png)
 * @GDK_PLUS: ![](plus.png)
 * @GDK_QUESTION_ARROW: ![](question_arrow.png)
 * @GDK_RIGHT_PTR: ![](right_ptr.png)
 * @GDK_RIGHT_SIDE: ![](right_side.png)
 * @GDK_RIGHT_TEE: ![](right_tee.png)
 * @GDK_RIGHTBUTTON: ![](rightbutton.png)
 * @GDK_RTL_LOGO: ![](rtl_logo.png)
 * @GDK_SAILBOAT: ![](sailboat.png)
 * @GDK_SB_DOWN_ARROW: ![](sb_down_arrow.png)
 * @GDK_SB_H_DOUBLE_ARROW: ![](sb_h_double_arrow.png)
 * @GDK_SB_LEFT_ARROW: ![](sb_left_arrow.png)
 * @GDK_SB_RIGHT_ARROW: ![](sb_right_arrow.png)
 * @GDK_SB_UP_ARROW: ![](sb_up_arrow.png)
 * @GDK_SB_V_DOUBLE_ARROW: ![](sb_v_double_arrow.png)
 * @GDK_SHUTTLE: ![](shuttle.png)
 * @GDK_SIZING: ![](sizing.png)
 * @GDK_SPIDER: ![](spider.png)
 * @GDK_SPRAYCAN: ![](spraycan.png)
 * @GDK_STAR: ![](star.png)
 * @GDK_TARGET: ![](target.png)
 * @GDK_TCROSS: ![](tcross.png)
 * @GDK_TOP_LEFT_ARROW: ![](top_left_arrow.png)
 * @GDK_TOP_LEFT_CORNER: ![](top_left_corner.png)
 * @GDK_TOP_RIGHT_CORNER: ![](top_right_corner.png)
 * @GDK_TOP_SIDE: ![](top_side.png)
 * @GDK_TOP_TEE: ![](top_tee.png)
 * @GDK_TREK: ![](trek.png)
 * @GDK_UL_ANGLE: ![](ul_angle.png)
 * @GDK_UMBRELLA: ![](umbrella.png)
 * @GDK_UR_ANGLE: ![](ur_angle.png)
 * @GDK_WATCH: ![](watch.png)
 * @GDK_XTERM: ![](xterm.png)
 * @GDK_LAST_CURSOR: last cursor type
 * @GDK_BLANK_CURSOR: Blank cursor. Since 2.16
 * @GDK_CURSOR_IS_PIXMAP: type of cursors constructed with
 *   gdk_cursor_new_from_pixbuf()
 *
 * Predefined cursors.
 *
 * Note that these IDs are directly taken from the X cursor font, and many
 * of these cursors are either not useful, or are not available on other platforms.
 *
 * The recommended way to create cursors is to use gdk_cursor_new_from_name().
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

/* Cursors
 */

GDK_AVAILABLE_IN_ALL
GType      gdk_cursor_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkCursor* gdk_cursor_new_for_display	 (GdkDisplay      *display,
					  GdkCursorType    cursor_type);
GDK_DEPRECATED_IN_3_16_FOR(gdk_cursor_new_for_display)
GdkCursor* gdk_cursor_new		 (GdkCursorType	   cursor_type);
GDK_AVAILABLE_IN_ALL
GdkCursor* gdk_cursor_new_from_pixbuf	 (GdkDisplay      *display,
					  GdkPixbuf       *pixbuf,
					  gint             x,
					  gint             y);
GDK_AVAILABLE_IN_3_10
GdkCursor* gdk_cursor_new_from_surface	 (GdkDisplay      *display,
					  cairo_surface_t *surface,
					  gdouble          x,
					  gdouble          y);
GDK_AVAILABLE_IN_ALL
GdkCursor*  gdk_cursor_new_from_name	 (GdkDisplay      *display,
					  const gchar     *name);
GDK_AVAILABLE_IN_ALL
GdkDisplay* gdk_cursor_get_display	 (GdkCursor	  *cursor);
GDK_DEPRECATED_IN_3_0_FOR(g_object_ref)
GdkCursor * gdk_cursor_ref               (GdkCursor       *cursor);
GDK_DEPRECATED_IN_3_0_FOR(g_object_unref)
void        gdk_cursor_unref             (GdkCursor       *cursor);
GDK_AVAILABLE_IN_ALL
GdkPixbuf*  gdk_cursor_get_image         (GdkCursor       *cursor);
GDK_AVAILABLE_IN_3_10
cairo_surface_t *gdk_cursor_get_surface  (GdkCursor       *cursor,
					  gdouble         *x_hot,
					  gdouble         *y_hot);
GDK_AVAILABLE_IN_ALL
GdkCursorType gdk_cursor_get_cursor_type (GdkCursor       *cursor);


G_END_DECLS

#endif /* __GDK_CURSOR_H__ */
