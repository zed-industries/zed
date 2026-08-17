#ifndef __CURL_TYPECHECK_GCC_H
#define __CURL_TYPECHECK_GCC_H
/***************************************************************************
 *                                  _   _ ____  _
 *  Project                     ___| | | |  _ \| |
 *                             / __| | | | |_) | |
 *                            | (__| |_| |  _ <| |___
 *                             \___|\___/|_| \_\_____|
 *
 * Copyright (C) 1998 - 2009, Daniel Stenberg, <daniel@haxx.se>, et al.
 *
 * This software is licensed as described in the file COPYING, which
 * you should have received as part of this distribution. The terms
 * are also available at http://curl.haxx.se/docs/copyright.html.
 *
 * You may opt to use, copy, modify, merge, publish, distribute and/or sell
 * copies of the Software, and permit persons to whom the Software is
 * furnished to do so, under the terms of the COPYING file.
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY OF ANY
 * KIND, either express or implied.
 *
 * $Id: typecheck-gcc.h,v 1.9 2009-01-25 23:26:31 bagder Exp $
 ***************************************************************************/

/* wraps curl_easy_setopt() with typechecking */

/* To add a new kind of warning, add an
 *   if(_curl_is_sometype_option(_curl_opt) && ! _curl_is_sometype(value))
 *     _curl_easy_setopt_err_sometype();
 * block and define _curl_is_sometype_option, _curl_is_sometype and
 * _curl_easy_setopt_err_sometype below
 *
 * To add an option that uses the same type as an existing option, you'll just
 * need to extend the appropriate _curl_*_option macro
 */
#define curl_easy_setopt(handle, option, value)                               \
__extension__ ({                                                              \
  __typeof__ (option) _curl_opt = option;                                     \
  if (__builtin_constant_p(_curl_opt)) {                                      \
    if (_curl_is_long_option(_curl_opt) && !_curl_is_long(value))             \
      _curl_easy_setopt_err_long();                                           \
    if (_curl_is_off_t_option(_curl_opt) && !_curl_is_off_t(value))           \
      _curl_easy_setopt_err_curl_off_t();                                     \
    if (_curl_is_string_option(_curl_opt) && !_curl_is_string(value))         \
      _curl_easy_setopt_err_string();                                         \
    if (_curl_is_write_cb_option(_curl_opt) && !_curl_is_write_cb(value))     \
      _curl_easy_setopt_err_write_callback();                                 \
    if ((_curl_opt) == CURLOPT_READFUNCTION && !_curl_is_read_cb(value))      \
      _curl_easy_setopt_err_read_cb();                                        \
    if ((_curl_opt) == CURLOPT_IOCTLFUNCTION && !_curl_is_ioctl_cb(value))    \
      _curl_easy_setopt_err_ioctl_cb();                                       \
    if ((_curl_opt) == CURLOPT_SOCKOPTFUNCTION && !_curl_is_sockopt_cb(value))\
      _curl_easy_setopt_err_sockopt_cb();                                     \
    if ((_curl_opt) == CURLOPT_OPENSOCKETFUNCTION &&                          \
            !_curl_is_opensocket_cb(value))                                   \
      _curl_easy_setopt_err_opensocket_cb();                                  \
    if ((_curl_opt) == CURLOPT_PROGRESSFUNCTION &&                            \
            !_curl_is_progress_cb(value))                                     \
      _curl_easy_setopt_err_progress_cb();                                    \
    if ((_curl_opt) == CURLOPT_DEBUGFUNCTION && !_curl_is_debug_cb(value))    \
      _curl_easy_setopt_err_debug_cb();                                       \
    if ((_curl_opt) == CURLOPT_SSL_CTX_FUNCTION &&                            \
            !_curl_is_ssl_ctx_cb(value))                                      \
      _curl_easy_setopt_err_ssl_ctx_cb();                                     \
    if (_curl_is_conv_cb_option(_curl_opt) && !_curl_is_conv_cb(value))       \
      _curl_easy_setopt_err_conv_cb();                                        \
    if ((_curl_opt) == CURLOPT_SEEKFUNCTION && !_curl_is_seek_cb(value))      \
      _curl_easy_setopt_err_seek_cb();                                        \
    if (_curl_is_cb_data_option(_curl_opt) && !_curl_is_cb_data(value))       \
      _curl_easy_setopt_err_cb_data();                                        \
    if ((_curl_opt) == CURLOPT_ERRORBUFFER && !_curl_is_error_buffer(value))  \
      _curl_easy_setopt_err_error_buffer();                                   \
    if ((_curl_opt) == CURLOPT_STDERR && !_curl_is_FILE(value))               \
      _curl_easy_setopt_err_FILE();                                           \
    if (_curl_is_postfields_option(_curl_opt) && !_curl_is_postfields(value)) \
      _curl_easy_setopt_err_postfields();                                     \
    if ((_curl_opt) == CURLOPT_HTTPPOST &&                                    \
            !_curl_is_arr((value), struct curl_httppost))                     \
      _curl_easy_setopt_err_curl_httpost();                                   \
    if (_curl_is_slist_option(_curl_opt) &&                                   \
            !_curl_is_arr((value), struct curl_slist))                        \
      _curl_easy_setopt_err_curl_slist();                                     \
    if ((_curl_opt) == CURLOPT_SHARE && !_curl_is_ptr((value), CURLSH))       \
      _curl_easy_setopt_err_CURLSH();                                         \
  }                                                                           \
  curl_easy_setopt(handle, _curl_opt, value);                                 \
})

/* wraps curl_easy_getinfo() with typechecking */
/* FIXME: don't allow const pointers */
#define curl_easy_getinfo(handle, info, arg)                                  \
__extension__ ({                                                              \
  __typeof__ (info) _curl_info = info;                                        \
  if (__builtin_constant_p(_curl_info)) {                                     \
    if (_curl_is_string_info(_curl_info) && !_curl_is_arr((arg), char *))     \
      _curl_easy_getinfo_err_string();                                        \
    if (_curl_is_long_info(_curl_info) && !_curl_is_arr((arg), long))         \
      _curl_easy_getinfo_err_long();                                          \
    if (_curl_is_double_info(_curl_info) && !_curl_is_arr((arg), double))     \
      _curl_easy_getinfo_err_double();                                        \
    if (_curl_is_slist_info(_curl_info) &&                                    \
           !_curl_is_arr((arg), struct curl_slist *))                         \
      _curl_easy_getinfo_err_curl_slist();                                    \
  }                                                                           \
  curl_easy_getinfo(handle, _curl_info, arg);                                 \
})

/* TODO: typechecking for curl_share_setopt() and curl_multi_setopt(),
 * for now just make sure that the functions are called with three
 * arguments
 */
#define curl_share_setopt(share,opt,param) curl_share_setopt(share,opt,param)
#define curl_multi_setopt(handle,opt,param) curl_multi_setopt(handle,opt,param)


/* the actual warnings, triggered by calling the _curl_easy_setopt_err*
 * functions */

/* To define a new warning, use _CURL_WARNING(identifier, "message") */
#define _CURL_WARNING(id, message)                                            \
  static void __attribute__((warning(message))) __attribute__((unused))       \
  __attribute__((noinline)) id(void) { __asm__(""); }

_CURL_WARNING(_curl_easy_setopt_err_long,
  "curl_easy_setopt expects a long argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_curl_off_t,
  "curl_easy_setopt expects a curl_off_t argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_string,
  "curl_easy_setopt expects a string (char* or char[]) argument for this option"
  )
_CURL_WARNING(_curl_easy_setopt_err_write_callback,
  "curl_easy_setopt expects a curl_write_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_read_cb,
  "curl_easy_setopt expects a curl_read_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_ioctl_cb,
  "curl_easy_setopt expects a curl_ioctl_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_sockopt_cb,
  "curl_easy_setopt expects a curl_sockopt_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_opensocket_cb,
  "curl_easy_setopt expects a curl_opensocket_callback argument for this option"
  )
_CURL_WARNING(_curl_easy_setopt_err_progress_cb,
  "curl_easy_setopt expects a curl_progress_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_debug_cb,
  "curl_easy_setopt expects a curl_debug_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_ssl_ctx_cb,
  "curl_easy_setopt expects a curl_ssl_ctx_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_conv_cb,
  "curl_easy_setopt expects a curl_conv_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_seek_cb,
  "curl_easy_setopt expects a curl_seek_callback argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_cb_data,
  "curl_easy_setopt expects a private data pointer as argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_error_buffer,
  "curl_easy_setopt expects a char buffer of CURL_ERROR_SIZE as argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_FILE,
  "curl_easy_setopt expects a FILE* argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_postfields,
  "curl_easy_setopt expects a void* or char* argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_curl_httpost,
  "curl_easy_setopt expects a struct curl_httppost* argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_curl_slist,
  "curl_easy_setopt expects a struct curl_slist* argument for this option")
_CURL_WARNING(_curl_easy_setopt_err_CURLSH,
  "curl_easy_setopt expects a CURLSH* argument for this option")

_CURL_WARNING(_curl_easy_getinfo_err_string,
  "curl_easy_getinfo expects a pointer to char * for this info")
_CURL_WARNING(_curl_easy_getinfo_err_long,
  "curl_easy_getinfo expects a pointer to long for this info")
_CURL_WARNING(_curl_easy_getinfo_err_double,
  "curl_easy_getinfo expects a pointer to double for this info")
_CURL_WARNING(_curl_easy_getinfo_err_curl_slist,
  "curl_easy_getinfo expects a pointer to struct curl_slist * for this info")

/* groups of curl_easy_setops options that take the same type of argument */

/* To add a new option to one of the groups, just add
 *   (option) == CURLOPT_SOMETHING
 * to the or-expression. If the option takes a long or curl_off_t, you don't
 * have to do anything
 */

/* evaluates to true if option takes a long argument */
#define _curl_is_long_option(option)                                          \
  (0 < (option) && (option) < CURLOPTTYPE_OBJECTPOINT)

#define _curl_is_off_t_option(option)                                         \
  ((option) > CURLOPTTYPE_OFF_T)

/* evaluates to true if option takes a char* argument */
#define _curl_is_string_option(option)                                        \
  ((option) == CURLOPT_URL ||                                                 \
   (option) == CURLOPT_PROXY ||                                               \
   (option) == CURLOPT_INTERFACE ||                                           \
   (option) == CURLOPT_NETRC_FILE ||                                          \
   (option) == CURLOPT_USERPWD ||                                             \
   (option) == CURLOPT_USERNAME ||                                            \
   (option) == CURLOPT_PASSWORD ||                                            \
   (option) == CURLOPT_PROXYUSERPWD ||                                        \
   (option) == CURLOPT_PROXYUSERNAME ||                                       \
   (option) == CURLOPT_PROXYPASSWORD ||                                       \
   (option) == CURLOPT_NOPROXY ||                                             \
   (option) == CURLOPT_ENCODING ||                                            \
   (option) == CURLOPT_REFERER ||                                             \
   (option) == CURLOPT_USERAGENT ||                                           \
   (option) == CURLOPT_COOKIE ||                                              \
   (option) == CURLOPT_COOKIEFILE ||                                          \
   (option) == CURLOPT_COOKIEJAR ||                                           \
   (option) == CURLOPT_COOKIELIST ||                                          \
   (option) == CURLOPT_FTPPORT ||                                             \
   (option) == CURLOPT_FTP_ALTERNATIVE_TO_USER ||                             \
   (option) == CURLOPT_FTP_ACCOUNT ||                                         \
   (option) == CURLOPT_RANGE ||                                               \
   (option) == CURLOPT_CUSTOMREQUEST ||                                       \
   (option) == CURLOPT_SSLCERT ||                                             \
   (option) == CURLOPT_SSLCERTTYPE ||                                         \
   (option) == CURLOPT_SSLKEY ||                                              \
   (option) == CURLOPT_SSLKEYTYPE ||                                          \
   (option) == CURLOPT_KEYPASSWD ||                                           \
   (option) == CURLOPT_SSLENGINE ||                                           \
   (option) == CURLOPT_CAINFO ||                                              \
   (option) == CURLOPT_CAPATH ||                                              \
   (option) == CURLOPT_RANDOM_FILE ||                                         \
   (option) == CURLOPT_EGDSOCKET ||                                           \
   (option) == CURLOPT_SSL_CIPHER_LIST ||                                     \
   (option) == CURLOPT_KRBLEVEL ||                                            \
   (option) == CURLOPT_SSH_HOST_PUBLIC_KEY_MD5 ||                             \
   (option) == CURLOPT_SSH_PUBLIC_KEYFILE ||                                  \
   (option) == CURLOPT_SSH_PRIVATE_KEYFILE ||                                 \
   (option) == CURLOPT_CRLFILE ||                                             \
   (option) == CURLOPT_ISSUERCERT ||                                          \
   0)

/* evaluates to true if option takes a curl_write_callback argument */
#define _curl_is_write_cb_option(option)                                      \
  ((option) == CURLOPT_HEADERFUNCTION ||                                      \
   (option) == CURLOPT_WRITEFUNCTION)

/* evaluates to true if option takes a curl_conv_callback argument */
#define _curl_is_conv_cb_option(option)                                       \
  ((option) == CURLOPT_CONV_TO_NETWORK_FUNCTION ||                            \
   (option) == CURLOPT_CONV_FROM_NETWORK_FUNCTION ||                          \
   (option) == CURLOPT_CONV_FROM_UTF8_FUNCTION)

/* evaluates to true if option takes a data argument to pass to a callback */
#define _curl_is_cb_data_option(option)                                       \
  ((option) == CURLOPT_WRITEDATA ||                                           \
   (option) == CURLOPT_READDATA ||                                            \
   (option) == CURLOPT_IOCTLDATA ||                                           \
   (option) == CURLOPT_SOCKOPTDATA ||                                         \
   (option) == CURLOPT_OPENSOCKETDATA ||                                      \
   (option) == CURLOPT_PROGRESSDATA ||                                        \
   (option) == CURLOPT_WRITEHEADER ||                                         \
   (option) == CURLOPT_DEBUGDATA ||                                           \
   (option) == CURLOPT_SSL_CTX_DATA ||                                        \
   (option) == CURLOPT_SEEKDATA ||                                            \
   (option) == CURLOPT_PRIVATE ||                                             \
   0)

/* evaluates to true if option takes a POST data argument (void* or char*) */
#define _curl_is_postfields_option(option)                                    \
  ((option) == CURLOPT_POSTFIELDS ||                                          \
   (option) == CURLOPT_COPYPOSTFIELDS ||                                      \
   0)

/* evaluates to true if option takes a struct curl_slist * argument */
#define _curl_is_slist_option(option)                                         \
  ((option) == CURLOPT_HTTPHEADER ||                                          \
   (option) == CURLOPT_HTTP200ALIASES ||                                      \
   (option) == CURLOPT_QUOTE ||                                               \
   (option) == CURLOPT_POSTQUOTE ||                                           \
   (option) == CURLOPT_PREQUOTE ||                                            \
   (option) == CURLOPT_TELNETOPTIONS ||                                       \
   0)

/* groups of curl_easy_getinfo infos that take the same type of argument */

/* evaluates to true if info expects a pointer to char * argument */
#define _curl_is_string_info(info)                                            \
  (CURLINFO_STRING < (info) && (info) < CURLINFO_LONG)

/* evaluates to true if info expects a pointer to long argument */
#define _curl_is_long_info(info)                                              \
  (CURLINFO_LONG < (info) && (info) < CURLINFO_DOUBLE)

/* evaluates to true if info expects a pointer to double argument */
#define _curl_is_double_info(info)                                            \
  (CURLINFO_DOUBLE < (info) && (info) < CURLINFO_SLIST)

/* true if info expects a pointer to struct curl_slist * argument */
#define _curl_is_slist_info(info)                                             \
  (CURLINFO_SLIST < (info))


/* typecheck helpers -- check whether given expression has requested type*/

/* For pointers, you can use the _curl_is_ptr/_curl_is_arr macros,
 * otherwise define a new macro. Search for __builtin_types_compatible_p
 * in the GCC manual.
 * NOTE: these macros MUST NOT EVALUATE their arguments! The argument is
 * the actual expression passed to the curl_easy_setopt macro. This
 * means that you can only apply the sizeof and __typeof__ operators, no
 * == or whatsoever.
 */

/* XXX: should evaluate to true iff expr is a pointer */
#define _curl_is_any_ptr(expr)                                                \
  (sizeof(expr) == sizeof(void*))

/* evaluates to true if expr is NULL */
/* XXX: must not evaluate expr, so this check is not accurate */
#define _curl_is_NULL(expr)                                                   \
  (__builtin_types_compatible_p(__typeof__(expr), __typeof__(NULL)))

/* evaluates to true if expr is type*, const type* or NULL */
#define _curl_is_ptr(expr, type)                                              \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), type *) ||                  \
   __builtin_types_compatible_p(__typeof__(expr), const type *))

/* evaluates to true if expr is one of type[], type*, NULL or const type* */
#define _curl_is_arr(expr, type)                                              \
  (_curl_is_ptr((expr), type) ||                                              \
   __builtin_types_compatible_p(__typeof__(expr), type []))

/* evaluates to true if expr is a string */
#define _curl_is_string(expr)                                                 \
  (_curl_is_arr((expr), char) ||                                              \
   _curl_is_arr((expr), signed char) ||                                       \
   _curl_is_arr((expr), unsigned char))

/* evaluates to true if expr is a long (no matter the signedness)
 * XXX: for now, int is also accepted (and therefore short and char, which
 * are promoted to int when passed to a variadic function) */
#define _curl_is_long(expr)                                                   \
  (__builtin_types_compatible_p(__typeof__(expr), long) ||                    \
   __builtin_types_compatible_p(__typeof__(expr), signed long) ||             \
   __builtin_types_compatible_p(__typeof__(expr), unsigned long) ||           \
   __builtin_types_compatible_p(__typeof__(expr), int) ||                     \
   __builtin_types_compatible_p(__typeof__(expr), signed int) ||              \
   __builtin_types_compatible_p(__typeof__(expr), unsigned int) ||            \
   __builtin_types_compatible_p(__typeof__(expr), short) ||                   \
   __builtin_types_compatible_p(__typeof__(expr), signed short) ||            \
   __builtin_types_compatible_p(__typeof__(expr), unsigned short) ||          \
   __builtin_types_compatible_p(__typeof__(expr), char) ||                    \
   __builtin_types_compatible_p(__typeof__(expr), signed char) ||             \
   __builtin_types_compatible_p(__typeof__(expr), unsigned char))

/* evaluates to true if expr is of type curl_off_t */
#define _curl_is_off_t(expr)                                                  \
  (__builtin_types_compatible_p(__typeof__(expr), curl_off_t))

/* evaluates to true if expr is abuffer suitable for CURLOPT_ERRORBUFFER */
/* XXX: also check size of an char[] array? */
#define _curl_is_error_buffer(expr)                                           \
  (__builtin_types_compatible_p(__typeof__(expr), char *) ||                  \
   __builtin_types_compatible_p(__typeof__(expr), char[]))

/* evaluates to true if expr is of type (const) void* or (const) FILE* */
#if 0
#define _curl_is_cb_data(expr)                                                \
  (_curl_is_ptr((expr), void) ||                                              \
   _curl_is_ptr((expr), FILE))
#else /* be less strict */
#define _curl_is_cb_data(expr)                                                \
  _curl_is_any_ptr(expr)
#endif

/* evaluates to true if expr is of type FILE* */
#define _curl_is_FILE(expr)                                                   \
  (__builtin_types_compatible_p(__typeof__(expr), FILE *))

/* evaluates to true if expr can be passed as POST data (void* or char*) */
#define _curl_is_postfields(expr)                                             \
  (_curl_is_ptr((expr), void) ||                                              \
   _curl_is_arr((expr), char))

/* FIXME: the whole callback checking is messy...
 * The idea is to tolerate char vs. void and const vs. not const
 * pointers in arguments at least
 */
/* helper: __builtin_types_compatible_p distinguishes between functions and
 * function pointers, hide it */
#define _curl_callback_compatible(func, type)                                 \
  (__builtin_types_compatible_p(__typeof__(func), type) ||                    \
   __builtin_types_compatible_p(__typeof__(func), type*))

/* evaluates to true if expr is of type curl_read_callback or "similar" */
#define _curl_is_read_cb(expr)                                          \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), __typeof__(fread)) ||       \
   __builtin_types_compatible_p(__typeof__(expr), curl_read_callback) ||      \
   _curl_callback_compatible((expr), _curl_read_callback1) ||                 \
   _curl_callback_compatible((expr), _curl_read_callback2) ||                 \
   _curl_callback_compatible((expr), _curl_read_callback3) ||                 \
   _curl_callback_compatible((expr), _curl_read_callback4) ||                 \
   _curl_callback_compatible((expr), _curl_read_callback5) ||                 \
   _curl_callback_compatible((expr), _curl_read_callback6))
typedef size_t (_curl_read_callback1)(char *, size_t, size_t, void*);
typedef size_t (_curl_read_callback2)(char *, size_t, size_t, const void*);
typedef size_t (_curl_read_callback3)(char *, size_t, size_t, FILE*);
typedef size_t (_curl_read_callback4)(void *, size_t, size_t, void*);
typedef size_t (_curl_read_callback5)(void *, size_t, size_t, const void*);
typedef size_t (_curl_read_callback6)(void *, size_t, size_t, FILE*);

/* evaluates to true if expr is of type curl_write_callback or "similar" */
#define _curl_is_write_cb(expr)                                               \
  (_curl_is_read_cb(expr) ||                                            \
   __builtin_types_compatible_p(__typeof__(expr), __typeof__(fwrite)) ||      \
   __builtin_types_compatible_p(__typeof__(expr), curl_write_callback) ||     \
   _curl_callback_compatible((expr), _curl_write_callback1) ||                \
   _curl_callback_compatible((expr), _curl_write_callback2) ||                \
   _curl_callback_compatible((expr), _curl_write_callback3) ||                \
   _curl_callback_compatible((expr), _curl_write_callback4) ||                \
   _curl_callback_compatible((expr), _curl_write_callback5) ||                \
   _curl_callback_compatible((expr), _curl_write_callback6))
typedef size_t (_curl_write_callback1)(const char *, size_t, size_t, void*);
typedef size_t (_curl_write_callback2)(const char *, size_t, size_t,
                                       const void*);
typedef size_t (_curl_write_callback3)(const char *, size_t, size_t, FILE*);
typedef size_t (_curl_write_callback4)(const void *, size_t, size_t, void*);
typedef size_t (_curl_write_callback5)(const void *, size_t, size_t,
                                       const void*);
typedef size_t (_curl_write_callback6)(const void *, size_t, size_t, FILE*);

/* evaluates to true if expr is of type curl_ioctl_callback or "similar" */
#define _curl_is_ioctl_cb(expr)                                         \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_ioctl_callback) ||     \
   _curl_callback_compatible((expr), _curl_ioctl_callback1) ||                \
   _curl_callback_compatible((expr), _curl_ioctl_callback2) ||                \
   _curl_callback_compatible((expr), _curl_ioctl_callback3) ||                \
   _curl_callback_compatible((expr), _curl_ioctl_callback4))
typedef curlioerr (_curl_ioctl_callback1)(CURL *, int, void*);
typedef curlioerr (_curl_ioctl_callback2)(CURL *, int, const void*);
typedef curlioerr (_curl_ioctl_callback3)(CURL *, curliocmd, void*);
typedef curlioerr (_curl_ioctl_callback4)(CURL *, curliocmd, const void*);

/* evaluates to true if expr is of type curl_sockopt_callback or "similar" */
#define _curl_is_sockopt_cb(expr)                                       \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_sockopt_callback) ||   \
   _curl_callback_compatible((expr), _curl_sockopt_callback1) ||              \
   _curl_callback_compatible((expr), _curl_sockopt_callback2))
typedef int (_curl_sockopt_callback1)(void *, curl_socket_t, curlsocktype);
typedef int (_curl_sockopt_callback2)(const void *, curl_socket_t,
                                      curlsocktype);

/* evaluates to true if expr is of type curl_opensocket_callback or "similar" */
#define _curl_is_opensocket_cb(expr)                                    \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_opensocket_callback) ||\
   _curl_callback_compatible((expr), _curl_opensocket_callback1) ||           \
   _curl_callback_compatible((expr), _curl_opensocket_callback2) ||           \
   _curl_callback_compatible((expr), _curl_opensocket_callback3) ||           \
   _curl_callback_compatible((expr), _curl_opensocket_callback4))
typedef curl_socket_t (_curl_opensocket_callback1)
  (void *, curlsocktype, struct curl_sockaddr *);
typedef curl_socket_t (_curl_opensocket_callback2)
  (void *, curlsocktype, const struct curl_sockaddr *);
typedef curl_socket_t (_curl_opensocket_callback3)
  (const void *, curlsocktype, struct curl_sockaddr *);
typedef curl_socket_t (_curl_opensocket_callback4)
  (const void *, curlsocktype, const struct curl_sockaddr *);

/* evaluates to true if expr is of type curl_progress_callback or "similar" */
#define _curl_is_progress_cb(expr)                                      \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_progress_callback) ||  \
   _curl_callback_compatible((expr), _curl_progress_callback1) ||             \
   _curl_callback_compatible((expr), _curl_progress_callback2))
typedef int (_curl_progress_callback1)(void *,
    double, double, double, double);
typedef int (_curl_progress_callback2)(const void *,
    double, double, double, double);

/* evaluates to true if expr is of type curl_debug_callback or "similar" */
#define _curl_is_debug_cb(expr)                                         \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_debug_callback) ||     \
   _curl_callback_compatible((expr), _curl_debug_callback1) ||                \
   _curl_callback_compatible((expr), _curl_debug_callback2) ||                \
   _curl_callback_compatible((expr), _curl_debug_callback3) ||                \
   _curl_callback_compatible((expr), _curl_debug_callback4))
typedef int (_curl_debug_callback1) (CURL *,
    curl_infotype, char *, size_t, void *);
typedef int (_curl_debug_callback2) (CURL *,
    curl_infotype, char *, size_t, const void *);
typedef int (_curl_debug_callback3) (CURL *,
    curl_infotype, const char *, size_t, void *);
typedef int (_curl_debug_callback4) (CURL *,
    curl_infotype, const char *, size_t, const void *);

/* evaluates to true if expr is of type curl_ssl_ctx_callback or "similar" */
/* this is getting even messier... */
#define _curl_is_ssl_ctx_cb(expr)                                       \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_ssl_ctx_callback) ||   \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback1) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback2) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback3) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback4) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback5) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback6) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback7) ||              \
   _curl_callback_compatible((expr), _curl_ssl_ctx_callback8))
typedef CURLcode (_curl_ssl_ctx_callback1)(CURL *, void *, void *);
typedef CURLcode (_curl_ssl_ctx_callback2)(CURL *, void *, const void *);
typedef CURLcode (_curl_ssl_ctx_callback3)(CURL *, const void *, void *);
typedef CURLcode (_curl_ssl_ctx_callback4)(CURL *, const void *, const void *);
#ifdef HEADER_SSL_H
/* hack: if we included OpenSSL's ssl.h, we know about SSL_CTX
 * this will of course break if we're included before OpenSSL headers...
 */
typedef CURLcode (_curl_ssl_ctx_callback5)(CURL *, SSL_CTX, void *);
typedef CURLcode (_curl_ssl_ctx_callback6)(CURL *, SSL_CTX, const void *);
typedef CURLcode (_curl_ssl_ctx_callback7)(CURL *, const SSL_CTX, void *);
typedef CURLcode (_curl_ssl_ctx_callback8)(CURL *, const SSL_CTX, const void *);
#else
typedef _curl_ssl_ctx_callback1 _curl_ssl_ctx_callback5;
typedef _curl_ssl_ctx_callback1 _curl_ssl_ctx_callback6;
typedef _curl_ssl_ctx_callback1 _curl_ssl_ctx_callback7;
typedef _curl_ssl_ctx_callback1 _curl_ssl_ctx_callback8;
#endif

/* evaluates to true if expr is of type curl_conv_callback or "similar" */
#define _curl_is_conv_cb(expr)                                          \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_conv_callback) ||      \
   _curl_callback_compatible((expr), _curl_conv_callback1) ||                 \
   _curl_callback_compatible((expr), _curl_conv_callback2) ||                 \
   _curl_callback_compatible((expr), _curl_conv_callback3) ||                 \
   _curl_callback_compatible((expr), _curl_conv_callback4))
typedef CURLcode (*_curl_conv_callback1)(char *, size_t length);
typedef CURLcode (*_curl_conv_callback2)(const char *, size_t length);
typedef CURLcode (*_curl_conv_callback3)(void *, size_t length);
typedef CURLcode (*_curl_conv_callback4)(const void *, size_t length);

/* evaluates to true if expr is of type curl_seek_callback or "similar" */
#define _curl_is_seek_cb(expr)                                          \
  (_curl_is_NULL(expr) ||                                                     \
   __builtin_types_compatible_p(__typeof__(expr), curl_seek_callback) ||      \
   _curl_callback_compatible((expr), _curl_seek_callback1) ||                 \
   _curl_callback_compatible((expr), _curl_seek_callback2))
typedef CURLcode (*_curl_seek_callback1)(void *, curl_off_t, int);
typedef CURLcode (*_curl_seek_callback2)(const void *, curl_off_t, int);


#endif /* __CURL_TYPECHECK_GCC_H */
