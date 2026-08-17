/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * GtkSpinButton widget for GTK+
 * Copyright (C) 1998 Lars Hamann and Stefan Jeske
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

#ifndef __GTK_SPIN_BUTTON_H__
#define __GTK_SPIN_BUTTON_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkentry.h>
#include <gtk/gtkadjustment.h>


G_BEGIN_DECLS

#define GTK_TYPE_SPIN_BUTTON                  (gtk_spin_button_get_type ())
#define GTK_SPIN_BUTTON(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SPIN_BUTTON, GtkSpinButton))
#define GTK_SPIN_BUTTON_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SPIN_BUTTON, GtkSpinButtonClass))
#define GTK_IS_SPIN_BUTTON(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SPIN_BUTTON))
#define GTK_IS_SPIN_BUTTON_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SPIN_BUTTON))
#define GTK_SPIN_BUTTON_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SPIN_BUTTON, GtkSpinButtonClass))

#define GTK_INPUT_ERROR -1

typedef enum
{
  GTK_UPDATE_ALWAYS,
  GTK_UPDATE_IF_VALID
} GtkSpinButtonUpdatePolicy;

typedef enum
{
  GTK_SPIN_STEP_FORWARD,
  GTK_SPIN_STEP_BACKWARD,
  GTK_SPIN_PAGE_FORWARD,
  GTK_SPIN_PAGE_BACKWARD,
  GTK_SPIN_HOME,
  GTK_SPIN_END,
  GTK_SPIN_USER_DEFINED
} GtkSpinType;


typedef struct _GtkSpinButton	    GtkSpinButton;
typedef struct _GtkSpinButtonClass  GtkSpinButtonClass;


struct _GtkSpinButton
{
  GtkEntry entry;

  GtkAdjustment *GSEAL (adjustment);

  GdkWindow *GSEAL (panel);

  guint32 GSEAL (timer);

  gdouble GSEAL (climb_rate);
  gdouble GSEAL (timer_step);

  GtkSpinButtonUpdatePolicy GSEAL (update_policy);

  guint GSEAL (in_child) : 2;
  guint GSEAL (click_child) : 2; /* valid: GTK_ARROW_UP=0, GTK_ARROW_DOWN=1 or 2=NONE/BOTH */
  guint GSEAL (button) : 2;
  guint GSEAL (need_timer) : 1;
  guint GSEAL (timer_calls) : 3;
  guint GSEAL (digits) : 10;
  guint GSEAL (numeric) : 1;
  guint GSEAL (wrap) : 1;
  guint GSEAL (snap_to_ticks) : 1;
};

struct _GtkSpinButtonClass
{
  GtkEntryClass parent_class;

  gint (*input)  (GtkSpinButton *spin_button,
		  gdouble       *new_value);
  gint (*output) (GtkSpinButton *spin_button);
  void (*value_changed) (GtkSpinButton *spin_button);

  /* Action signals for keybindings, do not connect to these */
  void (*change_value) (GtkSpinButton *spin_button,
			GtkScrollType  scroll);

  void (*wrapped) (GtkSpinButton *spin_button);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};


GType		gtk_spin_button_get_type	   (void) G_GNUC_CONST;

void		gtk_spin_button_configure	   (GtkSpinButton  *spin_button,
						    GtkAdjustment  *adjustment,
						    gdouble	    climb_rate,
						    guint	    digits);

GtkWidget*	gtk_spin_button_new		   (GtkAdjustment  *adjustment,
						    gdouble	    climb_rate,
						    guint	    digits);

GtkWidget*	gtk_spin_button_new_with_range	   (gdouble  min,
						    gdouble  max,
						    gdouble  step);

void		gtk_spin_button_set_adjustment	   (GtkSpinButton  *spin_button,
						    GtkAdjustment  *adjustment);

GtkAdjustment*	gtk_spin_button_get_adjustment	   (GtkSpinButton  *spin_button);

void		gtk_spin_button_set_digits	   (GtkSpinButton  *spin_button,
						    guint	    digits);
guint           gtk_spin_button_get_digits         (GtkSpinButton  *spin_button);

void		gtk_spin_button_set_increments	   (GtkSpinButton  *spin_button,
						    gdouble         step,
						    gdouble         page);
void            gtk_spin_button_get_increments     (GtkSpinButton  *spin_button,
						    gdouble        *step,
						    gdouble        *page);

void		gtk_spin_button_set_range	   (GtkSpinButton  *spin_button,
						    gdouble         min,
						    gdouble         max);
void            gtk_spin_button_get_range          (GtkSpinButton  *spin_button,
						    gdouble        *min,
						    gdouble        *max);

gdouble		gtk_spin_button_get_value          (GtkSpinButton  *spin_button);

gint		gtk_spin_button_get_value_as_int   (GtkSpinButton  *spin_button);

void		gtk_spin_button_set_value	   (GtkSpinButton  *spin_button,
						    gdouble	    value);

void		gtk_spin_button_set_update_policy  (GtkSpinButton  *spin_button,
						    GtkSpinButtonUpdatePolicy  policy);
GtkSpinButtonUpdatePolicy gtk_spin_button_get_update_policy (GtkSpinButton *spin_button);

void		gtk_spin_button_set_numeric	   (GtkSpinButton  *spin_button,
						    gboolean	    numeric);
gboolean        gtk_spin_button_get_numeric        (GtkSpinButton  *spin_button);

void		gtk_spin_button_spin		   (GtkSpinButton  *spin_button,
						    GtkSpinType     direction,
						    gdouble	    increment);

void		gtk_spin_button_set_wrap	   (GtkSpinButton  *spin_button,
						    gboolean	    wrap);
gboolean        gtk_spin_button_get_wrap           (GtkSpinButton  *spin_button);

void		gtk_spin_button_set_snap_to_ticks  (GtkSpinButton  *spin_button,
						    gboolean	    snap_to_ticks);
gboolean        gtk_spin_button_get_snap_to_ticks  (GtkSpinButton  *spin_button);
void            gtk_spin_button_update             (GtkSpinButton  *spin_button);


#ifndef GTK_DISABLE_DEPRECATED
#define gtk_spin_button_get_value_as_float gtk_spin_button_get_value
#endif

G_END_DECLS

#endif /* __GTK_SPIN_BUTTON_H__ */
