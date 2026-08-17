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
 * Modified by the GTK+ Team and others 1997-2001.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_BUTTON_H__
#define __GTK_BUTTON_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_BUTTON                 (gtk_button_get_type ())
#define GTK_BUTTON(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BUTTON, GtkButton))
#define GTK_BUTTON_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_BUTTON, GtkButtonClass))
#define GTK_IS_BUTTON(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BUTTON))
#define GTK_IS_BUTTON_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_BUTTON))
#define GTK_BUTTON_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_BUTTON, GtkButtonClass))

typedef struct _GtkButton             GtkButton;
typedef struct _GtkButtonPrivate      GtkButtonPrivate;
typedef struct _GtkButtonClass        GtkButtonClass;

struct _GtkButton
{
  /*< private >*/
  GtkWidget parent_instance;
};

/**
 * GtkButtonClass:
 * @parent_class: The parent class.
 * @clicked: Signal emitted when the button has been activated (pressed and released).
 * @activate: Signal that causes the button to animate press then
 *    release. Applications should never connect to this signal, but use
 *    the @clicked signal.
 */
struct _GtkButtonClass
{
  GtkWidgetClass        parent_class;

  /*< public >*/

  void (* clicked)  (GtkButton *button);
  void (* activate) (GtkButton *button);

  /*< private >*/

  gpointer padding[8];
};


GDK_AVAILABLE_IN_ALL
GType          gtk_button_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_button_new               (void);
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_button_new_with_label    (const char     *label);
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_button_new_from_icon_name (const char     *icon_name);
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_button_new_with_mnemonic (const char     *label);

GDK_AVAILABLE_IN_ALL
void                  gtk_button_set_has_frame      (GtkButton      *button,
						     gboolean        has_frame);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_button_get_has_frame      (GtkButton      *button);
GDK_AVAILABLE_IN_ALL
void                  gtk_button_set_label          (GtkButton      *button,
						     const char     *label);
GDK_AVAILABLE_IN_ALL
const char *         gtk_button_get_label          (GtkButton      *button);
GDK_AVAILABLE_IN_ALL
void                  gtk_button_set_use_underline  (GtkButton      *button,
						     gboolean        use_underline);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_button_get_use_underline  (GtkButton      *button);

GDK_AVAILABLE_IN_ALL
void                  gtk_button_set_icon_name      (GtkButton      *button,
                                                     const char     *icon_name);
GDK_AVAILABLE_IN_ALL
const char *          gtk_button_get_icon_name      (GtkButton      *button);

GDK_AVAILABLE_IN_ALL
void                  gtk_button_set_child          (GtkButton      *button,
                                                     GtkWidget      *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_button_get_child          (GtkButton      *button);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkButton, g_object_unref)

G_END_DECLS

#endif /* __GTK_BUTTON_H__ */
