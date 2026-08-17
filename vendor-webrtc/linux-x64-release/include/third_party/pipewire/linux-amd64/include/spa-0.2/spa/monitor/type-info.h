/* Simple Plugin API
 *
 * Copyright © 2021 Collabora Ltd.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_DEVICE_TYPE_INFO_H
#define SPA_DEVICE_TYPE_INFO_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/type-info.h>

#include <spa/monitor/event.h>

/**
 * \addtogroup spa_device
 * \{
 */

#define SPA_TYPE_INFO_DeviceEvent		SPA_TYPE_INFO_EVENT_BASE "Device"
#define SPA_TYPE_INFO_DEVICE_EVENT_BASE		SPA_TYPE_INFO_DeviceEvent ":"

#define SPA_TYPE_INFO_DeviceEventId		SPA_TYPE_INFO_ENUM_BASE "DeviceEventId"
#define SPA_TYPE_INFO_DEVICE_EVENT_ID_BASE	SPA_TYPE_INFO_DeviceEventId ":"

static const struct spa_type_info spa_type_device_event_id[] = {
	{ SPA_DEVICE_EVENT_ObjectConfig, SPA_TYPE_EVENT_Device, SPA_TYPE_INFO_DEVICE_EVENT_ID_BASE "ObjectConfig", NULL },
	{ 0, 0, NULL, NULL },
};

static const struct spa_type_info spa_type_device_event[] = {
	{ SPA_EVENT_DEVICE_START, SPA_TYPE_Id, SPA_TYPE_INFO_DEVICE_EVENT_BASE, spa_type_device_event_id },
	{ SPA_EVENT_DEVICE_Object, SPA_TYPE_Int, SPA_TYPE_INFO_DEVICE_EVENT_BASE "Object", NULL },
	{ SPA_EVENT_DEVICE_Props, SPA_TYPE_OBJECT_Props, SPA_TYPE_INFO_DEVICE_EVENT_BASE "Props", NULL },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_DEVICE_TYPE_INFO_H */
