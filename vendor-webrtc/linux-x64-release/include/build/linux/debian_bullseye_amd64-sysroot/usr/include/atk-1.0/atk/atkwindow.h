/* ATK -  Accessibility Toolkit
 * Copyright (c) 2011 SUSE LINUX Products GmbH, Nuernberg, Germany.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __ATK_WINDOW_H__
#define __ATK_WINDOW_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>

G_BEGIN_DECLS

/*
 * AtkWindow describes signals pertaining to on-screen windows.
 */


#define ATK_TYPE_WINDOW                    (atk_window_get_type ())
#define ATK_IS_WINDOW(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_WINDOW)
#define ATK_WINDOW(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_WINDOW, AtkWindow)
#define ATK_WINDOW_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_WINDOW, AtkWindowIface))

typedef struct _AtkWindow AtkWindow; /* Dummy typedef */
typedef struct _AtkWindowIface AtkWindowIface;

struct _AtkWindowIface
{
  GTypeInterface parent;
};

ATK_AVAILABLE_IN_2_2
GType atk_window_get_type (void);
G_END_DECLS

#endif /* __ATK_WINDOW_H__ */
