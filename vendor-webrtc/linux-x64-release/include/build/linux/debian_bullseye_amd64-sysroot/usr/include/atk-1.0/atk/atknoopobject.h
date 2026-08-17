/* ATK -  Accessibility Toolkit
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

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkversion.h>

#ifndef __ATK_NO_OP_OBJECT_H__
#define __ATK_NO_OP_OBJECT_H__

G_BEGIN_DECLS

#define ATK_TYPE_NO_OP_OBJECT                (atk_no_op_object_get_type ())
#define ATK_NO_OP_OBJECT(obj)                (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_NO_OP_OBJECT, AtkNoOpObject))
#define ATK_NO_OP_OBJECT_CLASS(klass)        (G_TYPE_CHECK_CLASS_CAST ((klass), ATK_TYPE_NO_OP_OBJECT, AtkNoOpObjectClass))
#define ATK_IS_NO_OP_OBJECT(obj)             (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_NO_OP_OBJECT))
#define ATK_IS_NO_OP_OBJECT_CLASS(klass)     (G_TYPE_CHECK_CLASS_TYPE ((klass), ATK_TYPE_NO_OP_OBJECT))
#define ATK_NO_OP_OBJECT_GET_CLASS(obj)      (G_TYPE_INSTANCE_GET_CLASS ((obj), ATK_TYPE_NO_OP_OBJECT, AtkNoOpObjectClass))

typedef struct _AtkNoOpObject                   AtkNoOpObject;
typedef struct _AtkNoOpObjectClass              AtkNoOpObjectClass;

struct _AtkNoOpObject
{
  AtkObject     parent;
};

ATK_AVAILABLE_IN_ALL
GType atk_no_op_object_get_type (void);

struct _AtkNoOpObjectClass
{
  AtkObjectClass parent_class;
};

ATK_AVAILABLE_IN_ALL
AtkObject *atk_no_op_object_new (GObject  *obj);

G_END_DECLS

#endif /* __ATK_NO_OP_OBJECT_H__ */
