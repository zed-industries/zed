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

#ifndef __GTK_PROGRESS_BAR_H__
#define __GTK_PROGRESS_BAR_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkprogress.h>


G_BEGIN_DECLS

#define GTK_TYPE_PROGRESS_BAR            (gtk_progress_bar_get_type ())
#define GTK_PROGRESS_BAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PROGRESS_BAR, GtkProgressBar))
#define GTK_PROGRESS_BAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PROGRESS_BAR, GtkProgressBarClass))
#define GTK_IS_PROGRESS_BAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PROGRESS_BAR))
#define GTK_IS_PROGRESS_BAR_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PROGRESS_BAR))
#define GTK_PROGRESS_BAR_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PROGRESS_BAR, GtkProgressBarClass))


typedef struct _GtkProgressBar       GtkProgressBar;
typedef struct _GtkProgressBarClass  GtkProgressBarClass;

typedef enum
{
  GTK_PROGRESS_CONTINUOUS,
  GTK_PROGRESS_DISCRETE
} GtkProgressBarStyle;

typedef enum
{
  GTK_PROGRESS_LEFT_TO_RIGHT,
  GTK_PROGRESS_RIGHT_TO_LEFT,
  GTK_PROGRESS_BOTTOM_TO_TOP,
  GTK_PROGRESS_TOP_TO_BOTTOM
} GtkProgressBarOrientation;

struct _GtkProgressBar
{
  GtkProgress progress;

  GtkProgressBarStyle       GSEAL (bar_style);
  GtkProgressBarOrientation GSEAL (orientation);

  guint GSEAL (blocks);
  gint  GSEAL (in_block);

  gint  GSEAL (activity_pos);
  guint GSEAL (activity_step);
  guint GSEAL (activity_blocks);

  gdouble GSEAL (pulse_fraction);

  guint GSEAL (activity_dir) : 1;
  guint GSEAL (ellipsize) : 3;
  guint GSEAL (dirty) : 1;
};

struct _GtkProgressBarClass
{
  GtkProgressClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType      gtk_progress_bar_get_type             (void) G_GNUC_CONST;
GtkWidget* gtk_progress_bar_new                  (void);

/*
 * GtkProgress/GtkProgressBar had serious problems in GTK 1.2.
 *
 *  - Only 3 or 4 functions are really needed for 95% of progress
 *    interfaces; GtkProgress[Bar] had about 25 functions, and
 *    didn't even include these 3 or 4.
 *  - In activity mode, the API involves setting the adjustment
 *    to any random value, just to have the side effect of
 *    calling the progress bar update function - the adjustment
 *    is totally ignored in activity mode
 *  - You set the activity step as a pixel value, which means to
 *    set the activity step you basically need to connect to
 *    size_allocate
 *  - There are ctree_set_expander_style()-functions, to randomly
 *    change look-and-feel for no good reason
 *  - The split between GtkProgress and GtkProgressBar makes no sense
 *    to me whatsoever.
 *
 * This was a big wart on GTK and made people waste lots of time,
 * both learning and using the interface.
 *
 * So, I have added what I feel is the correct API, and marked all the
 * rest deprecated. However, the changes are 100% backward-compatible and
 * should break no existing code.
 *
 * The following 9 functions are the new programming interface.
 */
void       gtk_progress_bar_pulse                (GtkProgressBar *pbar);
void       gtk_progress_bar_set_text             (GtkProgressBar *pbar,
                                                  const gchar    *text);
void       gtk_progress_bar_set_fraction         (GtkProgressBar *pbar,
                                                  gdouble         fraction);

void       gtk_progress_bar_set_pulse_step       (GtkProgressBar *pbar,
                                                  gdouble         fraction);
void       gtk_progress_bar_set_orientation      (GtkProgressBar *pbar,
						  GtkProgressBarOrientation orientation);

const gchar*          gtk_progress_bar_get_text       (GtkProgressBar *pbar);
gdouble               gtk_progress_bar_get_fraction   (GtkProgressBar *pbar);
gdouble               gtk_progress_bar_get_pulse_step (GtkProgressBar *pbar);

GtkProgressBarOrientation gtk_progress_bar_get_orientation (GtkProgressBar *pbar);
void               gtk_progress_bar_set_ellipsize (GtkProgressBar     *pbar,
						   PangoEllipsizeMode  mode);
PangoEllipsizeMode gtk_progress_bar_get_ellipsize (GtkProgressBar     *pbar);


#ifndef GTK_DISABLE_DEPRECATED

/* Everything below here is deprecated */
GtkWidget* gtk_progress_bar_new_with_adjustment  (GtkAdjustment  *adjustment);
void       gtk_progress_bar_set_bar_style        (GtkProgressBar *pbar,
						  GtkProgressBarStyle style);
void       gtk_progress_bar_set_discrete_blocks  (GtkProgressBar *pbar,
						  guint           blocks);
/* set_activity_step() is not only deprecated, it doesn't even work.
 * (Of course, it wasn't usable anyway, you had to set it from a size_allocate
 * handler or something)
 */
void       gtk_progress_bar_set_activity_step    (GtkProgressBar *pbar,
                                                  guint           step);
void       gtk_progress_bar_set_activity_blocks  (GtkProgressBar *pbar,
						  guint           blocks);
void       gtk_progress_bar_update               (GtkProgressBar *pbar,
						  gdouble         percentage);

#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_PROGRESS_BAR_H__ */
