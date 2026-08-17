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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
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

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_SPIN_BUTTON                  (gtk_spin_button_get_type ())
#define GTK_SPIN_BUTTON(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SPIN_BUTTON, GtkSpinButton))
#define GTK_IS_SPIN_BUTTON(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SPIN_BUTTON))

/**
 * GTK_INPUT_ERROR:
 *
 * Constant to return from a signal handler for the ::input
 * signal in case of conversion failure.
 *
 * See [signal@Gtk.SpinButton::input].
 */
#define GTK_INPUT_ERROR -1

/**
 * GtkSpinButtonUpdatePolicy:
 * @GTK_UPDATE_ALWAYS: When refreshing your `GtkSpinButton`, the value is
 *   always displayed
 * @GTK_UPDATE_IF_VALID: When refreshing your `GtkSpinButton`, the value is
 *   only displayed if it is valid within the bounds of the spin button's
 *   adjustment
 *
 * Determines whether the spin button displays values outside the adjustment
 * bounds.
 *
 * See [method@Gtk.SpinButton.set_update_policy].
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

GDK_AVAILABLE_IN_ALL
GType           gtk_spin_button_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_configure          (GtkSpinButton  *spin_button,
                                                    GtkAdjustment  *adjustment,
                                                    double          climb_rate,
                                                    guint           digits);

GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_spin_button_new                (GtkAdjustment  *adjustment,
                                                    double          climb_rate,
                                                    guint           digits);

GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_spin_button_new_with_range     (double   min,
                                                    double   max,
                                                    double   step);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_adjustment     (GtkSpinButton  *spin_button,
                                                    GtkAdjustment  *adjustment);

GDK_AVAILABLE_IN_ALL
GtkAdjustment*  gtk_spin_button_get_adjustment     (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_digits         (GtkSpinButton  *spin_button,
                                                    guint           digits);
GDK_AVAILABLE_IN_ALL
guint           gtk_spin_button_get_digits         (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_increments     (GtkSpinButton  *spin_button,
                                                    double          step,
                                                    double          page);
GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_get_increments     (GtkSpinButton  *spin_button,
                                                    double         *step,
                                                    double         *page);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_range          (GtkSpinButton  *spin_button,
                                                    double          min,
                                                    double          max);
GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_get_range          (GtkSpinButton  *spin_button,
                                                    double         *min,
                                                    double         *max);

GDK_AVAILABLE_IN_ALL
double          gtk_spin_button_get_value          (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
int             gtk_spin_button_get_value_as_int   (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_value          (GtkSpinButton  *spin_button,
                                                    double          value);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_update_policy  (GtkSpinButton  *spin_button,
                                                    GtkSpinButtonUpdatePolicy  policy);
GDK_AVAILABLE_IN_ALL
GtkSpinButtonUpdatePolicy gtk_spin_button_get_update_policy (GtkSpinButton *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_numeric        (GtkSpinButton  *spin_button,
                                                    gboolean        numeric);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_spin_button_get_numeric        (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_spin               (GtkSpinButton  *spin_button,
                                                    GtkSpinType     direction,
                                                    double          increment);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_wrap           (GtkSpinButton  *spin_button,
                                                    gboolean        wrap);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_spin_button_get_wrap           (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_snap_to_ticks  (GtkSpinButton  *spin_button,
                                                    gboolean        snap_to_ticks);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_spin_button_get_snap_to_ticks  (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_set_climb_rate     (GtkSpinButton  *spin_button,
                                                    double          climb_rate);
GDK_AVAILABLE_IN_ALL
double          gtk_spin_button_get_climb_rate     (GtkSpinButton  *spin_button);

GDK_AVAILABLE_IN_ALL
void            gtk_spin_button_update             (GtkSpinButton  *spin_button);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSpinButton, g_object_unref)

G_END_DECLS

#endif /* __GTK_SPIN_BUTTON_H__ */
