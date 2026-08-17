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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_TABLE_H__
#define __GTK_TABLE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>

G_BEGIN_DECLS

#define GTK_TYPE_TABLE			(gtk_table_get_type ())
#define GTK_TABLE(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TABLE, GtkTable))
#define GTK_TABLE_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TABLE, GtkTableClass))
#define GTK_IS_TABLE(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TABLE))
#define GTK_IS_TABLE_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TABLE))
#define GTK_TABLE_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TABLE, GtkTableClass))


typedef struct _GtkTable              GtkTable;
typedef struct _GtkTablePrivate       GtkTablePrivate;
typedef struct _GtkTableClass         GtkTableClass;
typedef struct _GtkTableChild         GtkTableChild;
typedef struct _GtkTableRowCol        GtkTableRowCol;

struct _GtkTable
{
  GtkContainer container;

  /*< private >*/
  GtkTablePrivate *priv;
};

struct _GtkTableClass
{
  GtkContainerClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

struct _GtkTableChild
{
  GtkWidget *widget;
  guint16 left_attach;
  guint16 right_attach;
  guint16 top_attach;
  guint16 bottom_attach;
  guint16 xpadding;
  guint16 ypadding;
  guint xexpand : 1;
  guint yexpand : 1;
  guint xshrink : 1;
  guint yshrink : 1;
  guint xfill : 1;
  guint yfill : 1;
};

struct _GtkTableRowCol
{
  guint16 requisition;
  guint16 allocation;
  guint16 spacing;
  guint need_expand : 1;
  guint need_shrink : 1;
  guint expand : 1;
  guint shrink : 1;
  guint empty : 1;
};

/**
 * GtkAttachOptions:
 * @GTK_EXPAND: the widget should expand to take up any extra space in its
 * container that has been allocated.
 * @GTK_SHRINK: the widget should shrink as and when possible.
 * @GTK_FILL: the widget should fill the space allocated to it.
 *
 * Denotes the expansion properties that a widget will have when it (or its
 * parent) is resized.
 */
typedef enum
{
  GTK_EXPAND = 1 << 0,
  GTK_SHRINK = 1 << 1,
  GTK_FILL   = 1 << 2
} GtkAttachOptions;


GDK_DEPRECATED_IN_3_4
GType	   gtk_table_get_type	      (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
GtkWidget* gtk_table_new	      (guint		rows,
				       guint		columns,
				       gboolean		homogeneous);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_resize	      (GtkTable	       *table,
				       guint            rows,
				       guint            columns);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_attach	      (GtkTable	       *table,
				       GtkWidget       *child,
				       guint		left_attach,
				       guint		right_attach,
				       guint		top_attach,
				       guint		bottom_attach,
				       GtkAttachOptions xoptions,
				       GtkAttachOptions yoptions,
				       guint		xpadding,
				       guint		ypadding);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_attach_defaults  (GtkTable	       *table,
				       GtkWidget       *widget,
				       guint		left_attach,
				       guint		right_attach,
				       guint		top_attach,
				       guint		bottom_attach);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_set_row_spacing  (GtkTable	       *table,
				       guint		row,
				       guint		spacing);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
guint      gtk_table_get_row_spacing  (GtkTable        *table,
				       guint            row);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_set_col_spacing  (GtkTable	       *table,
				       guint		column,
				       guint		spacing);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
guint      gtk_table_get_col_spacing  (GtkTable        *table,
				       guint            column);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_set_row_spacings (GtkTable	       *table,
				       guint		spacing);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
guint      gtk_table_get_default_row_spacing (GtkTable        *table);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_set_col_spacings (GtkTable	       *table,
				       guint		spacing);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
guint      gtk_table_get_default_col_spacing (GtkTable        *table);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void	   gtk_table_set_homogeneous  (GtkTable	       *table,
				       gboolean		homogeneous);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
gboolean   gtk_table_get_homogeneous  (GtkTable        *table);
GDK_DEPRECATED_IN_3_4_FOR(GtkGrid)
void       gtk_table_get_size         (GtkTable        *table,
                                       guint           *rows,
                                       guint           *columns);

G_END_DECLS

#endif /* __GTK_TABLE_H__ */
