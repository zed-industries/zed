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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#ifndef __GTK_BOX_H__
#define __GTK_BOX_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>


G_BEGIN_DECLS


#define GTK_TYPE_BOX            (gtk_box_get_type ())
#define GTK_BOX(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BOX, GtkBox))
#define GTK_BOX_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_BOX, GtkBoxClass))
#define GTK_IS_BOX(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BOX))
#define GTK_IS_BOX_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_BOX))
#define GTK_BOX_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_BOX, GtkBoxClass))


typedef struct _GtkBox	      GtkBox;
typedef struct _GtkBoxClass   GtkBoxClass;

struct _GtkBox
{
  GtkContainer container;

  /*< public >*/
  GList *GSEAL (children);
  gint16 GSEAL (spacing);
  guint GSEAL (homogeneous) : 1;
};

struct _GtkBoxClass
{
  GtkContainerClass parent_class;
};

/**
 * GtkBoxChild:
 * @widget: the child widget, packed into the GtkBox.
 * @padding: the number of extra pixels to put between this child and its
 *  neighbors, set when packed, zero by default.
 * @expand: flag indicates whether extra space should be given to this child.
 *  Any extra space given to the parent GtkBox is divided up among all children
 *  with this attribute set to %TRUE; set when packed, %TRUE by default.
 * @fill: flag indicates whether any extra space given to this child due to its
 *  @expand attribute being set is actually allocated to the child, rather than
 *  being used as padding around the widget; set when packed, %TRUE by default.
 * @pack: one of #GtkPackType indicating whether the child is packed with
 *  reference to the start (top/left) or end (bottom/right) of the GtkBox.
 * @is_secondary: %TRUE if the child is secondary
 *
 * The #GtkBoxChild holds a child widget of #GtkBox and describes how the child
 * is to be packed into the #GtkBox. All fields of this #GtkBoxChild should be
 * considered read-only and they should never be set directly by an application.
 * Use gtk_box_query_child_packing() and gtk_box_set_child_packing() to query
 * and set the #GtkBoxChild.padding, #GtkBoxChild.expand, #GtkBoxChild.fill and
 * #GtkBoxChild.pack fields.
 *
 * Deprecated: 2.22: Use gtk_container_get_children() instead.
 */
#if !defined (GTK_DISABLE_DEPRECATED) || defined (GTK_COMPILATION)
typedef struct _GtkBoxChild   GtkBoxChild;
struct _GtkBoxChild
{
  GtkWidget *widget;
  guint16 padding;
  guint expand : 1;
  guint fill : 1;
  guint pack : 1;
  guint is_secondary : 1;
};
#endif

GType       gtk_box_get_type            (void) G_GNUC_CONST;
GtkWidget* _gtk_box_new                 (GtkOrientation  orientation,
                                         gboolean        homogeneous,
                                         gint            spacing);

void        gtk_box_pack_start          (GtkBox         *box,
                                         GtkWidget      *child,
                                         gboolean        expand,
                                         gboolean        fill,
                                         guint           padding);
void        gtk_box_pack_end            (GtkBox         *box,
                                         GtkWidget      *child,
                                         gboolean        expand,
                                         gboolean        fill,
                                         guint           padding);

#ifndef GTK_DISABLE_DEPRECATED
void        gtk_box_pack_start_defaults (GtkBox         *box,
                                         GtkWidget      *widget);
void        gtk_box_pack_end_defaults   (GtkBox         *box,
                                         GtkWidget      *widget);
#endif

void        gtk_box_set_homogeneous     (GtkBox         *box,
                                         gboolean        homogeneous);
gboolean    gtk_box_get_homogeneous     (GtkBox         *box);
void        gtk_box_set_spacing         (GtkBox         *box,
                                         gint            spacing);
gint        gtk_box_get_spacing         (GtkBox         *box);

void        gtk_box_reorder_child       (GtkBox         *box,
                                         GtkWidget      *child,
                                         gint            position);

void        gtk_box_query_child_packing (GtkBox         *box,
                                         GtkWidget      *child,
                                         gboolean       *expand,
                                         gboolean       *fill,
                                         guint          *padding,
                                         GtkPackType    *pack_type);
void        gtk_box_set_child_packing   (GtkBox         *box,
                                         GtkWidget      *child,
                                         gboolean        expand,
                                         gboolean        fill,
                                         guint           padding,
                                         GtkPackType     pack_type);

/* internal API */
void        _gtk_box_set_old_defaults   (GtkBox         *box);
gboolean    _gtk_box_get_spacing_set    (GtkBox         *box);
void        _gtk_box_set_spacing_set    (GtkBox         *box,
                                         gboolean        spacing_set);

G_END_DECLS

#endif /* __GTK_BOX_H__ */
