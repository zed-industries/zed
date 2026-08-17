/* GTK - The GIMP Toolkit
 * Copyright (C) 1998 David Abilleira Freijeiro <odaf@nexo.es>
 * All rights reserved
 * Based on gnome-color-picker by Federico Mena <federico@nuclecu.unam.mx>
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */
/*
 * Modified by the GTK+ Team and others 2003.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_FONT_BUTTON_H__
#define __GTK_FONT_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbutton.h>


G_BEGIN_DECLS

/* GtkFontButton is a button widget that allow user to select a font.
 */

#define GTK_TYPE_FONT_BUTTON             (gtk_font_button_get_type ())
#define GTK_FONT_BUTTON(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FONT_BUTTON, GtkFontButton))
#define GTK_FONT_BUTTON_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FONT_BUTTON, GtkFontButtonClass))
#define GTK_IS_FONT_BUTTON(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FONT_BUTTON))
#define GTK_IS_FONT_BUTTON_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FONT_BUTTON))
#define GTK_FONT_BUTTON_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FONT_BUTTON, GtkFontButtonClass))

typedef struct _GtkFontButton        GtkFontButton;
typedef struct _GtkFontButtonClass   GtkFontButtonClass;
typedef struct _GtkFontButtonPrivate GtkFontButtonPrivate;

struct _GtkFontButton {
  GtkButton button;

  /*< private >*/
  GtkFontButtonPrivate *priv;
};

struct _GtkFontButtonClass {
  GtkButtonClass parent_class;

  /* font_set signal is emitted when font is chosen */
  void (* font_set) (GtkFontButton *gfp);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType                 gtk_font_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_font_button_new            (void);
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_font_button_new_with_font  (const gchar   *fontname);

GDK_AVAILABLE_IN_ALL
const gchar *         gtk_font_button_get_title      (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_title      (GtkFontButton *font_button,
                                                      const gchar   *title);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_font_button_get_use_font   (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_use_font   (GtkFontButton *font_button,
                                                      gboolean       use_font);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_font_button_get_use_size   (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_use_size   (GtkFontButton *font_button,
                                                      gboolean       use_size);
GDK_DEPRECATED_IN_3_22
const gchar *         gtk_font_button_get_font_name  (GtkFontButton *font_button);
GDK_DEPRECATED_IN_3_22
gboolean              gtk_font_button_set_font_name  (GtkFontButton *font_button,
                                                      const gchar   *fontname);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_font_button_get_show_style (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_show_style (GtkFontButton *font_button,
                                                      gboolean       show_style);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_font_button_get_show_size  (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_show_size  (GtkFontButton *font_button,
                                                      gboolean       show_size);

G_END_DECLS


#endif /* __GTK_FONT_BUTTON_H__ */
