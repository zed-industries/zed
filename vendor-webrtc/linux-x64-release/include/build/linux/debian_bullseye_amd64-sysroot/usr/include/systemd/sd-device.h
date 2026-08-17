/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosddevicehfoo
#define foosddevicehfoo

/***
  systemd is free software; you can redistribute it and/or modify it
  under the terms of the GNU Lesser General Public License as published by
  the Free Software Foundation; either version 2.1 of the License, or
  (at your option) any later version.

  systemd is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public License
  along with systemd; If not, see <https://www.gnu.org/licenses/>.
***/

#include <errno.h>
#include <inttypes.h>
#include <sys/stat.h>
#include <sys/sysmacros.h>
#include <sys/types.h>

#include "sd-event.h"
#include "sd-id128.h"

#include "_sd-common.h"

_SD_BEGIN_DECLARATIONS;

typedef struct sd_device sd_device;
typedef struct sd_device_enumerator sd_device_enumerator;
typedef struct sd_device_monitor sd_device_monitor;

__extension__ typedef enum sd_device_action_t {
        SD_DEVICE_ADD,
        SD_DEVICE_REMOVE,
        SD_DEVICE_CHANGE,
        SD_DEVICE_MOVE,
        SD_DEVICE_ONLINE,
        SD_DEVICE_OFFLINE,
        SD_DEVICE_BIND,
        SD_DEVICE_UNBIND,
        _SD_DEVICE_ACTION_MAX,
        _SD_DEVICE_ACTION_INVALID = -EINVAL,
        _SD_ENUM_FORCE_S64(DEVICE_ACTION)
} sd_device_action_t;

/* callback */

typedef int (*sd_device_monitor_handler_t)(sd_device_monitor *m, sd_device *device, void *userdata);

/* device */

sd_device *sd_device_ref(sd_device *device);
sd_device *sd_device_unref(sd_device *device);

int sd_device_new_from_syspath(sd_device **ret, const char *syspath);
int sd_device_new_from_devnum(sd_device **ret, char type, dev_t devnum);
int sd_device_new_from_subsystem_sysname(sd_device **ret, const char *subsystem, const char *sysname);
int sd_device_new_from_device_id(sd_device **ret, const char *id);
int sd_device_new_from_stat_rdev(sd_device **ret, const struct stat *st);
int sd_device_new_from_devname(sd_device **ret, const char *devname);
int sd_device_new_from_path(sd_device **ret, const char *path);
int sd_device_new_from_ifname(sd_device **ret, const char *ifname);
int sd_device_new_from_ifindex(sd_device **ret, int ifindex);

int sd_device_new_child(sd_device **ret, sd_device *device, const char *suffix);

int sd_device_get_parent(sd_device *child, sd_device **ret);
int sd_device_get_parent_with_subsystem_devtype(sd_device *child, const char *subsystem, const char *devtype, sd_device **ret);

int sd_device_get_syspath(sd_device *device, const char **ret);
int sd_device_get_subsystem(sd_device *device, const char **ret);
int sd_device_get_devtype(sd_device *device, const char **ret);
int sd_device_get_devnum(sd_device *device, dev_t *devnum);
int sd_device_get_ifindex(sd_device *device, int *ifindex);
int sd_device_get_driver(sd_device *device, const char **ret);
int sd_device_get_devpath(sd_device *device, const char **ret);
int sd_device_get_devname(sd_device *device, const char **ret);
int sd_device_get_sysname(sd_device *device, const char **ret);
int sd_device_get_sysnum(sd_device *device, const char **ret);
int sd_device_get_action(sd_device *device, sd_device_action_t *ret);
int sd_device_get_seqnum(sd_device *device, uint64_t *ret);
int sd_device_get_diskseq(sd_device *device, uint64_t *ret);

int sd_device_get_is_initialized(sd_device *device);
int sd_device_get_usec_initialized(sd_device *device, uint64_t *ret);
int sd_device_get_usec_since_initialized(sd_device *device, uint64_t *ret);

const char *sd_device_get_tag_first(sd_device *device);
const char *sd_device_get_tag_next(sd_device *device);
const char *sd_device_get_current_tag_first(sd_device *device);
const char *sd_device_get_current_tag_next(sd_device *device);
const char *sd_device_get_devlink_first(sd_device *device);
const char *sd_device_get_devlink_next(sd_device *device);
const char *sd_device_get_property_first(sd_device *device, const char **value);
const char *sd_device_get_property_next(sd_device *device, const char **value);
const char *sd_device_get_sysattr_first(sd_device *device);
const char *sd_device_get_sysattr_next(sd_device *device);
sd_device *sd_device_get_child_first(sd_device *device, const char **ret_suffix);
sd_device *sd_device_get_child_next(sd_device *device, const char **ret_suffix);

int sd_device_has_tag(sd_device *device, const char *tag);
int sd_device_has_current_tag(sd_device *device, const char *tag);
int sd_device_get_property_value(sd_device *device, const char *key, const char **value);
int sd_device_get_trigger_uuid(sd_device *device, sd_id128_t *ret);
int sd_device_get_sysattr_value(sd_device *device, const char *sysattr, const char **_value);

int sd_device_set_sysattr_value(sd_device *device, const char *sysattr, const char *value);
int sd_device_set_sysattr_valuef(sd_device *device, const char *sysattr, const char *format, ...) _sd_printf_(3, 4);
int sd_device_trigger(sd_device *device, sd_device_action_t action);
int sd_device_trigger_with_uuid(sd_device *device, sd_device_action_t action, sd_id128_t *ret_uuid);
int sd_device_open(sd_device *device, int flags);

/* device enumerator */

int sd_device_enumerator_new(sd_device_enumerator **ret);
sd_device_enumerator *sd_device_enumerator_ref(sd_device_enumerator *enumerator);
sd_device_enumerator *sd_device_enumerator_unref(sd_device_enumerator *enumerator);

sd_device *sd_device_enumerator_get_device_first(sd_device_enumerator *enumerator);
sd_device *sd_device_enumerator_get_device_next(sd_device_enumerator *enumerator);
sd_device *sd_device_enumerator_get_subsystem_first(sd_device_enumerator *enumerator);
sd_device *sd_device_enumerator_get_subsystem_next(sd_device_enumerator *enumerator);

int sd_device_enumerator_add_match_subsystem(sd_device_enumerator *enumerator, const char *subsystem, int match);
int sd_device_enumerator_add_match_sysattr(sd_device_enumerator *enumerator, const char *sysattr, const char *value, int match);
int sd_device_enumerator_add_match_property(sd_device_enumerator *enumerator, const char *property, const char *value);
int sd_device_enumerator_add_match_sysname(sd_device_enumerator *enumerator, const char *sysname);
int sd_device_enumerator_add_nomatch_sysname(sd_device_enumerator *enumerator, const char *sysname);
int sd_device_enumerator_add_match_tag(sd_device_enumerator *enumerator, const char *tag);
int sd_device_enumerator_add_match_parent(sd_device_enumerator *enumerator, sd_device *parent);
int sd_device_enumerator_allow_uninitialized(sd_device_enumerator *enumerator);

/* device monitor */

int sd_device_monitor_new(sd_device_monitor **ret);
sd_device_monitor *sd_device_monitor_ref(sd_device_monitor *m);
sd_device_monitor *sd_device_monitor_unref(sd_device_monitor *m);

int sd_device_monitor_set_receive_buffer_size(sd_device_monitor *m, size_t size);
int sd_device_monitor_attach_event(sd_device_monitor *m, sd_event *event);
int sd_device_monitor_detach_event(sd_device_monitor *m);
sd_event *sd_device_monitor_get_event(sd_device_monitor *m);
sd_event_source *sd_device_monitor_get_event_source(sd_device_monitor *m);
int sd_device_monitor_set_description(sd_device_monitor *m, const char *description);
int sd_device_monitor_get_description(sd_device_monitor *m, const char **ret);
int sd_device_monitor_start(sd_device_monitor *m, sd_device_monitor_handler_t callback, void *userdata);
int sd_device_monitor_stop(sd_device_monitor *m);

int sd_device_monitor_filter_add_match_subsystem_devtype(sd_device_monitor *m, const char *subsystem, const char *devtype);
int sd_device_monitor_filter_add_match_tag(sd_device_monitor *m, const char *tag);
int sd_device_monitor_filter_add_match_sysattr(sd_device_monitor *m, const char *sysattr, const char *value, int match);
int sd_device_monitor_filter_add_match_parent(sd_device_monitor *m, sd_device *device, int match);
int sd_device_monitor_filter_update(sd_device_monitor *m);
int sd_device_monitor_filter_remove(sd_device_monitor *m);

_SD_DEFINE_POINTER_CLEANUP_FUNC(sd_device, sd_device_unref);
_SD_DEFINE_POINTER_CLEANUP_FUNC(sd_device_enumerator, sd_device_enumerator_unref);
_SD_DEFINE_POINTER_CLEANUP_FUNC(sd_device_monitor, sd_device_monitor_unref);

_SD_END_DECLARATIONS;

#endif
