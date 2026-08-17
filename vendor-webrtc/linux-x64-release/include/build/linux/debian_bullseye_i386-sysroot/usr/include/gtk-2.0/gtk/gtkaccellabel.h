/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * GtkAccelLabel: GtkLabel with accelerator monitoring facilities.
 * Copyright (C) 1998 Tim Janik
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
 * Modified by the GTK+ Team and others 1997-2001.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_ACCEL_LABEL_H__
#define __GTK_ACCEL_LABEL_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtklabel.h>


G_BEGIN_DECLS

#define GTK_TYPE_ACCEL_LABEL		(gtk_accel_label_get_type ())
#define GTK_ACCEL_LABEL(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACCEL_LABEL, GtkAccelLabel))
#define GTK_ACCEL_LABEL_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ACCEL_LABEL, GtkAccelLabelClass))
#define GTK_IS_ACCEL_LABEL(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACCEL_LABEL))
#define GTK_IS_ACCEL_LABEL_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ACCEL_LABEL))
#define GTK_ACCEL_LABEL_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ACCEL_LABEL, GtkAccelLabelClass))


typedef struct _GtkAccelLabel	    GtkAccelLabel;
typedef struct _GtkAccelLabelClass  GtkAccelLabelClass;

/**
 * GtkAccelLabel:
 *
 * The #GtkAccelLabel-struct struct contains private data only, and
 * should be accessed using the functions below.
 */
struct _GtkAccelLabel
{
  GtkLabel label;

  guint          GSEAL (gtk_reserved);
  guint          GSEAL (accel_padding);      /* should be style property? */
  GtkWidget     *GSEAL (accel_widget);       /* done*/
  GClosure      *GSEAL (accel_closure);      /* has set function */
  GtkAccelGroup *GSEAL (accel_group);        /* set by set_accel_closure() */
  gchar         *GSEAL (accel_string);       /* has set function */
  guint16        GSEAL (accel_string_width); /* seems to be private */
};

struct _GtkAccelLabelClass
{
  GtkLabelClass	 parent_class;

  gchar		*signal_quote1;
  gchar		*signal_quote2;
  gchar		*mod_name_shift;
  gchar		*mod_name_control;
  gchar		*mod_name_alt;
  gchar		*mod_separator;
  gchar		*accel_seperator;
  guint		 latin1_to_char : 1;
  
  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

#ifndef GTK_DISABLE_DEPRECATED
#define	gtk_accel_label_accelerator_width	gtk_accel_label_get_accel_width
#endif /* GTK_DISABLE_DEPRECATED */

GType	   gtk_accel_label_get_type	     (void) G_GNUC_CONST;
GtkWidget* gtk_accel_label_new		     (const gchar   *string);
GtkWidget* gtk_accel_label_get_accel_widget  (GtkAccelLabel *accel_label);
guint	   gtk_accel_label_get_accel_width   (GtkAccelLabel *accel_label);
void	   gtk_accel_label_set_accel_widget  (GtkAccelLabel *accel_label,
					      GtkWidget	    *accel_widget);
void	   gtk_accel_label_set_accel_closure (GtkAccelLabel *accel_label,
					      GClosure	    *accel_closure);
gboolean   gtk_accel_label_refetch           (GtkAccelLabel *accel_label);

/* private */
gchar *    _gtk_accel_label_class_get_accelerator_label (GtkAccelLabelClass *klass,
							 guint               accelerator_key,
							 GdkModifierType     accelerator_mods);

G_END_DECLS

#endif /* __GTK_ACCEL_LABEL_H__ */
