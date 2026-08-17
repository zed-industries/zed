// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

/*
 * Shared definitions for upb Lua modules.
 */

#ifndef UPB_LUA_UPB_H_
#define UPB_LUA_UPB_H_

#include "lauxlib.h"
#include "upb/message/message.h"
#include "upb/reflection/def.h"
#include "upb/reflection/message.h"

/* Lua changes its API in incompatible ways in every minor release.
 * This is some shim code to paper over the differences. */

#if LUA_VERSION_NUM == 501
#define lua_rawlen lua_objlen
#define lua_setuservalue(L, idx) lua_setfenv(L, idx)
#define lua_getuservalue(L, idx) lua_getfenv(L, idx)
#define lupb_setfuncs(L, l) luaL_register(L, NULL, l)
#elif LUA_VERSION_NUM >= 502 && LUA_VERSION_NUM <= 504
#define lupb_setfuncs(L, l) luaL_setfuncs(L, l, 0)
#else
#error Only Lua 5.1-5.4 are supported
#endif

/* Create a new userdata with the given type and |n| uservals, which are popped
 * from the stack to initialize the userdata. */
void* lupb_newuserdata(lua_State* L, size_t size, int n, const char* type);

#if LUA_VERSION_NUM < 504
/* Polyfills for this Lua 5.4 function.  Pushes userval |n| for the userdata at
 * |index|. */
int lua_setiuservalue(lua_State* L, int index, int n);
int lua_getiuservalue(lua_State* L, int index, int n);
#endif

/* Registers a type with the given name, methods, and metamethods. */
void lupb_register_type(lua_State* L, const char* name, const luaL_Reg* m,
                        const luaL_Reg* mm);

/* Checks the given upb_Status and throws a Lua error if it is not ok. */
void lupb_checkstatus(lua_State* L, upb_Status* s);

int luaopen_lupb(lua_State* L);

/* C <-> Lua value conversions. ***********************************************/

/* Custom check/push functions.  Unlike the Lua equivalents, they are pinned to
 * specific C types (instead of lua_Number, etc), and do not allow any implicit
 * conversion or data loss. */
int64_t lupb_checkint64(lua_State* L, int narg);
int32_t lupb_checkint32(lua_State* L, int narg);
uint64_t lupb_checkuint64(lua_State* L, int narg);
uint32_t lupb_checkuint32(lua_State* L, int narg);
double lupb_checkdouble(lua_State* L, int narg);
float lupb_checkfloat(lua_State* L, int narg);
bool lupb_checkbool(lua_State* L, int narg);
const char* lupb_checkstring(lua_State* L, int narg, size_t* len);
const char* lupb_checkname(lua_State* L, int narg);

void lupb_pushint64(lua_State* L, int64_t val);
void lupb_pushint32(lua_State* L, int32_t val);
void lupb_pushuint64(lua_State* L, uint64_t val);
void lupb_pushuint32(lua_State* L, uint32_t val);

/** From def.c. ***************************************************************/

const upb_MessageDef* lupb_MessageDef_check(lua_State* L, int narg);
const upb_EnumDef* lupb_EnumDef_check(lua_State* L, int narg);
const upb_FieldDef* lupb_FieldDef_check(lua_State* L, int narg);
upb_DefPool* lupb_DefPool_check(lua_State* L, int narg);
void lupb_MessageDef_pushsubmsgdef(lua_State* L, const upb_FieldDef* f);

void lupb_def_registertypes(lua_State* L);

/** From msg.c. ***************************************************************/

void lupb_pushmsgval(lua_State* L, int container, upb_CType type,
                     upb_MessageValue val);
int lupb_MessageDef_call(lua_State* L);
upb_Arena* lupb_Arena_pushnew(lua_State* L);

void lupb_msg_registertypes(lua_State* L);

#define lupb_assert(L, predicate) \
  if (!(predicate))               \
    luaL_error(L, "internal error: %s, %s:%d ", #predicate, __FILE__, __LINE__);

#define LUPB_UNUSED(var) (void)var

#if defined(__GNUC__) || defined(__clang__)
#define LUPB_UNREACHABLE()   \
  do {                       \
    assert(0);               \
    __builtin_unreachable(); \
  } while (0)
#else
#define LUPB_UNREACHABLE() \
  do {                     \
    assert(0);             \
  } while (0)
#endif

#endif /* UPB_LUA_UPB_H_ */
