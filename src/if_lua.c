/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Lua interface by Luis Carvalho
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include "vim.h"
#include "version.h"

#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>

#if __STDC_VERSION__ >= 199901L
#  define LUAV_INLINE inline
#else
#  define LUAV_INLINE
#endif

// Only do the following when the feature is enabled.  Needed for "make
// depend".
#if defined(FEAT_LUA) || defined(PROTO)

#define LUAVIM_CHUNKNAME "vim chunk"
#define LUAVIM_NAME "vim"
#define LUAVIM_EVALNAME "luaeval"
#define LUAVIM_EVALHEADER "local _A=select(1,...) return "

#ifdef LUA_RELEASE
# define LUAVIM_VERSION LUA_RELEASE
#else
# define LUAVIM_VERSION LUA_VERSION
#endif

typedef buf_T *luaV_Buffer;
typedef win_T *luaV_Window;
typedef dict_T *luaV_Dict;
typedef list_T *luaV_List;
typedef blob_T *luaV_Blob;
typedef struct {
    char_u	*name;	// funcref
    dict_T	*self;	// selfdict
} luaV_Funcref;
typedef int (*msgfunc_T)(char *);

typedef struct {
    int lua_funcref;    // ref to a lua func
    int lua_tableref;   // ref to a lua table if metatable else LUA_NOREF. used
			// for __call
    lua_State *L;
} luaV_CFuncState;

static const char LUAVIM_DICT[] = "dict";
static const char LUAVIM_LIST[] = "list";
static const char LUAVIM_BLOB[] = "blob";
static const char LUAVIM_FUNCREF[] = "funcref";
static const char LUAVIM_BUFFER[] = "buffer";
static const char LUAVIM_WINDOW[] = "window";
static const char LUAVIM_FREE[] = "luaV_free";
static const char LUAVIM_LUAEVAL[] = "luaV_luaeval";
static const char LUAVIM_SETREF[] = "luaV_setref";

static const char LUA___CALL[] = "__call";

// get/setudata manage references to vim userdata in a cache table through
// object pointers (light userdata). The cache table itself is retrieved
// from the registry.

static const char LUAVIM_UDATA_CACHE[] = "luaV_udata_cache";

#define luaV_getfield(L, s) \
    lua_pushlightuserdata((L), (void *)(s)); \
    lua_rawget((L), LUA_REGISTRYINDEX)
#define luaV_checksandbox(L) \
    if (sandbox) luaL_error((L), "not allowed in sandbox")
#define luaV_msg(L) luaV_msgfunc((L), (msgfunc_T) msg)
#define luaV_emsg(L) luaV_msgfunc((L), (msgfunc_T) emsg)
#define luaV_checktypval(L, a, v, msg) \
    do { \
	if (luaV_totypval(L, a, v) == FAIL) \
	    luaL_error(L, msg ": cannot convert value"); \
    } while (0)

static luaV_List *luaV_pushlist(lua_State *L, list_T *lis);
static luaV_Dict *luaV_pushdict(lua_State *L, dict_T *dic);
static luaV_Blob *luaV_pushblob(lua_State *L, blob_T *blo);
static luaV_Funcref *luaV_pushfuncref(lua_State *L, char_u *name);
static int luaV_call_lua_func(int argcount, typval_T *argvars, typval_T *rettv, void *state);
static void luaV_call_lua_func_free(void *state);

#if LUA_VERSION_NUM <= 501
#define luaV_register(L, l) luaL_register(L, NULL, l)
#define luaL_typeerror luaL_typerror
#else
#define luaV_register(L, l) luaL_setfuncs(L, l, 0)
#endif

#ifdef DYNAMIC_LUA

#ifdef MSWIN
# define load_dll vimLoadLib
# define symbol_from_dll GetProcAddress
# define close_dll FreeLibrary
# define load_dll_error GetWin32Error
#else
# include <dlfcn.h>
# define HANDLE void*
# define load_dll(n) dlopen((n), RTLD_LAZY|RTLD_GLOBAL)
# define symbol_from_dll dlsym
# define close_dll dlclose
# define load_dll_error dlerror
#endif

// lauxlib
#if LUA_VERSION_NUM <= 501
#define luaL_register dll_luaL_register
#define luaL_prepbuffer dll_luaL_prepbuffer
#define luaL_openlib dll_luaL_openlib
#define luaL_typerror dll_luaL_typerror
#define luaL_loadfile dll_luaL_loadfile
#define luaL_loadbuffer dll_luaL_loadbuffer
#else
#define luaL_prepbuffsize dll_luaL_prepbuffsize
#define luaL_setfuncs dll_luaL_setfuncs
#define luaL_loadfilex dll_luaL_loadfilex
#define luaL_loadbufferx dll_luaL_loadbufferx
#define luaL_argerror dll_luaL_argerror
#endif
#if LUA_VERSION_NUM >= 504
#define luaL_typeerror dll_luaL_typeerror
#endif
#define luaL_checkany dll_luaL_checkany
#define luaL_checklstring dll_luaL_checklstring
#define luaL_checkinteger dll_luaL_checkinteger
#define luaL_optinteger dll_luaL_optinteger
#define luaL_checktype dll_luaL_checktype
#define luaL_error dll_luaL_error
#define luaL_newstate dll_luaL_newstate
#define luaL_buffinit dll_luaL_buffinit
#define luaL_addlstring dll_luaL_addlstring
#define luaL_pushresult dll_luaL_pushresult
#define luaL_loadstring dll_luaL_loadstring
#define luaL_ref dll_luaL_ref
#define luaL_unref dll_luaL_unref
// lua
#if LUA_VERSION_NUM <= 501
#define lua_tonumber dll_lua_tonumber
#define lua_tointeger dll_lua_tointeger
#define lua_call dll_lua_call
#define lua_pcall dll_lua_pcall
#else
#define lua_tonumberx dll_lua_tonumberx
#define lua_tointegerx dll_lua_tointegerx
#define lua_callk dll_lua_callk
#define lua_pcallk dll_lua_pcallk
#define lua_getglobal dll_lua_getglobal
#define lua_setglobal dll_lua_setglobal
#endif
#if LUA_VERSION_NUM <= 502
#define lua_replace dll_lua_replace
#define lua_remove dll_lua_remove
#endif
#if LUA_VERSION_NUM >= 503
#define lua_rotate dll_lua_rotate
#define lua_copy dll_lua_copy
#endif
#define lua_typename dll_lua_typename
#define lua_close dll_lua_close
#define lua_gettop dll_lua_gettop
#define lua_settop dll_lua_settop
#define lua_pushvalue dll_lua_pushvalue
#define lua_isnumber dll_lua_isnumber
#define lua_isstring dll_lua_isstring
#define lua_type dll_lua_type
#define lua_rawequal dll_lua_rawequal
#define lua_toboolean dll_lua_toboolean
#define lua_tolstring dll_lua_tolstring
#define lua_touserdata dll_lua_touserdata
#define lua_pushnil dll_lua_pushnil
#define lua_pushnumber dll_lua_pushnumber
#define lua_pushinteger dll_lua_pushinteger
#define lua_pushlstring dll_lua_pushlstring
#define lua_pushstring dll_lua_pushstring
#define lua_pushfstring dll_lua_pushfstring
#define lua_pushcclosure dll_lua_pushcclosure
#define lua_pushboolean dll_lua_pushboolean
#define lua_pushlightuserdata dll_lua_pushlightuserdata
#define lua_getfield dll_lua_getfield
#define lua_rawget dll_lua_rawget
#define lua_rawgeti dll_lua_rawgeti
#define lua_createtable dll_lua_createtable
#define lua_settable dll_lua_settable
#if LUA_VERSION_NUM >= 504
 #define lua_newuserdatauv dll_lua_newuserdatauv
#else
 #define lua_newuserdata dll_lua_newuserdata
#endif
#define lua_getmetatable dll_lua_getmetatable
#define lua_setfield dll_lua_setfield
#define lua_rawset dll_lua_rawset
#define lua_rawseti dll_lua_rawseti
#define lua_setmetatable dll_lua_setmetatable
#define lua_next dll_lua_next
// libs
#define luaopen_base dll_luaopen_base
#define luaopen_table dll_luaopen_table
#define luaopen_string dll_luaopen_string
#define luaopen_math dll_luaopen_math
#define luaopen_io dll_luaopen_io
#define luaopen_os dll_luaopen_os
#define luaopen_package dll_luaopen_package
#define luaopen_debug dll_luaopen_debug
#define luaL_openlibs dll_luaL_openlibs

// lauxlib
#if LUA_VERSION_NUM <= 501
void (*dll_luaL_register) (lua_State *L, const char *libname, const luaL_Reg *l);
char *(*dll_luaL_prepbuffer) (luaL_Buffer *B);
void (*dll_luaL_openlib) (lua_State *L, const char *libname, const luaL_Reg *l, int nup);
int (*dll_luaL_typerror) (lua_State *L, int narg, const char *tname);
int (*dll_luaL_loadfile) (lua_State *L, const char *filename);
int (*dll_luaL_loadbuffer) (lua_State *L, const char *buff, size_t sz, const char *name);
#else
char *(*dll_luaL_prepbuffsize) (luaL_Buffer *B, size_t sz);
void (*dll_luaL_setfuncs) (lua_State *L, const luaL_Reg *l, int nup);
int (*dll_luaL_loadfilex) (lua_State *L, const char *filename, const char *mode);
int (*dll_luaL_loadbufferx) (lua_State *L, const char *buff, size_t sz, const char *name, const char *mode);
int (*dll_luaL_argerror) (lua_State *L, int numarg, const char *extramsg);
#endif
#if LUA_VERSION_NUM >= 504
int (*dll_luaL_typeerror) (lua_State *L, int narg, const char *tname);
#endif
void (*dll_luaL_checkany) (lua_State *L, int narg);
const char *(*dll_luaL_checklstring) (lua_State *L, int numArg, size_t *l);
lua_Integer (*dll_luaL_checkinteger) (lua_State *L, int numArg);
lua_Integer (*dll_luaL_optinteger) (lua_State *L, int nArg, lua_Integer def);
void (*dll_luaL_checktype) (lua_State *L, int narg, int t);
int (*dll_luaL_error) (lua_State *L, const char *fmt, ...);
lua_State *(*dll_luaL_newstate) (void);
void (*dll_luaL_buffinit) (lua_State *L, luaL_Buffer *B);
void (*dll_luaL_addlstring) (luaL_Buffer *B, const char *s, size_t l);
void (*dll_luaL_pushresult) (luaL_Buffer *B);
int (*dll_luaL_loadstring) (lua_State *L, const char *s);
int (*dll_luaL_ref) (lua_State *L, int idx);
#if LUA_VERSION_NUM <= 502
void (*dll_luaL_unref) (lua_State *L, int idx, int n);
#else
void (*dll_luaL_unref) (lua_State *L, int idx, lua_Integer n);
#endif
// lua
#if LUA_VERSION_NUM <= 501
lua_Number (*dll_lua_tonumber) (lua_State *L, int idx);
lua_Integer (*dll_lua_tointeger) (lua_State *L, int idx);
void (*dll_lua_call) (lua_State *L, int nargs, int nresults);
int (*dll_lua_pcall) (lua_State *L, int nargs, int nresults, int errfunc);
#else
lua_Number (*dll_lua_tonumberx) (lua_State *L, int idx, int *isnum);
lua_Integer (*dll_lua_tointegerx) (lua_State *L, int idx, int *isnum);
void (*dll_lua_callk) (lua_State *L, int nargs, int nresults, int ctx,
	lua_CFunction k);
int (*dll_lua_pcallk) (lua_State *L, int nargs, int nresults, int errfunc,
	int ctx, lua_CFunction k);
void (*dll_lua_getglobal) (lua_State *L, const char *var);
void (*dll_lua_setglobal) (lua_State *L, const char *var);
#endif
#if LUA_VERSION_NUM <= 502
void (*dll_lua_replace) (lua_State *L, int idx);
void (*dll_lua_remove) (lua_State *L, int idx);
#endif
#if LUA_VERSION_NUM >= 503
void  (*dll_lua_rotate) (lua_State *L, int idx, int n);
void (*dll_lua_copy) (lua_State *L, int fromidx, int toidx);
#endif
const char *(*dll_lua_typename) (lua_State *L, int tp);
void       (*dll_lua_close) (lua_State *L);
int (*dll_lua_gettop) (lua_State *L);
void (*dll_lua_settop) (lua_State *L, int idx);
void (*dll_lua_pushvalue) (lua_State *L, int idx);
int (*dll_lua_isnumber) (lua_State *L, int idx);
int (*dll_lua_isstring) (lua_State *L, int idx);
int (*dll_lua_type) (lua_State *L, int idx);
int (*dll_lua_rawequal) (lua_State *L, int idx1, int idx2);
int (*dll_lua_toboolean) (lua_State *L, int idx);
const char *(*dll_lua_tolstring) (lua_State *L, int idx, size_t *len);
void *(*dll_lua_touserdata) (lua_State *L, int idx);
void (*dll_lua_pushnil) (lua_State *L);
void (*dll_lua_pushnumber) (lua_State *L, lua_Number n);
void (*dll_lua_pushinteger) (lua_State *L, lua_Integer n);
void (*dll_lua_pushlstring) (lua_State *L, const char *s, size_t l);
void (*dll_lua_pushstring) (lua_State *L, const char *s);
const char *(*dll_lua_pushfstring) (lua_State *L, const char *fmt, ...);
void (*dll_lua_pushcclosure) (lua_State *L, lua_CFunction fn, int n);
void (*dll_lua_pushboolean) (lua_State *L, int b);
void (*dll_lua_pushlightuserdata) (lua_State *L, void *p);
void (*dll_lua_getfield) (lua_State *L, int idx, const char *k);
#if LUA_VERSION_NUM <= 502
void (*dll_lua_rawget) (lua_State *L, int idx);
void (*dll_lua_rawgeti) (lua_State *L, int idx, int n);
#else
int (*dll_lua_rawget) (lua_State *L, int idx);
int (*dll_lua_rawgeti) (lua_State *L, int idx, lua_Integer n);
#endif
void (*dll_lua_createtable) (lua_State *L, int narr, int nrec);
void (*dll_lua_settable) (lua_State *L, int idx);
#if LUA_VERSION_NUM >= 504
void *(*dll_lua_newuserdatauv) (lua_State *L, size_t sz, int nuvalue);
#else
void *(*dll_lua_newuserdata) (lua_State *L, size_t sz);
#endif
int (*dll_lua_getmetatable) (lua_State *L, int objindex);
void (*dll_lua_setfield) (lua_State *L, int idx, const char *k);
void (*dll_lua_rawset) (lua_State *L, int idx);
#if LUA_VERSION_NUM <= 502
void (*dll_lua_rawseti) (lua_State *L, int idx, int n);
#else
void (*dll_lua_rawseti) (lua_State *L, int idx, lua_Integer n);
#endif
int (*dll_lua_setmetatable) (lua_State *L, int objindex);
int (*dll_lua_next) (lua_State *L, int idx);
// libs
int (*dll_luaopen_base) (lua_State *L);
int (*dll_luaopen_table) (lua_State *L);
int (*dll_luaopen_string) (lua_State *L);
int (*dll_luaopen_math) (lua_State *L);
int (*dll_luaopen_io) (lua_State *L);
int (*dll_luaopen_os) (lua_State *L);
int (*dll_luaopen_package) (lua_State *L);
int (*dll_luaopen_debug) (lua_State *L);
void (*dll_luaL_openlibs) (lua_State *L);

typedef void **luaV_function;
typedef struct {
    const char *name;
    luaV_function func;
} luaV_Reg;

static const luaV_Reg luaV_dll[] = {
    // lauxlib
#if LUA_VERSION_NUM <= 501
    {"luaL_register", (luaV_function) &dll_luaL_register},
    {"luaL_prepbuffer", (luaV_function) &dll_luaL_prepbuffer},
    {"luaL_openlib", (luaV_function) &dll_luaL_openlib},
    {"luaL_typerror", (luaV_function) &dll_luaL_typerror},
    {"luaL_loadfile", (luaV_function) &dll_luaL_loadfile},
    {"luaL_loadbuffer", (luaV_function) &dll_luaL_loadbuffer},
#else
    {"luaL_prepbuffsize", (luaV_function) &dll_luaL_prepbuffsize},
    {"luaL_setfuncs", (luaV_function) &dll_luaL_setfuncs},
    {"luaL_loadfilex", (luaV_function) &dll_luaL_loadfilex},
    {"luaL_loadbufferx", (luaV_function) &dll_luaL_loadbufferx},
    {"luaL_argerror", (luaV_function) &dll_luaL_argerror},
#endif
#if LUA_VERSION_NUM >= 504
    {"luaL_typeerror", (luaV_function) &dll_luaL_typeerror},
#endif
    {"luaL_checkany", (luaV_function) &dll_luaL_checkany},
    {"luaL_checklstring", (luaV_function) &dll_luaL_checklstring},
    {"luaL_checkinteger", (luaV_function) &dll_luaL_checkinteger},
    {"luaL_optinteger", (luaV_function) &dll_luaL_optinteger},
    {"luaL_checktype", (luaV_function) &dll_luaL_checktype},
    {"luaL_error", (luaV_function) &dll_luaL_error},
    {"luaL_newstate", (luaV_function) &dll_luaL_newstate},
    {"luaL_buffinit", (luaV_function) &dll_luaL_buffinit},
    {"luaL_addlstring", (luaV_function) &dll_luaL_addlstring},
    {"luaL_pushresult", (luaV_function) &dll_luaL_pushresult},
    {"luaL_loadstring", (luaV_function) &dll_luaL_loadstring},
    {"luaL_ref", (luaV_function) &dll_luaL_ref},
    {"luaL_unref", (luaV_function) &dll_luaL_unref},
    // lua
#if LUA_VERSION_NUM <= 501
    {"lua_tonumber", (luaV_function) &dll_lua_tonumber},
    {"lua_tointeger", (luaV_function) &dll_lua_tointeger},
    {"lua_call", (luaV_function) &dll_lua_call},
    {"lua_pcall", (luaV_function) &dll_lua_pcall},
#else
    {"lua_tonumberx", (luaV_function) &dll_lua_tonumberx},
    {"lua_tointegerx", (luaV_function) &dll_lua_tointegerx},
    {"lua_callk", (luaV_function) &dll_lua_callk},
    {"lua_pcallk", (luaV_function) &dll_lua_pcallk},
    {"lua_getglobal", (luaV_function) &dll_lua_getglobal},
    {"lua_setglobal", (luaV_function) &dll_lua_setglobal},
#endif
#if LUA_VERSION_NUM <= 502
    {"lua_replace", (luaV_function) &dll_lua_replace},
    {"lua_remove", (luaV_function) &dll_lua_remove},
#endif
#if LUA_VERSION_NUM >= 503
    {"lua_rotate", (luaV_function) &dll_lua_rotate},
    {"lua_copy", (luaV_function) &dll_lua_copy},
#endif
    {"lua_typename", (luaV_function) &dll_lua_typename},
    {"lua_close", (luaV_function) &dll_lua_close},
    {"lua_gettop", (luaV_function) &dll_lua_gettop},
    {"lua_settop", (luaV_function) &dll_lua_settop},
    {"lua_pushvalue", (luaV_function) &dll_lua_pushvalue},
    {"lua_isnumber", (luaV_function) &dll_lua_isnumber},
    {"lua_isstring", (luaV_function) &dll_lua_isstring},
    {"lua_type", (luaV_function) &dll_lua_type},
    {"lua_rawequal", (luaV_function) &dll_lua_rawequal},
    {"lua_toboolean", (luaV_function) &dll_lua_toboolean},
    {"lua_tolstring", (luaV_function) &dll_lua_tolstring},
    {"lua_touserdata", (luaV_function) &dll_lua_touserdata},
    {"lua_pushnil", (luaV_function) &dll_lua_pushnil},
    {"lua_pushnumber", (luaV_function) &dll_lua_pushnumber},
    {"lua_pushinteger", (luaV_function) &dll_lua_pushinteger},
    {"lua_pushlstring", (luaV_function) &dll_lua_pushlstring},
    {"lua_pushstring", (luaV_function) &dll_lua_pushstring},
    {"lua_pushfstring", (luaV_function) &dll_lua_pushfstring},
    {"lua_pushcclosure", (luaV_function) &dll_lua_pushcclosure},
    {"lua_pushboolean", (luaV_function) &dll_lua_pushboolean},
    {"lua_pushlightuserdata", (luaV_function) &dll_lua_pushlightuserdata},
    {"lua_getfield", (luaV_function) &dll_lua_getfield},
    {"lua_rawget", (luaV_function) &dll_lua_rawget},
    {"lua_rawgeti", (luaV_function) &dll_lua_rawgeti},
    {"lua_createtable", (luaV_function) &dll_lua_createtable},
    {"lua_settable", (luaV_function) &dll_lua_settable},
#if LUA_VERSION_NUM >= 504
    {"lua_newuserdatauv", (luaV_function) &dll_lua_newuserdatauv},
#else
    {"lua_newuserdata", (luaV_function) &dll_lua_newuserdata},
#endif
    {"lua_getmetatable", (luaV_function) &dll_lua_getmetatable},
    {"lua_setfield", (luaV_function) &dll_lua_setfield},
    {"lua_rawset", (luaV_function) &dll_lua_rawset},
    {"lua_rawseti", (luaV_function) &dll_lua_rawseti},
    {"lua_setmetatable", (luaV_function) &dll_lua_setmetatable},
    {"lua_next", (luaV_function) &dll_lua_next},
    // libs
    {"luaopen_base", (luaV_function) &dll_luaopen_base},
    {"luaopen_table", (luaV_function) &dll_luaopen_table},
    {"luaopen_string", (luaV_function) &dll_luaopen_string},
    {"luaopen_math", (luaV_function) &dll_luaopen_math},
    {"luaopen_io", (luaV_function) &dll_luaopen_io},
    {"luaopen_os", (luaV_function) &dll_luaopen_os},
    {"luaopen_package", (luaV_function) &dll_luaopen_package},
    {"luaopen_debug", (luaV_function) &dll_luaopen_debug},
    {"luaL_openlibs", (luaV_function) &dll_luaL_openlibs},
    {NULL, NULL}
};

static HANDLE hinstLua = NULL;

    static int
lua_link_init(char *libname, int verbose)
{
    const luaV_Reg *reg;
    if (hinstLua) return OK;
    hinstLua = load_dll(libname);
    if (!hinstLua)
    {
	if (verbose)
	    semsg(_(e_could_not_load_library_str_str), libname, load_dll_error());
	return FAIL;
    }
    for (reg = luaV_dll; reg->func; reg++)
    {
	if ((*reg->func = symbol_from_dll(hinstLua, reg->name)) == NULL)
	{
	    close_dll(hinstLua);
	    hinstLua = 0;
	    if (verbose)
		semsg(_(e_could_not_load_library_function_str), reg->name);
	    return FAIL;
	}
    }
    return OK;
}
#endif // DYNAMIC_LUA

#if defined(DYNAMIC_LUA) || defined(PROTO)
    int
lua_enabled(int verbose)
{
    return lua_link_init((char *)p_luadll, verbose) == OK;
}
#endif

#if LUA_VERSION_NUM > 501 && LUA_VERSION_NUM < 504
    static int
luaL_typeerror(lua_State *L, int narg, const char *tname)
{
    const char *msg = lua_pushfstring(L, "%s expected, got %s",
	    tname, luaL_typename(L, narg));
    return luaL_argerror(L, narg, msg);
}
#endif

    static LUAV_INLINE void
luaV_getudata(lua_State *L, void *v)
{
    lua_pushlightuserdata(L, (void *) LUAVIM_UDATA_CACHE);
    lua_rawget(L, LUA_REGISTRYINDEX);  // now the cache table is at the top of the stack
    lua_pushlightuserdata(L, v);
    lua_rawget(L, -2);
    lua_remove(L, -2);  // remove the cache table from the stack
}

    static LUAV_INLINE void
luaV_setudata(lua_State *L, void *v)
{
    lua_pushlightuserdata(L, (void *) LUAVIM_UDATA_CACHE);
    lua_rawget(L, LUA_REGISTRYINDEX);  // cache table is at -1
    lua_pushlightuserdata(L, v);       // ...now at -2
    lua_pushvalue(L, -3);	       // copy the userdata (cache at -3)
    lua_rawset(L, -3);		       // consumes two stack items
    lua_pop(L, 1);		       // and remove the cache table
}

// =======   Internal   =======

    static void
luaV_newmetatable(lua_State *L, const char *tname)
{
    lua_newtable(L);
    lua_pushlightuserdata(L, (void *) tname);
    lua_pushvalue(L, -2);
    lua_rawset(L, LUA_REGISTRYINDEX);
}

    static void *
luaV_toudata(lua_State *L, int ud, const char *tname)
{
    void *p = lua_touserdata(L, ud);

    if (p == NULL)
	return NULL;

    // value is userdata
    if (lua_getmetatable(L, ud)) // does it have a metatable?
    {
	luaV_getfield(L, tname); // get metatable
	if (lua_rawequal(L, -1, -2)) // MTs match?
	{
	    lua_pop(L, 2); // MTs
	    return p;
	}
    }

    return NULL;
}

    static void *
luaV_checkcache(lua_State *L, void *p)
{
    luaV_getudata(L, p);
    if (lua_isnil(L, -1)) luaL_error(L, "invalid object");
    lua_pop(L, 1);
    return p;
}

#define luaV_unbox(L,luatyp,ud) (*((luatyp *) lua_touserdata((L),(ud))))

#define luaV_checkvalid(L,luatyp,ud) \
    luaV_checkcache((L), (void *) luaV_unbox((L),luatyp,(ud)))

    static void *
luaV_checkudata(lua_State *L, int ud, const char *tname)
{
    void *p = luaV_toudata(L, ud, tname);
    if (p == NULL) luaL_typeerror(L, ud, tname);
    return p;
}

    static void
luaV_pushtypval(lua_State *L, typval_T *tv)
{
    if (tv == NULL)
    {
	lua_pushnil(L);
	return;
    }
    switch (tv->v_type)
    {
	case VAR_STRING:
	    lua_pushstring(L, tv->vval.v_string == NULL
					    ? "" : (char *)tv->vval.v_string);
	    break;
	case VAR_NUMBER:
	    lua_pushinteger(L, (int) tv->vval.v_number);
	    break;
	case VAR_FLOAT:
	    lua_pushnumber(L, (lua_Number) tv->vval.v_float);
	    break;
	case VAR_LIST:
	    luaV_pushlist(L, tv->vval.v_list);
	    break;
	case VAR_DICT:
	    luaV_pushdict(L, tv->vval.v_dict);
	    break;
	case VAR_BOOL:
	case VAR_SPECIAL:
	    if (tv->vval.v_number <= VVAL_TRUE)
		lua_pushinteger(L, (int) tv->vval.v_number);
	    else
		lua_pushnil(L);
	    break;
	case VAR_FUNC:
	    luaV_pushfuncref(L, tv->vval.v_string);
	    break;
	case VAR_PARTIAL:
	    // TODO: handle partial arguments
	    luaV_pushfuncref(L, partial_name(tv->vval.v_partial));
	    break;

	case VAR_BLOB:
	    luaV_pushblob(L, tv->vval.v_blob);
	    break;
	default:
	    lua_pushnil(L);
    }
}

/*
 * Converts lua value at 'pos' to typval 'tv'.
 * Returns OK or FAIL.
 */
    static int
luaV_totypval(lua_State *L, int pos, typval_T *tv)
{
    int status = OK;

    tv->v_lock = 0;

    switch (lua_type(L, pos))
    {
	case LUA_TBOOLEAN:
	    tv->v_type = VAR_BOOL;
	    tv->vval.v_number = (varnumber_T) lua_toboolean(L, pos);
	    break;
	case LUA_TNIL:
	    tv->v_type = VAR_SPECIAL;
	    tv->vval.v_number = VVAL_NULL;
	    break;
	case LUA_TSTRING:
	    tv->v_type = VAR_STRING;
	    tv->vval.v_string = vim_strsave((char_u *) lua_tostring(L, pos));
	    break;
	case LUA_TNUMBER:
	{
	    const lua_Number n = lua_tonumber(L, pos);

	    if (n > (lua_Number)INT64_MAX || n < (lua_Number)INT64_MIN
		    || ((lua_Number)((varnumber_T)n)) != n)
	    {
		tv->v_type = VAR_FLOAT;
		tv->vval.v_float = (float_T)n;
	    }
	    else
	    {
		tv->v_type = VAR_NUMBER;
		tv->vval.v_number = (varnumber_T)n;
	    }
	}
	    break;
	case LUA_TFUNCTION:
	{
	    char_u *name;
	    luaV_CFuncState *state;

	    lua_pushvalue(L, pos);
	    state = ALLOC_CLEAR_ONE(luaV_CFuncState);
	    state->lua_funcref = luaL_ref(L, LUA_REGISTRYINDEX);
	    state->L = L;
	    state->lua_tableref = LUA_NOREF;
	    name = register_cfunc(&luaV_call_lua_func,
					      &luaV_call_lua_func_free, state);
	    tv->v_type = VAR_FUNC;
	    tv->vval.v_string = vim_strsave(name);
	    break;
	}
	case LUA_TTABLE:
	{
	    int lua_tableref;

	    lua_pushvalue(L, pos);
	    lua_tableref = luaL_ref(L, LUA_REGISTRYINDEX);
	    if (lua_getmetatable(L, pos))
	    {
		lua_getfield(L, -1, LUA___CALL);
		if (lua_isfunction(L, -1))
		{
		    char_u *name;
		    int lua_funcref = luaL_ref(L, LUA_REGISTRYINDEX);
		    luaV_CFuncState *state = ALLOC_CLEAR_ONE(luaV_CFuncState);

		    state->lua_funcref = lua_funcref;
		    state->L = L;
		    state->lua_tableref = lua_tableref;
		    name = register_cfunc(&luaV_call_lua_func,
					      &luaV_call_lua_func_free, state);
		    tv->v_type = VAR_FUNC;
		    tv->vval.v_string = vim_strsave(name);
		    break;
		}
	    }
	    tv->v_type = VAR_NUMBER;
	    tv->vval.v_number = 0;
	    status = FAIL;
	    break;
	}
	case LUA_TUSERDATA:
	{
	    void *p = lua_touserdata(L, pos);

	    if (lua_getmetatable(L, pos)) // has metatable?
	    {
		// check list
		luaV_getfield(L, LUAVIM_LIST);
		if (lua_rawequal(L, -1, -2))
		{
		    tv->v_type = VAR_LIST;
		    tv->vval.v_list = *((luaV_List *) p);
		    ++tv->vval.v_list->lv_refcount;
		    lua_pop(L, 2); // MTs
		    break;
		}
		// check dict
		luaV_getfield(L, LUAVIM_DICT);
		if (lua_rawequal(L, -1, -3))
		{
		    tv->v_type = VAR_DICT;
		    tv->vval.v_dict = *((luaV_Dict *) p);
		    ++tv->vval.v_dict->dv_refcount;
		    lua_pop(L, 3); // MTs
		    break;
		}
		// check blob
		luaV_getfield(L, LUAVIM_BLOB);
		if (lua_rawequal(L, -1, -4))
		{
		    tv->v_type = VAR_BLOB;
		    tv->vval.v_blob = *((luaV_Blob *) p);
		    ++tv->vval.v_blob->bv_refcount;
		    lua_pop(L, 4); // MTs
		    break;
		}
		// check funcref
		luaV_getfield(L, LUAVIM_FUNCREF);
		if (lua_rawequal(L, -1, -5))
		{
		    luaV_Funcref *f = (luaV_Funcref *) p;

		    func_ref(f->name);
		    tv->v_type = VAR_FUNC;
		    tv->vval.v_string = vim_strsave(f->name);
		    lua_pop(L, 5); // MTs
		    break;
		}
		lua_pop(L, 4); // MTs
	    }
	}
	// FALLTHROUGH
	default:
	    tv->v_type = VAR_NUMBER;
	    tv->vval.v_number = 0;
	    status = FAIL;
    }
    return status;
}

/*
 * similar to luaL_addlstring, but replaces \0 with \n if toline and
 * \n with \0 otherwise
 */
    static void
luaV_addlstring(luaL_Buffer *b, const char *s, size_t l, int toline)
{
    while (l--)
    {
	if (*s == '\0' && toline)
	    luaL_addchar(b, '\n');
	else if (*s == '\n' && !toline)
	    luaL_addchar(b, '\0');
	else
	    luaL_addchar(b, *s);
	s++;
    }
}

    static void
luaV_pushline(lua_State *L, buf_T *buf, linenr_T n)
{
    const char *s = (const char *) ml_get_buf(buf, n, FALSE);
    luaL_Buffer b;
    luaL_buffinit(L, &b);
    luaV_addlstring(&b, s, strlen(s), 0);
    luaL_pushresult(&b);
}

    static char_u *
luaV_toline(lua_State *L, int pos)
{
    size_t l;
    const char *s = lua_tolstring(L, pos, &l);

    luaL_Buffer b;
    luaL_buffinit(L, &b);
    luaV_addlstring(&b, s, l, 1);
    luaL_pushresult(&b);
    return (char_u *) lua_tostring(L, -1);
}

/*
 * pops a string s from the top of the stack and calls mf(t) for pieces t of
 * s separated by newlines
 */
    static void
luaV_msgfunc(lua_State *L, msgfunc_T mf)
{
    luaL_Buffer b;
    size_t l;
    const char *p, *s = lua_tolstring(L, -1, &l);
    luaL_buffinit(L, &b);
    luaV_addlstring(&b, s, l, 0);
    luaL_pushresult(&b);
    // break string
    p = s = lua_tolstring(L, -1, &l);
    while (l--)
    {
	if (*p++ == '\0') // break?
	{
	    mf((char *)s);
	    s = p;
	}
    }
    mf((char *)s);
    lua_pop(L, 2); // original and modified strings
}

#define luaV_newtype(typ,tname,luatyp,luatname) \
	static luatyp * \
    luaV_new##tname(lua_State *L, typ *obj) \
    { \
	luatyp *o = (luatyp *) lua_newuserdata(L, sizeof(luatyp)); \
	*o = obj; \
	luaV_setudata(L, obj); /* cache[obj] = udata */ \
	luaV_getfield(L, luatname); \
	lua_setmetatable(L, -2); \
	return o; \
    }

#define luaV_pushtype(typ,tname,luatyp) \
	static luatyp * \
    luaV_push##tname(lua_State *L, typ *obj) \
    { \
	luatyp *o = NULL; \
	if (obj == NULL) \
	    lua_pushnil(L); \
	else \
	{ \
	    luaV_getudata(L, obj); \
	    if (lua_isnil(L, -1)) /* not interned? */ \
	    { \
		lua_pop(L, 1); \
		o = luaV_new##tname(L, obj); \
	    } \
	    else \
		o = (luatyp *) lua_touserdata(L, -1); \
	} \
	return o; \
    }

#define luaV_type_tostring(tname,luatname) \
	static int \
    luaV_##tname##_tostring(lua_State *L) \
    { \
	lua_pushfstring(L, "%s: %p", luatname, lua_touserdata(L, 1)); \
	return 1; \
    }

// =======   List type   =======

    static luaV_List *
luaV_newlist(lua_State *L, list_T *lis)
{
    luaV_List *l = (luaV_List *) lua_newuserdata(L, sizeof(luaV_List));
    *l = lis;
    lis->lv_refcount++; // reference in Lua
    luaV_setudata(L, lis); // cache[lis] = udata
    luaV_getfield(L, LUAVIM_LIST);
    lua_setmetatable(L, -2);
    return l;
}

luaV_pushtype(list_T, list, luaV_List)
luaV_type_tostring(list, LUAVIM_LIST)

    static int
luaV_list_len(lua_State *L)
{
    list_T *l = luaV_unbox(L, luaV_List, 1);
    lua_pushinteger(L, (int) list_len(l));
    return 1;
}

    static int
luaV_list_iter(lua_State *L)
{
    listitem_T *li = (listitem_T *) lua_touserdata(L, lua_upvalueindex(1));
    if (li == NULL) return 0;
    luaV_pushtypval(L, &li->li_tv);
    lua_pushlightuserdata(L, (void *) li->li_next);
    lua_replace(L, lua_upvalueindex(1));
    return 1;
}

    static int
luaV_list_call(lua_State *L)
{
    list_T *l = luaV_unbox(L, luaV_List, 1);
    lua_pushlightuserdata(L, (void *) l->lv_first);
    lua_pushcclosure(L, luaV_list_iter, 1);
    return 1;
}

    static int
luaV_list_index(lua_State *L)
{
    list_T *l = luaV_unbox(L, luaV_List, 1);
    if (lua_isnumber(L, 2)) // list item?
    {
	long n = (long) luaL_checkinteger(L, 2);
	listitem_T *li;

	// Lua array index starts with 1 while Vim uses 0, subtract 1 to
	// normalize.
	n -= 1;
	li = list_find(l, n);
	if (li == NULL)
	    lua_pushnil(L);
	else
	    luaV_pushtypval(L, &li->li_tv);
    }
    else if (lua_isstring(L, 2)) // method?
    {
	const char *s = lua_tostring(L, 2);
	if (strncmp(s, "add", 3) == 0
		|| strncmp(s, "insert", 6) == 0)
	{
	    lua_getmetatable(L, 1);
	    lua_getfield(L, -1, s);
	}
	else
	    lua_pushnil(L);
    }
    else
	lua_pushnil(L);
    return 1;
}

    static int
luaV_list_newindex(lua_State *L)
{
    list_T *l = luaV_unbox(L, luaV_List, 1);
    long n = (long) luaL_checkinteger(L, 2);
    listitem_T *li;

    // Lua array index starts with 1 while Vim uses 0, subtract 1 to normalize.
    n -= 1;

    if (l->lv_lock)
	luaL_error(L, "list is locked");
    li = list_find(l, n);
    if (li == NULL)
    {
	if (!lua_isnil(L, 3))
	{
	    typval_T v;
	    luaV_checktypval(L, 3, &v, "inserting list item");
	    if (list_insert_tv(l, &v, li) == FAIL)
		luaL_error(L, "failed to add item to list");
	    clear_tv(&v);
	}
    }
    else
    {
	if (lua_isnil(L, 3)) // remove?
	{
	    vimlist_remove(l, li, li);
	    listitem_free(l, li);
	}
	else
	{
	    typval_T v;
	    luaV_checktypval(L, 3, &v, "setting list item");
	    clear_tv(&li->li_tv);
	    li->li_tv = v;
	}
    }
    return 0;
}

    static int
luaV_list_add(lua_State *L)
{
    luaV_List *lis = luaV_checkudata(L, 1, LUAVIM_LIST);
    list_T *l = (list_T *) luaV_checkcache(L, (void *) *lis);
    typval_T v;
    if (l->lv_lock)
	luaL_error(L, "list is locked");
    lua_settop(L, 2);
    luaV_checktypval(L, 2, &v, "adding list item");
    if (list_append_tv(l, &v) == FAIL)
	luaL_error(L, "failed to add item to list");
    clear_tv(&v);
    lua_settop(L, 1);
    return 1;
}

    static int
luaV_list_insert(lua_State *L)
{
    luaV_List *lis = luaV_checkudata(L, 1, LUAVIM_LIST);
    list_T *l = (list_T *) luaV_checkcache(L, (void *) *lis);
    long pos = (long) luaL_optinteger(L, 3, 0);
    listitem_T *li = NULL;
    typval_T v;
    if (l->lv_lock)
	luaL_error(L, "list is locked");
    if (pos < l->lv_len)
    {
	li = list_find(l, pos);
	if (li == NULL)
	    luaL_error(L, "invalid position");
    }
    lua_settop(L, 2);
    luaV_checktypval(L, 2, &v, "inserting list item");
    if (list_insert_tv(l, &v, li) == FAIL)
	luaL_error(L, "failed to add item to list");
    clear_tv(&v);
    lua_settop(L, 1);
    return 1;
}

static const luaL_Reg luaV_List_mt[] = {
    {"__tostring", luaV_list_tostring},
    {"__len", luaV_list_len},
    {"__call", luaV_list_call},
    {"__index", luaV_list_index},
    {"__newindex", luaV_list_newindex},
    {"add", luaV_list_add},
    {"insert", luaV_list_insert},
    {NULL, NULL}
};


// =======   Dict type   =======

    static luaV_Dict *
luaV_newdict(lua_State *L, dict_T *dic)
{
    luaV_Dict *d = (luaV_Dict *) lua_newuserdata(L, sizeof(luaV_Dict));
    *d = dic;
    dic->dv_refcount++; // reference in Lua
    luaV_setudata(L, dic); // cache[dic] = udata
    luaV_getfield(L, LUAVIM_DICT);
    lua_setmetatable(L, -2);
    return d;
}

luaV_pushtype(dict_T, dict, luaV_Dict)
luaV_type_tostring(dict, LUAVIM_DICT)

    static int
luaV_dict_len(lua_State *L)
{
    dict_T *d = luaV_unbox(L, luaV_Dict, 1);
    lua_pushinteger(L, (int) dict_len(d));
    return 1;
}

    static int
luaV_dict_iter(lua_State *L UNUSED)
{
#ifdef FEAT_EVAL
    hashitem_T *hi = (hashitem_T *) lua_touserdata(L, lua_upvalueindex(1));
    int n = lua_tointeger(L, lua_upvalueindex(2));
    dictitem_T *di;
    if (n <= 0) return 0;
    while (HASHITEM_EMPTY(hi)) hi++;
    di = dict_lookup(hi);
    lua_pushstring(L, (char *) hi->hi_key);
    luaV_pushtypval(L, &di->di_tv);
    lua_pushlightuserdata(L, (void *) (hi + 1));
    lua_replace(L, lua_upvalueindex(1));
    lua_pushinteger(L, n - 1);
    lua_replace(L, lua_upvalueindex(2));
    return 2;
#else
    return 0;
#endif
}

    static int
luaV_dict_call(lua_State *L)
{
    dict_T *d = luaV_unbox(L, luaV_Dict, 1);
    hashtab_T *ht = &d->dv_hashtab;
    lua_pushlightuserdata(L, (void *) ht->ht_array);
    lua_pushinteger(L, ht->ht_used); // # remaining items
    lua_pushcclosure(L, luaV_dict_iter, 2);
    return 1;
}

    static int
luaV_dict_index(lua_State *L)
{
    dict_T *d = luaV_unbox(L, luaV_Dict, 1);
    char_u *key = (char_u *) luaL_checkstring(L, 2);
    dictitem_T *di = dict_find(d, key, -1);

    if (di == NULL)
    {
	lua_pushnil(L);
	return 1;
    }

    luaV_pushtypval(L, &di->di_tv);
    if (di->di_tv.v_type == VAR_FUNC) // funcref?
    {
	luaV_Funcref *f = (luaV_Funcref *) lua_touserdata(L, -1);
	f->self = d; // keep "self" reference
	d->dv_refcount++;
    }

    return 1;
}

    static int
luaV_dict_newindex(lua_State *L)
{
    dict_T *d = luaV_unbox(L, luaV_Dict, 1);
    char_u *key = (char_u *) luaL_checkstring(L, 2);
    dictitem_T *di;
    typval_T tv;

    if (d->dv_lock)
	luaL_error(L, "dict is locked");
    if (key == NULL)
	return 0;
    if (*key == NUL)
	luaL_error(L, "empty key");
    if (!lua_isnil(L, 3)) // read value?
    {
	luaV_checktypval(L, 3, &tv, "setting dict item");
	if (d->dv_scope == VAR_DEF_SCOPE && tv.v_type == VAR_FUNC)
	{
	    clear_tv(&tv);
	    luaL_error(L, "cannot assign funcref to builtin scope");
	}
    }
    di = dict_find(d, key, -1);
    if (di == NULL) // non-existing key?
    {
	if (lua_isnil(L, 3))
	    return 0;
	di = dictitem_alloc(key);
	if (di == NULL)
	{
	    clear_tv(&tv);
	    return 0;
	}
	if (dict_add(d, di) == FAIL)
	{
	    vim_free(di);
	    clear_tv(&tv);
	    return 0;
	}
    }
    else
	clear_tv(&di->di_tv);
    if (lua_isnil(L, 3)) // remove?
    {
	hashitem_T *hi = hash_find(&d->dv_hashtab, di->di_key);
	hash_remove(&d->dv_hashtab, hi, "Lua new index");
	dictitem_free(di);
    }
    else
	di->di_tv = tv;
    return 0;
}

static const luaL_Reg luaV_Dict_mt[] = {
    {"__tostring", luaV_dict_tostring},
    {"__len", luaV_dict_len},
    {"__call", luaV_dict_call},
    {"__index", luaV_dict_index},
    {"__newindex", luaV_dict_newindex},
    {NULL, NULL}
};


// =======   Blob type   =======

    static luaV_Blob *
luaV_newblob(lua_State *L, blob_T *blo)
{
    luaV_Blob *b = (luaV_Blob *) lua_newuserdata(L, sizeof(luaV_Blob));
    *b = blo;
    blo->bv_refcount++; // reference in Lua
    luaV_setudata(L, blo); // cache[blo] = udata
    luaV_getfield(L, LUAVIM_BLOB);
    lua_setmetatable(L, -2);
    return b;
}

luaV_pushtype(blob_T, blob, luaV_Blob)
luaV_type_tostring(blob, LUAVIM_BLOB)

    static int
luaV_blob_gc(lua_State *L)
{
    blob_T *b = luaV_unbox(L, luaV_Blob, 1);
    blob_unref(b);
    return 0;
}

    static int
luaV_blob_len(lua_State *L)
{
    blob_T *b = luaV_unbox(L, luaV_Blob, 1);
    lua_pushinteger(L, (int) blob_len(b));
    return 1;
}

    static int
luaV_blob_index(lua_State *L)
{
    blob_T *b = luaV_unbox(L, luaV_Blob, 1);
    if (lua_isnumber(L, 2))
    {
	int idx = luaL_checkinteger(L, 2);
	if (idx < blob_len(b))
	    lua_pushnumber(L, (lua_Number) blob_get(b, idx));
	else
	    lua_pushnil(L);
    }
    else if (lua_isstring(L, 2))
    {
	const char *s = lua_tostring(L, 2);
	if (strncmp(s, "add", 3) == 0)
	{
	    lua_getmetatable(L, 1);
	    lua_getfield(L, -1, s);
	}
	else
	    lua_pushnil(L);
    }
    else
	lua_pushnil(L);
    return 1;
}

    static int
luaV_blob_newindex(lua_State *L)
{
    blob_T *b = luaV_unbox(L, luaV_Blob, 1);
    if (b->bv_lock)
	luaL_error(L, "blob is locked");

    if (!lua_isnumber(L, 2))
	return 0;

    long len = blob_len(b);
    int idx = luaL_checkinteger(L, 2);
    int val = luaL_checkinteger(L, 3);
    if (idx < len || (idx == len && ga_grow(&b->bv_ga, 1) == OK))
    {
	blob_set(b, idx, (char_u) val);
	if (idx == len)
	    ++b->bv_ga.ga_len;
    }
    else
	luaL_error(L, "index out of range");

    return 0;
}

    static int
luaV_blob_add(lua_State *L)
{
    luaV_Blob *blo = luaV_checkudata(L, 1, LUAVIM_BLOB);
    blob_T *b = (blob_T *) luaV_checkcache(L, (void *) *blo);
    if (b->bv_lock)
	luaL_error(L, "blob is locked");
    lua_settop(L, 2);
    if (!lua_isstring(L, 2))
	luaL_error(L, "string expected, got %s", luaL_typename(L, 2));
    else
    {
	size_t i, l = 0;
	const char *s = lua_tolstring(L, 2, &l);

	if (ga_grow(&b->bv_ga, (int)l) == OK)
	    for (i = 0; i < l; ++i)
		ga_append(&b->bv_ga, s[i]);
    }
    lua_settop(L, 1);
    return 1;
}

static const luaL_Reg luaV_Blob_mt[] = {
    {"__tostring", luaV_blob_tostring},
    {"__gc", luaV_blob_gc},
    {"__len", luaV_blob_len},
    {"__index", luaV_blob_index},
    {"__newindex", luaV_blob_newindex},
    {"add", luaV_blob_add},
    {NULL, NULL}
};


// =======   Funcref type   =======

    static luaV_Funcref *
luaV_newfuncref(lua_State *L, char_u *name)
{
    luaV_Funcref *f = (luaV_Funcref *)lua_newuserdata(L, sizeof(luaV_Funcref));

    if (name != NULL)
    {
	func_ref(name);
	f->name = vim_strsave(name);
    }
    f->self = NULL;
    luaV_getfield(L, LUAVIM_FUNCREF);
    lua_setmetatable(L, -2);
    return f;
}

    static luaV_Funcref *
luaV_pushfuncref(lua_State *L, char_u *name)
{
    return luaV_newfuncref(L, name);
}


luaV_type_tostring(funcref, LUAVIM_FUNCREF)

    static int
luaV_funcref_gc(lua_State *L)
{
    luaV_Funcref *f = (luaV_Funcref *) lua_touserdata(L, 1);

    func_unref(f->name);
    vim_free(f->name);
    // NOTE: Don't call "dict_unref(f->self)", because the dict of "f->self"
    // will be (or has been already) freed by Vim's garbage collection.
    return 0;
}

// equivalent to string(funcref)
    static int
luaV_funcref_len(lua_State *L)
{
    luaV_Funcref *f = (luaV_Funcref *) lua_touserdata(L, 1);

    lua_pushstring(L, (const char *) f->name);
    return 1;
}

    static int
luaV_funcref_call(lua_State *L)
{
    luaV_Funcref *f = (luaV_Funcref *) lua_touserdata(L, 1);
    int i, n = lua_gettop(L) - 1; // #args
    int status = FAIL;
    typval_T args;
    typval_T rettv;

    args.v_type = VAR_LIST;
    args.vval.v_list = list_alloc();
    rettv.v_type = VAR_UNKNOWN; // as in clear_tv
    if (args.vval.v_list != NULL)
    {
	typval_T v;

	for (i = 0; i < n; i++)
	{
	    luaV_checktypval(L, i + 2, &v, "calling funcref");
	    list_append_tv(args.vval.v_list, &v);
	    clear_tv(&v);
	}
	status = func_call(f->name, &args, NULL, f->self, &rettv);
	if (status == OK)
	    luaV_pushtypval(L, &rettv);
	clear_tv(&args);
	clear_tv(&rettv);
    }
    if (status != OK)
	luaL_error(L, "cannot call funcref");
    return 1;
}

static const luaL_Reg luaV_Funcref_mt[] = {
    {"__tostring", luaV_funcref_tostring},
    {"__gc", luaV_funcref_gc},
    {"__len", luaV_funcref_len},
    {"__call", luaV_funcref_call},
    {NULL, NULL}
};


// =======   Buffer type   =======

luaV_newtype(buf_T, buffer, luaV_Buffer, LUAVIM_BUFFER)
luaV_pushtype(buf_T, buffer, luaV_Buffer)
luaV_type_tostring(buffer, LUAVIM_BUFFER)

    static int
luaV_buffer_len(lua_State *L)
{
    buf_T *b = (buf_T *) luaV_checkvalid(L, luaV_Buffer, 1);
    lua_pushinteger(L, b->b_ml.ml_line_count);
    return 1;
}

    static int
luaV_buffer_call(lua_State *L)
{
    buf_T *b = (buf_T *) luaV_checkvalid(L, luaV_Buffer, 1);
    lua_settop(L, 1);
    set_curbuf(b, DOBUF_SPLIT);
    return 1;
}

    static int
luaV_buffer_index(lua_State *L)
{
    buf_T *b = (buf_T *) luaV_checkvalid(L, luaV_Buffer, 1);
    linenr_T n = (linenr_T) lua_tointeger(L, 2);
    if (n > 0 && n <= b->b_ml.ml_line_count)
	luaV_pushline(L, b, n);
    else if (lua_isstring(L, 2))
    {
	const char *s = lua_tostring(L, 2);
	if (strncmp(s, "name", 4) == 0)
	    lua_pushstring(L, (b->b_sfname == NULL)
					? "" : (char *) b->b_sfname);
	else if (strncmp(s, "fname", 5) == 0)
	    lua_pushstring(L, (b->b_ffname == NULL)
					? "" : (char *) b->b_ffname);
	else if (strncmp(s, "number", 6) == 0)
	    lua_pushinteger(L, b->b_fnum);
	// methods
	else if (strncmp(s,   "insert", 6) == 0
		|| strncmp(s, "next", 4) == 0
		|| strncmp(s, "previous", 8) == 0
		|| strncmp(s, "isvalid", 7) == 0)
	{
	    lua_getmetatable(L, 1);
	    lua_getfield(L, -1, s);
	}
	else
	    lua_pushnil(L);
    }
    else
	lua_pushnil(L);
    return 1;
}

    static int
luaV_buffer_newindex(lua_State *L)
{
    buf_T *b = (buf_T *) luaV_checkvalid(L, luaV_Buffer, 1);
    linenr_T n = (linenr_T) luaL_checkinteger(L, 2);
#ifdef HAVE_SANDBOX
    luaV_checksandbox(L);
#endif
    if (n < 1 || n > b->b_ml.ml_line_count)
	luaL_error(L, "invalid line number");
    if (lua_isnil(L, 3)) // delete line
    {
	buf_T *buf = curbuf;
	curbuf = b;
	if (u_savedel(n, 1L) == FAIL)
	{
	    curbuf = buf;
	    luaL_error(L, "cannot save undo information");
	}
	else if (ml_delete(n) == FAIL)
	{
	    curbuf = buf;
	    luaL_error(L, "cannot delete line");
	}
	else
	{
	    deleted_lines_mark(n, 1L);
	    if (b == curwin->w_buffer) // fix cursor in current window?
	    {
		if (curwin->w_cursor.lnum >= n)
		{
		    if (curwin->w_cursor.lnum > n)
		    {
			curwin->w_cursor.lnum -= 1;
			check_cursor_col();
		    }
		    else
			check_cursor();
		    changed_cline_bef_curs();
		}
		invalidate_botline();
	    }
	}
	curbuf = buf;
    }
    else if (lua_isstring(L, 3)) // update line
    {
	buf_T *buf = curbuf;
	curbuf = b;
	if (u_savesub(n) == FAIL)
	{
	    curbuf = buf;
	    luaL_error(L, "cannot save undo information");
	}
	else if (ml_replace(n, luaV_toline(L, 3), TRUE) == FAIL)
	{
	    curbuf = buf;
	    luaL_error(L, "cannot replace line");
	}
	else
	    changed_bytes(n, 0);
	curbuf = buf;
	if (b == curwin->w_buffer)
	    check_cursor_col();
    }
    else
	luaL_error(L, "wrong argument to change line");
    return 0;
}

    static int
luaV_buffer_insert(lua_State *L)
{
    luaV_Buffer *lb = luaV_checkudata(L, 1, LUAVIM_BUFFER);
    buf_T *b = (buf_T *) luaV_checkcache(L, (void *) *lb);
    linenr_T last = b->b_ml.ml_line_count;
    linenr_T n = (linenr_T) luaL_optinteger(L, 3, last);
    buf_T *buf;
    luaL_checktype(L, 2, LUA_TSTRING);
#ifdef HAVE_SANDBOX
    luaV_checksandbox(L);
#endif
    // fix insertion line
    if (n < 0) n = 0;
    if (n > last) n = last;
    // insert
    buf = curbuf;
    curbuf = b;
    if (u_save(n, n + 1) == FAIL)
    {
	curbuf = buf;
	luaL_error(L, "cannot save undo information");
    }
    else if (ml_append(n, luaV_toline(L, 2), 0, FALSE) == FAIL)
    {
	curbuf = buf;
	luaL_error(L, "cannot insert line");
    }
    else
	appended_lines_mark(n, 1L);
    curbuf = buf;
    update_screen(UPD_VALID);
    return 0;
}

    static int
luaV_buffer_next(lua_State *L)
{
    luaV_Buffer *b = luaV_checkudata(L, 1, LUAVIM_BUFFER);
    buf_T *buf = (buf_T *) luaV_checkcache(L, (void *) *b);
    luaV_pushbuffer(L, buf->b_next);
    return 1;
}

    static int
luaV_buffer_previous(lua_State *L)
{
    luaV_Buffer *b = luaV_checkudata(L, 1, LUAVIM_BUFFER);
    buf_T *buf = (buf_T *) luaV_checkcache(L, (void *) *b);
    luaV_pushbuffer(L, buf->b_prev);
    return 1;
}

    static int
luaV_buffer_isvalid(lua_State *L)
{
    luaV_Buffer *b = luaV_checkudata(L, 1, LUAVIM_BUFFER);
    luaV_getudata(L, *b);
    lua_pushboolean(L, !lua_isnil(L, -1));
    return 1;
}

static const luaL_Reg luaV_Buffer_mt[] = {
    {"__tostring", luaV_buffer_tostring},
    {"__len", luaV_buffer_len},
    {"__call", luaV_buffer_call},
    {"__index", luaV_buffer_index},
    {"__newindex", luaV_buffer_newindex},
    {"insert", luaV_buffer_insert},
    {"next", luaV_buffer_next},
    {"previous", luaV_buffer_previous},
    {"isvalid", luaV_buffer_isvalid},
    {NULL, NULL}
};


// =======   Window type   =======

luaV_newtype(win_T, window, luaV_Window, LUAVIM_WINDOW)
luaV_pushtype(win_T, window, luaV_Window)
luaV_type_tostring(window, LUAVIM_WINDOW)

    static int
luaV_window_call(lua_State *L)
{
    win_T *w = (win_T *) luaV_checkvalid(L, luaV_Window, 1);
    lua_settop(L, 1);
    win_goto(w);
    return 1;
}

    static int
luaV_window_index(lua_State *L)
{
    win_T *w = (win_T *) luaV_checkvalid(L, luaV_Window, 1);
    const char *s = luaL_checkstring(L, 2);
    if (strncmp(s, "buffer", 6) == 0)
	luaV_pushbuffer(L, w->w_buffer);
    else if (strncmp(s, "line", 4) == 0)
	lua_pushinteger(L, w->w_cursor.lnum);
    else if (strncmp(s, "col", 3) == 0)
	lua_pushinteger(L, w->w_cursor.col + 1);
    else if (strncmp(s, "width", 5) == 0)
	lua_pushinteger(L, w->w_width);
    else if (strncmp(s, "height", 6) == 0)
	lua_pushinteger(L, w->w_height);
    // methods
    else if (strncmp(s,   "next", 4) == 0
	    || strncmp(s, "previous", 8) == 0
	    || strncmp(s, "isvalid", 7) == 0)
    {
	lua_getmetatable(L, 1);
	lua_getfield(L, -1, s);
    }
    else
	lua_pushnil(L);
    return 1;
}

    static int
luaV_window_newindex(lua_State *L)
{
    win_T *w = (win_T *) luaV_checkvalid(L, luaV_Window, 1);
    const char *s = luaL_checkstring(L, 2);
    int v = luaL_checkinteger(L, 3);
    if (strncmp(s, "line", 4) == 0)
    {
#ifdef HAVE_SANDBOX
	luaV_checksandbox(L);
#endif
	if (v < 1 || v > w->w_buffer->b_ml.ml_line_count)
	    luaL_error(L, "line out of range");
	w->w_cursor.lnum = v;
	update_screen(UPD_VALID);
    }
    else if (strncmp(s, "col", 3) == 0)
    {
#ifdef HAVE_SANDBOX
	luaV_checksandbox(L);
#endif
	w->w_cursor.col = v - 1;
	w->w_set_curswant = TRUE;
	update_screen(UPD_VALID);
    }
    else if (strncmp(s, "width", 5) == 0)
    {
	win_T *win = curwin;
#ifdef FEAT_GUI
	need_mouse_correct = TRUE;
#endif
	curwin = w;
	win_setwidth(v);
	curwin = win;
    }
    else if (strncmp(s, "height", 6) == 0)
    {
	win_T *win = curwin;
#ifdef FEAT_GUI
	need_mouse_correct = TRUE;
#endif
	curwin = w;
	win_setheight(v);
	curwin = win;
    }
    else
	luaL_error(L, "invalid window property: `%s'", s);
    return 0;
}

    static int
luaV_window_next(lua_State *L)
{
    luaV_Window *w = luaV_checkudata(L, 1, LUAVIM_WINDOW);
    win_T *win = (win_T *) luaV_checkcache(L, (void *) *w);
    luaV_pushwindow(L, win->w_next);
    return 1;
}

    static int
luaV_window_previous(lua_State *L)
{
    luaV_Window *w = luaV_checkudata(L, 1, LUAVIM_WINDOW);
    win_T *win = (win_T *) luaV_checkcache(L, (void *) *w);
    luaV_pushwindow(L, win->w_prev);
    return 1;
}

    static int
luaV_window_isvalid(lua_State *L)
{
    luaV_Window *w = luaV_checkudata(L, 1, LUAVIM_WINDOW);
    luaV_getudata(L, *w);
    lua_pushboolean(L, !lua_isnil(L, -1));
    return 1;
}

static const luaL_Reg luaV_Window_mt[] = {
    {"__tostring", luaV_window_tostring},
    {"__call", luaV_window_call},
    {"__index", luaV_window_index},
    {"__newindex", luaV_window_newindex},
    {"next", luaV_window_next},
    {"previous", luaV_window_previous},
    {"isvalid", luaV_window_isvalid},
    {NULL, NULL}
};


// =======   Vim module   =======

    static int
luaV_print(lua_State *L)
{
    int		i, n = lua_gettop(L); // nargs
    const char	*s;
    size_t	l;
    garray_T	msg_ga;

    ga_init2(&msg_ga, 1, 128);
    lua_getglobal(L, "tostring");
    for (i = 1; i <= n; i++)
    {
	lua_pushvalue(L, -1); // tostring
	lua_pushvalue(L, i); // arg
	lua_call(L, 1, 1);
	s = lua_tolstring(L, -1, &l);
	if (s == NULL)
	    return luaL_error(L, "cannot convert to string");
	if (i > 1)
	    ga_append(&msg_ga, ' '); // use space instead of tab
	ga_concat_len(&msg_ga, (char_u *)s, l);
	lua_pop(L, 1);
    }
    // Replace any "\n" with "\0"
    for (i = 0; i < msg_ga.ga_len; i++)
	if (((char *)msg_ga.ga_data)[i] == '\n')
	    ((char *)msg_ga.ga_data)[i] = '\0';
    lua_pushlstring(L, msg_ga.ga_data, msg_ga.ga_len);
    if (!got_int)
	luaV_msg(L);

    ga_clear(&msg_ga);
    return 0;
}

    static int
luaV_debug(lua_State *L)
{
    lua_settop(L, 0);
    lua_getglobal(L, "vim");
    lua_getfield(L, -1, "eval");
    lua_remove(L, -2); // vim.eval at position 1
    for (;;)
    {
	const char *input;
	size_t l;
	lua_pushvalue(L, 1); // vim.eval
	lua_pushliteral(L, "input('lua_debug> ')");
	lua_call(L, 1, 1); // return string
	input = lua_tolstring(L, -1, &l);
	if (l == 0 || strcmp(input, "cont") == 0)
	    return 0;
	msg_putchar('\n'); // avoid outputting on input line
	if (luaL_loadbuffer(L, input, l, "=(debug command)")
		|| lua_pcall(L, 0, 0, 0))
	    luaV_emsg(L);
	lua_settop(L, 1); // remove eventual returns, but keep vim.eval
    }
}

    static dict_T *
luaV_get_var_scope(lua_State *L)
{
    const char	*scope = luaL_checkstring(L, 1);
    dict_T	*dict = NULL;

    if (STRICMP((char *)scope, "g") == 0)
	dict = get_globvar_dict();
    else if (STRICMP((char *)scope, "v") == 0)
	dict = get_vimvar_dict();
    else if (STRICMP((char *)scope, "b") == 0)
	dict = curbuf->b_vars;
    else if (STRICMP((char *)scope, "w") == 0)
	dict = curwin->w_vars;
    else if (STRICMP((char *)scope, "t") == 0)
	dict = curtab->tp_vars;
    else
    {
	luaL_error(L, "invalid scope %s", scope);
	return NULL;
    }

    return dict;
}

    static int
luaV_setvar(lua_State *L)
{
    dict_T	*dict;
    dictitem_T	*di;
    size_t	len;
    char	*name;
    int		del;
    char	*error = NULL;

    name = (char *)luaL_checklstring(L, 3, &len);
    del = (lua_gettop(L) < 4) || lua_isnil(L, 4);

    dict = luaV_get_var_scope(L);
    if (dict == NULL)
	return 0;

    di = dict_find(dict, (char_u *)name, (int)len);
    if (di != NULL)
    {
	if (di->di_flags & DI_FLAGS_RO)
	    error = "variable is read-only";
	else if (di->di_flags & DI_FLAGS_LOCK)
	    error = "variable is locked";
	else if (del && di->di_flags & DI_FLAGS_FIX)
	    error = "variable is fixed";
	if (error != NULL)
	    return luaL_error(L, error);
    }
    else if (dict->dv_lock)
	return luaL_error(L, "Dictionary is locked");

    if (del)
    {
	// Delete the key
	if (di == NULL)
	    // Doesn't exist, nothing to do
	    return 0;
	// Delete the entry
	dictitem_remove(dict, di, "Lua delete variable");
    }
    else
    {
	// Update the key
	typval_T	tv;

	// Convert the lua value to a Vim script type in the temporary variable
	lua_pushvalue(L, 4);
	if (luaV_totypval(L, -1, &tv) == FAIL)
	    return luaL_error(L, "Couldn't convert lua value");

	if (di == NULL)
	{
	    // Need to create an entry
	    di = dictitem_alloc((char_u *)name);
	    if (di == NULL)
	    {
		clear_tv(&tv);
		return 0;
	    }
	    // Update the value
	    copy_tv(&tv, &di->di_tv);
	    if (dict_add(dict, di) == FAIL)
	    {
		dictitem_free(di);
		clear_tv(&tv);
		return luaL_error(L, "Couldn't add to dictionary");
	    }
	}
	else
	{
	    int type_error = FALSE;
	    if (dict == get_vimvar_dict()
	       && !before_set_vvar((char_u *)name, di, &tv, TRUE, &type_error))
	    {
		clear_tv(&tv);
		if (type_error)
		    return luaL_error(L,
				"Setting v:%s to value with wrong type", name);
		return 0;
	    }
	    // Clear the old value
	    clear_tv(&di->di_tv);
	    // Update the value
	    copy_tv(&tv, &di->di_tv);
	}

	// Clear the temporary variable
	clear_tv(&tv);
    }

    return 0;
}

    static int
luaV_getvar(lua_State *L)
{
    dict_T	*dict = luaV_get_var_scope(L);
    size_t	len;
    const char	*name = luaL_checklstring(L, 3, &len);
    dictitem_T	*di = dict_find(dict, (char_u *)name, (int)len);

    if (di == NULL)
	return 0;  // nil

    luaV_pushtypval(L, &di->di_tv);
    return 1;
}

    static int
luaV_command(lua_State *L)
{
    char_u  *s = vim_strsave((char_u *)luaL_checkstring(L, 1));

    execute_cmds_from_string(s);
    vim_free(s);
    update_screen(UPD_VALID);
    return 0;
}

    static int
luaV_eval(lua_State *L)
{
    typval_T *tv = eval_expr((char_u *) luaL_checkstring(L, 1), NULL);

    if (tv == NULL) luaL_error(L, "invalid expression");
    luaV_pushtypval(L, tv);
    free_tv(tv);
    return 1;
}

    static int
luaV_beep(lua_State *L UNUSED)
{
    vim_beep(BO_LANG);
    return 0;
}

    static int
luaV_line(lua_State *L)
{
    luaV_pushline(L, curbuf, curwin->w_cursor.lnum);
    return 1;
}

    static int
luaV_list(lua_State *L)
{
    list_T *l;
    int initarg = !lua_isnoneornil(L, 1);

    if (initarg && lua_type(L, 1) != LUA_TTABLE)
	luaL_error(L, "table expected, got %s", luaL_typename(L, 1));

    l = list_alloc();
    if (l == NULL)
    {
	lua_pushnil(L);
	return 1;
    }

    luaV_newlist(L, l);
    if (!initarg)
	return 1;

    // traverse table to init list
    int notnil, i = 0;
    typval_T v;
    do
    {
	lua_rawgeti(L, 1, ++i);
	notnil = !lua_isnil(L, -1);
	if (notnil)
	{
	    luaV_checktypval(L, -1, &v, "vim.list");
	    list_append_tv(l, &v);
	    clear_tv(&v);
	}
	lua_pop(L, 1); // value
    } while (notnil);

    return 1;
}

    static int
luaV_dict(lua_State *L)
{
    dict_T *d;
    int initarg = !lua_isnoneornil(L, 1);

    if (initarg && lua_type(L, 1) != LUA_TTABLE)
	luaL_error(L, "table expected, got %s", luaL_typename(L, 1));

    d = dict_alloc();
    if (d == NULL)
    {
	lua_pushnil(L);
	return 1;
    }

    luaV_newdict(L, d);
    if (!initarg)
	return 1;

    // traverse table to init dict
    lua_pushnil(L);
    while (lua_next(L, 1))
    {
	char_u *key;
	dictitem_T *di;
	typval_T v;

	lua_pushvalue(L, -2); // dup key in case it's a number
	key = (char_u *) lua_tostring(L, -1);
	if (key == NULL)
	{
	    lua_pushnil(L);
	    return 1;
	}
	if (*key == NUL)
	    luaL_error(L, "table has empty key");
	luaV_checktypval(L, -2, &v, "vim.dict"); // value
	di = dictitem_alloc(key);
	if (di == NULL || dict_add(d, di) == FAIL)
	{
	    vim_free(di);
	    lua_pushnil(L);
	    return 1;
	}
	di->di_tv = v;
	lua_pop(L, 2); // key copy and value
    }

    return 1;
}

    static int
luaV_blob(lua_State *L)
{
    blob_T *b;
    int initarg = !lua_isnoneornil(L, 1);

    if (initarg && !lua_isstring(L, 1))
	luaL_error(L, "string expected, got %s", luaL_typename(L, 1));

    b = blob_alloc();
    if (b == NULL)
    {
	lua_pushnil(L);
	return 1;
    }

    luaV_newblob(L, b);
    if (!initarg)
	return 1;

    // traverse table to init blob
    size_t i, l = 0;
    const char *s = lua_tolstring(L, 1, &l);

    if (ga_grow(&b->bv_ga, (int)l) == OK)
	for (i = 0; i < l; ++i)
	    ga_append(&b->bv_ga, s[i]);

    return 1;
}

    static int
luaV_funcref(lua_State *L)
{
    const char *name = luaL_checkstring(L, 1);
    // note: not checking if function exists (needs function_exists)
    if (name == NULL || *name == NUL || VIM_ISDIGIT(*name))
	luaL_error(L, "invalid function name: %s", name);
    luaV_newfuncref(L, (char_u *) name);
    return 1;
}

    static int
luaV_buffer(lua_State *L)
{
    buf_T *buf;
    if (lua_isstring(L, 1)) // get by number or name?
    {
	if (lua_isnumber(L, 1)) // by number?
	{
	    int n = lua_tointeger(L, 1);
	    FOR_ALL_BUFFERS(buf)
		if (buf->b_fnum == n) break;
	}
	else // by name
	{
	    size_t l;
	    const char *s = lua_tolstring(L, 1, &l);
	    FOR_ALL_BUFFERS(buf)
	    {
		if (buf->b_ffname == NULL || buf->b_sfname == NULL)
		{
		    if (l == 0) break;
		}
		else if (strncmp(s, (char *)buf->b_ffname, l) == 0
			|| strncmp(s, (char *)buf->b_sfname, l) == 0)
		    break;
	    }
	}
    }
    else
	buf = (lua_toboolean(L, 1)) ? firstbuf : curbuf; // first buffer?
    luaV_pushbuffer(L, buf);
    return 1;
}

    static int
luaV_window(lua_State *L)
{
    win_T *win;
    if (lua_isnumber(L, 1)) // get by number?
    {
	int n = lua_tointeger(L, 1);
	for (win = firstwin; win != NULL; win = win->w_next, n--)
	    if (n == 1) break;
    }
    else
	win = (lua_toboolean(L, 1)) ? firstwin : curwin; // first window?
    luaV_pushwindow(L, win);
    return 1;
}

    static int
luaV_open(lua_State *L)
{
    char_u *s = NULL;
#ifdef HAVE_SANDBOX
    luaV_checksandbox(L);
#endif
    if (lua_isstring(L, 1)) s = (char_u *) lua_tostring(L, 1);
    luaV_pushbuffer(L, buflist_new(s, NULL, 1L, BLN_LISTED));
    return 1;
}

    static int
luaV_type(lua_State *L)
{
    luaL_checkany(L, 1);
    if (lua_type(L, 1) == LUA_TUSERDATA) // check vim udata?
    {
	lua_settop(L, 1);
	if (lua_getmetatable(L, 1))
	{
	    luaV_getfield(L, LUAVIM_LIST);
	    if (lua_rawequal(L, -1, 2))
	    {
		lua_pushstring(L, "list");
		return 1;
	    }
	    luaV_getfield(L, LUAVIM_DICT);
	    if (lua_rawequal(L, -1, 2))
	    {
		lua_pushstring(L, "dict");
		return 1;
	    }
	    luaV_getfield(L, LUAVIM_BLOB);
	    if (lua_rawequal(L, -1, 2))
	    {
		lua_pushstring(L, "blob");
		return 1;
	    }
	    luaV_getfield(L, LUAVIM_FUNCREF);
	    if (lua_rawequal(L, -1, 2))
	    {
		lua_pushstring(L, "funcref");
		return 1;
	    }
	    luaV_getfield(L, LUAVIM_BUFFER);
	    if (lua_rawequal(L, -1, 2))
	    {
		lua_pushstring(L, "buffer");
		return 1;
	    }
	    luaV_getfield(L, LUAVIM_WINDOW);
	    if (lua_rawequal(L, -1, 2))
	    {
		lua_pushstring(L, "window");
		return 1;
	    }
	}
    }
    lua_pushstring(L, luaL_typename(L, 1)); // fallback
    return 1;
}

    static int
luaV_call(lua_State *L)
{
    int		argc = lua_gettop(L) - 1;
    size_t	funcname_len;
    char_u	*funcname;
    char	*error = NULL;
    typval_T	rettv;
    typval_T	argv[MAX_FUNC_ARGS + 1];
    int		i = 0;

    if (argc > MAX_FUNC_ARGS)
	return luaL_error(L, "Function called with too many arguments");

    funcname = (char_u *)luaL_checklstring(L, 1, &funcname_len);

    for (; i < argc; i++)
    {
	if (luaV_totypval(L, i + 2, &argv[i]) == FAIL)
	{
	    error = "lua: cannot convert value";
	    goto free_vim_args;
	}
    }

    argv[argc].v_type = VAR_UNKNOWN;

    if (call_vim_function(funcname, argc, argv, &rettv) == FAIL)
    {
	error = "lua: call_vim_function failed";
	goto free_vim_args;
    }

    luaV_pushtypval(L, &rettv);
    clear_tv(&rettv);

free_vim_args:
    while (i > 0)
	clear_tv(&argv[--i]);

    if (error == NULL)
	return 1;
    else
	return luaL_error(L, error);
}

/*
 * Return the Vim version as a Lua table
 */
    static int
luaV_version(lua_State *L)
{
    lua_newtable(L);
    lua_pushstring(L, "major");
    lua_pushinteger(L, VIM_VERSION_MAJOR);
    lua_settable(L, -3);
    lua_pushstring(L, "minor");
    lua_pushinteger(L, VIM_VERSION_MINOR);
    lua_settable(L, -3);
    lua_pushstring(L, "patch");
    lua_pushinteger(L, highest_patch());
    lua_settable(L, -3);
    return 1;
}

static const luaL_Reg luaV_module[] = {
    {"command", luaV_command},
    {"eval", luaV_eval},
    {"beep", luaV_beep},
    {"line", luaV_line},
    {"list", luaV_list},
    {"dict", luaV_dict},
    {"blob", luaV_blob},
    {"funcref", luaV_funcref},
    {"buffer", luaV_buffer},
    {"window", luaV_window},
    {"open", luaV_open},
    {"type", luaV_type},
    {"call", luaV_call},
    {"_getvar", luaV_getvar},
    {"_setvar", luaV_setvar},
    {"version", luaV_version},
    {"lua_version", NULL},
    {NULL, NULL}
};

/*
 * for freeing list, dict, buffer and window objects; lightuserdata as arg
 */
    static int
luaV_free(lua_State *L)
{
    lua_pushnil(L);
    luaV_setudata(L, lua_touserdata(L, 1));
    return 0;
}

    static int
luaV_luaeval(lua_State *L)
{
    luaL_Buffer b;
    size_t l;
    const char *str = lua_tolstring(L, 1, &l);
    typval_T *arg = (typval_T *) lua_touserdata(L, 2);
    typval_T *rettv = (typval_T *) lua_touserdata(L, 3);
    luaL_buffinit(L, &b);
    luaL_addlstring(&b, LUAVIM_EVALHEADER, sizeof(LUAVIM_EVALHEADER) - 1);
    luaL_addlstring(&b, str, l);
    luaL_pushresult(&b);
    str = lua_tolstring(L, -1, &l);
    if (luaL_loadbuffer(L, str, l, LUAVIM_EVALNAME)) // compile error?
    {
	luaV_emsg(L);
	return 0;
    }
    luaV_pushtypval(L, arg);
    if (lua_pcall(L, 1, 1, 0)) // running error?
    {
	luaV_emsg(L);
	return 0;
    }
    if (luaV_totypval(L, -1, rettv) == FAIL)
	emsg("luaeval: cannot convert value");
    return 0;
}

    static int
luaV_setref(lua_State *L)
{
    int copyID = lua_tointeger(L, 1);
    int abort = FALSE;

    lua_pushlightuserdata(L, (void *) LUAVIM_UDATA_CACHE);
    lua_rawget(L, LUA_REGISTRYINDEX);  // the cache table

    luaV_getfield(L, LUAVIM_LIST);
    luaV_getfield(L, LUAVIM_DICT);
    luaV_getfield(L, LUAVIM_FUNCREF);
    lua_pushnil(L);
    // traverse cache table
    while (!abort && lua_next(L, 2) != 0)
    {
	lua_getmetatable(L, -1);
	if (lua_rawequal(L, -1, 3)) // list?
	{
	    list_T *l = (list_T *)lua_touserdata(L, 6); // key

	    abort = set_ref_in_list(l, copyID);
	}
	else if (lua_rawequal(L, -1, 4)) // dict?
	{
	    dict_T *d = (dict_T *)lua_touserdata(L, 6); // key

	    abort = set_ref_in_dict(d, copyID);
	}
	else if (lua_rawequal(L, -1, 5)) // funcref?
	{
	    luaV_Funcref *f = (luaV_Funcref *)lua_touserdata(L, 6); // key

	    abort = set_ref_in_dict(f->self, copyID);
	}
	lua_pop(L, 2); // metatable and value
    }
    lua_pushinteger(L, abort);
    return 1;
}

    static int
luaV_pushversion(lua_State *L)
{
    int major = 0;
    int minor = 0;
    int patch = 0;
    char s[16];

    sscanf(LUAVIM_VERSION, "Lua %d.%d.%d", &major, &minor, &patch);
    vim_snprintf(s, sizeof(s), "%d.%d.%d", major, minor, patch);
    lua_pushstring(L, s);
    return 0;
}

#define LUA_VIM_FN_CODE \
    "vim.fn = setmetatable({}, {\n"\
    "  __index = function (t, key)\n"\
    "    local function _fn(...)\n"\
    "      return vim.call(key, ...)\n"\
    "    end\n"\
    "    t[key] = _fn\n"\
    "    return _fn\n"\
    "  end\n"\
    " })"

#define LUA_VIM_UPDATE_PACKAGE_PATHS \
    "local last_vim_paths = {}\n"\
    "vim._update_package_paths = function ()\n"\
    "  local cur_vim_paths = {}\n"\
    "  local function split(s, delimiter)\n"\
    "    result = {}\n"\
    "    for match in (s..delimiter):gmatch(\"(.-)\"..delimiter) do\n"\
    "      table.insert(result, match)\n"\
    "    end\n"\
    "    return result\n"\
    "  end\n"\
    "  local rtps = split(vim.eval('&runtimepath'), ',')\n"\
    "  local sep = package.config:sub(1, 1)\n"\
    "  for _, key in ipairs({'path', 'cpath'}) do\n"\
    "    local orig_str = package[key] .. ';'\n"\
    "    local pathtrails_ordered = {}\n"\
    "    -- Note: ignores trailing item without trailing `;`. Not using something\n"\
    "    -- simpler in order to preserve empty items (stand for default path).\n"\
    "    local orig = {}\n"\
    "    for s in orig_str:gmatch('[^;]*;') do\n"\
    "      s = s:sub(1, -2)  -- Strip trailing semicolon\n"\
    "      orig[#orig + 1] = s\n"\
    "    end\n"\
    "    if key == 'path' then\n"\
    "      -- /?.lua and /?/init.lua\n"\
    "      pathtrails_ordered = {sep .. '?.lua', sep .. '?' .. sep .. 'init.lua'}\n"\
    "    else\n"\
    "      local pathtrails = {}\n"\
    "      for _, s in ipairs(orig) do\n"\
    "        -- Find out path patterns. pathtrail should contain something like\n"\
    "        -- /?.so, \?.dll. This allows not to bother determining what correct\n"\
    "        -- suffixes are.\n"\
    "        local pathtrail = s:match('[/\\\\][^/\\\\]*%?.*$')\n"\
    "        if pathtrail and not pathtrails[pathtrail] then\n"\
    "          pathtrails[pathtrail] = true\n"\
    "          pathtrails_ordered[#pathtrails_ordered + 1] = pathtrail\n"\
    "        end\n"\
    "      end\n"\
    "    end\n"\
    "    local new = {}\n"\
    "    for _, rtp in ipairs(rtps) do\n"\
    "      if not rtp:match(';') then\n"\
    "        for _, pathtrail in pairs(pathtrails_ordered) do\n"\
    "          local new_path = rtp .. sep .. 'lua' .. pathtrail\n"\
    "          -- Always keep paths from &runtimepath at the start:\n"\
    "          -- append them here disregarding orig possibly containing one of them.\n"\
    "          new[#new + 1] = new_path\n"\
    "          cur_vim_paths[new_path] = true\n"\
    "        end\n"\
    "      end\n"\
    "    end\n"\
    "    for _, orig_path in ipairs(orig) do\n"\
    "      -- Handle removing obsolete paths originating from &runtimepath: such\n"\
    "      -- paths either belong to cur_nvim_paths and were already added above or\n"\
    "      -- to last_nvim_paths and should not be added at all if corresponding\n"\
    "      -- entry was removed from &runtimepath list.\n"\
    "      if not (cur_vim_paths[orig_path] or last_vim_paths[orig_path]) then\n"\
    "        new[#new + 1] = orig_path\n"\
    "      end\n"\
    "    end\n"\
    "    package[key] = table.concat(new, ';')\n"\
    "  end\n"\
    "  last_vim_paths = cur_vim_paths\n"\
    "end"

#define LUA_VIM_SETUP_VARIABLE_DICTS \
    "do\n"\
    "  local function make_dict_accessor(scope)\n"\
    "    local mt = {}\n"\
    "    function mt:__newindex(k, v)\n"\
    "      return vim._setvar(scope, 0, k, v)\n"\
    "    end\n"\
    "    function mt:__index(k)\n"\
    "      return vim._getvar(scope, 0, k)\n"\
    "    end\n"\
    "    return setmetatable({}, mt)\n"\
    "  end\n"\
    "  vim.g = make_dict_accessor('g')\n"\
    "  vim.v = make_dict_accessor('v')\n"\
    "  vim.b = make_dict_accessor('b')\n"\
    "  vim.w = make_dict_accessor('w')\n"\
    "  vim.t = make_dict_accessor('t')\n"\
    "end"

    static int
luaopen_vim(lua_State *L)
{
    lua_newtable(L);  // cache table
    lua_newtable(L);  // cache table's metatable
    lua_pushstring(L, "v");
    lua_setfield(L, -2, "__mode");
    lua_setmetatable(L, -2); // cache is weak-valued
    // put the cache table in the registry for luaV_get/setudata()
    lua_pushlightuserdata(L, (void *) LUAVIM_UDATA_CACHE);
    lua_pushvalue(L, -2);
    lua_rawset(L, LUA_REGISTRYINDEX);
    lua_pop(L, 1);  // we don't need the cache table here anymore
    // print
    lua_pushcfunction(L, luaV_print);
    lua_setglobal(L, "print");
    // debug.debug
    lua_getglobal(L, "debug");
    lua_pushcfunction(L, luaV_debug);
    lua_setfield(L, -2, "debug");
    lua_pop(L, 1);
    // free
    lua_pushlightuserdata(L, (void *) LUAVIM_FREE);
    lua_pushcfunction(L, luaV_free);
    lua_rawset(L, LUA_REGISTRYINDEX);
    // luaeval
    lua_pushlightuserdata(L, (void *) LUAVIM_LUAEVAL);
    lua_pushcfunction(L, luaV_luaeval);
    lua_rawset(L, LUA_REGISTRYINDEX);
    // setref
    lua_pushlightuserdata(L, (void *) LUAVIM_SETREF);
    lua_pushcfunction(L, luaV_setref);
    lua_rawset(L, LUA_REGISTRYINDEX);
    // register
    luaV_newmetatable(L, LUAVIM_LIST);
    luaV_register(L, luaV_List_mt);
    lua_pop(L, 1);
    luaV_newmetatable(L, LUAVIM_DICT);
    luaV_register(L, luaV_Dict_mt);
    lua_pop(L, 1);
    luaV_newmetatable(L, LUAVIM_BLOB);
    luaV_register(L, luaV_Blob_mt);
    lua_pop(L, 1);
    luaV_newmetatable(L, LUAVIM_FUNCREF);
    luaV_register(L, luaV_Funcref_mt);
    lua_pop(L, 1);
    luaV_newmetatable(L, LUAVIM_BUFFER);
    luaV_register(L, luaV_Buffer_mt);
    lua_pop(L, 1);
    luaV_newmetatable(L, LUAVIM_WINDOW);
    luaV_register(L, luaV_Window_mt);
    lua_pop(L, 1);
    lua_newtable(L); // vim table
    luaV_register(L, luaV_module);
    luaV_pushversion(L);
    lua_setfield(L, -2, "lua_version");
    lua_setglobal(L, LUAVIM_NAME);
    // custom code
    (void)luaL_dostring(L, LUA_VIM_FN_CODE);
    (void)luaL_dostring(L, LUA_VIM_UPDATE_PACKAGE_PATHS);
    (void)luaL_dostring(L, LUA_VIM_SETUP_VARIABLE_DICTS);

    lua_getglobal(L, LUAVIM_NAME);
    lua_getfield(L, -1, "_update_package_paths");

    if (lua_pcall(L, 0, 0, 0))
	luaV_emsg(L);

    return 0;
}

    static lua_State *
luaV_newstate(void)
{
    lua_State *L = luaL_newstate();
    luaL_openlibs(L); // core libs
    lua_pushcfunction(L, luaopen_vim); // vim
    lua_call(L, 0, 0);
    return L;
}

    static void
luaV_setrange(lua_State *L, int line1, int line2)
{
    lua_getglobal(L, LUAVIM_NAME);
    lua_pushinteger(L, line1);
    lua_setfield(L, -2, "firstline");
    lua_pushinteger(L, line2);
    lua_setfield(L, -2, "lastline");
    lua_pop(L, 1); // vim table
}


// =======   Interface   =======

static lua_State *L = NULL;

    static int
lua_isopen(void)
{
    return L != NULL;
}

    static int
lua_init(void)
{
    if (lua_isopen())
	return OK;

#ifdef DYNAMIC_LUA
    if (!lua_enabled(TRUE))
    {
	emsg(_("Lua library cannot be loaded."));
	return FAIL;
    }
#endif
    L = luaV_newstate();

    return OK;
}

    void
lua_end(void)
{
    if (!lua_isopen())
	return;

    lua_close(L);
    L = NULL;
}

/*
 * ex commands
 */
    void
ex_lua(exarg_T *eap)
{
    char *script = (char *)script_get(eap, eap->arg);

    if (!eap->skip && lua_init() == OK)
    {
	char *s = script != NULL ? script : (char *)eap->arg;

	luaV_setrange(L, eap->line1, eap->line2);
	if (luaL_loadbuffer(L, s, strlen(s), LUAVIM_CHUNKNAME)
		|| lua_pcall(L, 0, 0, 0))
	    luaV_emsg(L);
    }
    if (script != NULL)
	vim_free(script);
}

    void
ex_luado(exarg_T *eap)
{
    linenr_T l;
    const char *s = (const char *) eap->arg;
    luaL_Buffer b;
    size_t len;
    buf_T *was_curbuf = curbuf;

    if (lua_init() == FAIL) return;
    if (u_save(eap->line1 - 1, eap->line2 + 1) == FAIL)
    {
	emsg(_("cannot save undo information"));
	return;
    }
    luaV_setrange(L, eap->line1, eap->line2);
    luaL_buffinit(L, &b);
    luaL_addlstring(&b, "return function(line, linenr) ", 30); // header
    luaL_addlstring(&b, s, strlen(s));
    luaL_addlstring(&b, " end", 4); // footer
    luaL_pushresult(&b);
    s = lua_tolstring(L, -1, &len);
    if (luaL_loadbuffer(L, s, len, LUAVIM_CHUNKNAME))
    {
	luaV_emsg(L);
	lua_pop(L, 1); // function body
	return;
    }
    lua_call(L, 0, 1);
    lua_replace(L, -2); // function -> body
    for (l = eap->line1; l <= eap->line2; l++)
    {
	// Check the line number, the command may have deleted lines.
	if (l > curbuf->b_ml.ml_line_count)
	    break;

	lua_pushvalue(L, -1); // function
	luaV_pushline(L, curbuf, l); // current line as arg
	lua_pushinteger(L, l); // current line number as arg
	if (lua_pcall(L, 2, 1, 0))
	{
	    luaV_emsg(L);
	    break;
	}

	// Catch the command switching to another buffer.
	// Check the line number, the command may have deleted lines.
	if (curbuf != was_curbuf || l > curbuf->b_ml.ml_line_count)
	    break;

	if (lua_isstring(L, -1)) // update line?
	{
#ifdef HAVE_SANDBOX
	    luaV_checksandbox(L);
#endif
	    ml_replace(l, luaV_toline(L, -1), TRUE);
	    changed_bytes(l, 0);
	    lua_pop(L, 1); // result from luaV_toline
	}
	lua_pop(L, 1); // line
    }
    lua_pop(L, 1); // function
    check_cursor();
    update_screen(UPD_NOT_VALID);
}

    void
ex_luafile(exarg_T *eap)
{
    if (lua_init() == FAIL)
	return;
    if (!eap->skip)
    {
	luaV_setrange(L, eap->line1, eap->line2);
	if (luaL_loadfile(L, (char *) eap->arg) || lua_pcall(L, 0, 0, 0))
	    luaV_emsg(L);
    }
}

#define luaV_freetype(typ,tname) \
	void \
    lua_##tname##_free(typ *o) \
    { \
	if (!lua_isopen()) return; \
	luaV_getfield(L, LUAVIM_FREE); \
	lua_pushlightuserdata(L, (void *) o); \
	lua_call(L, 1, 0); \
    }

luaV_freetype(buf_T, buffer)
luaV_freetype(win_T, window)

    void
do_luaeval(char_u *str, typval_T *arg, typval_T *rettv)
{
    lua_init();
    luaV_getfield(L, LUAVIM_LUAEVAL);
    lua_pushstring(L, (char *) str);
    lua_pushlightuserdata(L, (void *) arg);
    lua_pushlightuserdata(L, (void *) rettv);
    lua_call(L, 3, 0);
}

    int
set_ref_in_lua(int copyID)
{
    int aborted = 0;

    if (!lua_isopen())
	return 0;

    luaV_getfield(L, LUAVIM_SETREF);
    // call the function with 1 arg, getting 1 result back
    lua_pushinteger(L, copyID);
    lua_call(L, 1, 1);
    // get the result
    aborted = lua_tointeger(L, -1);
    // pop result off the stack
    lua_pop(L, 1);

    return aborted;
}

    void
update_package_paths_in_lua(void)
{
    if (!lua_isopen())
	return;

    lua_getglobal(L, "vim");
    lua_getfield(L, -1, "_update_package_paths");

    if (lua_pcall(L, 0, 0, 0))
	luaV_emsg(L);
}

/*
 * Native C function callback
 */
    static int
luaV_call_lua_func(
	int	 argcount,
	typval_T *argvars,
	typval_T *rettv,
	void	 *state)
{
    int i;
    int luaargcount = argcount;
    luaV_CFuncState *funcstate = (luaV_CFuncState*)state;
    lua_rawgeti(funcstate->L, LUA_REGISTRYINDEX, funcstate->lua_funcref);

    if (funcstate->lua_tableref != LUA_NOREF)
    {
	// First arg for metatable __call method is a table
	luaargcount += 1;
	lua_rawgeti(funcstate->L, LUA_REGISTRYINDEX, funcstate->lua_tableref);
    }

    for (i = 0; i < argcount; ++i)
	luaV_pushtypval(funcstate->L, &argvars[i]);

    if (lua_pcall(funcstate->L, luaargcount, 1, 0))
    {
	luaV_emsg(funcstate->L);
	return (int)FCERR_OTHER;
    }

    luaV_checktypval(funcstate->L, -1, rettv, "get return value");
    return (int)FCERR_NONE;
}

/*
 * Free up any lua references held by the func state.
 */
    static void
luaV_call_lua_func_free(void *state)
{
    luaV_CFuncState *funcstate = (luaV_CFuncState*)state;
    luaL_unref(L, LUA_REGISTRYINDEX, funcstate->lua_funcref);
    funcstate->L = NULL;
    if (funcstate->lua_tableref != LUA_NOREF)
	luaL_unref(L, LUA_REGISTRYINDEX, funcstate->lua_tableref);
    VIM_CLEAR(funcstate);
}

#endif
