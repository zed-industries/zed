/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010 Richard Hughes <richard@hughsie.com>
 *
 * Licensed under the GNU Lesser General Public License Version 2.1
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
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301 USA
 */

/**
 * SECTION:cd-version
 * @short_description: Obtains the version for the installed colord
 *
 * These compile time macros allow the user to enable parts of client code
 * depending on the version of libcolord installed.
 *
 * See also: #CdClient, #CdDevice
 */

#if !defined (__COLORD_H_INSIDE__) && !defined (CD_COMPILATION)
#error "Only <colord.h> can be included directly."
#endif

#ifndef __CD_VERSION_H
#define __CD_VERSION_H

/**
 * CD_MAJOR_VERSION:
 *
 * The compile-time major version
 */
#define CD_MAJOR_VERSION				(1)

/**
 * CD_MINOR_VERSION:
 *
 * The compile-time minor version
 */
#define CD_MINOR_VERSION				(4)

/**
 * CD_MICRO_VERSION:
 *
 * The compile-time micro version
 */
#define CD_MICRO_VERSION				(5)

/**
 * CD_CHECK_VERSION:
 *
 * Check whether a colord version equal to or greater than
 * major.minor.micro.
 */
#define CD_CHECK_VERSION(major,minor,micro)    \
    (CD_MAJOR_VERSION > (major) || \
     (CD_MAJOR_VERSION == (major) && CD_MINOR_VERSION > (minor)) || \
     (CD_MAJOR_VERSION == (major) && CD_MINOR_VERSION == (minor) && \
      CD_MICRO_VERSION >= (micro)))

#endif /* __CD_VERSION_H */
