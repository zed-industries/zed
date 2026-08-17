/* ATK -  Accessibility Toolkit
 * Copyright 2009 Novell, Inc.
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

#ifndef __ATK_SOCKET_H__
#define __ATK_SOCKET_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkversion.h>

G_BEGIN_DECLS

#define ATK_TYPE_SOCKET               (atk_socket_get_type ())
#define ATK_SOCKET(obj)               (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_SOCKET, AtkSocket))
#define ATK_IS_SOCKET(obj)            (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_SOCKET))
#define ATK_SOCKET_CLASS(klass)       (G_TYPE_CHECK_CLASS_CAST ((klass), ATK_TYPE_SOCKET, AtkSocketClass))
#define ATK_IS_SOCKET_CLASS(klass)    (G_TYPE_CHECK_CLASS_TYPE ((klass), ATK_TYPE_SOCKET))
#define ATK_SOCKET_GET_CLASS(obj)     (G_TYPE_INSTANCE_GET_CLASS ((obj), ATK_TYPE_SOCKET, AtkSocketClass))

typedef struct _AtkSocket         AtkSocket;
typedef struct _AtkSocketClass    AtkSocketClass;

struct _AtkSocket
{
  AtkObject parent;

  /*< private >*/
  gchar* embedded_plug_id;
};

ATK_AVAILABLE_IN_ALL
GType atk_socket_get_type (void);

struct _AtkSocketClass
{
  AtkObjectClass parent_class;
  
  /* to be subscribed to by atk-bridge */

  /*< protected >*/
  void (* embed) (AtkSocket *obj, const gchar* plug_id);
};

ATK_AVAILABLE_IN_ALL
AtkObject*    atk_socket_new           (void);
ATK_AVAILABLE_IN_ALL
void          atk_socket_embed         (AtkSocket* obj, const gchar* plug_id);
ATK_AVAILABLE_IN_ALL
gboolean      atk_socket_is_occupied   (AtkSocket* obj);

G_END_DECLS

#endif /* __ATK_SOCKET_H__ */
