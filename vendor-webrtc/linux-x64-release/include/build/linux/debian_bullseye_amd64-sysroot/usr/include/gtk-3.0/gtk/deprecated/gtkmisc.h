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

#ifndef __GTK_MISC_H__
#define __GTK_MISC_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_MISC		       (gtk_misc_get_type ())
#define GTK_MISC(obj)		       (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MISC, GtkMisc))
#define GTK_MISC_CLASS(klass)	       (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MISC, GtkMiscClass))
#define GTK_IS_MISC(obj)	       (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MISC))
#define GTK_IS_MISC_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MISC))
#define GTK_MISC_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MISC, GtkMiscClass))


typedef struct _GtkMisc              GtkMisc;
typedef struct _GtkMiscPrivate       GtkMiscPrivate;
typedef struct _GtkMiscClass         GtkMiscClass;

struct _GtkMisc
{
  GtkWidget widget;

  /*< private >*/
  GtkMiscPrivate *priv;
};

struct _GtkMiscClass
{
  GtkWidgetClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_DEPRECATED_IN_3_14
GType   gtk_misc_get_type      (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_14
void	gtk_misc_set_alignment (GtkMisc *misc,
				gfloat	 xalign,
				gfloat	 yalign);
GDK_DEPRECATED_IN_3_14
void    gtk_misc_get_alignment (GtkMisc *misc,
				gfloat  *xalign,
				gfloat  *yalign);
GDK_DEPRECATED_IN_3_14
void	gtk_misc_set_padding   (GtkMisc *misc,
				gint	 xpad,
				gint	 ypad);
GDK_DEPRECATED_IN_3_14
void    gtk_misc_get_padding   (GtkMisc *misc,
				gint    *xpad,
				gint    *ypad);

void   _gtk_misc_get_padding_and_border	(GtkMisc   *misc,
					 GtkBorder *border);

G_END_DECLS

#endif /* __GTK_MISC_H__ */
