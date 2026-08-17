/* GTK - The GIMP Toolkit
 * Copyright (C) 1997 David Mosberger
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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_GAMMA_CURVE_H__
#define __GTK_GAMMA_CURVE_H__


#include <gtk/gtkvbox.h>


G_BEGIN_DECLS

#define GTK_TYPE_GAMMA_CURVE            (gtk_gamma_curve_get_type ())
#define GTK_GAMMA_CURVE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_GAMMA_CURVE, GtkGammaCurve))
#define GTK_GAMMA_CURVE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_GAMMA_CURVE, GtkGammaCurveClass))
#define GTK_IS_GAMMA_CURVE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_GAMMA_CURVE))
#define GTK_IS_GAMMA_CURVE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_GAMMA_CURVE))
#define GTK_GAMMA_CURVE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_GAMMA_CURVE, GtkGammaCurveClass))

typedef struct _GtkGammaCurve		GtkGammaCurve;
typedef struct _GtkGammaCurveClass	GtkGammaCurveClass;


struct _GtkGammaCurve
{
  GtkVBox vbox;

  GtkWidget *GSEAL (table);
  GtkWidget *GSEAL (curve);
  GtkWidget *GSEAL (button[5]);	/* spline, linear, free, gamma, reset */

  gfloat GSEAL (gamma);
  GtkWidget *GSEAL (gamma_dialog);
  GtkWidget *GSEAL (gamma_text);
};

struct _GtkGammaCurveClass
{
  GtkVBoxClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType      gtk_gamma_curve_get_type (void) G_GNUC_CONST;
GtkWidget* gtk_gamma_curve_new      (void);


G_END_DECLS

#endif /* __GTK_GAMMA_CURVE_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
