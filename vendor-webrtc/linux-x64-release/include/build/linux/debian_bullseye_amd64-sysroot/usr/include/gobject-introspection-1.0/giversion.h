/* Copyright (C) 2018 Christoph Reiter
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GIVERISON_H__
#define __GIVERISON_H__

#if !defined (__GIREPOSITORY_H_INSIDE__) && !defined (GI_COMPILATION)
#error "Only <girepository.h> can be included directly."
#endif

G_BEGIN_DECLS

#define GI_MAJOR_VERSION 1
#define GI_MINOR_VERSION 66
#define GI_MICRO_VERSION 1

#define GI_CHECK_VERSION(major,minor,micro) \
    (GI_MAJOR_VERSION > (major) || \
     (GI_MAJOR_VERSION == (major) && GI_MINOR_VERSION > (minor)) || \
     (GI_MAJOR_VERSION == (major) && GI_MINOR_VERSION == (minor) && \
      GI_MICRO_VERSION >= (micro)))

GI_AVAILABLE_IN_1_60
guint gi_get_major_version (void) G_GNUC_CONST;
GI_AVAILABLE_IN_1_60
guint gi_get_minor_version (void) G_GNUC_CONST;
GI_AVAILABLE_IN_1_60
guint gi_get_micro_version (void) G_GNUC_CONST;

G_END_DECLS

#endif  /* __GIVERISON_H__ */
