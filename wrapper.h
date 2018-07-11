#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

// Macros marked with a (*) means they are not implemented yet
//
/// # lua.h
//
/// #define lua_call(L,n,r) lua_callk(L, (n), (r), 0, NULL)
/// #define lua_pcall(L,n,r,f) lua_pcallk(L, (n), (r), (f), 0, NULL)
/// #define lua_yield(L,n) lua_yieldk(L, (n), 0, NULL)
/// (*) #define lua_getextraspace(L)	((void *)((char *)(L) - LUA_EXTRASPACE))
/// #define lua_tonumber(L,i)	lua_tonumberx(L,(i),NULL)
/// #define lua_tointeger(L,i)	lua_tointegerx(L,(i),NULL)
/// #define lua_pop(L,n)		lua_settop(L, -(n)-1)
/// #define lua_newtable(L)		lua_createtable(L, 0, 0)
/// (*) #define lua_register(L,n,f) (lua_pushcfunction(L, (f)), lua_setglobal(L, (n)))
/// #define lua_pushcfunction(L,f)	lua_pushcclosure(L, (f), 0)
/// #define lua_isfunction(L,n)	(lua_type(L, (n)) == LUA_TFUNCTION)
/// #define lua_istable(L,n)	(lua_type(L, (n)) == LUA_TTABLE)
/// #define lua_islightuserdata(L,n)	(lua_type(L, (n)) == LUA_TLIGHTUSERDATA)
/// #define lua_isnil(L,n)		(lua_type(L, (n)) == LUA_TNIL)
/// #define lua_isboolean(L,n)	(lua_type(L, (n)) == LUA_TBOOLEAN)
/// #define lua_isthread(L,n)	(lua_type(L, (n)) == LUA_TTHREAD)
/// #define lua_isnone(L,n)		(lua_type(L, (n)) == LUA_TNONE)
/// #define lua_isnoneornil(L, n)	(lua_type(L, (n)) <= 0)
/// #define lua_pushliteral(L, s)	lua_pushstring(L, "" s)
/// (*) #define lua_pushglobaltable(L)  ((void)lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS))
/// #define lua_tostring(L,i)	lua_tolstring(L, (i), NULL)
/// #define lua_insert(L,idx)	lua_rotate(L, (idx), 1)
/// (*) #define lua_remove(L,idx)	(lua_rotate(L, (idx), -1), lua_pop(L, 1))
/// (*) #define lua_replace(L,idx) (lua_copy(L, -1, (idx)), lua_pop(L, 1))
///
/// #if defined(LUA_COMPAT_APIINTCASTS)
///
/// #define lua_pushunsigned(L,n)	lua_pushinteger(L, (lua_Integer)(n))
/// #define lua_tounsignedx(L,i,is)	((lua_Unsigned)lua_tointegerx(L,i,is))
/// #define lua_tounsigned(L,i)	lua_tounsignedx(L,(i),NULL)
///
/// #endif
///
/// ===
///
/// # lauxlib.h
///
/// (*) #define luaL_checkversion(L)  luaL_checkversion_(L, LUA_VERSION_NUM, LUAL_NUMSIZES)
/// (*) #define luaL_loadfile(L,f) luaL_loadfilex(L,f,NULL)
/// (*) #define luaL_newlibtable(L,l)	lua_createtable(L, 0, sizeof(l)/sizeof((l)[0]) - 1)
/// (*) #define luaL_newlib(L,l)  (luaL_checkversion(L), luaL_newlibtable(L,l), luaL_setfuncs(L,l,0))
/// (*) #define luaL_argcheck(L, cond,arg,extramsg)	((void)((cond) || luaL_argerror(L, (arg), (extramsg))))
/// (*) #define luaL_checkstring(L,n)	(luaL_checklstring(L, (n), NULL))
/// (*) #define luaL_optstring(L,n,d)	(luaL_optlstring(L, (n), (d), NULL))
/// (*) #define luaL_typename(L,i)	lua_typename(L, lua_type(L,(i)))
/// (*) #define luaL_dofile(L, fn) (luaL_loadfile(L, fn) || lua_pcall(L, 0, LUA_MULTRET, 0))
/// (*) #define luaL_dostring(L, s) (luaL_loadstring(L, s) || lua_pcall(L, 0, LUA_MULTRET, 0))
/// (*) #define luaL_getmetatable(L,n)	(lua_getfield(L, LUA_REGISTRYINDEX, (n)))
/// (*) #define luaL_opt(L,f,n,d)	(lua_isnoneornil(L,(n)) ? (d) : f(L,(n)))
/// (*) #define luaL_loadbuffer(L,s,sz,n) luaL_loadbufferx(L,s,sz,n,NULL)
///
/// (*) #define luaL_addchar(B,c) ((void)((B)->n < (B)->size || luaL_prepbuffsize((B), 1)), ((B)->b[(B)->n++] = (c)))
/// (*) #define luaL_addsize(B,s) ((B)->n += (s))
/// (*) #define luaL_prepbuffer(B) luaL_prepbuffsize(B, LUAL_BUFFERSIZE)
///
/// #if defined(LUA_COMPAT_APIINTCASTS)
///
/// (*) #define luaL_checkunsigned(L,a)	((lua_Unsigned)luaL_checkinteger(L,a))
/// (*) #define luaL_optunsigned(L,a,d)	((lua_Unsigned)luaL_optinteger(L,a,(lua_Integer)(d)))
/// (*) #define luaL_checkint(L,n)	((int)luaL_checkinteger(L, (n)))
/// (*) #define luaL_optint(L,n,d)	((int)luaL_optinteger(L, (n), (d)))
/// (*) #define luaL_checklong(L,n)	((long)luaL_checkinteger(L, (n)))
/// (*) #define luaL_optlong(L,n,d)	((long)luaL_optinteger(L, (n), (d)))
///
/// #endif
