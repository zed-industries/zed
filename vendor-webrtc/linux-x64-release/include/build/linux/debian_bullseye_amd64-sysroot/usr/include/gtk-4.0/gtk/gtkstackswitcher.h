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
 */

#ifndef __GTK_STACK_SWITCHER_H__
#define __GTK_STACK_SWITCHER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkstack.h>

G_BEGIN_DECLS

#define GTK_TYPE_STACK_SWITCHER            (gtk_stack_switcher_get_type ())
#define GTK_STACK_SWITCHER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STACK_SWITCHER, GtkStackSwitcher))
#define GTK_IS_STACK_SWITCHER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STACK_SWITCHER))

typedef struct _GtkStackSwitcher              GtkStackSwitcher;


GDK_AVAILABLE_IN_ALL
GType        gtk_stack_switcher_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *  gtk_stack_switcher_new               (void);
GDK_AVAILABLE_IN_ALL
void         gtk_stack_switcher_set_stack         (GtkStackSwitcher *switcher,
                                                   GtkStack         *stack);
GDK_AVAILABLE_IN_ALL
GtkStack *   gtk_stack_switcher_get_stack         (GtkStackSwitcher *switcher);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkStackSwitcher, g_object_unref)

G_END_DECLS

#endif /* __GTK_STACK_SWITCHER_H__ */
