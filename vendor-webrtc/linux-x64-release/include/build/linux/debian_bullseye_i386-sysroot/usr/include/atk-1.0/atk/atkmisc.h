/* ATK -  Accessibility Toolkit
 * Copyright 2007 Sun Microsystems Inc.
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

#ifndef __ATK_MISC_H__
#define __ATK_MISC_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <glib-object.h>
#include <atk/atkversion.h>

/* We prefix variable declarations so they can
 * properly get exported in Windows DLLs.
 */
#ifndef ATK_VAR
#  ifdef G_PLATFORM_WIN32
#    ifdef ATK_STATIC_COMPILATION
#      define ATK_VAR extern
#    else /* !ATK_STATIC_COMPILATION */
#      ifdef ATK_COMPILATION
#        ifdef DLL_EXPORT
#          define ATK_VAR _ATK_EXTERN
#        else /* !DLL_EXPORT */
#          define ATK_VAR extern
#        endif /* !DLL_EXPORT */
#      else /* !ATK_COMPILATION */
#        define ATK_VAR extern __declspec(dllimport)
#      endif /* !ATK_COMPILATION */
#    endif /* !ATK_STATIC_COMPILATION */
#  else /* !G_PLATFORM_WIN32 */
#    define ATK_VAR _ATK_EXTERN
#  endif /* !G_PLATFORM_WIN32 */
#endif /* ATK_VAR */

G_BEGIN_DECLS

#define ATK_TYPE_MISC                   (atk_misc_get_type ())
#define ATK_IS_MISC(obj)                G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_MISC)
#define ATK_MISC(obj)                   G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_MISC, AtkMisc)
#define ATK_MISC_CLASS(klass)                   (G_TYPE_CHECK_CLASS_CAST ((klass), ATK_TYPE_MISC, AtkMiscClass))
#define ATK_IS_MISC_CLASS(klass)                (G_TYPE_CHECK_CLASS_TYPE ((klass), ATK_TYPE_MISC))
#define ATK_MISC_GET_CLASS(obj)                 (G_TYPE_INSTANCE_GET_CLASS ((obj), ATK_TYPE_MISC, AtkMiscClass))


#ifndef _TYPEDEF_ATK_MISC_
#define _TYPEDEF_ATK_MISC_
typedef struct _AtkMisc      AtkMisc;
typedef struct _AtkMiscClass AtkMiscClass;
#endif

struct _AtkMisc
{
  GObject parent;
};

/*
 * Singleton instance - only the ATK implementation layer for
 * a given GUI toolkit/application instance should touch this
 * symbol directly.
 *
 * Deprecated: Since 2.12.
 */
ATK_VAR AtkMisc *atk_misc_instance;

/**
 * AtkMiscClass:
 * @threads_enter: This virtual function is deprecated since 2.12 and
 *   it should not be overriden.
 * @threads_leave: This virtual function is deprecated sice 2.12 and
 *   it should not be overriden.
 *
 * Usage of AtkMisc is deprecated since 2.12 and heavily discouraged.
 */
struct _AtkMiscClass
{
   GObjectClass parent;
   void   (* threads_enter)                     (AtkMisc *misc);
   void   (* threads_leave)                     (AtkMisc *misc);
   gpointer vfuncs[32]; /* future bincompat */
};

ATK_DEPRECATED_IN_2_12
GType atk_misc_get_type (void);

ATK_DEPRECATED_IN_2_12
void     atk_misc_threads_enter  (AtkMisc *misc);
ATK_DEPRECATED_IN_2_12
void     atk_misc_threads_leave  (AtkMisc *misc);
ATK_DEPRECATED_IN_2_12
const AtkMisc *atk_misc_get_instance (void);

G_END_DECLS

#endif /* __ATK_MISC_H__ */
