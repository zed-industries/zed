/*
 * Copyright (c) 2013 Red Hat, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * Author: Alexander Larsson <alexl@redhat.com>
 *
 */

#ifndef __GTK_STACK_H__
#define __GTK_STACK_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>

G_BEGIN_DECLS


#define GTK_TYPE_STACK (gtk_stack_get_type ())
#define GTK_STACK(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STACK, GtkStack))
#define GTK_STACK_CLASS(klass) (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_STACK, GtkStackClass))
#define GTK_IS_STACK(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STACK))
#define GTK_IS_STACK_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_STACK))
#define GTK_STACK_GET_CLASS(obj) (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_STACK, GtkStackClass))

typedef struct _GtkStack GtkStack;
typedef struct _GtkStackClass GtkStackClass;

typedef enum {
  GTK_STACK_TRANSITION_TYPE_NONE,
  GTK_STACK_TRANSITION_TYPE_CROSSFADE,
  GTK_STACK_TRANSITION_TYPE_SLIDE_RIGHT,
  GTK_STACK_TRANSITION_TYPE_SLIDE_LEFT,
  GTK_STACK_TRANSITION_TYPE_SLIDE_UP,
  GTK_STACK_TRANSITION_TYPE_SLIDE_DOWN,
  GTK_STACK_TRANSITION_TYPE_SLIDE_LEFT_RIGHT,
  GTK_STACK_TRANSITION_TYPE_SLIDE_UP_DOWN,
  GTK_STACK_TRANSITION_TYPE_OVER_UP,
  GTK_STACK_TRANSITION_TYPE_OVER_DOWN,
  GTK_STACK_TRANSITION_TYPE_OVER_LEFT,
  GTK_STACK_TRANSITION_TYPE_OVER_RIGHT,
  GTK_STACK_TRANSITION_TYPE_UNDER_UP,
  GTK_STACK_TRANSITION_TYPE_UNDER_DOWN,
  GTK_STACK_TRANSITION_TYPE_UNDER_LEFT,
  GTK_STACK_TRANSITION_TYPE_UNDER_RIGHT,
  GTK_STACK_TRANSITION_TYPE_OVER_UP_DOWN,
  GTK_STACK_TRANSITION_TYPE_OVER_DOWN_UP,
  GTK_STACK_TRANSITION_TYPE_OVER_LEFT_RIGHT,
  GTK_STACK_TRANSITION_TYPE_OVER_RIGHT_LEFT
} GtkStackTransitionType;

struct _GtkStack {
  GtkContainer parent_instance;
};

struct _GtkStackClass {
  GtkContainerClass parent_class;
};

GDK_AVAILABLE_IN_3_10
GType                  gtk_stack_get_type                (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_10
GtkWidget *            gtk_stack_new                     (void);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_add_named               (GtkStack               *stack,
                                                          GtkWidget              *child,
                                                          const gchar            *name);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_add_titled              (GtkStack               *stack,
                                                          GtkWidget              *child,
                                                          const gchar            *name,
                                                          const gchar            *title);
GDK_AVAILABLE_IN_3_12
GtkWidget *            gtk_stack_get_child_by_name       (GtkStack               *stack,
                                                          const gchar            *name);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_set_visible_child       (GtkStack               *stack,
                                                          GtkWidget              *child);
GDK_AVAILABLE_IN_3_10
GtkWidget *            gtk_stack_get_visible_child       (GtkStack               *stack);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_set_visible_child_name  (GtkStack               *stack,
                                                          const gchar            *name);
GDK_AVAILABLE_IN_3_10
const gchar *          gtk_stack_get_visible_child_name  (GtkStack               *stack);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_set_visible_child_full  (GtkStack               *stack,
                                                          const gchar            *name,
                                                          GtkStackTransitionType  transition);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_set_homogeneous         (GtkStack               *stack,
                                                          gboolean                homogeneous);
GDK_AVAILABLE_IN_3_10
gboolean               gtk_stack_get_homogeneous         (GtkStack               *stack);
GDK_AVAILABLE_IN_3_16
void                   gtk_stack_set_hhomogeneous        (GtkStack               *stack,
                                                          gboolean                hhomogeneous);
GDK_AVAILABLE_IN_3_16
gboolean               gtk_stack_get_hhomogeneous        (GtkStack               *stack);
GDK_AVAILABLE_IN_3_16
void                   gtk_stack_set_vhomogeneous        (GtkStack               *stack,
                                                          gboolean                vhomogeneous);
GDK_AVAILABLE_IN_3_16
gboolean               gtk_stack_get_vhomogeneous        (GtkStack               *stack);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_set_transition_duration (GtkStack               *stack,
                                                          guint                   duration);
GDK_AVAILABLE_IN_3_10
guint                  gtk_stack_get_transition_duration (GtkStack               *stack);
GDK_AVAILABLE_IN_3_10
void                   gtk_stack_set_transition_type     (GtkStack               *stack,
                                                          GtkStackTransitionType  transition);
GDK_AVAILABLE_IN_3_10
GtkStackTransitionType gtk_stack_get_transition_type     (GtkStack               *stack);
GDK_AVAILABLE_IN_3_12
gboolean               gtk_stack_get_transition_running  (GtkStack               *stack);
GDK_AVAILABLE_IN_3_18
void                   gtk_stack_set_interpolate_size    (GtkStack *stack,
                                                          gboolean  interpolate_size);
GDK_AVAILABLE_IN_3_18
gboolean               gtk_stack_get_interpolate_size    (GtkStack *stack);
G_END_DECLS

#endif
