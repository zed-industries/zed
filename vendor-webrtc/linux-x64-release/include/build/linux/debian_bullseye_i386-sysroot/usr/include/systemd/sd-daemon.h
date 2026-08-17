/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosddaemonhfoo
#define foosddaemonhfoo

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
#include <sys/types.h>
#include <sys/socket.h>

#include "_sd-common.h"

_SD_BEGIN_DECLARATIONS;

/*
  The following functionality is provided:

  - Support for logging with log levels on stderr
  - File descriptor passing for socket-based activation
  - Daemon startup and status notification
  - Detection of systemd boots

  See sd-daemon(3) for more information.
*/

/*
  Log levels for usage on stderr:

          fprintf(stderr, SD_NOTICE "Hello World!\n");

  This is similar to printk() usage in the kernel.
*/
#define SD_EMERG   "<0>"  /* system is unusable */
#define SD_ALERT   "<1>"  /* action must be taken immediately */
#define SD_CRIT    "<2>"  /* critical conditions */
#define SD_ERR     "<3>"  /* error conditions */
#define SD_WARNING "<4>"  /* warning conditions */
#define SD_NOTICE  "<5>"  /* normal but significant condition */
#define SD_INFO    "<6>"  /* informational */
#define SD_DEBUG   "<7>"  /* debug-level messages */

/* The first passed file descriptor is fd 3 */
#define SD_LISTEN_FDS_START 3

/*
  Returns how many file descriptors have been passed, or a negative
  errno code on failure. Optionally, removes the $LISTEN_FDS and
  $LISTEN_PID file descriptors from the environment (recommended, but
  problematic in threaded environments). If r is the return value of
  this function you'll find the file descriptors passed as fds
  SD_LISTEN_FDS_START to SD_LISTEN_FDS_START+r-1. Returns a negative
  errno style error code on failure. This function call ensures that
  the FD_CLOEXEC flag is set for the passed file descriptors, to make
  sure they are not passed on to child processes. If FD_CLOEXEC shall
  not be set, the caller needs to unset it after this call for all file
  descriptors that are used.

  See sd_listen_fds(3) for more information.
*/
int sd_listen_fds(int unset_environment);

int sd_listen_fds_with_names(int unset_environment, char ***names);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if
  the file descriptor is a FIFO in the file system stored under the
  specified path, 0 otherwise. If path is NULL a path name check will
  not be done and the call only verifies if the file descriptor
  refers to a FIFO. Returns a negative errno style error code on
  failure.

  See sd_is_fifo(3) for more information.
*/
int sd_is_fifo(int fd, const char *path);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if
  the file descriptor is a special character device on the file
  system stored under the specified path, 0 otherwise.
  If path is NULL a path name check will not be done and the call
  only verifies if the file descriptor refers to a special character.
  Returns a negative errno style error code on failure.

  See sd_is_special(3) for more information.
*/
int sd_is_special(int fd, const char *path);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if
  the file descriptor is a socket of the specified family (AF_INET,
  ...) and type (SOCK_DGRAM, SOCK_STREAM, ...), 0 otherwise. If
  family is 0 a socket family check will not be done. If type is 0 a
  socket type check will not be done and the call only verifies if
  the file descriptor refers to a socket. If listening is > 0 it is
  verified that the socket is in listening mode. (i.e. listen() has
  been called) If listening is == 0 it is verified that the socket is
  not in listening mode. If listening is < 0 no listening mode check
  is done. Returns a negative errno style error code on failure.

  See sd_is_socket(3) for more information.
*/
int sd_is_socket(int fd, int family, int type, int listening);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if
  the file descriptor is an Internet socket, of the specified family
  (either AF_INET or AF_INET6) and the specified type (SOCK_DGRAM,
  SOCK_STREAM, ...), 0 otherwise. If version is 0 a protocol version
  check is not done. If type is 0 a socket type check will not be
  done. If port is 0 a socket port check will not be done. The
  listening flag is used the same way as in sd_is_socket(). Returns a
  negative errno style error code on failure.

  See sd_is_socket_inet(3) for more information.
*/
int sd_is_socket_inet(int fd, int family, int type, int listening, uint16_t port);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if the
  file descriptor is an Internet socket of the specified type
  (SOCK_DGRAM, SOCK_STREAM, ...), and if the address of the socket is
  the same as the address specified by addr. The listening flag is used
  the same way as in sd_is_socket(). Returns a negative errno style
  error code on failure.

  See sd_is_socket_sockaddr(3) for more information.
*/
int sd_is_socket_sockaddr(int fd, int type, const struct sockaddr* addr, unsigned addr_len, int listening);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if
  the file descriptor is an AF_UNIX socket of the specified type
  (SOCK_DGRAM, SOCK_STREAM, ...) and path, 0 otherwise. If type is 0
  a socket type check will not be done. If path is NULL a socket path
  check will not be done. For normal AF_UNIX sockets set length to
  0. For abstract namespace sockets set length to the length of the
  socket name (including the initial 0 byte), and pass the full
  socket path in path (including the initial 0 byte). The listening
  flag is used the same way as in sd_is_socket(). Returns a negative
  errno style error code on failure.

  See sd_is_socket_unix(3) for more information.
*/
int sd_is_socket_unix(int fd, int type, int listening, const char *path, size_t length);

/*
  Helper call for identifying a passed file descriptor. Returns 1 if
  the file descriptor is a POSIX Message Queue of the specified name,
  0 otherwise. If path is NULL a message queue name check is not
  done. Returns a negative errno style error code on failure.

  See sd_is_mq(3) for more information.
*/
int sd_is_mq(int fd, const char *path);

/*
  Informs systemd about changed daemon state. This takes a number of
  newline separated environment-style variable assignments in a
  string. The following variables are known:

     MAINPID=...  The main PID of a daemon, in case systemd did not
                  fork off the process itself. Example: "MAINPID=4711"

     READY=1      Tells systemd that daemon startup or daemon reload
                  is finished (only relevant for services of Type=notify).
                  The passed argument is a boolean "1" or "0". Since there
                  is little value in signaling non-readiness the only
                  value daemons should send is "READY=1".

     RELOADING=1  Tell systemd that the daemon began reloading its
                  configuration. When the configuration has been
                  reloaded completely, READY=1 should be sent to inform
                  systemd about this.

     STOPPING=1   Tells systemd that the daemon is about to go down.

     STATUS=...   Passes a single-line status string back to systemd
                  that describes the daemon state. This is free-form
                  and can be used for various purposes: general state
                  feedback, fsck-like programs could pass completion
                  percentages and failing programs could pass a human
                  readable error message. Example: "STATUS=Completed
                  66% of file system check..."

     ERRNO=...    If a daemon fails, the errno-style error code,
                  formatted as string. Example: "ERRNO=2" for ENOENT.

     BUSERROR=... If a daemon fails, the D-Bus error-style error
                  code. Example: "BUSERROR=org.freedesktop.DBus.Error.TimedOut"

     WATCHDOG=1   Tells systemd to update the watchdog timestamp.
                  Services using this feature should do this in
                  regular intervals. A watchdog framework can use the
                  timestamps to detect failed services. Also see
                  sd_watchdog_enabled() below.

     WATCHDOG_USEC=...
                  Reset watchdog_usec value during runtime.
                  To reset watchdog_usec value, start the service again.
                  Example: "WATCHDOG_USEC=20000000"

     FDSTORE=1    Store the file descriptors passed along with the
                  message in the per-service file descriptor store,
                  and pass them to the main process again on next
                  invocation. This variable is only supported with
                  sd_pid_notify_with_fds().

     FDSTOREREMOVE=1
                  Remove one or more file descriptors from the file
                  descriptor store, identified by the name specified
                  in FDNAME=, see below.

     FDNAME=      A name to assign to new file descriptors stored in the
                  file descriptor store, or the name of the file descriptors
                  to remove in case of FDSTOREREMOVE=1.

  Daemons can choose to send additional variables. However, it is
  recommended to prefix variable names not listed above with X_.

  Returns a negative errno-style error code on failure. Returns > 0
  if systemd could be notified, 0 if it couldn't possibly because
  systemd is not running.

  Example: When a daemon finished starting up, it could issue this
  call to notify systemd about it:

     sd_notify(0, "READY=1");

  See sd_notifyf() for more complete examples.

  See sd_notify(3) for more information.
*/
int sd_notify(int unset_environment, const char *state);

/*
  Similar to sd_notify() but takes a format string.

  Example 1: A daemon could send the following after initialization:

     sd_notifyf(0, "READY=1\n"
                   "STATUS=Processing requests...\n"
                   "MAINPID=%lu",
                   (unsigned long) getpid());

  Example 2: A daemon could send the following shortly before
  exiting, on failure:

     sd_notifyf(0, "STATUS=Failed to start up: %s\n"
                   "ERRNO=%i",
                   strerror_r(errnum, (char[1024]){}, 1024),
                   errnum);

  See sd_notifyf(3) for more information.
*/
int sd_notifyf(int unset_environment, const char *format, ...) _sd_printf_(2,3);

/*
  Similar to sd_notify(), but send the message on behalf of another
  process, if the appropriate permissions are available.
*/
int sd_pid_notify(pid_t pid, int unset_environment, const char *state);

/*
  Similar to sd_notifyf(), but send the message on behalf of another
  process, if the appropriate permissions are available.
*/
int sd_pid_notifyf(pid_t pid, int unset_environment, const char *format, ...) _sd_printf_(3,4);

/*
  Similar to sd_pid_notify(), but also passes the specified fd array
  to the service manager for storage. This is particularly useful for
  FDSTORE=1 messages.
*/
int sd_pid_notify_with_fds(pid_t pid, int unset_environment, const char *state, const int *fds, unsigned n_fds);

/*
  Returns > 0 if synchronization with systemd succeeded.  Returns < 0
  on error. Returns 0 if $NOTIFY_SOCKET was not set. Note that the
  timeout parameter of this function call takes the timeout in µs, and
  will be passed to ppoll(2), hence the behaviour will be similar to
  ppoll(2). This function can be called after sending a status message
  to systemd, if one needs to synchronize against reception of the
  status messages sent before this call is made. Therefore, this
  cannot be used to know if the status message was processed
  successfully, but to only synchronize against its consumption.
*/
int sd_notify_barrier(int unset_environment, uint64_t timeout);

/*
  Returns > 0 if the system was booted with systemd. Returns < 0 on
  error. Returns 0 if the system was not booted with systemd. Note
  that all of the functions above handle non-systemd boots just
  fine. You should NOT protect them with a call to this function. Also
  note that this function checks whether the system, not the user
  session is controlled by systemd. However the functions above work
  for both user and system services.

  See sd_booted(3) for more information.
*/
int sd_booted(void);

/*
  Returns > 0 if the service manager expects watchdog keep-alive
  events to be sent regularly via sd_notify(0, "WATCHDOG=1"). Returns
  0 if it does not expect this. If the usec argument is non-NULL
  returns the watchdog timeout in µs after which the service manager
  will act on a process that has not sent a watchdog keep alive
  message. This function is useful to implement services that
  recognize automatically if they are being run under supervision of
  systemd with WatchdogSec= set. It is recommended for clients to
  generate keep-alive pings via sd_notify(0, "WATCHDOG=1") every half
  of the returned time.

  See sd_watchdog_enabled(3) for more information.
*/
int sd_watchdog_enabled(int unset_environment, uint64_t *usec);

_SD_END_DECLARATIONS;

#endif
