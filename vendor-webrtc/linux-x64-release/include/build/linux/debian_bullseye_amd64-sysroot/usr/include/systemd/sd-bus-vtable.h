/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosdbusvtablehfoo
#define foosdbusvtablehfoo

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

typedef struct sd_bus_vtable sd_bus_vtable;

#include "sd-bus.h"

enum {
        _SD_BUS_VTABLE_START             = '<',
        _SD_BUS_VTABLE_END               = '>',
        _SD_BUS_VTABLE_METHOD            = 'M',
        _SD_BUS_VTABLE_SIGNAL            = 'S',
        _SD_BUS_VTABLE_PROPERTY          = 'P',
        _SD_BUS_VTABLE_WRITABLE_PROPERTY = 'W'
};

__extension__ enum {
        SD_BUS_VTABLE_DEPRECATED                   = 1ULL << 0,
        SD_BUS_VTABLE_HIDDEN                       = 1ULL << 1,
        SD_BUS_VTABLE_UNPRIVILEGED                 = 1ULL << 2,
        SD_BUS_VTABLE_METHOD_NO_REPLY              = 1ULL << 3,
        SD_BUS_VTABLE_PROPERTY_CONST               = 1ULL << 4,
        SD_BUS_VTABLE_PROPERTY_EMITS_CHANGE        = 1ULL << 5,
        SD_BUS_VTABLE_PROPERTY_EMITS_INVALIDATION  = 1ULL << 6,
        SD_BUS_VTABLE_PROPERTY_EXPLICIT            = 1ULL << 7,
        SD_BUS_VTABLE_SENSITIVE                    = 1ULL << 8, /* covers both directions: method call + reply */
        SD_BUS_VTABLE_ABSOLUTE_OFFSET              = 1ULL << 9,
        _SD_BUS_VTABLE_CAPABILITY_MASK             = 0xFFFFULL << 40
};

#define SD_BUS_VTABLE_CAPABILITY(x) ((uint64_t) (((x)+1) & 0xFFFF) << 40)

enum {
        _SD_BUS_VTABLE_PARAM_NAMES     = 1 << 0
};

extern const unsigned sd_bus_object_vtable_format;

/* Note: unused areas in the sd_bus_vtable[] array must be initialized to 0. The structure contains an embedded
 * union, and the compiler is NOT required to initialize the unused areas of the union when the rest of the
 * structure is initialized. Normally the array is defined as read-only data, in which case the linker places
 * it in the BSS section, which is always fully initialized, so this is not a concern. But if the array is
 * created on the stack or on the heap, care must be taken to initialize the unused areas, for examply by
 * first memsetting the whole region to zero before filling the data in. */

struct sd_bus_vtable {
        /* Please do not initialize this structure directly, use the
         * macros below instead */

        __extension__ uint8_t type:8;
        __extension__ uint64_t flags:56;
        union {
                struct {
                        size_t element_size;
                        uint64_t features;
                        const unsigned *vtable_format_reference;
                } start;
                struct {
                        /* This field exists only to make sure we have something to initialize in
                         * SD_BUS_VTABLE_END in a way that is both compatible with pedantic versions of C and
                         * C++. It's unused otherwise. */
                        size_t _reserved;
                } end;
                struct {
                        const char *member;
                        const char *signature;
                        const char *result;
                        sd_bus_message_handler_t handler;
                        size_t offset;
                        const char *names;
                } method;
                struct {
                        const char *member;
                        const char *signature;
                        const char *names;
                } signal;
                struct {
                        const char *member;
                        const char *signature;
                        sd_bus_property_get_t get;
                        sd_bus_property_set_t set;
                        size_t offset;
                } property;
        } x;
};

#define SD_BUS_VTABLE_START(_flags)                                     \
        {                                                               \
                .type = _SD_BUS_VTABLE_START,                           \
                .flags = _flags,                                        \
                .x = {                                                  \
                        .start = {                                      \
                                .element_size = sizeof(sd_bus_vtable),  \
                                .features = _SD_BUS_VTABLE_PARAM_NAMES, \
                                .vtable_format_reference = &sd_bus_object_vtable_format, \
                        },                                              \
                },                                                      \
        }

/* helper macro to format method and signal parameters, one at a time */
#define SD_BUS_PARAM(x) #x "\0"

#define SD_BUS_METHOD_WITH_NAMES_OFFSET(_member, _signature, _in_names, _result, _out_names, _handler, _offset, _flags)  \
        {                                                               \
                .type = _SD_BUS_VTABLE_METHOD,                          \
                .flags = _flags,                                        \
                .x = {                                                  \
                        .method = {                                     \
                                .member = _member,                      \
                                .signature = _signature,                \
                                .result = _result,                      \
                                .handler = _handler,                    \
                                .offset = _offset,                      \
                                .names = _in_names _out_names,          \
                        },                                              \
                },                                                      \
        }
#define SD_BUS_METHOD_WITH_OFFSET(_member, _signature, _result, _handler, _offset, _flags)   \
        SD_BUS_METHOD_WITH_NAMES_OFFSET(_member, _signature, "", _result, "", _handler, _offset, _flags)
#define SD_BUS_METHOD_WITH_NAMES(_member, _signature, _in_names, _result, _out_names, _handler, _flags)   \
        SD_BUS_METHOD_WITH_NAMES_OFFSET(_member, _signature, _in_names, _result, _out_names, _handler, 0, _flags)
#define SD_BUS_METHOD(_member, _signature, _result, _handler, _flags)   \
        SD_BUS_METHOD_WITH_NAMES_OFFSET(_member, _signature, "", _result, "", _handler, 0, _flags)

#define SD_BUS_SIGNAL_WITH_NAMES(_member, _signature, _out_names, _flags)                      \
        {                                                               \
                .type = _SD_BUS_VTABLE_SIGNAL,                          \
                .flags = _flags,                                        \
                .x = {                                                  \
                        .signal = {                                     \
                                .member = _member,                      \
                                .signature = _signature,                \
                                .names = _out_names,                    \
                        },                                              \
                },                                                      \
                        }
#define SD_BUS_SIGNAL(_member, _signature, _flags)                      \
        SD_BUS_SIGNAL_WITH_NAMES(_member, _signature, "", _flags)

#define SD_BUS_PROPERTY(_member, _signature, _get, _offset, _flags)     \
        {                                                               \
                .type = _SD_BUS_VTABLE_PROPERTY,                        \
                .flags = _flags,                                        \
                .x = {                                                  \
                        .property = {                                   \
                                .member = _member,                      \
                                .signature = _signature,                \
                                .get = _get,                            \
                                .set = NULL,                            \
                                .offset = _offset,                      \
                        },                                              \
                },                                                      \
        }

#define SD_BUS_WRITABLE_PROPERTY(_member, _signature, _get, _set, _offset, _flags) \
        {                                                               \
                .type = _SD_BUS_VTABLE_WRITABLE_PROPERTY,               \
                .flags = _flags,                                        \
                .x = {                                                  \
                        .property = {                                   \
                                .member = _member,                      \
                                .signature = _signature,                \
                                .get = _get,                            \
                                .set = _set,                            \
                                .offset = _offset,                      \
                        },                                              \
                },                                                      \
        }

#define SD_BUS_VTABLE_END                                               \
        {                                                               \
                .type = _SD_BUS_VTABLE_END,                             \
                .flags = 0,                                             \
                .x = {                                                  \
                        .end = {                                        \
                                ._reserved = 0,                         \
                        },                                              \
                },                                                      \
        }

#define _SD_ECHO(X) X
#define _SD_CONCAT(X) #X "\0"

#define _SD_VARARGS_FOREACH_SEQ(_01, _02, _03, _04, _05, _06, _07, _08, _09, _10, \
                                _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, \
                                _21, _22, _23, _24, _25, _26, _27, _28, _29, _30, \
                                _31, _32, _33, _34, _35, _36, _37, _38, _39, _40, \
                                _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, \
                                NAME, ...) NAME

#define _SD_VARARGS_FOREACH_EVEN_00(FN)
#define _SD_VARARGS_FOREACH_EVEN_01(FN, X)         FN(X)
#define _SD_VARARGS_FOREACH_EVEN_02(FN, X, Y)      FN(X)
#define _SD_VARARGS_FOREACH_EVEN_04(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_02(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_06(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_04(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_08(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_06(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_10(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_08(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_12(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_10(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_14(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_12(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_16(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_14(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_18(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_16(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_20(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_18(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_22(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_20(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_24(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_22(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_26(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_24(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_28(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_26(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_30(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_28(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_32(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_30(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_34(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_32(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_36(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_34(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_38(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_36(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_40(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_38(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_42(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_40(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_44(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_42(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_46(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_44(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_48(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_46(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_EVEN_50(FN, X, Y, ...) FN(X) _SD_VARARGS_FOREACH_EVEN_48(FN, __VA_ARGS__)

#define _SD_VARARGS_FOREACH_EVEN(FN, ...)                                                 \
        _SD_VARARGS_FOREACH_SEQ(__VA_ARGS__,                                              \
                                _SD_VARARGS_FOREACH_EVEN_50, _SD_VARARGS_FOREACH_EVEN_49, \
                                _SD_VARARGS_FOREACH_EVEN_48, _SD_VARARGS_FOREACH_EVEN_47, \
                                _SD_VARARGS_FOREACH_EVEN_46, _SD_VARARGS_FOREACH_EVEN_45, \
                                _SD_VARARGS_FOREACH_EVEN_44, _SD_VARARGS_FOREACH_EVEN_43, \
                                _SD_VARARGS_FOREACH_EVEN_42, _SD_VARARGS_FOREACH_EVEN_41, \
                                _SD_VARARGS_FOREACH_EVEN_40, _SD_VARARGS_FOREACH_EVEN_39, \
                                _SD_VARARGS_FOREACH_EVEN_38, _SD_VARARGS_FOREACH_EVEN_37, \
                                _SD_VARARGS_FOREACH_EVEN_36, _SD_VARARGS_FOREACH_EVEN_35, \
                                _SD_VARARGS_FOREACH_EVEN_34, _SD_VARARGS_FOREACH_EVEN_33, \
                                _SD_VARARGS_FOREACH_EVEN_32, _SD_VARARGS_FOREACH_EVEN_31, \
                                _SD_VARARGS_FOREACH_EVEN_30, _SD_VARARGS_FOREACH_EVEN_29, \
                                _SD_VARARGS_FOREACH_EVEN_28, _SD_VARARGS_FOREACH_EVEN_27, \
                                _SD_VARARGS_FOREACH_EVEN_26, _SD_VARARGS_FOREACH_EVEN_25, \
                                _SD_VARARGS_FOREACH_EVEN_24, _SD_VARARGS_FOREACH_EVEN_23, \
                                _SD_VARARGS_FOREACH_EVEN_22, _SD_VARARGS_FOREACH_EVEN_21, \
                                _SD_VARARGS_FOREACH_EVEN_20, _SD_VARARGS_FOREACH_EVEN_19, \
                                _SD_VARARGS_FOREACH_EVEN_18, _SD_VARARGS_FOREACH_EVEN_17, \
                                _SD_VARARGS_FOREACH_EVEN_16, _SD_VARARGS_FOREACH_EVEN_15, \
                                _SD_VARARGS_FOREACH_EVEN_14, _SD_VARARGS_FOREACH_EVEN_13, \
                                _SD_VARARGS_FOREACH_EVEN_12, _SD_VARARGS_FOREACH_EVEN_11, \
                                _SD_VARARGS_FOREACH_EVEN_10, _SD_VARARGS_FOREACH_EVEN_09, \
                                _SD_VARARGS_FOREACH_EVEN_08, _SD_VARARGS_FOREACH_EVEN_07, \
                                _SD_VARARGS_FOREACH_EVEN_06, _SD_VARARGS_FOREACH_EVEN_05, \
                                _SD_VARARGS_FOREACH_EVEN_04, _SD_VARARGS_FOREACH_EVEN_03, \
                                _SD_VARARGS_FOREACH_EVEN_02, _SD_VARARGS_FOREACH_EVEN_01, \
                                _SD_VARARGS_FOREACH_EVEN_00)                              \
                                (FN, __VA_ARGS__)

#define _SD_VARARGS_FOREACH_ODD_00(FN)
#define _SD_VARARGS_FOREACH_ODD_01(FN, X)
#define _SD_VARARGS_FOREACH_ODD_02(FN, X, Y)      FN(Y)
#define _SD_VARARGS_FOREACH_ODD_04(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_02(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_06(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_04(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_08(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_06(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_10(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_08(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_12(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_10(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_14(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_12(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_16(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_14(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_18(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_16(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_20(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_18(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_22(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_20(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_24(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_22(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_26(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_24(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_28(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_26(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_30(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_28(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_32(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_30(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_34(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_32(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_36(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_34(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_38(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_36(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_40(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_38(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_42(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_40(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_44(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_42(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_46(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_44(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_48(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_46(FN, __VA_ARGS__)
#define _SD_VARARGS_FOREACH_ODD_50(FN, X, Y, ...) FN(Y) _SD_VARARGS_FOREACH_ODD_48(FN, __VA_ARGS__)

#define _SD_VARARGS_FOREACH_ODD(FN, ...)                                                \
        _SD_VARARGS_FOREACH_SEQ(__VA_ARGS__,                                            \
                                _SD_VARARGS_FOREACH_ODD_50, _SD_VARARGS_FOREACH_ODD_49, \
                                _SD_VARARGS_FOREACH_ODD_48, _SD_VARARGS_FOREACH_ODD_47, \
                                _SD_VARARGS_FOREACH_ODD_46, _SD_VARARGS_FOREACH_ODD_45, \
                                _SD_VARARGS_FOREACH_ODD_44, _SD_VARARGS_FOREACH_ODD_43, \
                                _SD_VARARGS_FOREACH_ODD_42, _SD_VARARGS_FOREACH_ODD_41, \
                                _SD_VARARGS_FOREACH_ODD_40, _SD_VARARGS_FOREACH_ODD_39, \
                                _SD_VARARGS_FOREACH_ODD_38, _SD_VARARGS_FOREACH_ODD_37, \
                                _SD_VARARGS_FOREACH_ODD_36, _SD_VARARGS_FOREACH_ODD_35, \
                                _SD_VARARGS_FOREACH_ODD_34, _SD_VARARGS_FOREACH_ODD_33, \
                                _SD_VARARGS_FOREACH_ODD_32, _SD_VARARGS_FOREACH_ODD_31, \
                                _SD_VARARGS_FOREACH_ODD_30, _SD_VARARGS_FOREACH_ODD_29, \
                                _SD_VARARGS_FOREACH_ODD_28, _SD_VARARGS_FOREACH_ODD_27, \
                                _SD_VARARGS_FOREACH_ODD_26, _SD_VARARGS_FOREACH_ODD_25, \
                                _SD_VARARGS_FOREACH_ODD_24, _SD_VARARGS_FOREACH_ODD_23, \
                                _SD_VARARGS_FOREACH_ODD_22, _SD_VARARGS_FOREACH_ODD_21, \
                                _SD_VARARGS_FOREACH_ODD_20, _SD_VARARGS_FOREACH_ODD_19, \
                                _SD_VARARGS_FOREACH_ODD_18, _SD_VARARGS_FOREACH_ODD_17, \
                                _SD_VARARGS_FOREACH_ODD_16, _SD_VARARGS_FOREACH_ODD_15, \
                                _SD_VARARGS_FOREACH_ODD_14, _SD_VARARGS_FOREACH_ODD_13, \
                                _SD_VARARGS_FOREACH_ODD_12, _SD_VARARGS_FOREACH_ODD_11, \
                                _SD_VARARGS_FOREACH_ODD_10, _SD_VARARGS_FOREACH_ODD_09, \
                                _SD_VARARGS_FOREACH_ODD_08, _SD_VARARGS_FOREACH_ODD_07, \
                                _SD_VARARGS_FOREACH_ODD_06, _SD_VARARGS_FOREACH_ODD_05, \
                                _SD_VARARGS_FOREACH_ODD_04, _SD_VARARGS_FOREACH_ODD_03, \
                                _SD_VARARGS_FOREACH_ODD_02, _SD_VARARGS_FOREACH_ODD_01, \
                                _SD_VARARGS_FOREACH_ODD_00)                             \
                                (FN, __VA_ARGS__)

#define SD_BUS_ARGS(...) __VA_ARGS__
#define SD_BUS_RESULT(...) __VA_ARGS__

#define SD_BUS_NO_ARGS SD_BUS_ARGS(NULL)
#define SD_BUS_NO_RESULT SD_BUS_RESULT(NULL)

#define SD_BUS_METHOD_WITH_ARGS(_member, _args, _result, _handler, _flags)                 \
        SD_BUS_METHOD_WITH_NAMES(_member,                                                  \
                                 _SD_VARARGS_FOREACH_EVEN(_SD_ECHO, _args),                \
                                 _SD_VARARGS_FOREACH_ODD(_SD_CONCAT, _args),               \
                                 _SD_VARARGS_FOREACH_EVEN(_SD_ECHO, _result),              \
                                 _SD_VARARGS_FOREACH_ODD(_SD_CONCAT, _result) "\0",        \
                                 _handler, _flags)

#define SD_BUS_METHOD_WITH_ARGS_OFFSET(_member, _args, _result, _handler, _offset, _flags) \
        SD_BUS_METHOD_WITH_NAMES_OFFSET(_member,                                           \
                                        _SD_VARARGS_FOREACH_EVEN(_SD_ECHO, _args),         \
                                        _SD_VARARGS_FOREACH_ODD(_SD_CONCAT, _args),        \
                                        _SD_VARARGS_FOREACH_EVEN(_SD_ECHO, _result),       \
                                        _SD_VARARGS_FOREACH_ODD(_SD_CONCAT, _result) "\0", \
                                        _handler, _offset, _flags)

#define SD_BUS_SIGNAL_WITH_ARGS(_member, _args, _flags)                                    \
        SD_BUS_SIGNAL_WITH_NAMES(_member,                                                  \
                                 _SD_VARARGS_FOREACH_EVEN(_SD_ECHO, _args),                \
                                 _SD_VARARGS_FOREACH_ODD(_SD_CONCAT, _args) "\0",          \
                                 _flags)

_SD_END_DECLARATIONS;

#endif
