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

#ifndef __GTK_SCROLLED_WINDOW_H__
#define __GTK_SCROLLED_WINDOW_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS


#define GTK_TYPE_SCROLLED_WINDOW            (gtk_scrolled_window_get_type ())
#define GTK_SCROLLED_WINDOW(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SCROLLED_WINDOW, GtkScrolledWindow))
#define GTK_IS_SCROLLED_WINDOW(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SCROLLED_WINDOW))


typedef struct _GtkScrolledWindow GtkScrolledWindow;

/**
 * GtkCornerType:
 * @GTK_CORNER_TOP_LEFT: Place the scrollbars on the right and bottom of the
 *   widget (default behaviour).
 * @GTK_CORNER_BOTTOM_LEFT: Place the scrollbars on the top and right of the
 *   widget.
 * @GTK_CORNER_TOP_RIGHT: Place the scrollbars on the left and bottom of the
 *   widget.
 * @GTK_CORNER_BOTTOM_RIGHT: Place the scrollbars on the top and left of the
 *   widget.
 *
 * Specifies which corner a child widget should be placed in when packed into
 * a `GtkScrolledWindow.`
 *
 * This is effectively the opposite of where the scroll bars are placed.
 */
typedef enum
{
  GTK_CORNER_TOP_LEFT,
  GTK_CORNER_BOTTOM_LEFT,
  GTK_CORNER_TOP_RIGHT,
  GTK_CORNER_BOTTOM_RIGHT
} GtkCornerType;


/**
 * GtkPolicyType:
 * @GTK_POLICY_ALWAYS: The scrollbar is always visible. The view size is
 *   independent of the content.
 * @GTK_POLICY_AUTOMATIC: The scrollbar will appear and disappear as necessary.
 *   For example, when all of a `GtkTreeView` can not be seen.
 * @GTK_POLICY_NEVER: The scrollbar should never appear. In this mode the
 *   content determines the size.
 * @GTK_POLICY_EXTERNAL: Don't show a scrollbar, but don't force the
 *   size to follow the content. This can be used e.g. to make multiple
 *   scrolled windows share a scrollbar.
 *
 * Determines how the size should be computed to achieve the one of the
 * visibility mode for the scrollbars.
 */
typedef enum
{
  GTK_POLICY_ALWAYS,
  GTK_POLICY_AUTOMATIC,
  GTK_POLICY_NEVER,
  GTK_POLICY_EXTERNAL
} GtkPolicyType;


GDK_AVAILABLE_IN_ALL
GType          gtk_scrolled_window_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_scrolled_window_new               (void);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_hadjustment   (GtkScrolledWindow *scrolled_window,
                                                      GtkAdjustment     *hadjustment);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_vadjustment   (GtkScrolledWindow *scrolled_window,
                                                      GtkAdjustment     *vadjustment);
GDK_AVAILABLE_IN_ALL
GtkAdjustment* gtk_scrolled_window_get_hadjustment   (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
GtkAdjustment* gtk_scrolled_window_get_vadjustment   (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_scrolled_window_get_hscrollbar    (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_scrolled_window_get_vscrollbar    (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_policy        (GtkScrolledWindow *scrolled_window,
                                                      GtkPolicyType      hscrollbar_policy,
                                                      GtkPolicyType      vscrollbar_policy);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_get_policy        (GtkScrolledWindow *scrolled_window,
                                                      GtkPolicyType     *hscrollbar_policy,
                                                      GtkPolicyType     *vscrollbar_policy);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_placement     (GtkScrolledWindow *scrolled_window,
                                                      GtkCornerType      window_placement);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_unset_placement   (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
GtkCornerType  gtk_scrolled_window_get_placement     (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_has_frame     (GtkScrolledWindow *scrolled_window,
                                                      gboolean           has_frame);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_scrolled_window_get_has_frame     (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
int            gtk_scrolled_window_get_min_content_width  (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_min_content_width  (GtkScrolledWindow *scrolled_window,
                                                           int                width);
GDK_AVAILABLE_IN_ALL
int            gtk_scrolled_window_get_min_content_height (GtkScrolledWindow *scrolled_window);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_min_content_height (GtkScrolledWindow *scrolled_window,
                                                           int                height);
GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_kinetic_scrolling  (GtkScrolledWindow *scrolled_window,
                                                           gboolean           kinetic_scrolling);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_scrolled_window_get_kinetic_scrolling  (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_overlay_scrolling  (GtkScrolledWindow *scrolled_window,
                                                           gboolean           overlay_scrolling);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_scrolled_window_get_overlay_scrolling (GtkScrolledWindow   *scrolled_window);

GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_max_content_width  (GtkScrolledWindow *scrolled_window,
                                                           int                width);
GDK_AVAILABLE_IN_ALL
int            gtk_scrolled_window_get_max_content_width  (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_max_content_height (GtkScrolledWindow *scrolled_window,
                                                           int                height);
GDK_AVAILABLE_IN_ALL
int            gtk_scrolled_window_get_max_content_height (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_propagate_natural_width  (GtkScrolledWindow *scrolled_window,
                                                                 gboolean           propagate);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_scrolled_window_get_propagate_natural_width  (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_propagate_natural_height (GtkScrolledWindow *scrolled_window,
                                                                 gboolean           propagate);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_scrolled_window_get_propagate_natural_height (GtkScrolledWindow *scrolled_window);

GDK_AVAILABLE_IN_ALL
void           gtk_scrolled_window_set_child        (GtkScrolledWindow *scrolled_window,
                                                     GtkWidget         *child);
GDK_AVAILABLE_IN_ALL
GtkWidget     *gtk_scrolled_window_get_child        (GtkScrolledWindow *scrolled_window);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkScrolledWindow, g_object_unref)

G_END_DECLS


#endif /* __GTK_SCROLLED_WINDOW_H__ */
