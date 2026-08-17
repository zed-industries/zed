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

#define GTK_TYPE_FONT_BUTTON             (gtk_font_button_get_type ())
#define GTK_FONT_BUTTON(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FONT_BUTTON, GtkFontButton))
#define GTK_IS_FONT_BUTTON(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FONT_BUTTON))

typedef struct _GtkFontButton        GtkFontButton;

GDK_AVAILABLE_IN_ALL
GType                 gtk_font_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_font_button_new            (void);
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_font_button_new_with_font  (const char    *fontname);

GDK_AVAILABLE_IN_ALL
const char *         gtk_font_button_get_title      (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_title      (GtkFontButton *font_button,
                                                      const char    *title);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_font_button_get_modal      (GtkFontButton *font_button);
GDK_AVAILABLE_IN_ALL
void                  gtk_font_button_set_modal      (GtkFontButton *font_button,
                                                      gboolean       modal);
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

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFontButton, g_object_unref)

G_END_DECLS


#endif /* __GTK_FONT_BUTTON_H__ */
