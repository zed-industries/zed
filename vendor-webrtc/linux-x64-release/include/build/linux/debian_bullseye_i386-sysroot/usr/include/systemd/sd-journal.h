/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosdjournalhfoo
#define foosdjournalhfoo

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

#include <inttypes.h>
#include <stdarg.h>
#include <sys/types.h>
#include <sys/uio.h>
#include <syslog.h>

#include "sd-id128.h"

#include "_sd-common.h"

/* Journal APIs. See sd-journal(3) for more information. */

_SD_BEGIN_DECLARATIONS;

/* Write to daemon */
int sd_journal_print(int priority, const char *format, ...) _sd_printf_(2, 3);
int sd_journal_printv(int priority, const char *format, va_list ap) _sd_printf_(2, 0);
int sd_journal_send(const char *format, ...) _sd_printf_(1, 0) _sd_sentinel_;
int sd_journal_sendv(const struct iovec *iov, int n);
int sd_journal_perror(const char *message);

/* Used by the macros below. You probably don't want to call this directly. */
int sd_journal_print_with_location(int priority, const char *file, const char *line, const char *func, const char *format, ...) _sd_printf_(5, 6);
int sd_journal_printv_with_location(int priority, const char *file, const char *line, const char *func, const char *format, va_list ap) _sd_printf_(5, 0);
int sd_journal_send_with_location(const char *file, const char *line, const char *func, const char *format, ...) _sd_printf_(4, 0) _sd_sentinel_;
int sd_journal_sendv_with_location(const char *file, const char *line, const char *func, const struct iovec *iov, int n);
int sd_journal_perror_with_location(const char *file, const char *line, const char *func, const char *message);

/* implicitly add code location to messages sent, if this is enabled */
#ifndef SD_JOURNAL_SUPPRESS_LOCATION

#define sd_journal_print(priority, ...) sd_journal_print_with_location(priority, "CODE_FILE=" __FILE__, "CODE_LINE=" _SD_STRINGIFY(__LINE__), __func__, __VA_ARGS__)
#define sd_journal_printv(priority, format, ap) sd_journal_printv_with_location(priority, "CODE_FILE=" __FILE__, "CODE_LINE=" _SD_STRINGIFY(__LINE__), __func__, format, ap)
#define sd_journal_send(...) sd_journal_send_with_location("CODE_FILE=" __FILE__, "CODE_LINE=" _SD_STRINGIFY(__LINE__), __func__, __VA_ARGS__)
#define sd_journal_sendv(iovec, n) sd_journal_sendv_with_location("CODE_FILE=" __FILE__, "CODE_LINE=" _SD_STRINGIFY(__LINE__), __func__, iovec, n)
#define sd_journal_perror(message) sd_journal_perror_with_location("CODE_FILE=" __FILE__, "CODE_LINE=" _SD_STRINGIFY(__LINE__), __func__, message)

#endif

int sd_journal_stream_fd(const char *identifier, int priority, int level_prefix);

/* Browse journal stream */

typedef struct sd_journal sd_journal;

/* Open flags */
enum {
        SD_JOURNAL_LOCAL_ONLY                = 1 << 0,
        SD_JOURNAL_RUNTIME_ONLY              = 1 << 1,
        SD_JOURNAL_SYSTEM                    = 1 << 2,
        SD_JOURNAL_CURRENT_USER              = 1 << 3,
        SD_JOURNAL_OS_ROOT                   = 1 << 4,
        SD_JOURNAL_ALL_NAMESPACES            = 1 << 5, /* Show all namespaces, not just the default or specified one */
        SD_JOURNAL_INCLUDE_DEFAULT_NAMESPACE = 1 << 6, /* Show default namespace in addition to specified one */

        SD_JOURNAL_SYSTEM_ONLY _sd_deprecated_ = SD_JOURNAL_SYSTEM /* old name */
};

/* Wakeup event types */
enum {
        SD_JOURNAL_NOP,
        SD_JOURNAL_APPEND,
        SD_JOURNAL_INVALIDATE
};

int sd_journal_open(sd_journal **ret, int flags);
int sd_journal_open_namespace(sd_journal **ret, const char *name_space, int flags);
int sd_journal_open_directory(sd_journal **ret, const char *path, int flags);
int sd_journal_open_directory_fd(sd_journal **ret, int fd, int flags);
int sd_journal_open_files(sd_journal **ret, const char **paths, int flags);
int sd_journal_open_files_fd(sd_journal **ret, int fds[], unsigned n_fds, int flags);
int sd_journal_open_container(sd_journal **ret, const char *machine, int flags) _sd_deprecated_;
void sd_journal_close(sd_journal *j);

int sd_journal_previous(sd_journal *j);
int sd_journal_next(sd_journal *j);

int sd_journal_previous_skip(sd_journal *j, uint64_t skip);
int sd_journal_next_skip(sd_journal *j, uint64_t skip);

int sd_journal_get_realtime_usec(sd_journal *j, uint64_t *ret);
int sd_journal_get_monotonic_usec(sd_journal *j, uint64_t *ret, sd_id128_t *ret_boot_id);

int sd_journal_set_data_threshold(sd_journal *j, size_t sz);
int sd_journal_get_data_threshold(sd_journal *j, size_t *sz);

int sd_journal_get_data(sd_journal *j, const char *field, const void **data, size_t *l);
int sd_journal_enumerate_data(sd_journal *j, const void **data, size_t *l);
int sd_journal_enumerate_available_data(sd_journal *j, const void **data, size_t *l);
void sd_journal_restart_data(sd_journal *j);

int sd_journal_add_match(sd_journal *j, const void *data, size_t size);
int sd_journal_add_disjunction(sd_journal *j);
int sd_journal_add_conjunction(sd_journal *j);
void sd_journal_flush_matches(sd_journal *j);

int sd_journal_seek_head(sd_journal *j);
int sd_journal_seek_tail(sd_journal *j);
int sd_journal_seek_monotonic_usec(sd_journal *j, sd_id128_t boot_id, uint64_t usec);
int sd_journal_seek_realtime_usec(sd_journal *j, uint64_t usec);
int sd_journal_seek_cursor(sd_journal *j, const char *cursor);

int sd_journal_get_cursor(sd_journal *j, char **cursor);
int sd_journal_test_cursor(sd_journal *j, const char *cursor);

int sd_journal_get_cutoff_realtime_usec(sd_journal *j, uint64_t *from, uint64_t *to);
int sd_journal_get_cutoff_monotonic_usec(sd_journal *j, const sd_id128_t boot_id, uint64_t *from, uint64_t *to);

int sd_journal_get_usage(sd_journal *j, uint64_t *bytes);

int sd_journal_query_unique(sd_journal *j, const char *field);
int sd_journal_enumerate_unique(sd_journal *j, const void **data, size_t *l);
int sd_journal_enumerate_available_unique(sd_journal *j, const void **data, size_t *l);
void sd_journal_restart_unique(sd_journal *j);

int sd_journal_enumerate_fields(sd_journal *j, const char **field);
void sd_journal_restart_fields(sd_journal *j);

int sd_journal_get_fd(sd_journal *j);
int sd_journal_get_events(sd_journal *j);
int sd_journal_get_timeout(sd_journal *j, uint64_t *timeout_usec);
int sd_journal_process(sd_journal *j);
int sd_journal_wait(sd_journal *j, uint64_t timeout_usec);
int sd_journal_reliable_fd(sd_journal *j);

int sd_journal_get_catalog(sd_journal *j, char **text);
int sd_journal_get_catalog_for_message_id(sd_id128_t id, char **text);

int sd_journal_has_runtime_files(sd_journal *j);
int sd_journal_has_persistent_files(sd_journal *j);

/* The inverse condition avoids ambiguity of dangling 'else' after the macro */
#define SD_JOURNAL_FOREACH(j)                                           \
        if (sd_journal_seek_head(j) < 0) { }                            \
        else while (sd_journal_next(j) > 0)

/* The inverse condition avoids ambiguity of dangling 'else' after the macro */
#define SD_JOURNAL_FOREACH_BACKWARDS(j)                                 \
        if (sd_journal_seek_tail(j) < 0) { }                            \
        else while (sd_journal_previous(j) > 0)

/* Iterate through all available data fields of the current journal entry */
#define SD_JOURNAL_FOREACH_DATA(j, data, l)                             \
        for (sd_journal_restart_data(j); sd_journal_enumerate_available_data((j), &(data), &(l)) > 0; )

/* Iterate through all available values of a specific field */
#define SD_JOURNAL_FOREACH_UNIQUE(j, data, l)                           \
        for (sd_journal_restart_unique(j); sd_journal_enumerate_available_unique((j), &(data), &(l)) > 0; )

/* Iterate through all known field names */
#define SD_JOURNAL_FOREACH_FIELD(j, field) \
        for (sd_journal_restart_fields(j); sd_journal_enumerate_fields((j), &(field)) > 0; )

_SD_DEFINE_POINTER_CLEANUP_FUNC(sd_journal, sd_journal_close);

_SD_END_DECLARATIONS;

#endif
