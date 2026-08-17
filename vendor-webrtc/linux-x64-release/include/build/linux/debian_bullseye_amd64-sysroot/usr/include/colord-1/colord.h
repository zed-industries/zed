/* -*- Mode: C; tab-width: 8; indent-tabs-mode: t; c-basic-offset: 8 -*-
 *
 * Copyright (C) 2010-2013 Richard Hughes <richard@hughsie.com>
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
 * SECTION:colord
 * @short_description: Client objects for accessing colord
 *
 * These objects allow client programs to get access to devices that can
 * be color managed and profiles for changing them.
 *
 * See also: #CdClient, #CdDevice
 */

#ifndef __COLORD_H__
#define __COLORD_H__

#define __COLORD_H_INSIDE__

#include <colord/cd-buffer.h>
#include <colord/cd-client.h>
#include <colord/cd-client-sync.h>
#include <colord/cd-color.h>
#include <colord/cd-device.h>
#include <colord/cd-device-sync.h>
#include <colord/cd-dom.h>
#include <colord/cd-edid.h>
#include <colord/cd-enum.h>
#include <colord/cd-icc.h>
#include <colord/cd-icc-store.h>
#include <colord/cd-icc-utils.h>
#include <colord/cd-interp-akima.h>
#include <colord/cd-interp-linear.h>
#include <colord/cd-interp.h>
#include <colord/cd-it8.h>
#include <colord/cd-it8-utils.h>
#include <colord/cd-math.h>
#include <colord/cd-profile.h>
#include <colord/cd-profile-sync.h>
#include <colord/cd-quirk.h>
#include <colord/cd-sensor.h>
#include <colord/cd-sensor-sync.h>
#include <colord/cd-spectrum.h>
#include <colord/cd-transform.h>
#include <colord/cd-version.h>

#undef __COLORD_H_INSIDE__

#endif /* __COLORD_H__ */

