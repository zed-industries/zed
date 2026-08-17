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

#include <gtk/gtkwidget.h>
#include <gtk/gtkselectionmodel.h>

G_BEGIN_DECLS


#define GTK_TYPE_STACK (gtk_stack_get_type ())
#define GTK_STACK(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STACK, GtkStack))
#define GTK_IS_STACK(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STACK))

typedef struct _GtkStack GtkStack;

#define GTK_TYPE_STACK_PAGE (gtk_stack_page_get_type ())
#define GTK_STACK_PAGE(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STACK_PAGE, GtkStackPage))
#define GTK_IS_STACK_PAGE(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STACK_PAGE))

typedef struct _GtkStackPage GtkStackPage;

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
  GTK_STACK_TRANSITION_TYPE_OVER_RIGHT_LEFT,
  GTK_STACK_TRANSITION_TYPE_ROTATE_LEFT,
  GTK_STACK_TRANSITION_TYPE_ROTATE_RIGHT,
  GTK_STACK_TRANSITION_TYPE_ROTATE_LEFT_RIGHT
} GtkStackTransitionType;

GDK_AVAILABLE_IN_ALL
GType                  gtk_stack_page_get_type            (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *            gtk_stack_page_get_child           (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_page_get_visible         (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_page_set_visible         (GtkStackPage           *self,
                                                           gboolean                visible);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_page_get_needs_attention (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_page_set_needs_attention (GtkStackPage           *self,
                                                           gboolean                setting);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_page_get_use_underline   (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_page_set_use_underline   (GtkStackPage           *self,
                                                           gboolean                setting);
GDK_AVAILABLE_IN_ALL
const char *           gtk_stack_page_get_name            (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_page_set_name            (GtkStackPage           *self,
                                                            const char            *setting);
GDK_AVAILABLE_IN_ALL
const char *           gtk_stack_page_get_title           (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_page_set_title           (GtkStackPage           *self,
                                                           const char             *setting);
GDK_AVAILABLE_IN_ALL
const char *           gtk_stack_page_get_icon_name       (GtkStackPage           *self);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_page_set_icon_name       (GtkStackPage           *self,
                                                           const char             *setting);



GDK_AVAILABLE_IN_ALL
GType                  gtk_stack_get_type                (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *            gtk_stack_new                     (void);
GDK_AVAILABLE_IN_ALL
GtkStackPage *         gtk_stack_add_child               (GtkStack               *stack,
                                                          GtkWidget              *child);
GDK_AVAILABLE_IN_ALL
GtkStackPage *         gtk_stack_add_named               (GtkStack               *stack,
                                                          GtkWidget              *child,
                                                          const char             *name);
GDK_AVAILABLE_IN_ALL
GtkStackPage *         gtk_stack_add_titled              (GtkStack               *stack,
                                                          GtkWidget              *child,
                                                          const char             *name,
                                                          const char             *title);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_remove                  (GtkStack               *stack,
                                                          GtkWidget              *child);

GDK_AVAILABLE_IN_ALL
GtkStackPage *         gtk_stack_get_page                (GtkStack               *stack,
                                                          GtkWidget              *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *            gtk_stack_get_child_by_name       (GtkStack               *stack,
                                                          const char             *name);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_visible_child       (GtkStack               *stack,
                                                          GtkWidget              *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *            gtk_stack_get_visible_child       (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_visible_child_name  (GtkStack               *stack,
                                                          const char             *name);
GDK_AVAILABLE_IN_ALL
const char *          gtk_stack_get_visible_child_name  (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_visible_child_full  (GtkStack               *stack,
                                                          const char             *name,
                                                          GtkStackTransitionType  transition);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_hhomogeneous        (GtkStack               *stack,
                                                          gboolean                hhomogeneous);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_get_hhomogeneous        (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_vhomogeneous        (GtkStack               *stack,
                                                          gboolean                vhomogeneous);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_get_vhomogeneous        (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_transition_duration (GtkStack               *stack,
                                                          guint                   duration);
GDK_AVAILABLE_IN_ALL
guint                  gtk_stack_get_transition_duration (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_transition_type     (GtkStack               *stack,
                                                          GtkStackTransitionType  transition);
GDK_AVAILABLE_IN_ALL
GtkStackTransitionType gtk_stack_get_transition_type     (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_get_transition_running  (GtkStack               *stack);
GDK_AVAILABLE_IN_ALL
void                   gtk_stack_set_interpolate_size    (GtkStack *stack,
                                                          gboolean  interpolate_size);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_stack_get_interpolate_size    (GtkStack *stack);

GDK_AVAILABLE_IN_ALL
GtkSelectionModel *    gtk_stack_get_pages               (GtkStack *stack);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkStack, g_object_unref)

G_END_DECLS

#endif
