/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2003 Sun Microsystems, Inc.
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 *
 * Authors:
 *	Mark McLoughlin <mark@skynet.ie>
 */

#ifndef __GTK_EXPANDER_H__
#define __GTK_EXPANDER_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>

G_BEGIN_DECLS

#define GTK_TYPE_EXPANDER            (gtk_expander_get_type ())
#define GTK_EXPANDER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_EXPANDER, GtkExpander))
#define GTK_EXPANDER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_EXPANDER, GtkExpanderClass))
#define GTK_IS_EXPANDER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_EXPANDER))
#define GTK_IS_EXPANDER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_EXPANDER))
#define GTK_EXPANDER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_EXPANDER, GtkExpanderClass))

typedef struct _GtkExpander        GtkExpander;
typedef struct _GtkExpanderClass   GtkExpanderClass;
typedef struct _GtkExpanderPrivate GtkExpanderPrivate;

struct _GtkExpander
{
  GtkBin              bin;

  GtkExpanderPrivate *GSEAL (priv);
};

struct _GtkExpanderClass
{
  GtkBinClass    parent_class;

  /* Key binding signal; to get notification on the expansion
   * state connect to notify:expanded.
   */
  void        (* activate) (GtkExpander *expander);
};

GType                 gtk_expander_get_type          (void) G_GNUC_CONST;

GtkWidget            *gtk_expander_new               (const gchar *label);
GtkWidget            *gtk_expander_new_with_mnemonic (const gchar *label);

void                  gtk_expander_set_expanded      (GtkExpander *expander,
						      gboolean     expanded);
gboolean              gtk_expander_get_expanded      (GtkExpander *expander);

/* Spacing between the expander/label and the child */
void                  gtk_expander_set_spacing       (GtkExpander *expander,
						      gint         spacing);
gint                  gtk_expander_get_spacing       (GtkExpander *expander);

void                  gtk_expander_set_label         (GtkExpander *expander,
						      const gchar *label);
const gchar *         gtk_expander_get_label         (GtkExpander *expander);

void                  gtk_expander_set_use_underline (GtkExpander *expander,
						      gboolean     use_underline);
gboolean              gtk_expander_get_use_underline (GtkExpander *expander);

void                  gtk_expander_set_use_markup    (GtkExpander *expander,
						      gboolean    use_markup);
gboolean              gtk_expander_get_use_markup    (GtkExpander *expander);

void                  gtk_expander_set_label_widget  (GtkExpander *expander,
						      GtkWidget   *label_widget);
GtkWidget            *gtk_expander_get_label_widget  (GtkExpander *expander);
void                  gtk_expander_set_label_fill    (GtkExpander *expander,
						      gboolean     label_fill);
gboolean              gtk_expander_get_label_fill    (GtkExpander *expander);

G_END_DECLS

#endif /* __GTK_EXPANDER_H__ */
