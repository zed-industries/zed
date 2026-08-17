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

#ifndef __GTK_PANED_H__
#define __GTK_PANED_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>

G_BEGIN_DECLS

#define GTK_TYPE_PANED                  (gtk_paned_get_type ())
#define GTK_PANED(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PANED, GtkPaned))
#define GTK_PANED_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PANED, GtkPanedClass))
#define GTK_IS_PANED(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PANED))
#define GTK_IS_PANED_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PANED))
#define GTK_PANED_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PANED, GtkPanedClass))


typedef struct _GtkPaned        GtkPaned;
typedef struct _GtkPanedClass   GtkPanedClass;
typedef struct _GtkPanedPrivate GtkPanedPrivate;

struct _GtkPaned
{
  GtkContainer container;

  /*< private >*/
  GtkPanedPrivate *priv;
};

struct _GtkPanedClass
{
  GtkContainerClass parent_class;

  gboolean (* cycle_child_focus)   (GtkPaned      *paned,
				    gboolean       reverse);
  gboolean (* toggle_handle_focus) (GtkPaned      *paned);
  gboolean (* move_handle)         (GtkPaned      *paned,
				    GtkScrollType  scroll);
  gboolean (* cycle_handle_focus)  (GtkPaned      *paned,
				    gboolean       reverse);
  gboolean (* accept_position)     (GtkPaned	  *paned);
  gboolean (* cancel_position)     (GtkPaned	  *paned);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType       gtk_paned_get_type     (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_paned_new          (GtkOrientation orientation);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_add1         (GtkPaned       *paned,
                                    GtkWidget      *child);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_add2         (GtkPaned       *paned,
                                    GtkWidget      *child);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_pack1        (GtkPaned       *paned,
                                    GtkWidget      *child,
                                    gboolean        resize,
                                    gboolean        shrink);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_pack2        (GtkPaned       *paned,
                                    GtkWidget      *child,
                                    gboolean        resize,
                                    gboolean        shrink);

GDK_AVAILABLE_IN_ALL
gint        gtk_paned_get_position (GtkPaned       *paned);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_position (GtkPaned       *paned,
                                    gint            position);

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_paned_get_child1   (GtkPaned       *paned);
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_paned_get_child2   (GtkPaned       *paned);

GDK_AVAILABLE_IN_ALL
GdkWindow * gtk_paned_get_handle_window (GtkPaned  *paned);

GDK_AVAILABLE_IN_3_16
void        gtk_paned_set_wide_handle (GtkPaned    *paned,
                                       gboolean     wide);
GDK_AVAILABLE_IN_3_16
gboolean    gtk_paned_get_wide_handle (GtkPaned    *paned);


G_END_DECLS

#endif /* __GTK_PANED_H__ */
