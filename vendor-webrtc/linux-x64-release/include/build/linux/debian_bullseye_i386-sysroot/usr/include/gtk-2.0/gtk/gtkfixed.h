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

#ifndef __GTK_FIXED_H__
#define __GTK_FIXED_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>


G_BEGIN_DECLS

#define GTK_TYPE_FIXED                  (gtk_fixed_get_type ())
#define GTK_FIXED(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FIXED, GtkFixed))
#define GTK_FIXED_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FIXED, GtkFixedClass))
#define GTK_IS_FIXED(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FIXED))
#define GTK_IS_FIXED_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FIXED))
#define GTK_FIXED_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FIXED, GtkFixedClass))


typedef struct _GtkFixed        GtkFixed;
typedef struct _GtkFixedClass   GtkFixedClass;
typedef struct _GtkFixedChild   GtkFixedChild;

struct _GtkFixed
{
  GtkContainer container;

  GList *GSEAL (children);
};

struct _GtkFixedClass
{
  GtkContainerClass parent_class;
};

struct _GtkFixedChild
{
  GtkWidget *widget;
  gint x;
  gint y;
};


GType      gtk_fixed_get_type          (void) G_GNUC_CONST;
GtkWidget* gtk_fixed_new               (void);
void       gtk_fixed_put               (GtkFixed       *fixed,
                                        GtkWidget      *widget,
                                        gint            x,
                                        gint            y);
void       gtk_fixed_move              (GtkFixed       *fixed,
                                        GtkWidget      *widget,
                                        gint            x,
                                        gint            y);
#ifndef GTK_DISABLE_DEPRECATED
void       gtk_fixed_set_has_window    (GtkFixed       *fixed,
					gboolean        has_window);
gboolean   gtk_fixed_get_has_window    (GtkFixed       *fixed);
#endif

G_END_DECLS

#endif /* __GTK_FIXED_H__ */
