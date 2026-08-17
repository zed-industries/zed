/*
 * Copyright © 2011  Google, Inc.
 *
 *  This is part of HarfBuzz, a text shaping library.
 *
 * Permission is hereby granted, without written agreement and without
 * license or royalty fees, to use, copy, modify, and distribute this
 * software and its documentation for any purpose, provided that the
 * above copyright notice and the following two paragraphs appear in
 * all copies of this software.
 *
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE TO ANY PARTY FOR
 * DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES
 * ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN
 * IF THE COPYRIGHT HOLDER HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * THE COPYRIGHT HOLDER SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING,
 * BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS
 * ON AN "AS IS" BASIS, AND THE COPYRIGHT HOLDER HAS NO OBLIGATION TO
 * PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 * Google Author(s): Behdad Esfahbod
 */

#ifndef HB_TEST_H
#define HB_TEST_H

#include <hb-config.hh>

#include <hb-glib.h>

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#ifdef HAVE_STDBOOL_H
# include <stdbool.h>
#else
typedef short bool;
# ifndef true
#  define true 1
# endif
# ifndef false
#  define false 0
# endif
#endif

HB_BEGIN_DECLS

/* Just in case */
#undef G_DISABLE_ASSERT

#define HB_UNUSED	G_GNUC_UNUSED

/* Misc */

/* This is too ugly to be public API, but quite handy. */
#define HB_TAG_CHAR4(s)   (HB_TAG(((const char *) s)[0], \
				  ((const char *) s)[1], \
				  ((const char *) s)[2], \
				  ((const char *) s)[3]))

#define HB_FACE_ADD_TABLE(face, tag, data) \
	do { \
	  hb_blob_t *blob = hb_blob_create_or_fail ((data), \
						    sizeof (data), \
						    HB_MEMORY_MODE_READONLY, \
						    NULL, NULL); \
	  hb_face_builder_add_table ((face), \
				     HB_TAG_CHAR4(tag), \
				     blob); \
	  hb_blob_destroy (blob); \
	} while (0)

static inline const char *
srcdir (void)
{
  static const char *s;

  if (!s) {
    s = getenv ("srcdir");

#ifdef SRCDIR
    if (!s || !s[0])
      s = SRCDIR;
#endif

    if (!s || !s[0])
      s = ".";
  }

  return s;
}


/* Helpers */

static inline void
hb_test_init (int *argc, char ***argv)
{
  g_test_init (argc, argv, NULL);
}

static inline int
hb_test_run (void)
{
  return g_test_run ();
}

/* Bugzilla helpers */

static inline void
hb_test_bug (const char *uri_base, unsigned int number)
{
  char *s = g_strdup_printf ("%u", number);

  g_test_bug_base (uri_base);
  g_test_bug (s);

  g_free (s);
}

static inline void
hb_test_bug_freedesktop (unsigned int number)
{
  hb_test_bug ("https://bugs.freedesktop.org/", number);
}

static inline void
hb_test_bug_gnome (unsigned int number)
{
  hb_test_bug ("https://bugzilla.gnome.org/", number);
}

static inline void
hb_test_bug_mozilla (unsigned int number)
{
  hb_test_bug ("https://bugzilla.mozilla.org/", number);
}

static inline void
hb_test_bug_redhat (unsigned int number)
{
  hb_test_bug ("https://bugzilla.redhat.com/", number);
}


/* Wrap glib test functions to simplify.  Should have been in glib already. */

/* Drops the "test_" prefix and converts '_' to '/'.
 * Essentially builds test path from function name. */
static inline char *
hb_test_normalize_path (const char *path)
{
  char *s, *p;

  g_assert (0 == strncmp (path, "test_", 5));
  path += 4;

  s = g_strdup (path);
  for (p = s; *p; p++)
    if (*p == '_')
      *p = '/';

  return s;
}


#if GLIB_CHECK_VERSION(2,25,12)
typedef GTestFunc        hb_test_func_t;
typedef GTestDataFunc    hb_test_data_func_t;
typedef GTestFixtureFunc hb_test_fixture_func_t;
#else
typedef void (*hb_test_func_t)         (void);
typedef void (*hb_test_data_func_t)    (gconstpointer user_data);
typedef void (*hb_test_fixture_func_t) (void);
#endif

#if !GLIB_CHECK_VERSION(2,30,0)
#define g_test_fail() g_error("Test failed")
#endif
#ifndef g_assert_true
#define g_assert_true g_assert
#endif
#ifndef g_assert_false
#define g_assert_false(expr) g_assert(!(expr))
#endif
#ifndef g_assert_nonnull
#define g_assert_nonnull  g_assert
#endif
#ifndef g_assert_cmpmem
#define g_assert_cmpmem(m1, l1, m2, l2) g_assert_true (l1 == l2 && memcmp (m1, m2, l1) == 0)
#endif

static inline void hb_test_assert_blobs_equal (hb_blob_t *expected_blob, hb_blob_t *actual_blob)
{
  unsigned int expected_length, actual_length;
  const char *raw_expected = hb_blob_get_data (expected_blob, &expected_length);
  const char *raw_actual = hb_blob_get_data (actual_blob, &actual_length);
  g_assert_cmpint(expected_length, ==, actual_length);
  if (memcmp (raw_expected, raw_actual, expected_length) != 0)
  {
    for (unsigned int i = 0; i < expected_length; i++)
    {
      int expected = *(raw_expected + i);
      int actual = *(raw_actual + i);
      if (expected != actual) fprintf(stderr, "+%u %02x != %02x\n", i, expected, actual);
      else fprintf(stderr, "+%u %02x\n", i, expected);
    }
  }
  g_assert_cmpint(0, ==, memcmp(raw_expected, raw_actual, expected_length));
}

static inline void
hb_test_add_func (const char *test_path,
		  hb_test_func_t   test_func)
{
  char *normal_path = hb_test_normalize_path (test_path);
  g_test_add_func (normal_path, test_func);
  g_free (normal_path);
}
#define hb_test_add(Func) hb_test_add_func (#Func, Func)


static inline void
hb_test_add_data_func (const char          *test_path,
		       gconstpointer        test_data,
		       hb_test_data_func_t  test_func)
{
  char *normal_path = hb_test_normalize_path (test_path);
  g_test_add_data_func (normal_path, test_data, test_func);
  g_free (normal_path);
}
#define hb_test_add_data(UserData, Func) hb_test_add_data_func (#Func, UserData, Func)

static inline void
hb_test_add_data_func_flavor (const char          *test_path,
			      const char          *flavor,
			      gconstpointer        test_data,
			      hb_test_data_func_t  test_func)
{
  if (flavor && *flavor)
  {
    char *path = g_strdup_printf ("%s/%s", test_path, flavor);
    hb_test_add_data_func (path, test_data, test_func);
    g_free (path);
  }
  else
    hb_test_add_data_func (test_path, test_data, test_func);
}
#define hb_test_add_data_flavor(UserData, Flavor, Func) hb_test_add_data_func_flavor (#Func, Flavor, UserData, Func)

#define hb_test_add_flavor(Flavor, Func) hb_test_add_data_flavor (Flavor, Flavor, Func)

static inline void
hb_test_add_vtable (const char             *test_path,
		    gsize                   data_size,
		    gconstpointer           test_data,
		    hb_test_fixture_func_t  data_setup,
		    hb_test_fixture_func_t  data_test,
		    hb_test_fixture_func_t  data_teardown)
{
  char *normal_path = hb_test_normalize_path (test_path);
  g_test_add_vtable (normal_path, data_size, test_data, data_setup, data_test, data_teardown);
  g_free (normal_path);
}
#define hb_test_add_fixture(FixturePrefix, UserData, Func) \
G_STMT_START { \
  typedef G_PASTE (FixturePrefix, _t) Fixture; \
  void (*add_vtable) (const char*, gsize, gconstpointer, \
		      void (*) (Fixture*, gconstpointer), \
		      void (*) (Fixture*, gconstpointer), \
		      void (*) (Fixture*, gconstpointer)) \
	= (void (*) (const gchar *, gsize, gconstpointer, \
		     void (*) (Fixture*, gconstpointer), \
		     void (*) (Fixture*, gconstpointer), \
		     void (*) (Fixture*, gconstpointer))) hb_test_add_vtable; \
  add_vtable (#Func, sizeof (G_PASTE (FixturePrefix, _t)), UserData, \
	      G_PASTE (FixturePrefix, _init), Func, G_PASTE (FixturePrefix, _finish)); \
} G_STMT_END

static inline void
hb_test_add_vtable_flavor (const char             *test_path,
			   const char             *flavor,
			   gsize                   data_size,
			   gconstpointer           test_data,
			   hb_test_fixture_func_t  data_setup,
			   hb_test_fixture_func_t  data_test,
			   hb_test_fixture_func_t  data_teardown)
{
  char *path = g_strdup_printf ("%s/%s", test_path, flavor);
  hb_test_add_vtable (path, data_size, test_data, data_setup, data_test, data_teardown);
  g_free (path);
}
#define hb_test_add_fixture_flavor(FixturePrefix, UserData, Flavor, Func) \
G_STMT_START { \
  typedef G_PASTE (FixturePrefix, _t) Fixture; \
  void (*add_vtable) (const char*, const char *, gsize, gconstpointer, \
		      void (*) (Fixture*, gconstpointer), \
		      void (*) (Fixture*, gconstpointer), \
		      void (*) (Fixture*, gconstpointer)) \
	= (void (*) (const gchar *, const char *, gsize, gconstpointer, \
		     void (*) (Fixture*, gconstpointer), \
		     void (*) (Fixture*, gconstpointer), \
		     void (*) (Fixture*, gconstpointer))) hb_test_add_vtable_flavor; \
  add_vtable (#Func, Flavor, sizeof (G_PASTE (FixturePrefix, _t)), UserData, \
	      G_PASTE (FixturePrefix, _init), Func, G_PASTE (FixturePrefix, _finish)); \
} G_STMT_END


static inline char *
hb_test_resolve_path (const char *path)
{
#if GLIB_CHECK_VERSION(2,37,2)
  if (path[0] != '/')
    return g_test_build_filename (G_TEST_DIST, path, NULL);
#endif
  return g_strdup (path);
}

static inline hb_face_t *
hb_test_open_font_file_with_index (const char *font_path, unsigned face_index)
{
  char *path = hb_test_resolve_path (font_path);

  hb_blob_t *blob = hb_blob_create_from_file_or_fail (path);
  hb_face_t *face;
  if (!blob)
    g_error ("Font %s not found.", path);

  face = hb_face_create (blob, face_index);
  hb_blob_destroy (blob);

  g_free (path);

  return face;
}
static inline hb_face_t *
hb_test_open_font_file (const char *font_path)
{
  return hb_test_open_font_file_with_index (font_path, 0);
}

HB_END_DECLS

#endif /* HB_TEST_H */
