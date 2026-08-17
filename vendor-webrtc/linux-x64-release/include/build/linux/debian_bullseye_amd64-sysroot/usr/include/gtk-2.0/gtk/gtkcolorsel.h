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

#ifndef __GTK_COLOR_SELECTION_H__
#define __GTK_COLOR_SELECTION_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gtk/gtkvbox.h>

G_BEGIN_DECLS

#define GTK_TYPE_COLOR_SELECTION			(gtk_color_selection_get_type ())
#define GTK_COLOR_SELECTION(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COLOR_SELECTION, GtkColorSelection))
#define GTK_COLOR_SELECTION_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_COLOR_SELECTION, GtkColorSelectionClass))
#define GTK_IS_COLOR_SELECTION(obj)			(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COLOR_SELECTION))
#define GTK_IS_COLOR_SELECTION_CLASS(klass)		(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_COLOR_SELECTION))
#define GTK_COLOR_SELECTION_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_COLOR_SELECTION, GtkColorSelectionClass))


typedef struct _GtkColorSelection       GtkColorSelection;
typedef struct _GtkColorSelectionClass  GtkColorSelectionClass;


typedef void (* GtkColorSelectionChangePaletteFunc) (const GdkColor    *colors,
                                                     gint               n_colors);
typedef void (* GtkColorSelectionChangePaletteWithScreenFunc) (GdkScreen         *screen,
							       const GdkColor    *colors,
							       gint               n_colors);

struct _GtkColorSelection
{
  GtkVBox parent_instance;

  /* < private_data > */
  gpointer GSEAL (private_data);
};

struct _GtkColorSelectionClass
{
  GtkVBoxClass parent_class;

  void (*color_changed)	(GtkColorSelection *color_selection);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


/* ColorSelection */

GType      gtk_color_selection_get_type                (void) G_GNUC_CONST;
GtkWidget *gtk_color_selection_new                     (void);
gboolean   gtk_color_selection_get_has_opacity_control (GtkColorSelection *colorsel);
void       gtk_color_selection_set_has_opacity_control (GtkColorSelection *colorsel,
							gboolean           has_opacity);
gboolean   gtk_color_selection_get_has_palette         (GtkColorSelection *colorsel);
void       gtk_color_selection_set_has_palette         (GtkColorSelection *colorsel,
							gboolean           has_palette);


void     gtk_color_selection_set_current_color   (GtkColorSelection *colorsel,
						  const GdkColor    *color);
void     gtk_color_selection_set_current_alpha   (GtkColorSelection *colorsel,
						  guint16            alpha);
void     gtk_color_selection_get_current_color   (GtkColorSelection *colorsel,
						  GdkColor          *color);
guint16  gtk_color_selection_get_current_alpha   (GtkColorSelection *colorsel);
void     gtk_color_selection_set_previous_color  (GtkColorSelection *colorsel,
						  const GdkColor    *color);
void     gtk_color_selection_set_previous_alpha  (GtkColorSelection *colorsel,
						  guint16            alpha);
void     gtk_color_selection_get_previous_color  (GtkColorSelection *colorsel,
						  GdkColor          *color);
guint16  gtk_color_selection_get_previous_alpha  (GtkColorSelection *colorsel);

gboolean gtk_color_selection_is_adjusting        (GtkColorSelection *colorsel);

gboolean gtk_color_selection_palette_from_string (const gchar       *str,
                                                  GdkColor         **colors,
                                                  gint              *n_colors);
gchar*   gtk_color_selection_palette_to_string   (const GdkColor    *colors,
                                                  gint               n_colors);

#ifndef GTK_DISABLE_DEPRECATED
#ifndef GDK_MULTIHEAD_SAFE
GtkColorSelectionChangePaletteFunc           gtk_color_selection_set_change_palette_hook             (GtkColorSelectionChangePaletteFunc           func);
#endif
#endif

GtkColorSelectionChangePaletteWithScreenFunc gtk_color_selection_set_change_palette_with_screen_hook (GtkColorSelectionChangePaletteWithScreenFunc func);

#ifndef GTK_DISABLE_DEPRECATED
/* Deprecated calls: */
void gtk_color_selection_set_color         (GtkColorSelection *colorsel,
					    gdouble           *color);
void gtk_color_selection_get_color         (GtkColorSelection *colorsel,
					    gdouble           *color);
void gtk_color_selection_set_update_policy (GtkColorSelection *colorsel,
					    GtkUpdateType      policy);
#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_COLOR_SELECTION_H__ */
