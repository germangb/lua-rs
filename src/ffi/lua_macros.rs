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
