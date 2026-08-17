/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosdhwdbhfoo
#define foosdhwdbhfoo

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

#include "_sd-common.h"

_SD_BEGIN_DECLARATIONS;

typedef struct sd_hwdb sd_hwdb;

sd_hwdb *sd_hwdb_ref(sd_hwdb *hwdb);
sd_hwdb *sd_hwdb_unref(sd_hwdb *hwdb);

int sd_hwdb_new(sd_hwdb **ret);
int sd_hwdb_new_from_path(const char *path, sd_hwdb **ret);

int sd_hwdb_get(sd_hwdb *hwdb, const char *modalias, const char *key, const char **value);

int sd_hwdb_seek(sd_hwdb *hwdb, const char *modalias);
int sd_hwdb_enumerate(sd_hwdb *hwdb, const char **key, const char **value);

/* the inverse condition avoids ambiguity of dangling 'else' after the macro */
#define SD_HWDB_FOREACH_PROPERTY(hwdb, modalias, key, value)            \
        if (sd_hwdb_seek(hwdb, modalias) < 0) { }                       \
        else while (sd_hwdb_enumerate(hwdb, &(key), &(value)) > 0)

_SD_DEFINE_POINTER_CLEANUP_FUNC(sd_hwdb, sd_hwdb_unref);

_SD_END_DECLARATIONS;

#endif
