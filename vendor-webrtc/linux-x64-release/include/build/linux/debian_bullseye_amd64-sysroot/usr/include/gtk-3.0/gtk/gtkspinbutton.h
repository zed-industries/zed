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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SPIN_BUTTON_H__
#define __GTK_SPIN_BUTTON_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkentry.h>


G_BEGIN_DECLS

#define GTK_TYPE_SPIN_BUTTON                  (gtk_spin_button_get_type ())
#define GTK_SPIN_BUTTON(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SPIN_BUTTON, GtkSpinButton))
#define GTK_SPIN_BUTTON_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SPIN_BUTTON, GtkSpinButtonClass))
#define GTK_IS_SPIN_BUTTON(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SPIN_BUTTON))
#define GTK_IS_SPIN_BUTTON_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SPIN_BUTTON))
#define GTK_SPIN_BUTTON_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SPIN_BUTTON, GtkSpinButtonClass))

/**
 * GTK_INPUT_ERROR:
 *
 * Constant to return from a signal handler for the #GtkSpinButton::input
 * signal in case of conversion failure.
 */
#define GTK_INPUT_ERROR -1

/**
 * GtkSpinButtonUpdatePolicy:
 * @GTK_UPDATE_ALWAYS: When refreshing your #GtkSpinButton, the value is
 *     always displayed
 * @GTK_UPDATE_IF_VALID: When refreshing your #GtkSpinButton, the value is
 *     only displayed if it is valid within the bounds of the spin button's
 *     adjustment
 *
 * The spin button update policy determines whether the spin button displays
 * values even if they are outside the bounds of its adjustment.
 * See gtk_spin_button_set_update_policy().
 */
typedef enum
{
  GTK_UPDATE_ALWAYS,
  GTK_UPDATE_IF_VALID
} GtkSpinButtonUpdatePolicy;

/**
 * GtkSpinType:
 * @GTK_SPIN_STEP_FORWARD: Increment by the adjustments step increment.
 * @GTK_SPIN_STEP_BACKWARD: Decrement by the adjustments step increment.
 * @GTK_SPIN_PAGE_FORWARD: Increment by the adjustments page increment.
 * @GTK_SPIN_PAGE_BACKWARD: Decrement by the adjustments page increment.
 * @GTK_SPIN_HOME: Go to the adjustments lower bound.
 * @GTK_SPIN_END: Go to the adjustments upper bound.
 * @GTK_SPIN_USER_DEFINED: Change by a specified amount.
 *
 * The values of the GtkSpinType enumeration are used to specify the
 * change to make in gtk_spin_button_spin().
 */
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


typedef struct _GtkSpinButton              GtkSpinButton;
typedef struct _GtkSpinButtonPrivate       GtkSpinButtonPrivate;
typedef struct _GtkSpinButtonClass         GtkSpinButtonClass;

/**
 * GtkSpinButton:
 *
 * The #GtkSpinButton-struct contains only private data and should
 * not be directly modified.
 */
struct _GtkSpinButton
{
  GtkEntry entry;

  /*< private >*/
  GtkSpinButtonPrivate *priv;
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
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType		gtk_spin_button_get_type	   (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_configure	   (GtkSpinButton  *spin_button,
						    GtkAdjustment  *adjustment,
						    gdouble	    climb_rate,
						    guint	    digits);

GDK_AVAILABLE_IN_ALL
GtkWidget*	gtk_spin_button_new		   (GtkAdjustment  *adjustment,
						    gdouble	    climb_rate,
						    guint	    digits);

GDK_AVAILABLE_IN_ALL
GtkWidget*	gtk_spin_button_new_with_range	   (gdouble  min,
						    gdouble  max,
						    gdouble  step);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_adjustment	   (GtkSpinButton  *spin_button,
						    GtkAdjustment  *adjustment);

GDK_AVAILABLE_IN_ALL
GtkAdjustment*	gtk_spin_button_get_adjustment	   (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_digits	   (GtkSpinButton  *spin_button,
						    guint	    digits);
GDK_AVAILABLE_IN_ALL
guint           gtk_spin_button_get_digits         (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_increments	   (GtkSpinButton  *spin_button,
						    gdouble         step,
						    gdouble         page);
GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_get_increments     (GtkSpinButton  *spin_button,
						    gdouble        *step,
						    gdouble        *page);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_range	   (GtkSpinButton  *spin_button,
						    gdouble         min,
						    gdouble         max);
GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_get_range          (GtkSpinButton  *spin_button,
						    gdouble        *min,
						    gdouble        *max);

GDK_AVAILABLE_IN_ALL
gdouble		gtk_spin_button_get_value          (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
gint		gtk_spin_button_get_value_as_int   (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_value	   (GtkSpinButton  *spin_button,
						    gdouble	    value);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_update_policy  (GtkSpinButton  *spin_button,
						    GtkSpinButtonUpdatePolicy  policy);
GDK_AVAILABLE_IN_ALL
GtkSpinButtonUpdatePolicy gtk_spin_button_get_update_policy (GtkSpinButton *spin_button);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_numeric	   (GtkSpinButton  *spin_button,
						    gboolean	    numeric);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_spin_button_get_numeric        (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_spin		   (GtkSpinButton  *spin_button,
						    GtkSpinType     direction,
						    gdouble	    increment);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_wrap	   (GtkSpinButton  *spin_button,
						    gboolean	    wrap);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_spin_button_get_wrap           (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void		gtk_spin_button_set_snap_to_ticks  (GtkSpinButton  *spin_button,
						    gboolean	    snap_to_ticks);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_spin_button_get_snap_to_ticks  (GtkSpinButton  *spin_button);
GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_update             (GtkSpinButton  *spin_button);

/* private */
void            _gtk_spin_button_get_panels        (GtkSpinButton  *spin_button,
                                                    GdkWindow     **down_panel,
                                                    GdkWindow     **up_panel);

G_END_DECLS

#endif /* __GTK_SPIN_BUTTON_H__ */
