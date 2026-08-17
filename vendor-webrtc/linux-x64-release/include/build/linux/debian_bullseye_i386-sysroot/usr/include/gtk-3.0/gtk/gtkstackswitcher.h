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

#include <gtk/gtkbox.h>
#include <gtk/gtkstack.h>

G_BEGIN_DECLS

#define GTK_TYPE_STACK_SWITCHER            (gtk_stack_switcher_get_type ())
#define GTK_STACK_SWITCHER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STACK_SWITCHER, GtkStackSwitcher))
#define GTK_STACK_SWITCHER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_STACK_SWITCHER, GtkStackSwitcherClass))
#define GTK_IS_STACK_SWITCHER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STACK_SWITCHER))
#define GTK_IS_STACK_SWITCHER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_STACK_SWITCHER))
#define GTK_STACK_SWITCHER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_STACK_SWITCHER, GtkStackSwitcherClass))

typedef struct _GtkStackSwitcher              GtkStackSwitcher;
typedef struct _GtkStackSwitcherClass         GtkStackSwitcherClass;

struct _GtkStackSwitcher
{
  GtkBox widget;
};

struct _GtkStackSwitcherClass
{
  GtkBoxClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_3_10
GType        gtk_stack_switcher_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_10
GtkWidget *  gtk_stack_switcher_new               (void);
GDK_AVAILABLE_IN_3_10
void         gtk_stack_switcher_set_stack         (GtkStackSwitcher *switcher,
                                                   GtkStack         *stack);
GDK_AVAILABLE_IN_3_10
GtkStack *   gtk_stack_switcher_get_stack         (GtkStackSwitcher *switcher);

G_END_DECLS

#endif /* __GTK_STACK_SWITCHER_H__ */
