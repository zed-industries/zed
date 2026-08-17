/* gtkcellrenderertoggle.h
 * Copyright (C) 2000  Red Hat, Inc.,  Jonathan Blandford <jrb@redhat.com>
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

#ifndef __GTK_CELL_RENDERER_TOGGLE_H__
#define __GTK_CELL_RENDERER_TOGGLE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellrenderer.h>


G_BEGIN_DECLS


#define GTK_TYPE_CELL_RENDERER_TOGGLE			(gtk_cell_renderer_toggle_get_type ())
#define GTK_CELL_RENDERER_TOGGLE(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_RENDERER_TOGGLE, GtkCellRendererToggle))
#define GTK_CELL_RENDERER_TOGGLE_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CELL_RENDERER_TOGGLE, GtkCellRendererToggleClass))
#define GTK_IS_CELL_RENDERER_TOGGLE(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_RENDERER_TOGGLE))
#define GTK_IS_CELL_RENDERER_TOGGLE_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CELL_RENDERER_TOGGLE))
#define GTK_CELL_RENDERER_TOGGLE_GET_CLASS(obj)         (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CELL_RENDERER_TOGGLE, GtkCellRendererToggleClass))

typedef struct _GtkCellRendererToggle              GtkCellRendererToggle;
typedef struct _GtkCellRendererTogglePrivate       GtkCellRendererTogglePrivate;
typedef struct _GtkCellRendererToggleClass         GtkCellRendererToggleClass;

struct _GtkCellRendererToggle
{
  GtkCellRenderer parent;

  /*< private >*/
  GtkCellRendererTogglePrivate *priv;
};

struct _GtkCellRendererToggleClass
{
  GtkCellRendererClass parent_class;

  void (* toggled) (GtkCellRendererToggle *cell_renderer_toggle,
		    const gchar                 *path);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType            gtk_cell_renderer_toggle_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkCellRenderer *gtk_cell_renderer_toggle_new            (void);

GDK_AVAILABLE_IN_ALL
gboolean         gtk_cell_renderer_toggle_get_radio      (GtkCellRendererToggle *toggle);
GDK_AVAILABLE_IN_ALL
void             gtk_cell_renderer_toggle_set_radio      (GtkCellRendererToggle *toggle,
                                                          gboolean               radio);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_cell_renderer_toggle_get_active      (GtkCellRendererToggle *toggle);
GDK_AVAILABLE_IN_ALL
void            gtk_cell_renderer_toggle_set_active      (GtkCellRendererToggle *toggle,
                                                          gboolean               setting);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_cell_renderer_toggle_get_activatable (GtkCellRendererToggle *toggle);
GDK_AVAILABLE_IN_ALL
void            gtk_cell_renderer_toggle_set_activatable (GtkCellRendererToggle *toggle,
                                                          gboolean               setting);


G_END_DECLS

#endif /* __GTK_CELL_RENDERER_TOGGLE_H__ */
