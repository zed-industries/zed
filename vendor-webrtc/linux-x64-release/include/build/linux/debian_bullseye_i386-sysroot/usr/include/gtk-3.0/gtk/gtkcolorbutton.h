/*
 * GTK - The GIMP Toolkit
 * Copyright (C) 1998, 1999 Red Hat, Inc.
 * All rights reserved.
 *
 * This Library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 *
 * This Library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/* Color picker button for GNOME
 *
 * Author: Federico Mena <federico@nuclecu.unam.mx>
 *
 * Modified by the GTK+ Team and others 2003.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_COLOR_BUTTON_H__
#define __GTK_COLOR_BUTTON_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbutton.h>

G_BEGIN_DECLS


#define GTK_TYPE_COLOR_BUTTON             (gtk_color_button_get_type ())
#define GTK_COLOR_BUTTON(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COLOR_BUTTON, GtkColorButton))
#define GTK_COLOR_BUTTON_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_COLOR_BUTTON, GtkColorButtonClass))
#define GTK_IS_COLOR_BUTTON(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COLOR_BUTTON))
#define GTK_IS_COLOR_BUTTON_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_COLOR_BUTTON))
#define GTK_COLOR_BUTTON_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_COLOR_BUTTON, GtkColorButtonClass))

typedef struct _GtkColorButton          GtkColorButton;
typedef struct _GtkColorButtonClass     GtkColorButtonClass;
typedef struct _GtkColorButtonPrivate   GtkColorButtonPrivate;

struct _GtkColorButton {
  GtkButton button;

  /*< private >*/
  GtkColorButtonPrivate *priv;
};

struct _GtkColorButtonClass {
  GtkButtonClass parent_class;

  void (* color_set) (GtkColorButton *cp);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType        gtk_color_button_get_type      (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *  gtk_color_button_new           (void);
GDK_AVAILABLE_IN_ALL
GtkWidget *  gtk_color_button_new_with_rgba (const GdkRGBA  *rgba);
GDK_AVAILABLE_IN_ALL
void         gtk_color_button_set_title     (GtkColorButton *button,
                                             const gchar    *title);
GDK_AVAILABLE_IN_ALL
const gchar *gtk_color_button_get_title     (GtkColorButton *button);

GDK_DEPRECATED_IN_3_4_FOR(gtk_color_button_new_with_rgba)
GtkWidget *gtk_color_button_new_with_color (const GdkColor *color);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_rgba)
void       gtk_color_button_set_color      (GtkColorButton *button,
                                            const GdkColor *color);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_rgba)
void       gtk_color_button_get_color      (GtkColorButton *button,
                                            GdkColor       *color);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_rgba)
void       gtk_color_button_set_alpha      (GtkColorButton *button,
                                            guint16         alpha);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_rgba)
guint16    gtk_color_button_get_alpha      (GtkColorButton *button);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_use_alpha)
void         gtk_color_button_set_use_alpha (GtkColorButton *button,
                                             gboolean        use_alpha);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_use_alpha)
gboolean     gtk_color_button_get_use_alpha (GtkColorButton *button);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_rgba)
void         gtk_color_button_set_rgba      (GtkColorButton *button,
                                             const GdkRGBA  *rgba);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_rgba)
void         gtk_color_button_get_rgba      (GtkColorButton *button,
                                             GdkRGBA        *rgba);

G_END_DECLS

#endif  /* __GTK_COLOR_BUTTON_H__ */
