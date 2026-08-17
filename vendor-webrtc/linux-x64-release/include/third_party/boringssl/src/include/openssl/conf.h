// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CONF_H
#define OPENSSL_HEADER_CONF_H

#include <openssl/base.h>   // IWYU pragma: export

#include <openssl/stack.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Config files.
//
// This library handles OpenSSL's config files, which look like:
//
//   # Comment
//
//   # This key is in the default section.
//   key=value
//
//   [section_name]
//   key2=value2
//
// Config files are represented by a |CONF|. Use of this module is strongly
// discouraged. It is a remnant of the OpenSSL command-line tool. Parsing an
// untrusted input as a config file risks string injection and denial of service
// vulnerabilities.


struct conf_value_st {
  char *section;
  char *name;
  char *value;
};

DEFINE_STACK_OF(CONF_VALUE)


// NCONF_new returns a fresh, empty |CONF|, or NULL on error. The |method|
// argument must be NULL.
OPENSSL_EXPORT CONF *NCONF_new(void *method);

// NCONF_free frees all the data owned by |conf| and then |conf| itself.
OPENSSL_EXPORT void NCONF_free(CONF *conf);

// NCONF_load parses the file named |filename| and adds the values found to
// |conf|. It returns one on success and zero on error. In the event of an
// error, if |out_error_line| is not NULL, |*out_error_line| is set to the
// number of the line that contained the error.
OPENSSL_EXPORT int NCONF_load(CONF *conf, const char *filename,
                              long *out_error_line);

// NCONF_load_bio acts like |NCONF_load| but reads from |bio| rather than from
// a named file.
OPENSSL_EXPORT int NCONF_load_bio(CONF *conf, BIO *bio, long *out_error_line);

// NCONF_get_section returns a stack of values for a given section in |conf|.
// If |section| is NULL, the default section is returned. It returns NULL on
// error.
OPENSSL_EXPORT const STACK_OF(CONF_VALUE) *NCONF_get_section(
    const CONF *conf, const char *section);

// NCONF_get_string returns the value of the key |name|, in section |section|.
// The |section| argument may be NULL to indicate the default section. It
// returns the value or NULL on error.
OPENSSL_EXPORT const char *NCONF_get_string(const CONF *conf,
                                            const char *section,
                                            const char *name);


// Deprecated functions

// These defines do nothing but are provided to make old code easier to
// compile.
#define CONF_MFLAGS_DEFAULT_SECTION 0
#define CONF_MFLAGS_IGNORE_MISSING_FILE 0

// CONF_modules_load_file returns one. BoringSSL is defined to have no config
// file options, thus loading from |filename| always succeeds by doing nothing.
OPENSSL_EXPORT int CONF_modules_load_file(const char *filename,
                                          const char *appname,
                                          unsigned long flags);

// CONF_modules_free does nothing.
OPENSSL_EXPORT void CONF_modules_free(void);

// OPENSSL_config does nothing.
OPENSSL_EXPORT void OPENSSL_config(const char *config_name);

// OPENSSL_no_config does nothing.
OPENSSL_EXPORT void OPENSSL_no_config(void);


#if defined(__cplusplus)
}  // extern C

extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(CONF, NCONF_free)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#define CONF_R_LIST_CANNOT_BE_NULL 100
#define CONF_R_MISSING_CLOSE_SQUARE_BRACKET 101
#define CONF_R_MISSING_EQUAL_SIGN 102
#define CONF_R_NO_CLOSE_BRACE 103
#define CONF_R_UNABLE_TO_CREATE_NEW_SECTION 104
#define CONF_R_VARIABLE_HAS_NO_VALUE 105
#define CONF_R_VARIABLE_EXPANSION_TOO_LONG 106
#define CONF_R_VARIABLE_EXPANSION_NOT_SUPPORTED 107

#endif  // OPENSSL_HEADER_THREAD_H
