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

#ifndef __GTK_BUTTON_BOX_H__
#define __GTK_BUTTON_BOX_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbox.h>


G_BEGIN_DECLS  

#define GTK_TYPE_BUTTON_BOX             (gtk_button_box_get_type ())
#define GTK_BUTTON_BOX(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BUTTON_BOX, GtkButtonBox))
#define GTK_BUTTON_BOX_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_BUTTON_BOX, GtkButtonBoxClass))
#define GTK_IS_BUTTON_BOX(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BUTTON_BOX))
#define GTK_IS_BUTTON_BOX_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_BUTTON_BOX))
#define GTK_BUTTON_BOX_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_BUTTON_BOX, GtkButtonBoxClass))


typedef struct _GtkButtonBox              GtkButtonBox;
typedef struct _GtkButtonBoxPrivate       GtkButtonBoxPrivate;
typedef struct _GtkButtonBoxClass         GtkButtonBoxClass;

struct _GtkButtonBox
{
  GtkBox box;

  /*< private >*/
  GtkButtonBoxPrivate *priv;
};

/**
 * GtkButtonBoxClass:
 * @parent_class: The parent class.
 */
struct _GtkButtonBoxClass
{
  GtkBoxClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


/**
 * GtkButtonBoxStyle:
 * @GTK_BUTTONBOX_SPREAD: Buttons are evenly spread across the box.
 * @GTK_BUTTONBOX_EDGE: Buttons are placed at the edges of the box.
 * @GTK_BUTTONBOX_START: Buttons are grouped towards the start of the box,
 *   (on the left for a HBox, or the top for a VBox).
 * @GTK_BUTTONBOX_END: Buttons are grouped towards the end of the box,
 *   (on the right for a HBox, or the bottom for a VBox).
 * @GTK_BUTTONBOX_CENTER: Buttons are centered in the box. Since 2.12.
 * @GTK_BUTTONBOX_EXPAND: Buttons expand to fill the box. This entails giving
 *   buttons a "linked" appearance, making button sizes homogeneous, and
 *   setting spacing to 0 (same as calling gtk_box_set_homogeneous() and
 *   gtk_box_set_spacing() manually). Since 3.12.
 *
 * Used to dictate the style that a #GtkButtonBox uses to layout the buttons it
 * contains.
 */
typedef enum
{
  GTK_BUTTONBOX_SPREAD = 1,
  GTK_BUTTONBOX_EDGE,
  GTK_BUTTONBOX_START,
  GTK_BUTTONBOX_END,
  GTK_BUTTONBOX_CENTER,
  GTK_BUTTONBOX_EXPAND
} GtkButtonBoxStyle;


GDK_AVAILABLE_IN_ALL
GType             gtk_button_box_get_type            (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget       * gtk_button_box_new                 (GtkOrientation     orientation);
GDK_AVAILABLE_IN_ALL
GtkButtonBoxStyle gtk_button_box_get_layout          (GtkButtonBox      *widget);
GDK_AVAILABLE_IN_ALL
void              gtk_button_box_set_layout          (GtkButtonBox      *widget,
                                                      GtkButtonBoxStyle  layout_style);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_button_box_get_child_secondary (GtkButtonBox      *widget,
                                                      GtkWidget         *child);
GDK_AVAILABLE_IN_ALL
void              gtk_button_box_set_child_secondary (GtkButtonBox      *widget,
                                                      GtkWidget         *child,
                                                      gboolean           is_secondary);
GDK_AVAILABLE_IN_3_2
gboolean          gtk_button_box_get_child_non_homogeneous (GtkButtonBox *widget,
                                                            GtkWidget    *child);
GDK_AVAILABLE_IN_3_2
void              gtk_button_box_set_child_non_homogeneous (GtkButtonBox *widget,
                                                            GtkWidget    *child,
                                                            gboolean      non_homogeneous);


G_END_DECLS

#endif /* __GTK_BUTTON_BOX_H__ */
