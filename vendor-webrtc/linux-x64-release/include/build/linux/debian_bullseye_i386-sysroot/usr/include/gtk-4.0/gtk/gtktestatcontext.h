/* gtktestatcontext.h: Test AT context
 *
 * Copyright 2020  GNOME Foundation
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#pragma once

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkatcontext.h>

G_BEGIN_DECLS

/**
 * gtk_test_accessible_assert_role:
 * @accessible: a `GtkAccessible`
 * @role: a `GtkAccessibleRole`
 *
 * Checks whether a `GtkAccessible` implementation has the given @role,
 * and raises an assertion if the condition is failed.
 */
#define gtk_test_accessible_assert_role(accessible,role) \
G_STMT_START { \
  GtkAccessible *__a = GTK_ACCESSIBLE (accessible); \
  GtkAccessibleRole __r1 = (role); \
  GtkAccessibleRole __r2 = gtk_accessible_get_accessible_role (__a); \
  if (__r1 == __r2) ; else { \
    gtk_test_accessible_assertion_message_role (G_LOG_DOMAIN, __FILE__, __LINE__, G_STRFUNC, \
                                                #accessible ".accessible-role == " #role, \
                                                __a, __r1, __r2); \
  } \
} G_STMT_END

/**
 * gtk_test_accessible_assert_property:
 * @accessible: a `GtkAccessible`
 * @property: a `GtkAccessibleProperty`
 * @...: the value of @property
 *
 * Checks whether a `GtkAccessible` implementation has its accessible
 * property set to the expected value, and raises an assertion if the
 * condition is not satisfied.
 */
#define gtk_test_accessible_assert_property(accessible,property,...) \
G_STMT_START { \
  GtkAccessible *__a = GTK_ACCESSIBLE (accessible); \
  GtkAccessibleProperty __p = (property); \
  char *value__ = gtk_test_accessible_check_property (__a, __p, __VA_ARGS__); \
  if (value__ == NULL) ; else { \
    char *msg__ = g_strdup_printf ("assertion failed: (" #accessible ".accessible-property(" #property ") == " # __VA_ARGS__ "): value = '%s'", value__); \
    g_assertion_message (G_LOG_DOMAIN, __FILE__, __LINE__, G_STRFUNC, msg__); \
    g_free (msg__); \
  } \
} G_STMT_END

/**
 * gtk_test_accessible_assert_relation:
 * @accessible: a `GtkAccessible`
 * @relation: a `GtkAccessibleRelation`
 * @...: the expected value of @relation
 *
 * Checks whether a `GtkAccessible` implementation has its accessible
 * relation set to the expected value, and raises an assertion if the
 * condition is not satisfied.
 */
#define gtk_test_accessible_assert_relation(accessible,relation,...) \
G_STMT_START { \
  GtkAccessible *__a = GTK_ACCESSIBLE (accessible); \
  GtkAccessibleRelation __r = (relation); \
  char *value__ = gtk_test_accessible_check_relation (__a, __r, __VA_ARGS__); \
  if (value__ == NULL); else { \
    char *msg__ = g_strdup_printf ("assertion failed: (" #accessible ".accessible-relation(" #relation ") == " # __VA_ARGS__ "): value = '%s'", value__); \
    g_assertion_message (G_LOG_DOMAIN, __FILE__, __LINE__, G_STRFUNC, msg__); \
    g_free (msg__); \
  } \
} G_STMT_END

/**
 * gtk_test_accessible_assert_state:
 * @accessible: a `GtkAccessible`
 * @state: a `GtkAccessibleRelation`
 * @...: the expected value of @state
 *
 * Checks whether a `GtkAccessible` implementation has its accessible
 * state set to the expected value, and raises an assertion if the
 * condition is not satisfied.
 */
#define gtk_test_accessible_assert_state(accessible,state,...) \
G_STMT_START { \
  GtkAccessible *__a = GTK_ACCESSIBLE (accessible); \
  GtkAccessibleState __s = (state); \
  char *value__ = gtk_test_accessible_check_state (__a, __s, __VA_ARGS__); \
  if (value__ == NULL); else { \
    char *msg__ = g_strdup_printf ("assertion failed: (" #accessible ".accessible-state(" #state ") == " # __VA_ARGS__ "): value = '%s'", value__); \
    g_assertion_message (G_LOG_DOMAIN, __FILE__, __LINE__, G_STRFUNC, msg__); \
    g_free (msg__); \
  } \
} G_STMT_END

GDK_AVAILABLE_IN_ALL
gboolean        gtk_test_accessible_has_role            (GtkAccessible         *accessible,
                                                         GtkAccessibleRole      role);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_test_accessible_has_property        (GtkAccessible         *accessible,
                                                         GtkAccessibleProperty  property);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_test_accessible_has_relation        (GtkAccessible         *accessible,
                                                         GtkAccessibleRelation  relation);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_test_accessible_has_state           (GtkAccessible         *accessible,
                                                         GtkAccessibleState     state);

GDK_AVAILABLE_IN_ALL
char *          gtk_test_accessible_check_property      (GtkAccessible         *accessible,
                                                         GtkAccessibleProperty  property,
                                                         ...);
GDK_AVAILABLE_IN_ALL
char *          gtk_test_accessible_check_relation      (GtkAccessible         *accessible,
                                                         GtkAccessibleRelation  relation,
                                                         ...);
GDK_AVAILABLE_IN_ALL
char *          gtk_test_accessible_check_state         (GtkAccessible         *accessible,
                                                         GtkAccessibleState     state,
                                                         ...);

GDK_AVAILABLE_IN_ALL
void    gtk_test_accessible_assertion_message_role      (const char        *domain,
                                                         const char        *file,
                                                         int                line,
                                                         const char        *func,
                                                         const char        *expr,
                                                         GtkAccessible     *accessible,
                                                         GtkAccessibleRole  expected_role,
                                                         GtkAccessibleRole  actual_role);

G_END_DECLS
