// Copyright 2018 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OSLOGIN_COMPAT_H
#define OSLOGIN_COMPAT_H

#ifdef __FreeBSD__

#include <nsswitch.h>

#define DECLARE_NSS_METHOD_TABLE(name, ...)                     \
    static ns_mtab name[] = {__VA_ARGS__};

#define NSS_METHOD(method, func) {                              \
    .database = NSDB_PASSWD,                                    \
    .name = #method,                                            \
    .method = __nss_compat_ ## method,                          \
    .mdata = (void*)func                                        \
}

#define NSS_REGISTER_METHODS(methods) ns_mtab *                 \
nss_module_register (const char *name, unsigned int *size,      \
                        nss_module_unregister_fn *unregister)   \
{                                                               \
    *size = sizeof (methods) / sizeof (methods[0]);             \
    *unregister = NULL;                                         \
    return (methods);                                           \
}

#define OSLOGIN_PASSWD_CACHE_PATH "/usr/local/etc/oslogin_passwd.cache"
#define OSLOGIN_GROUP_CACHE_PATH "/usr/local/etc/oslogin_group.cache"
#define PASSWD_PATH "/usr/local/etc/passwd"

#define K_DEFAULT_PFILE_PATH "/usr/local/etc/oslogin_passwd.cache"
#define K_DEFAULT_BACKUP_PFILE_PATH "/usr/local/etc/oslogin_passwd.cache.bak"
#define K_DEFAULT_GFILE_PATH "/usr/local/etc/oslogin_group.cache"
#define K_DEFAULT_BACKUP_GFILE_PATH "/usr/local/etc/oslogin_group.cache.bak"

#define PAM_SYSLOG(pamh, ...) syslog(__VA_ARGS__)
#define DEFAULT_SHELL "/bin/sh"
#define DEFAULT_PASSWD "*"

#else /* __FreeBSD__ */

#include <security/pam_ext.h>

#define OSLOGIN_PASSWD_CACHE_PATH "/etc/oslogin_passwd.cache"
#define OSLOGIN_GROUP_CACHE_PATH "/etc/oslogin_group.cache"
#define PASSWD_PATH "/etc/passwd"

#define K_DEFAULT_PFILE_PATH "/etc/oslogin_passwd.cache"
#define K_DEFAULT_BACKUP_PFILE_PATH "/etc/oslogin_passwd.cache.bak"
#define K_DEFAULT_GFILE_PATH "/etc/oslogin_group.cache"
#define K_DEFAULT_BACKUP_GFILE_PATH "/etc/oslogin_group.cache.bak"

#define PAM_SYSLOG pam_syslog
#define DEFAULT_SHELL "/bin/bash"
#define DEFAULT_PASSWD "*"

#define DECLARE_NSS_METHOD_TABLE(name, ...)
#define NSS_METHOD_PROTOTYPE(m)
#define NSS_REGISTER_METHODS(methods)

#endif /* __FreeBSD__ */

#endif /* OSLOGIN_COMPAT_H */
