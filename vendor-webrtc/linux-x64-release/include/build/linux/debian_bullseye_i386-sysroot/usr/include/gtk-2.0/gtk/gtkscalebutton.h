/* GTK - The GIMP Toolkit
 * Copyright (C) 2005 Ronald S. Bultje
 * Copyright (C) 2006, 2007 Christian Persch
 * Copyright (C) 2006 Jan Arne Petersen
 * Copyright (C) 2007 Red Hat, Inc.
 *
 * Authors:
 * - Ronald S. Bultje <rbultje@ronald.bitfreak.net>
 * - Bastien Nocera <bnocera@redhat.com>
 * - Jan Arne Petersen <jpetersen@jpetersen.org>
 * - Christian Persch <chpe@svn.gnome.org>
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
 * Modified by the GTK+ Team and others 2007.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SCALE_BUTTON_H__
#define __GTK_SCALE_BUTTON_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbutton.h>

G_BEGIN_DECLS

#define GTK_TYPE_SCALE_BUTTON                 (gtk_scale_button_get_type ())
#define GTK_SCALE_BUTTON(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SCALE_BUTTON, GtkScaleButton))
#define GTK_SCALE_BUTTON_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SCALE_BUTTON, GtkScaleButtonClass))
#define GTK_IS_SCALE_BUTTON(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SCALE_BUTTON))
#define GTK_IS_SCALE_BUTTON_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SCALE_BUTTON))
#define GTK_SCALE_BUTTON_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SCALE_BUTTON, GtkScaleButtonClass))

typedef struct _GtkScaleButton        GtkScaleButton;
typedef struct _GtkScaleButtonClass   GtkScaleButtonClass;
typedef struct _GtkScaleButtonPrivate GtkScaleButtonPrivate;

struct _GtkScaleButton
{
  GtkButton parent;

  GtkWidget *GSEAL (plus_button);
  GtkWidget *GSEAL (minus_button);

  /*< private >*/
  GtkScaleButtonPrivate *GSEAL (priv);
};

struct _GtkScaleButtonClass
{
  GtkButtonClass parent_class;

  /* signals */
  void	(* value_changed) (GtkScaleButton *button,
                           gdouble         value);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType            gtk_scale_button_get_type         (void) G_GNUC_CONST;
GtkWidget *      gtk_scale_button_new              (GtkIconSize      size,
                                                    gdouble          min,
                                                    gdouble          max,
                                                    gdouble          step,
                                                    const gchar    **icons);
void             gtk_scale_button_set_icons        (GtkScaleButton  *button,
                                                    const gchar    **icons);
gdouble          gtk_scale_button_get_value        (GtkScaleButton  *button);
void             gtk_scale_button_set_value        (GtkScaleButton  *button,
                                                    gdouble          value);
GtkAdjustment *  gtk_scale_button_get_adjustment   (GtkScaleButton  *button);
void             gtk_scale_button_set_adjustment   (GtkScaleButton  *button,
                                                    GtkAdjustment   *adjustment);
GtkWidget *      gtk_scale_button_get_plus_button  (GtkScaleButton  *button);
GtkWidget *      gtk_scale_button_get_minus_button (GtkScaleButton  *button);
GtkWidget *      gtk_scale_button_get_popup        (GtkScaleButton  *button);

#ifndef GTK_DISABLE_DEPRECATED

GtkOrientation   gtk_scale_button_get_orientation  (GtkScaleButton  *button);
void             gtk_scale_button_set_orientation  (GtkScaleButton  *button,
                                                    GtkOrientation   orientation);

#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_SCALE_BUTTON_H__ */
