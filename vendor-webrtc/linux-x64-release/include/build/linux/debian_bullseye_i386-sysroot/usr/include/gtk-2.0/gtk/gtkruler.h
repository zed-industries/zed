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

/*
 * NOTE this widget is considered too specialized/little-used for
 * GTK+, and will in the future be moved to some other package.  If
 * your application needs this widget, feel free to use it, as the
 * widget does work and is useful in some applications; it's just not
 * of general interest. However, we are not accepting new features for
 * the widget, and it will eventually move out of the GTK+
 * distribution.
 */

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_RULER_H__
#define __GTK_RULER_H__


#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_RULER            (gtk_ruler_get_type ())
#define GTK_RULER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RULER, GtkRuler))
#define GTK_RULER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RULER, GtkRulerClass))
#define GTK_IS_RULER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RULER))
#define GTK_IS_RULER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RULER))
#define GTK_RULER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RULER, GtkRulerClass))


typedef struct _GtkRuler        GtkRuler;
typedef struct _GtkRulerClass   GtkRulerClass;
typedef struct _GtkRulerMetric  GtkRulerMetric;

/* All distances below are in 1/72nd's of an inch. (According to
 * Adobe that's a point, but points are really 1/72.27 in.)
 */
struct _GtkRuler
{
  GtkWidget widget;

  GdkPixmap *GSEAL (backing_store);
  GdkGC *GSEAL (non_gr_exp_gc);		/* unused */
  GtkRulerMetric *GSEAL (metric);
  gint GSEAL (xsrc);
  gint GSEAL (ysrc);
  gint GSEAL (slider_size);

  /* The upper limit of the ruler (in points) */
  gdouble GSEAL (lower);
  /* The lower limit of the ruler */
  gdouble GSEAL (upper);
  /* The position of the mark on the ruler */
  gdouble GSEAL (position);
  /* The maximum size of the ruler */
  gdouble GSEAL (max_size);
};

struct _GtkRulerClass
{
  GtkWidgetClass parent_class;

  void (* draw_ticks) (GtkRuler *ruler);
  void (* draw_pos)   (GtkRuler *ruler);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

struct _GtkRulerMetric
{
  gchar *metric_name;
  gchar *abbrev;
  /* This should be points_per_unit. This is the size of the unit
   * in 1/72nd's of an inch and has nothing to do with screen pixels */
  gdouble pixels_per_unit;
  gdouble ruler_scale[10];
  gint subdivide[5];        /* five possible modes of subdivision */
};


GType           gtk_ruler_get_type   (void) G_GNUC_CONST;
void            gtk_ruler_set_metric (GtkRuler       *ruler,
                                      GtkMetricType   metric);
GtkMetricType   gtk_ruler_get_metric (GtkRuler       *ruler);
void            gtk_ruler_set_range  (GtkRuler       *ruler,
                                      gdouble         lower,
                                      gdouble         upper,
                                      gdouble         position,
                                      gdouble         max_size);
void            gtk_ruler_get_range  (GtkRuler       *ruler,
                                      gdouble        *lower,
                                      gdouble        *upper,
                                      gdouble        *position,
                                      gdouble        *max_size);

void            gtk_ruler_draw_ticks (GtkRuler       *ruler);
void            gtk_ruler_draw_pos   (GtkRuler       *ruler);

G_END_DECLS

#endif /* __GTK_RULER_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
