/* GTK - The GIMP Toolkit
 * Copyright (C) 2000 Red Hat, Inc.
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

#ifndef __GTK_COLOR_SELECTION_H__
#define __GTK_COLOR_SELECTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gtk/gtkbox.h>

G_BEGIN_DECLS

#define GTK_TYPE_COLOR_SELECTION			(gtk_color_selection_get_type ())
#define GTK_COLOR_SELECTION(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COLOR_SELECTION, GtkColorSelection))
#define GTK_COLOR_SELECTION_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_COLOR_SELECTION, GtkColorSelectionClass))
#define GTK_IS_COLOR_SELECTION(obj)			(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COLOR_SELECTION))
#define GTK_IS_COLOR_SELECTION_CLASS(klass)		(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_COLOR_SELECTION))
#define GTK_COLOR_SELECTION_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_COLOR_SELECTION, GtkColorSelectionClass))


typedef struct _GtkColorSelection       GtkColorSelection;
typedef struct _GtkColorSelectionPrivate  GtkColorSelectionPrivate;
typedef struct _GtkColorSelectionClass  GtkColorSelectionClass;

/**
 * GtkColorSelectionChangePaletteFunc:
 * @colors: (array length=n_colors): Array of colors
 * @n_colors: Number of colors in the array
 *
 * Deprecated: 3.4
 */
typedef void (* GtkColorSelectionChangePaletteFunc) (const GdkColor    *colors,
                                                     gint               n_colors);

/**
 * GtkColorSelectionChangePaletteWithScreenFunc:
 * @screen:
 * @colors: (array length=n_colors): Array of colors
 * @n_colors: Number of colors in the array
 *
 * Since: 2.2
 * Deprecated: 3.4
 */
typedef void (* GtkColorSelectionChangePaletteWithScreenFunc) (GdkScreen         *screen,
							       const GdkColor    *colors,
							       gint               n_colors);

struct _GtkColorSelection
{
  GtkBox parent_instance;

  /*< private >*/
  GtkColorSelectionPrivate *private_data;
};

/**
 * GtkColorSelectionClass:
 * @parent_class: The parent class.
 * @color_changed:
 */
struct _GtkColorSelectionClass
{
  GtkBoxClass parent_class;

  void (*color_changed)	(GtkColorSelection *color_selection);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


/* ColorSelection */

GDK_DEPRECATED_IN_3_4
GType      gtk_color_selection_get_type                (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_widget_new)
GtkWidget *gtk_color_selection_new                     (void);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_use_alpha)
gboolean   gtk_color_selection_get_has_opacity_control (GtkColorSelection *colorsel);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_use_alpha)
void       gtk_color_selection_set_has_opacity_control (GtkColorSelection *colorsel,
							gboolean           has_opacity);
GDK_DEPRECATED_IN_3_4
gboolean   gtk_color_selection_get_has_palette         (GtkColorSelection *colorsel);
GDK_DEPRECATED_IN_3_4
void       gtk_color_selection_set_has_palette         (GtkColorSelection *colorsel,
							gboolean           has_palette);


GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_rgba)
void     gtk_color_selection_set_current_alpha   (GtkColorSelection *colorsel,
						  guint16            alpha);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_rgba)
guint16  gtk_color_selection_get_current_alpha   (GtkColorSelection *colorsel);
GDK_DEPRECATED_IN_3_4
void     gtk_color_selection_set_previous_alpha  (GtkColorSelection *colorsel,
						  guint16            alpha);
GDK_DEPRECATED_IN_3_4
guint16  gtk_color_selection_get_previous_alpha  (GtkColorSelection *colorsel);

GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_rgba)
void     gtk_color_selection_set_current_rgba    (GtkColorSelection *colorsel,
                                                  const GdkRGBA     *rgba);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_rgba)
void     gtk_color_selection_get_current_rgba    (GtkColorSelection *colorsel,
                                                  GdkRGBA           *rgba);
GDK_DEPRECATED_IN_3_4
void     gtk_color_selection_set_previous_rgba   (GtkColorSelection *colorsel,
                                                  const GdkRGBA     *rgba);
GDK_DEPRECATED_IN_3_4
void     gtk_color_selection_get_previous_rgba   (GtkColorSelection *colorsel,
                                                  GdkRGBA           *rgba);

GDK_DEPRECATED_IN_3_4
gboolean gtk_color_selection_is_adjusting        (GtkColorSelection *colorsel);

GDK_DEPRECATED_IN_3_4
gboolean gtk_color_selection_palette_from_string (const gchar       *str,
                                                  GdkColor         **colors,
                                                  gint              *n_colors);
GDK_DEPRECATED_IN_3_4
gchar*   gtk_color_selection_palette_to_string   (const GdkColor    *colors,
                                                  gint               n_colors);

GDK_DEPRECATED_IN_3_4
GtkColorSelectionChangePaletteWithScreenFunc gtk_color_selection_set_change_palette_with_screen_hook (GtkColorSelectionChangePaletteWithScreenFunc func);

GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_set_rgba)
void     gtk_color_selection_set_current_color   (GtkColorSelection *colorsel,
                                                  const GdkColor    *color);
GDK_DEPRECATED_IN_3_4_FOR(gtk_color_chooser_get_rgba)
void     gtk_color_selection_get_current_color   (GtkColorSelection *colorsel,
                                                  GdkColor          *color);
GDK_DEPRECATED_IN_3_4
void     gtk_color_selection_set_previous_color  (GtkColorSelection *colorsel,
                                                  const GdkColor    *color);
GDK_DEPRECATED_IN_3_4
void     gtk_color_selection_get_previous_color  (GtkColorSelection *colorsel,
                                                  GdkColor          *color);


G_END_DECLS

#endif /* __GTK_COLOR_SELECTION_H__ */
