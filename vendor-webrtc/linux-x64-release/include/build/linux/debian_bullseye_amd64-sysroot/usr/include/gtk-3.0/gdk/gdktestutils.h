/* Gdk testing utilities
 * Copyright (C) 2007 Imendio AB
 * Authors: Tim Janik
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

#ifndef __GDK_TEST_UTILS_H__
#define __GDK_TEST_UTILS_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkwindow.h>

G_BEGIN_DECLS


/**
 * SECTION:gdktestutils
 * @Short_description: Test utilities
 * @Title: Testing
 *
 * The functions in this section are intended to be used in test programs.
 * They allow to simulate some user input.
 */


/* --- Gdk Test Utility API --- */
GDK_AVAILABLE_IN_ALL
void            gdk_test_render_sync            (GdkWindow      *window);
GDK_AVAILABLE_IN_ALL
gboolean        gdk_test_simulate_key           (GdkWindow      *window,
                                                 gint            x,
                                                 gint            y,
                                                 guint           keyval,
                                                 GdkModifierType modifiers,
                                                 GdkEventType    key_pressrelease);
GDK_AVAILABLE_IN_ALL
gboolean        gdk_test_simulate_button        (GdkWindow      *window,
                                                 gint            x,
                                                 gint            y,
                                                 guint           button, /*1..3*/
                                                 GdkModifierType modifiers,
                                                 GdkEventType    button_pressrelease);

G_END_DECLS

#endif /* __GDK_TEST_UTILS_H__ */
