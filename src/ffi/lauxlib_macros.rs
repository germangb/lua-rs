// #define LUAL_NUMSIZES	(sizeof(lua_Integer)*16 + sizeof(lua_Number))
// #define luaL_checkversion(L)  luaL_checkversion_(L, LUA_VERSION_NUM, LUAL_NUMSIZES)
// #define luaL_loadfile(L,f) luaL_loadfilex(L,f,NULL)
// (*) #define luaL_newlibtable(L,l)	lua_createtable(L, 0, sizeof(l)/sizeof((l)[0]) - 1)
// (*) #define luaL_newlib(L,l)  (luaL_checkversion(L), luaL_newlibtable(L,l), luaL_setfuncs(L,l,0))
// (*) #define luaL_argcheck(L, cond,arg,extramsg)	((void)((cond) || luaL_argerror(L, (arg), (extramsg))))
// #define luaL_checkstring(L,n)	(luaL_checklstring(L, (n), NULL))
// #define luaL_optstring(L,n,d)	(luaL_optlstring(L, (n), (d), NULL))
// #define luaL_typename(L,i)	lua_typename(L, lua_type(L,(i)))
// #define luaL_dofile(L, fn) (luaL_loadfile(L, fn) || lua_pcall(L, 0, LUA_MULTRET, 0))
// #define luaL_dostring(L, s) (luaL_loadstring(L, s) || lua_pcall(L, 0, LUA_MULTRET, 0))
// #define luaL_getmetatable(L,n)	(lua_getfield(L, LUA_REGISTRYINDEX, (n)))
// (*)  #define luaL_opt(L,f,n,d)	(lua_isnoneornil(L,(n)) ? (d) : f(L,(n)))
// #define luaL_loadbuffer(L,s,sz,n) luaL_loadbufferx(L,s,sz,n,NULL)
//
// (*) #define luaL_addchar(B,c) ((void)((B)->n < (B)->size || luaL_prepbuffsize((B), 1)), ((B)->b[(B)->n++] = (c)))
// #define luaL_addsize(B,s) ((B)->n += (s))
// (*)  #define luaL_prepbuffer(B) luaL_prepbuffsize(B, LUAL_BUFFERSIZE)
//
// #if defined(LUA_COMPAT_APIINTCASTS)
//
// (*) #define luaL_checkunsigned(L,a)	((lua_Unsigned)luaL_checkinteger(L,a))
// (*) #define luaL_optunsigned(L,a,d)	((lua_Unsigned)luaL_optinteger(L,a,(lua_Integer)(d)))
// (*) #define luaL_checkint(L,n)	((int)luaL_checkinteger(L, (n)))
// (*) #define luaL_optint(L,n,d)	((int)luaL_optinteger(L, (n), (d)))
// (*) #define luaL_checklong(L,n)	((long)luaL_checkinteger(L, (n)))
// (*) #define luaL_optlong(L,n,d)	((long)luaL_optinteger(L, (n), (d)))
//
// #endif

#[inline]
pub unsafe fn luaL_checkversion(L: *mut lua_State) {
    //luaL_checkversion_(L, LUA_VERSION_NUM as _, LUAL_NUMSIZES as _)
    luaL_checkversion_(
        L,
        LUA_VERSION_NUM as _,
        (::std::mem::size_of::<lua_Integer>() * 16 + ::std::mem::size_of::<lua_Number>()) as _,
    )
}

#[inline]
pub unsafe fn luaL_loadfile(
    L: *mut lua_State,
    f: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    luaL_loadfilex(L, f, 0 as _)
}

#[inline]
pub unsafe fn luaL_newlibtable(L: *mut lua_State, l: *const luaL_Reg){
    unimplemented!()
	//lua_createtable(L, 0, len - 1)
}

#[inline]
pub unsafe fn luaL_newlib(L: *mut lua_State, l: *const luaL_Reg) {
  luaL_checkversion(L);
  luaL_newlibtable(L,l);
  luaL_setfuncs(L,l,0);
}

#[inline]
pub unsafe fn luaL_argcheck(
    L: *mut lua_State,
    cond : :: std :: os :: raw :: c_int ,
    arg : :: std :: os :: raw :: c_int ,
    extramsg: * const :: std :: os :: raw :: c_char,
) {
    if cond == 0 {
        luaL_argerror(L, arg, extramsg);
    }
}

#[inline]
pub unsafe fn luaL_checkstring(
    L: *mut lua_State,
    n: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    luaL_checklstring(L, n, 0 as _)
}

#[inline]
pub unsafe fn luaL_optstring(
    L: *mut lua_State,
    n: ::std::os::raw::c_int,
    d: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    luaL_optlstring(L, n, d, 0 as _)
}

#[inline]
pub unsafe fn luaL_typename(
    L: *mut lua_State,
    i: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    lua_typename(L, lua_type(L, i))
}

#[inline]
pub unsafe fn luaL_dofile(L: *mut lua_State, fn_: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let mut state = luaL_loadfile(L, fn_);
    if state == 0 {
        state = lua_pcall(L, 0, LUA_MULTRET, 0);
    }

    state
}

#[inline]
pub unsafe fn luaL_dostring(L: *mut lua_State, s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let mut state = luaL_loadstring(L, s);
    if state == 0 {
        state = lua_pcall(L, 0, LUA_MULTRET, 0);
    }

    state
}

#[inline]
pub unsafe fn luaL_getmetatable(
    L: *mut lua_State,
    n: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    lua_getfield(L, LUA_REGISTRYINDEX, n)
}

//#[inline]
//pub unsafe fn luaL_opt(L: *mut lua_State,f,n,d){
//	(lua_isnoneornil(L,(n)) ? (d) : f(L,(n)))
//}

#[inline]
pub unsafe fn luaL_loadbuffer(
    L: *mut lua_State,
    s: *const ::std::os::raw::c_char,
    sz: usize,
    n: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    luaL_loadbufferx(L, s, sz, n, 0 as _)
}

//#[inline]
//pub unsafe fn luaL_addchar(B: *mut luaL_Buffer, c){
//    ((void)((B)->n < (B)->size || luaL_prepbuffsize((B), 1)), ((B)->b[(B)->n++] = (c)))
//}

#[inline]
pub unsafe fn luaL_addsize(B: *mut luaL_Buffer, s: usize) {
    (*B).n += s;
}

//#[inline]
//pub unsafe fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut ::std::os::raw::c_char {
//    luaL_prepbuffsize(B, LUAL_BUFFERSIZE)
//}
