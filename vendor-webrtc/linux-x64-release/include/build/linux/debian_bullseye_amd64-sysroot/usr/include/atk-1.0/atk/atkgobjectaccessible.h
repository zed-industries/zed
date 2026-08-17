/* ATK - Accessibility Toolkit
 * Copyright 2001 Sun Microsystems Inc.
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

#ifndef __ATK_GOBJECT_ACCESSIBLE_H__
#define __ATK_GOBJECT_ACCESSIBLE_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>

G_BEGIN_DECLS

/*
 * The AtkGObjectAccessible class is provided as a basis for implementing
 * accessibility support for objects which are not GTK+ widgets
 */
#define ATK_TYPE_GOBJECT_ACCESSIBLE            (atk_gobject_accessible_get_type ())
#define ATK_GOBJECT_ACCESSIBLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_GOBJECT_ACCESSIBLE, AtkGObjectAccessible))
#define ATK_GOBJECT_ACCESSIBLE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), ATK_TYPE_GOBJECT_ACCESSIBLE, AtkGObjectAccessibleClass))
#define ATK_IS_GOBJECT_ACCESSIBLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_GOBJECT_ACCESSIBLE))
#define ATK_IS_GOBJECT_ACCESSIBLE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), ATK_TYPE_GOBJECT_ACCESSIBLE))
#define ATK_GOBJECT_ACCESSIBLE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), ATK_TYPE_GOBJECT_ACCESSIBLE, AtkGObjectAccessibleClass))

typedef struct _AtkGObjectAccessible                AtkGObjectAccessible;
typedef struct _AtkGObjectAccessibleClass           AtkGObjectAccessibleClass;

struct _AtkGObjectAccessible
{
  AtkObject parent;
};

ATK_AVAILABLE_IN_ALL
GType atk_gobject_accessible_get_type (void);

struct _AtkGObjectAccessibleClass
{
  AtkObjectClass parent_class;

  AtkFunction pad1;
  AtkFunction pad2;
};

ATK_AVAILABLE_IN_ALL
AtkObject *atk_gobject_accessible_for_object      (GObject           *obj);
ATK_AVAILABLE_IN_ALL
GObject   *atk_gobject_accessible_get_object      (AtkGObjectAccessible *obj);

G_END_DECLS

#endif /* __ATK_GOBJECT_ACCESSIBLE_H__ */
