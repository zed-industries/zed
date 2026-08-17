/*
 * Copyright © 2011 Ryan Lortie
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 *
 * Author: Ryan Lortie <desrt@desrt.ca>
 */

#ifndef __G_ATOMIC_H__
#define __G_ATOMIC_H__

#if !defined (__GLIB_H_INSIDE__) && !defined (GLIB_COMPILATION)
#error "Only <glib.h> can be included directly."
#endif

#include <glib/gtypes.h>

G_BEGIN_DECLS

GLIB_AVAILABLE_IN_ALL
gint                    g_atomic_int_get                      (const volatile gint *atomic);
GLIB_AVAILABLE_IN_ALL
void                    g_atomic_int_set                      (volatile gint  *atomic,
                                                               gint            newval);
GLIB_AVAILABLE_IN_ALL
void                    g_atomic_int_inc                      (volatile gint  *atomic);
GLIB_AVAILABLE_IN_ALL
gboolean                g_atomic_int_dec_and_test             (volatile gint  *atomic);
GLIB_AVAILABLE_IN_ALL
gboolean                g_atomic_int_compare_and_exchange     (volatile gint  *atomic,
                                                               gint            oldval,
                                                               gint            newval);
GLIB_AVAILABLE_IN_ALL
gint                    g_atomic_int_add                      (volatile gint  *atomic,
                                                               gint            val);
GLIB_AVAILABLE_IN_2_30
guint                   g_atomic_int_and                      (volatile guint *atomic,
                                                               guint           val);
GLIB_AVAILABLE_IN_2_30
guint                   g_atomic_int_or                       (volatile guint *atomic,
                                                               guint           val);
GLIB_AVAILABLE_IN_ALL
guint                   g_atomic_int_xor                      (volatile guint *atomic,
                                                               guint           val);

GLIB_AVAILABLE_IN_ALL
gpointer                g_atomic_pointer_get                  (const volatile void *atomic);
GLIB_AVAILABLE_IN_ALL
void                    g_atomic_pointer_set                  (volatile void  *atomic,
                                                               gpointer        newval);
GLIB_AVAILABLE_IN_ALL
gboolean                g_atomic_pointer_compare_and_exchange (volatile void  *atomic,
                                                               gpointer        oldval,
                                                               gpointer        newval);
GLIB_AVAILABLE_IN_ALL
gssize                  g_atomic_pointer_add                  (volatile void  *atomic,
                                                               gssize          val);
GLIB_AVAILABLE_IN_2_30
gsize                   g_atomic_pointer_and                  (volatile void  *atomic,
                                                               gsize           val);
GLIB_AVAILABLE_IN_2_30
gsize                   g_atomic_pointer_or                   (volatile void  *atomic,
                                                               gsize           val);
GLIB_AVAILABLE_IN_ALL
gsize                   g_atomic_pointer_xor                  (volatile void  *atomic,
                                                               gsize           val);

GLIB_DEPRECATED_IN_2_30_FOR(g_atomic_int_add)
gint                    g_atomic_int_exchange_and_add         (volatile gint  *atomic,
                                                               gint            val);

G_END_DECLS

#if defined(G_ATOMIC_LOCK_FREE) && defined(__GCC_HAVE_SYNC_COMPARE_AND_SWAP_4)

/* We prefer the new C11-style atomic extension of GCC if available */
#if defined(__ATOMIC_SEQ_CST)

#define g_atomic_int_get(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    gint gaig_temp;                                                          \
    (void) (0 ? *(atomic) ^ *(atomic) : 1);                                  \
    __atomic_load ((gint *)(atomic), &gaig_temp, __ATOMIC_SEQ_CST);          \
    (gint) gaig_temp;                                                        \
  }))
#define g_atomic_int_set(atomic, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    gint gais_temp = (gint) (newval);                                        \
    (void) (0 ? *(atomic) ^ (newval) : 1);                                   \
    __atomic_store ((gint *)(atomic), &gais_temp, __ATOMIC_SEQ_CST);         \
  }))

#if defined(g_has_typeof)
#define g_atomic_pointer_get(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    __typeof__(*(atomic)) gapg_temp_newval;                                  \
    __typeof__((atomic)) gapg_temp_atomic = (atomic);                        \
    __atomic_load (gapg_temp_atomic, &gapg_temp_newval, __ATOMIC_SEQ_CST);   \
    gapg_temp_newval;                                                        \
  }))
#define g_atomic_pointer_set(atomic, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    __typeof__((atomic)) gaps_temp_atomic = (atomic);                        \
    __typeof__(*(atomic)) gaps_temp_newval = (newval);                       \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    __atomic_store (gaps_temp_atomic, &gaps_temp_newval, __ATOMIC_SEQ_CST);  \
  }))
#else  /* if !defined(g_has_typeof) */
#define g_atomic_pointer_get(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    gpointer gapg_temp_newval;                                               \
    gpointer *gapg_temp_atomic = (gpointer *)(atomic);                       \
    __atomic_load (gapg_temp_atomic, &gapg_temp_newval, __ATOMIC_SEQ_CST);   \
    gapg_temp_newval;                                                        \
  }))
#define g_atomic_pointer_set(atomic, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    gpointer *gaps_temp_atomic = (gpointer *)(atomic);                       \
    gpointer gaps_temp_newval = (gpointer)(newval);                          \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    __atomic_store (gaps_temp_atomic, &gaps_temp_newval, __ATOMIC_SEQ_CST);  \
  }))
#endif  /* !defined(g_has_typeof) */

#define g_atomic_int_inc(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ *(atomic) : 1);                                  \
    (void) __atomic_fetch_add ((atomic), 1, __ATOMIC_SEQ_CST);               \
  }))
#define g_atomic_int_dec_and_test(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ *(atomic) : 1);                                  \
    __atomic_fetch_sub ((atomic), 1, __ATOMIC_SEQ_CST) == 1;                 \
  }))
#define g_atomic_int_compare_and_exchange(atomic, oldval, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    gint gaicae_oldval = (oldval);                                           \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (newval) ^ (oldval) : 1);                        \
    __atomic_compare_exchange_n ((atomic), &gaicae_oldval, (newval), FALSE, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST) ? TRUE : FALSE; \
  }))
#define g_atomic_int_add(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (gint) __atomic_fetch_add ((atomic), (val), __ATOMIC_SEQ_CST);           \
  }))
#define g_atomic_int_and(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (guint) __atomic_fetch_and ((atomic), (val), __ATOMIC_SEQ_CST);          \
  }))
#define g_atomic_int_or(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (guint) __atomic_fetch_or ((atomic), (val), __ATOMIC_SEQ_CST);           \
  }))
#define g_atomic_int_xor(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (guint) __atomic_fetch_xor ((atomic), (val), __ATOMIC_SEQ_CST);          \
  }))

#define g_atomic_pointer_compare_and_exchange(atomic, oldval, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof (oldval) == sizeof (gpointer));                  \
    __typeof__ ((oldval)) gapcae_oldval = (oldval);                          \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    __atomic_compare_exchange_n ((atomic), &gapcae_oldval, (newval), FALSE, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST) ? TRUE : FALSE; \
  }))
#define g_atomic_pointer_add(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gssize) __atomic_fetch_add ((atomic), (val), __ATOMIC_SEQ_CST);         \
  }))
#define g_atomic_pointer_and(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    volatile gsize *gapa_atomic = (volatile gsize *) (atomic);               \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gsize));                    \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gsize) __atomic_fetch_and (gapa_atomic, (val), __ATOMIC_SEQ_CST);       \
  }))
#define g_atomic_pointer_or(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    volatile gsize *gapo_atomic = (volatile gsize *) (atomic);               \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gsize));                    \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gsize) __atomic_fetch_or (gapo_atomic, (val), __ATOMIC_SEQ_CST);        \
  }))
#define g_atomic_pointer_xor(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    volatile gsize *gapx_atomic = (volatile gsize *) (atomic);               \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gsize));                    \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gsize) __atomic_fetch_xor (gapx_atomic, (val), __ATOMIC_SEQ_CST);       \
  }))

#else /* defined(__ATOMIC_SEQ_CST) */

/* We want to achieve __ATOMIC_SEQ_CST semantics here. See
 * https://en.cppreference.com/w/c/atomic/memory_order#Constants. For load
 * operations, that means performing an *acquire*:
 * > A load operation with this memory order performs the acquire operation on
 * > the affected memory location: no reads or writes in the current thread can
 * > be reordered before this load. All writes in other threads that release
 * > the same atomic variable are visible in the current thread.
 *
 * “no reads or writes in the current thread can be reordered before this load”
 * is implemented using a compiler barrier (a no-op `__asm__` section) to
 * prevent instruction reordering. Writes in other threads are synchronised
 * using `__sync_synchronize()`. It’s unclear from the GCC documentation whether
 * `__sync_synchronize()` acts as a compiler barrier, hence our explicit use of
 * one.
 *
 * For store operations, `__ATOMIC_SEQ_CST` means performing a *release*:
 * > A store operation with this memory order performs the release operation:
 * > no reads or writes in the current thread can be reordered after this store.
 * > All writes in the current thread are visible in other threads that acquire
 * > the same atomic variable (see Release-Acquire ordering below) and writes
 * > that carry a dependency into the atomic variable become visible in other
 * > threads that consume the same atomic (see Release-Consume ordering below).
 *
 * “no reads or writes in the current thread can be reordered after this store”
 * is implemented using a compiler barrier to prevent instruction reordering.
 * “All writes in the current thread are visible in other threads” is implemented
 * using `__sync_synchronize()`; similarly for “writes that carry a dependency”.
 */
#define g_atomic_int_get(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    gint gaig_result;                                                        \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ *(atomic) : 1);                                  \
    gaig_result = (gint) *(atomic);                                          \
    __sync_synchronize ();                                                   \
    __asm__ __volatile__ ("" : : : "memory");                                \
    gaig_result;                                                             \
  }))
#define g_atomic_int_set(atomic, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (newval) : 1);                                   \
    __sync_synchronize ();                                                   \
    __asm__ __volatile__ ("" : : : "memory");                                \
    *(atomic) = (newval);                                                    \
  }))
#define g_atomic_pointer_get(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    gpointer gapg_result;                                                    \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    gapg_result = (gpointer) *(atomic);                                      \
    __sync_synchronize ();                                                   \
    __asm__ __volatile__ ("" : : : "memory");                                \
    gapg_result;                                                             \
  }))
#define g_atomic_pointer_set(atomic, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    __sync_synchronize ();                                                   \
    __asm__ __volatile__ ("" : : : "memory");                                \
    *(atomic) = (__typeof__ (*(atomic))) (gsize) (newval);                   \
  }))

#define g_atomic_int_inc(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ *(atomic) : 1);                                  \
    (void) __sync_fetch_and_add ((atomic), 1);                               \
  }))
#define g_atomic_int_dec_and_test(atomic) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ *(atomic) : 1);                                  \
    __sync_fetch_and_sub ((atomic), 1) == 1;                                 \
  }))
#define g_atomic_int_compare_and_exchange(atomic, oldval, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (newval) ^ (oldval) : 1);                        \
    __sync_bool_compare_and_swap ((atomic), (oldval), (newval)) ? TRUE : FALSE; \
  }))
#define g_atomic_int_add(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (gint) __sync_fetch_and_add ((atomic), (val));                           \
  }))
#define g_atomic_int_and(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (guint) __sync_fetch_and_and ((atomic), (val));                          \
  }))
#define g_atomic_int_or(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (guint) __sync_fetch_and_or ((atomic), (val));                           \
  }))
#define g_atomic_int_xor(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gint));                     \
    (void) (0 ? *(atomic) ^ (val) : 1);                                      \
    (guint) __sync_fetch_and_xor ((atomic), (val));                          \
  }))

#define g_atomic_pointer_compare_and_exchange(atomic, oldval, newval) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    __sync_bool_compare_and_swap ((atomic), (oldval), (newval)) ? TRUE : FALSE; \
  }))
#define g_atomic_pointer_add(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gssize) __sync_fetch_and_add ((atomic), (val));                         \
  }))
#define g_atomic_pointer_and(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gsize) __sync_fetch_and_and ((atomic), (val));                          \
  }))
#define g_atomic_pointer_or(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gsize) __sync_fetch_and_or ((atomic), (val));                           \
  }))
#define g_atomic_pointer_xor(atomic, val) \
  (G_GNUC_EXTENSION ({                                                       \
    G_STATIC_ASSERT (sizeof *(atomic) == sizeof (gpointer));                 \
    (void) (0 ? (gpointer) *(atomic) : NULL);                                \
    (void) (0 ? (val) ^ (val) : 1);                                          \
    (gsize) __sync_fetch_and_xor ((atomic), (val));                          \
  }))

#endif /* !defined(__ATOMIC_SEQ_CST) */

#else /* defined(G_ATOMIC_LOCK_FREE) && defined(__GCC_HAVE_SYNC_COMPARE_AND_SWAP_4) */

#define g_atomic_int_get(atomic) \
  (g_atomic_int_get ((gint *) (atomic)))
#define g_atomic_int_set(atomic, newval) \
  (g_atomic_int_set ((gint *) (atomic), (gint) (newval)))
#define g_atomic_int_compare_and_exchange(atomic, oldval, newval) \
  (g_atomic_int_compare_and_exchange ((gint *) (atomic), (oldval), (newval)))
#define g_atomic_int_add(atomic, val) \
  (g_atomic_int_add ((gint *) (atomic), (val)))
#define g_atomic_int_and(atomic, val) \
  (g_atomic_int_and ((guint *) (atomic), (val)))
#define g_atomic_int_or(atomic, val) \
  (g_atomic_int_or ((guint *) (atomic), (val)))
#define g_atomic_int_xor(atomic, val) \
  (g_atomic_int_xor ((guint *) (atomic), (val)))
#define g_atomic_int_inc(atomic) \
  (g_atomic_int_inc ((gint *) (atomic)))
#define g_atomic_int_dec_and_test(atomic) \
  (g_atomic_int_dec_and_test ((gint *) (atomic)))

#define g_atomic_pointer_get(atomic) \
  (g_atomic_pointer_get (atomic))
#define g_atomic_pointer_set(atomic, newval) \
  (g_atomic_pointer_set ((atomic), (gpointer) (newval)))
#define g_atomic_pointer_compare_and_exchange(atomic, oldval, newval) \
  (g_atomic_pointer_compare_and_exchange ((atomic), (gpointer) (oldval), (gpointer) (newval)))
#define g_atomic_pointer_add(atomic, val) \
  (g_atomic_pointer_add ((atomic), (gssize) (val)))
#define g_atomic_pointer_and(atomic, val) \
  (g_atomic_pointer_and ((atomic), (gsize) (val)))
#define g_atomic_pointer_or(atomic, val) \
  (g_atomic_pointer_or ((atomic), (gsize) (val)))
#define g_atomic_pointer_xor(atomic, val) \
  (g_atomic_pointer_xor ((atomic), (gsize) (val)))

#endif /* defined(__GNUC__) && defined(G_ATOMIC_OP_USE_GCC_BUILTINS) */

#endif /* __G_ATOMIC_H__ */
