/* GLIB - Library of useful routines for C programming
 * Copyright © 2020 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General
 * Public License along with this library; if not, see
 * <http://www.gnu.org/licenses/>.
 */

#pragma once

#if !defined (__GLIB_H_INSIDE__) && !defined (GLIB_COMPILATION)
#error "Only <glib.h> can be included directly."
#endif

#include <glib/gtypes.h>

G_BEGIN_DECLS

G_GNUC_BEGIN_IGNORE_DEPRECATIONS

typedef struct _GUri GUri;

GLIB_AVAILABLE_IN_2_66
GUri *       g_uri_ref              (GUri *uri);
GLIB_AVAILABLE_IN_2_66
void         g_uri_unref            (GUri *uri);

/**
 * GUriFlags:
 * @G_URI_FLAGS_NONE: No flags set.
 * @G_URI_FLAGS_PARSE_RELAXED: Parse the URI more relaxedly than the
 *     [RFC 3986](https://tools.ietf.org/html/rfc3986) grammar specifies,
 *     fixing up or ignoring common mistakes in URIs coming from external
 *     sources. This is also needed for some obscure URI schemes where `;`
 *     separates the host from the path. Don’t use this flag unless you need to.
 * @G_URI_FLAGS_HAS_PASSWORD: The userinfo field may contain a password,
 *     which will be separated from the username by `:`.
 * @G_URI_FLAGS_HAS_AUTH_PARAMS: The userinfo may contain additional
 *     authentication-related parameters, which will be separated from
 *     the username and/or password by `;`.
 * @G_URI_FLAGS_NON_DNS: The host component should not be assumed to be a
 *     DNS hostname or IP address (for example, for `smb` URIs with NetBIOS
 *     hostnames).
 * @G_URI_FLAGS_ENCODED: When parsing a URI, this indicates that `%`-encoded
 *     characters in the userinfo, path, query, and fragment fields
 *     should not be decoded. (And likewise the host field if
 *     %G_URI_FLAGS_NON_DNS is also set.) When building a URI, it indicates
 *     that you have already `%`-encoded the components, and so #GUri
 *     should not do any encoding itself.
 * @G_URI_FLAGS_ENCODED_QUERY: Same as %G_URI_FLAGS_ENCODED, for the query
 *     field only.
 * @G_URI_FLAGS_ENCODED_PATH: Same as %G_URI_FLAGS_ENCODED, for the path only.
 * @G_URI_FLAGS_ENCODED_FRAGMENT: Same as %G_URI_FLAGS_ENCODED, for the
 *     fragment only.
 *
 * Flags that describe a URI.
 *
 * When parsing a URI, if you need to choose different flags based on
 * the type of URI, you can use g_uri_peek_scheme() on the URI string
 * to check the scheme first, and use that to decide what flags to
 * parse it with.
 *
 * Since: 2.66
 */
GLIB_AVAILABLE_TYPE_IN_2_66
typedef enum {
  G_URI_FLAGS_NONE            = 0,
  G_URI_FLAGS_PARSE_RELAXED   = 1 << 0,
  G_URI_FLAGS_HAS_PASSWORD    = 1 << 1,
  G_URI_FLAGS_HAS_AUTH_PARAMS = 1 << 2,
  G_URI_FLAGS_ENCODED         = 1 << 3,
  G_URI_FLAGS_NON_DNS         = 1 << 4,
  G_URI_FLAGS_ENCODED_QUERY   = 1 << 5,
  G_URI_FLAGS_ENCODED_PATH    = 1 << 6,
  G_URI_FLAGS_ENCODED_FRAGMENT = 1 << 7,
} GUriFlags;

GLIB_AVAILABLE_IN_2_66
gboolean     g_uri_split            (const gchar  *uri_ref,
                                     GUriFlags     flags,
                                     gchar       **scheme,
                                     gchar       **userinfo,
                                     gchar       **host,
                                     gint         *port,
                                     gchar       **path,
                                     gchar       **query,
                                     gchar       **fragment,
                                     GError      **error);
GLIB_AVAILABLE_IN_2_66
gboolean     g_uri_split_with_user  (const gchar  *uri_ref,
                                     GUriFlags     flags,
                                     gchar       **scheme,
                                     gchar       **user,
                                     gchar       **password,
                                     gchar       **auth_params,
                                     gchar       **host,
                                     gint         *port,
                                     gchar       **path,
                                     gchar       **query,
                                     gchar       **fragment,
                                     GError      **error);
GLIB_AVAILABLE_IN_2_66
gboolean     g_uri_split_network    (const gchar  *uri_string,
                                     GUriFlags     flags,
                                     gchar       **scheme,
                                     gchar       **host,
                                     gint         *port,
                                     GError      **error);

GLIB_AVAILABLE_IN_2_66
gboolean     g_uri_is_valid         (const gchar  *uri_string,
                                     GUriFlags     flags,
                                     GError      **error);

GLIB_AVAILABLE_IN_2_66
gchar *      g_uri_join             (GUriFlags     flags,
                                     const gchar  *scheme,
                                     const gchar  *userinfo,
                                     const gchar  *host,
                                     gint          port,
                                     const gchar  *path,
                                     const gchar  *query,
                                     const gchar  *fragment);
GLIB_AVAILABLE_IN_2_66
gchar *      g_uri_join_with_user   (GUriFlags     flags,
                                     const gchar  *scheme,
                                     const gchar  *user,
                                     const gchar  *password,
                                     const gchar  *auth_params,
                                     const gchar  *host,
                                     gint          port,
                                     const gchar  *path,
                                     const gchar  *query,
                                     const gchar  *fragment);

GLIB_AVAILABLE_IN_2_66
GUri *       g_uri_parse            (const gchar  *uri_string,
                                     GUriFlags     flags,
                                     GError      **error);
GLIB_AVAILABLE_IN_2_66
GUri *       g_uri_parse_relative   (GUri         *base_uri,
                                     const gchar  *uri_ref,
                                     GUriFlags     flags,
                                     GError      **error);

GLIB_AVAILABLE_IN_2_66
gchar *      g_uri_resolve_relative (const gchar  *base_uri_string,
                                     const gchar  *uri_ref,
                                     GUriFlags     flags,
                                     GError      **error);

GLIB_AVAILABLE_IN_2_66
GUri *       g_uri_build            (GUriFlags     flags,
                                     const gchar  *scheme,
                                     const gchar  *userinfo,
                                     const gchar  *host,
                                     gint          port,
                                     const gchar  *path,
                                     const gchar  *query,
                                     const gchar  *fragment);
GLIB_AVAILABLE_IN_2_66
GUri *       g_uri_build_with_user  (GUriFlags     flags,
                                     const gchar  *scheme,
                                     const gchar  *user,
                                     const gchar  *password,
                                     const gchar  *auth_params,
                                     const gchar  *host,
                                     gint          port,
                                     const gchar  *path,
                                     const gchar  *query,
                                     const gchar  *fragment);

/**
 * GUriHideFlags:
 * @G_URI_HIDE_NONE: No flags set.
 * @G_URI_HIDE_USERINFO: Hide the userinfo.
 * @G_URI_HIDE_PASSWORD: Hide the password.
 * @G_URI_HIDE_AUTH_PARAMS: Hide the auth_params.
 * @G_URI_HIDE_QUERY: Hide the query.
 * @G_URI_HIDE_FRAGMENT: Hide the fragment.
 *
 * Flags describing what parts of the URI to hide in
 * g_uri_to_string_partial(). Note that %G_URI_HIDE_PASSWORD and
 * %G_URI_HIDE_AUTH_PARAMS will only work if the #GUri was parsed with
 * the corresponding flags.
 *
 * Since: 2.66
 */
GLIB_AVAILABLE_TYPE_IN_2_66
typedef enum {
  G_URI_HIDE_NONE        = 0,
  G_URI_HIDE_USERINFO    = 1 << 0,
  G_URI_HIDE_PASSWORD    = 1 << 1,
  G_URI_HIDE_AUTH_PARAMS = 1 << 2,
  G_URI_HIDE_QUERY       = 1 << 3,
  G_URI_HIDE_FRAGMENT    = 1 << 4,
} GUriHideFlags;

GLIB_AVAILABLE_IN_2_66
char *       g_uri_to_string         (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
char *       g_uri_to_string_partial (GUri          *uri,
                                      GUriHideFlags  flags);

GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_scheme        (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_userinfo      (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_user          (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_password      (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_auth_params   (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_host          (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
gint         g_uri_get_port          (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_path          (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_query         (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
const gchar *g_uri_get_fragment      (GUri          *uri);
GLIB_AVAILABLE_IN_2_66
GUriFlags    g_uri_get_flags         (GUri          *uri);

/**
 * GUriParamsFlags:
 * @G_URI_PARAMS_NONE: No flags set.
 * @G_URI_PARAMS_CASE_INSENSITIVE: Parameter names are case insensitive.
 * @G_URI_PARAMS_WWW_FORM: Replace `+` with space character. Only useful for
 *     URLs on the web, using the `https` or `http` schemas.
 * @G_URI_PARAMS_PARSE_RELAXED: See %G_URI_FLAGS_PARSE_RELAXED.
 *
 * Flags modifying the way parameters are handled by g_uri_parse_params() and
 * #GUriParamsIter.
 *
 * Since: 2.66
 */
GLIB_AVAILABLE_TYPE_IN_2_66
typedef enum {
  G_URI_PARAMS_NONE             = 0,
  G_URI_PARAMS_CASE_INSENSITIVE = 1 << 0,
  G_URI_PARAMS_WWW_FORM         = 1 << 1,
  G_URI_PARAMS_PARSE_RELAXED    = 1 << 2,
} GUriParamsFlags;

GLIB_AVAILABLE_IN_2_66
GHashTable *g_uri_parse_params       (const gchar    *params,
                                      gssize          length,
                                      const gchar    *separators,
                                      GUriParamsFlags flags,
                                      GError        **error);

typedef struct _GUriParamsIter GUriParamsIter;

struct _GUriParamsIter
{
  /*< private >*/
  gint     dummy0;
  gpointer dummy1;
  gpointer dummy2;
  guint8   dummy3[256];
};

GLIB_AVAILABLE_IN_2_66
void        g_uri_params_iter_init   (GUriParamsIter *iter,
                                      const gchar    *params,
                                      gssize          length,
                                      const gchar    *separators,
                                      GUriParamsFlags flags);

GLIB_AVAILABLE_IN_2_66
gboolean    g_uri_params_iter_next   (GUriParamsIter *iter,
                                      gchar         **attribute,
                                      gchar         **value,
                                      GError        **error);

/**
 * G_URI_ERROR:
 *
 * Error domain for URI methods. Errors in this domain will be from
 * the #GUriError enumeration. See #GError for information on error
 * domains.
 *
 * Since: 2.66
 */
#define G_URI_ERROR (g_uri_error_quark ()) GLIB_AVAILABLE_MACRO_IN_2_66
GLIB_AVAILABLE_IN_2_66
GQuark g_uri_error_quark (void);

/**
 * GUriError:
 * @G_URI_ERROR_FAILED: Generic error if no more specific error is available.
 *     See the error message for details.
 * @G_URI_ERROR_BAD_SCHEME: The scheme of a URI could not be parsed.
 * @G_URI_ERROR_BAD_USER: The user/userinfo of a URI could not be parsed.
 * @G_URI_ERROR_BAD_PASSWORD: The password of a URI could not be parsed.
 * @G_URI_ERROR_BAD_AUTH_PARAMS: The authentication parameters of a URI could not be parsed.
 * @G_URI_ERROR_BAD_HOST: The host of a URI could not be parsed.
 * @G_URI_ERROR_BAD_PORT: The port of a URI could not be parsed.
 * @G_URI_ERROR_BAD_PATH: The path of a URI could not be parsed.
 * @G_URI_ERROR_BAD_QUERY: The query of a URI could not be parsed.
 * @G_URI_ERROR_BAD_FRAGMENT: The fragment of a URI could not be parsed.
 *
 * Error codes returned by #GUri methods.
 *
 * Since: 2.66
 */
typedef enum {
  G_URI_ERROR_FAILED,
  G_URI_ERROR_BAD_SCHEME,
  G_URI_ERROR_BAD_USER,
  G_URI_ERROR_BAD_PASSWORD,
  G_URI_ERROR_BAD_AUTH_PARAMS,
  G_URI_ERROR_BAD_HOST,
  G_URI_ERROR_BAD_PORT,
  G_URI_ERROR_BAD_PATH,
  G_URI_ERROR_BAD_QUERY,
  G_URI_ERROR_BAD_FRAGMENT,
} GUriError;

/**
 * G_URI_RESERVED_CHARS_GENERIC_DELIMITERS:
 *
 * Generic delimiters characters as defined in
 * [RFC 3986](https://tools.ietf.org/html/rfc3986). Includes `:/?#[]@`.
 *
 * Since: 2.16
 **/
#define G_URI_RESERVED_CHARS_GENERIC_DELIMITERS ":/?#[]@"

/**
 * G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS:
 *
 * Subcomponent delimiter characters as defined in
 * [RFC 3986](https://tools.ietf.org/html/rfc3986). Includes `!$&'()*+,;=`.
 *
 * Since: 2.16
 **/
#define G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS "!$&'()*+,;="

/**
 * G_URI_RESERVED_CHARS_ALLOWED_IN_PATH_ELEMENT:
 *
 * Allowed characters in path elements. Includes `!$&'()*+,;=:@`.
 *
 * Since: 2.16
 **/
#define G_URI_RESERVED_CHARS_ALLOWED_IN_PATH_ELEMENT G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS ":@"

/**
 * G_URI_RESERVED_CHARS_ALLOWED_IN_PATH:
 *
 * Allowed characters in a path. Includes `!$&'()*+,;=:@/`.
 *
 * Since: 2.16
 **/
#define G_URI_RESERVED_CHARS_ALLOWED_IN_PATH G_URI_RESERVED_CHARS_ALLOWED_IN_PATH_ELEMENT "/"

/**
 * G_URI_RESERVED_CHARS_ALLOWED_IN_USERINFO:
 *
 * Allowed characters in userinfo as defined in
 * [RFC 3986](https://tools.ietf.org/html/rfc3986). Includes `!$&'()*+,;=:`.
 *
 * Since: 2.16
 **/
#define G_URI_RESERVED_CHARS_ALLOWED_IN_USERINFO G_URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS ":"

GLIB_AVAILABLE_IN_ALL
char *      g_uri_unescape_string  (const char *escaped_string,
                                    const char *illegal_characters);
GLIB_AVAILABLE_IN_ALL
char *      g_uri_unescape_segment (const char *escaped_string,
                                    const char *escaped_string_end,
                                    const char *illegal_characters);

GLIB_AVAILABLE_IN_ALL
char *      g_uri_parse_scheme     (const char *uri);
GLIB_AVAILABLE_IN_2_66
const char *g_uri_peek_scheme      (const char *uri);

GLIB_AVAILABLE_IN_ALL
char *      g_uri_escape_string    (const char *unescaped,
                                    const char *reserved_chars_allowed,
                                    gboolean    allow_utf8);

GLIB_AVAILABLE_IN_2_66
GBytes *    g_uri_unescape_bytes   (const char *escaped_string,
                                    gssize      length,
                                    const char *illegal_characters,
                                    GError    **error);

GLIB_AVAILABLE_IN_2_66
char *      g_uri_escape_bytes     (const guint8 *unescaped,
                                    gsize         length,
                                    const char   *reserved_chars_allowed);

G_GNUC_END_IGNORE_DEPRECATIONS

G_END_DECLS
