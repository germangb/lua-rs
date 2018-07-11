// #define lua_call(L,n,r) lua_callk(L, (n), (r), 0, NULL)
// #define lua_pcall(L,n,r,f) lua_pcallk(L, (n), (r), (f), 0, NULL)
// #define lua_yield(L,n) lua_yieldk(L, (n), 0, NULL)
// (*) #define lua_getextraspace(L)	((void *)((char *)(L) - LUA_EXTRASPACE))
// #define lua_tonumber(L,i)	lua_tonumberx(L,(i),NULL)
// #define lua_tointeger(L,i)	lua_tointegerx(L,(i),NULL)
// #define lua_pop(L,n)		lua_settop(L, -(n)-1)
// #define lua_newtable(L)		lua_createtable(L, 0, 0)
// (*) #define lua_register(L,n,f) (lua_pushcfunction(L, (f)), lua_setglobal(L, (n)))
// #define lua_pushcfunction(L,f)	lua_pushcclosure(L, (f), 0)
// #define lua_isfunction(L,n)	(lua_type(L, (n)) == LUA_TFUNCTION)
// #define lua_istable(L,n)	(lua_type(L, (n)) == LUA_TTABLE)
// #define lua_islightuserdata(L,n)	(lua_type(L, (n)) == LUA_TLIGHTUSERDATA)
// #define lua_isnil(L,n)		(lua_type(L, (n)) == LUA_TNIL)
// #define lua_isboolean(L,n)	(lua_type(L, (n)) == LUA_TBOOLEAN)
// #define lua_isthread(L,n)	(lua_type(L, (n)) == LUA_TTHREAD)
// #define lua_isnone(L,n)		(lua_type(L, (n)) == LUA_TNONE)
// #define lua_isnoneornil(L, n)	(lua_type(L, (n)) <= 0)
// #define lua_pushliteral(L, s)	lua_pushstring(L, "" s)
// (*) #define lua_pushglobaltable(L)  ((void)lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS))
// #define lua_tostring(L,i)	lua_tolstring(L, (i), NULL)
// #define lua_insert(L,idx)	lua_rotate(L, (idx), 1)
// (*) #define lua_remove(L,idx)	(lua_rotate(L, (idx), -1), lua_pop(L, 1))
// (*) #define lua_replace(L,idx) (lua_copy(L, -1, (idx)), lua_pop(L, 1))
//
// #if defined(LUA_COMPAT_APIINTCASTS)
//
// (*) #define lua_pushunsigned(L,n)	lua_pushinteger(L, (lua_Integer)(n))
// (*) #define lua_tounsignedx(L,i,is)	((lua_Unsigned)lua_tointegerx(L,i,is))
// (*) #define lua_tounsigned(L,i)	lua_tounsignedx(L,(i),NULL)
//
// #endif

#[inline]
pub unsafe fn lua_call(L: *mut lua_State, n: ::std::os::raw::c_int, r: ::std::os::raw::c_int) {
    lua_callk(L, n, r, 0, None)
}

#[inline]
pub unsafe fn lua_pcall(
    L: *mut lua_State,
    n: ::std::os::raw::c_int,
    r: ::std::os::raw::c_int,
    f: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    lua_pcallk(L, n, r, f, 0, None)
}

#[inline]
pub unsafe fn lua_yield(L: *mut lua_State, n: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    lua_yieldk(L, n, 0, None)
}

#[inline]
pub unsafe fn lua_tonumber(L: *mut lua_State, i: ::std::os::raw::c_int) -> lua_Number {
    lua_tonumberx(L, i, 0 as _)
}

#[inline]
pub unsafe fn lua_tointeger(L: *mut lua_State, i: ::std::os::raw::c_int) -> lua_Integer {
    lua_tointegerx(L, i, 0 as _)
}

#[inline]
pub unsafe fn lua_pop(L: *mut lua_State, n: ::std::os::raw::c_int) {
    lua_settop(L, -n - 1)
}

#[inline]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0)
}

#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0)
}

#[inline]
pub unsafe fn lua_isfunction(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TFUNCTION as _
}

#[inline]
pub unsafe fn lua_istable(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TTABLE as _
}

#[inline]
pub unsafe fn lua_islightuserdata(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TLIGHTUSERDATA as _
}

#[inline]
pub unsafe fn lua_isnil(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TNIL as _
}

#[inline]
pub unsafe fn lua_isboolean(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TBOOLEAN as _
}

#[inline]
pub unsafe fn lua_isthread(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TTHREAD as _
}

#[inline]
pub unsafe fn lua_isnone(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) == LUA_TNONE
}

#[inline]
pub unsafe fn lua_isnoneornil(L: *mut lua_State, n: ::std::os::raw::c_int) -> bool {
    lua_type(L, n) <= 0
}

#[inline]
pub unsafe fn lua_pushliteral(
    L: *mut lua_State,
    s: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    lua_pushstring(L, s)
}

#[inline]
pub unsafe fn lua_tostring(
    L: *mut lua_State,
    i: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    lua_tolstring(L, i, 0 as _)
}

#[inline]
pub unsafe fn lua_insert(L: *mut lua_State, idx: ::std::os::raw::c_int) {
    lua_rotate(L, idx, 1)
}
