/* automatically generated by rust-bindgen */

pub const LUA_INT_INT: u32 = 1;
pub const LUA_INT_LONG: u32 = 2;
pub const LUA_INT_LONGLONG: u32 = 3;
pub const LUA_FLOAT_FLOAT: u32 = 1;
pub const LUA_FLOAT_DOUBLE: u32 = 2;
pub const LUA_FLOAT_LONGDOUBLE: u32 = 3;
pub const LUA_INT_TYPE: u32 = 3;
pub const LUA_FLOAT_TYPE: u32 = 2;
pub const LUA_PATH_SEP: &'static [u8; 2usize] = b";\0";
pub const LUA_PATH_MARK: &'static [u8; 2usize] = b"?\0";
pub const LUA_EXEC_DIR: &'static [u8; 2usize] = b"!\0";
pub const LUA_ROOT: &'static [u8; 12usize] = b"/usr/local/\0";
pub const LUA_DIRSEP: &'static [u8; 2usize] = b"/\0";
pub const LUA_NUMBER_FRMLEN: &'static [u8; 1usize] = b"\0";
pub const LUA_NUMBER_FMT: &'static [u8; 6usize] = b"%.14g\0";
pub const LUA_INTEGER_FRMLEN: &'static [u8; 3usize] = b"ll\0";
pub const LUA_IDSIZE: u32 = 60;
pub const LUA_VERSION_MAJOR: &'static [u8; 2usize] = b"5\0";
pub const LUA_VERSION_MINOR: &'static [u8; 2usize] = b"3\0";
pub const LUA_VERSION_NUM: u32 = 503;
pub const LUA_VERSION_RELEASE: &'static [u8; 2usize] = b"4\0";
pub const LUA_VERSION: &'static [u8; 8usize] = b"Lua 5.3\0";
pub const LUA_RELEASE: &'static [u8; 10usize] = b"Lua 5.3.4\0";
pub const LUA_COPYRIGHT: &'static [u8; 52usize] =
    b"Lua 5.3.4  Copyright (C) 1994-2017 Lua.org, PUC-Rio\0";
pub const LUA_AUTHORS: &'static [u8; 48usize] =
    b"R. Ierusalimschy, L. H. de Figueiredo, W. Celes\0";
pub const LUA_SIGNATURE: &'static [u8; 5usize] = b"\x1BLua\0";
pub const LUA_MULTRET: i32 = -1;
pub const LUA_REGISTRYINDEX: i32 = -1001000;
pub const LUA_OK: u32 = 0;
pub const LUA_YIELD: u32 = 1;
pub const LUA_ERRRUN: u32 = 2;
pub const LUA_ERRSYNTAX: u32 = 3;
pub const LUA_ERRMEM: u32 = 4;
pub const LUA_ERRGCMM: u32 = 5;
pub const LUA_ERRERR: u32 = 6;
pub const LUA_TNONE: i32 = -1;
pub const LUA_TNIL: u32 = 0;
pub const LUA_TBOOLEAN: u32 = 1;
pub const LUA_TLIGHTUSERDATA: u32 = 2;
pub const LUA_TNUMBER: u32 = 3;
pub const LUA_TSTRING: u32 = 4;
pub const LUA_TTABLE: u32 = 5;
pub const LUA_TFUNCTION: u32 = 6;
pub const LUA_TUSERDATA: u32 = 7;
pub const LUA_TTHREAD: u32 = 8;
pub const LUA_NUMTAGS: u32 = 9;
pub const LUA_MINSTACK: u32 = 20;
pub const LUA_RIDX_MAINTHREAD: u32 = 1;
pub const LUA_RIDX_GLOBALS: u32 = 2;
pub const LUA_RIDX_LAST: u32 = 2;
pub const LUA_OPADD: u32 = 0;
pub const LUA_OPSUB: u32 = 1;
pub const LUA_OPMUL: u32 = 2;
pub const LUA_OPMOD: u32 = 3;
pub const LUA_OPPOW: u32 = 4;
pub const LUA_OPDIV: u32 = 5;
pub const LUA_OPIDIV: u32 = 6;
pub const LUA_OPBAND: u32 = 7;
pub const LUA_OPBOR: u32 = 8;
pub const LUA_OPBXOR: u32 = 9;
pub const LUA_OPSHL: u32 = 10;
pub const LUA_OPSHR: u32 = 11;
pub const LUA_OPUNM: u32 = 12;
pub const LUA_OPBNOT: u32 = 13;
pub const LUA_OPEQ: u32 = 0;
pub const LUA_OPLT: u32 = 1;
pub const LUA_OPLE: u32 = 2;
pub const LUA_GCSTOP: u32 = 0;
pub const LUA_GCRESTART: u32 = 1;
pub const LUA_GCCOLLECT: u32 = 2;
pub const LUA_GCCOUNT: u32 = 3;
pub const LUA_GCCOUNTB: u32 = 4;
pub const LUA_GCSTEP: u32 = 5;
pub const LUA_GCSETPAUSE: u32 = 6;
pub const LUA_GCSETSTEPMUL: u32 = 7;
pub const LUA_GCISRUNNING: u32 = 9;
pub const LUA_HOOKCALL: u32 = 0;
pub const LUA_HOOKRET: u32 = 1;
pub const LUA_HOOKLINE: u32 = 2;
pub const LUA_HOOKCOUNT: u32 = 3;
pub const LUA_HOOKTAILCALL: u32 = 4;
pub const LUA_MASKCALL: u32 = 1;
pub const LUA_MASKRET: u32 = 2;
pub const LUA_MASKLINE: u32 = 4;
pub const LUA_MASKCOUNT: u32 = 8;
pub const LUA_ERRFILE: u32 = 7;
pub const LUA_LOADED_TABLE: &'static [u8; 8usize] = b"_LOADED\0";
pub const LUA_PRELOAD_TABLE: &'static [u8; 9usize] = b"_PRELOAD\0";
pub const LUA_NOREF: i32 = -2;
pub const LUA_REFNIL: i32 = -1;
pub const LUA_FILEHANDLE: &'static [u8; 6usize] = b"FILE*\0";
pub const LUA_VERSUFFIX: &'static [u8; 5usize] = b"_5_3\0";
pub const LUA_COLIBNAME: &'static [u8; 10usize] = b"coroutine\0";
pub const LUA_TABLIBNAME: &'static [u8; 6usize] = b"table\0";
pub const LUA_IOLIBNAME: &'static [u8; 3usize] = b"io\0";
pub const LUA_OSLIBNAME: &'static [u8; 3usize] = b"os\0";
pub const LUA_STRLIBNAME: &'static [u8; 7usize] = b"string\0";
pub const LUA_UTF8LIBNAME: &'static [u8; 5usize] = b"utf8\0";
pub const LUA_BITLIBNAME: &'static [u8; 6usize] = b"bit32\0";
pub const LUA_MATHLIBNAME: &'static [u8; 5usize] = b"math\0";
pub const LUA_DBLIBNAME: &'static [u8; 6usize] = b"debug\0";
pub const LUA_LOADLIBNAME: &'static [u8; 8usize] = b"package\0";
pub type va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lua_State {
    _unused: [u8; 0],
}
pub type lua_Number = f64;
pub type lua_Integer = ::std::os::raw::c_longlong;
pub type lua_KContext = isize;
pub type lua_CFunction =
    ::std::option::Option<unsafe extern "C" fn(L: *mut lua_State) -> ::std::os::raw::c_int>;
pub type lua_KFunction = ::std::option::Option<
    unsafe extern "C" fn(L: *mut lua_State, status: ::std::os::raw::c_int, ctx: lua_KContext)
        -> ::std::os::raw::c_int,
>;
pub type lua_Reader = ::std::option::Option<
    unsafe extern "C" fn(L: *mut lua_State, ud: *mut ::std::os::raw::c_void, sz: *mut usize)
        -> *const ::std::os::raw::c_char,
>;
pub type lua_Writer = ::std::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        p: *const ::std::os::raw::c_void,
        sz: usize,
        ud: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type lua_Alloc = ::std::option::Option<
    unsafe extern "C" fn(
        ud: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut ::std::os::raw::c_void,
>;
extern "C" {
    pub fn lua_newstate(f: lua_Alloc, ud: *mut ::std::os::raw::c_void) -> *mut lua_State;
}
extern "C" {
    pub fn lua_close(L: *mut lua_State);
}
extern "C" {
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
}
extern "C" {
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
}
extern "C" {
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
}
extern "C" {
    pub fn lua_absindex(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gettop(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_settop(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_pushvalue(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_rotate(L: *mut lua_State, idx: ::std::os::raw::c_int, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_copy(
        L: *mut lua_State,
        fromidx: ::std::os::raw::c_int,
        toidx: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_checkstack(L: *mut lua_State, n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_isnumber(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_isstring(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_iscfunction(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_isinteger(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_isuserdata(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_type(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_typename(
        L: *mut lua_State,
        tp: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_tonumberx(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        isnum: *mut ::std::os::raw::c_int,
    ) -> lua_Number;
}
extern "C" {
    pub fn lua_tointegerx(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        isnum: *mut ::std::os::raw::c_int,
    ) -> lua_Integer;
}
extern "C" {
    pub fn lua_toboolean(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_tolstring(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        len: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_rawlen(L: *mut lua_State, idx: ::std::os::raw::c_int) -> usize;
}
extern "C" {
    pub fn lua_tocfunction(L: *mut lua_State, idx: ::std::os::raw::c_int) -> lua_CFunction;
}
extern "C" {
    pub fn lua_touserdata(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_tothread(L: *mut lua_State, idx: ::std::os::raw::c_int) -> *mut lua_State;
}
extern "C" {
    pub fn lua_topointer(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_arith(L: *mut lua_State, op: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_rawequal(
        L: *mut lua_State,
        idx1: ::std::os::raw::c_int,
        idx2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_compare(
        L: *mut lua_State,
        idx1: ::std::os::raw::c_int,
        idx2: ::std::os::raw::c_int,
        op: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_pushnil(L: *mut lua_State);
}
extern "C" {
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
}
extern "C" {
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
}
extern "C" {
    pub fn lua_pushlstring(
        L: *mut lua_State,
        s: *const ::std::os::raw::c_char,
        len: usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_pushstring(
        L: *mut lua_State,
        s: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const ::std::os::raw::c_char,
        argp: *mut __va_list_tag,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_pushcclosure(L: *mut lua_State, fn_: lua_CFunction, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_pushboolean(L: *mut lua_State, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn lua_pushthread(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getglobal(
        L: *mut lua_State,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gettable(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getfield(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_geti(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        n: lua_Integer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_rawget(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_rawgeti(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        n: lua_Integer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_rawgetp(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        p: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_createtable(
        L: *mut lua_State,
        narr: ::std::os::raw::c_int,
        nrec: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_newuserdata(L: *mut lua_State, sz: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_getmetatable(
        L: *mut lua_State,
        objindex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getuservalue(L: *mut lua_State, idx: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_setglobal(L: *mut lua_State, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn lua_settable(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_setfield(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn lua_seti(L: *mut lua_State, idx: ::std::os::raw::c_int, n: lua_Integer);
}
extern "C" {
    pub fn lua_rawset(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_rawseti(L: *mut lua_State, idx: ::std::os::raw::c_int, n: lua_Integer);
}
extern "C" {
    pub fn lua_rawsetp(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        p: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn lua_setmetatable(
        L: *mut lua_State,
        objindex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_setuservalue(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_callk(
        L: *mut lua_State,
        nargs: ::std::os::raw::c_int,
        nresults: ::std::os::raw::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    );
}
extern "C" {
    pub fn lua_pcallk(
        L: *mut lua_State,
        nargs: ::std::os::raw::c_int,
        nresults: ::std::os::raw::c_int,
        errfunc: ::std::os::raw::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut ::std::os::raw::c_void,
        chunkname: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_dump(
        L: *mut lua_State,
        writer: lua_Writer,
        data: *mut ::std::os::raw::c_void,
        strip: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_yieldk(
        L: *mut lua_State,
        nresults: ::std::os::raw::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_resume(
        L: *mut lua_State,
        from: *mut lua_State,
        narg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_status(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_isyieldable(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gc(
        L: *mut lua_State,
        what: ::std::os::raw::c_int,
        data: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_error(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_next(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_concat(L: *mut lua_State, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_len(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_stringtonumber(L: *mut lua_State, s: *const ::std::os::raw::c_char) -> usize;
}
extern "C" {
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut ::std::os::raw::c_void) -> lua_Alloc;
}
extern "C" {
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut ::std::os::raw::c_void);
}
pub type lua_Hook =
    ::std::option::Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;
extern "C" {
    pub fn lua_getstack(
        L: *mut lua_State,
        level: ::std::os::raw::c_int,
        ar: *mut lua_Debug,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getinfo(
        L: *mut lua_State,
        what: *const ::std::os::raw::c_char,
        ar: *mut lua_Debug,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_upvalueid(
        L: *mut lua_State,
        fidx: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        fidx2: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
}
extern "C" {
    pub fn lua_gethookmask(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gethookcount(L: *mut lua_State) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lua_Debug {
    pub event: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
    pub namewhat: *const ::std::os::raw::c_char,
    pub what: *const ::std::os::raw::c_char,
    pub source: *const ::std::os::raw::c_char,
    pub currentline: ::std::os::raw::c_int,
    pub linedefined: ::std::os::raw::c_int,
    pub lastlinedefined: ::std::os::raw::c_int,
    pub nups: ::std::os::raw::c_uchar,
    pub nparams: ::std::os::raw::c_uchar,
    pub isvararg: ::std::os::raw::c_char,
    pub istailcall: ::std::os::raw::c_char,
    pub short_src: [::std::os::raw::c_char; 60usize],
    pub i_ci: *mut CallInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct luaL_Reg {
    pub name: *const ::std::os::raw::c_char,
    pub func: lua_CFunction,
}
extern "C" {
    pub fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: usize);
}
extern "C" {
    pub fn luaL_getmetafield(
        L: *mut lua_State,
        obj: ::std::os::raw::c_int,
        e: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_callmeta(
        L: *mut lua_State,
        obj: ::std::os::raw::c_int,
        e: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_tolstring(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        len: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_argerror(
        L: *mut lua_State,
        arg: ::std::os::raw::c_int,
        extramsg: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_checklstring(
        L: *mut lua_State,
        arg: ::std::os::raw::c_int,
        l: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_optlstring(
        L: *mut lua_State,
        arg: ::std::os::raw::c_int,
        def: *const ::std::os::raw::c_char,
        l: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_checknumber(L: *mut lua_State, arg: ::std::os::raw::c_int) -> lua_Number;
}
extern "C" {
    pub fn luaL_optnumber(
        L: *mut lua_State,
        arg: ::std::os::raw::c_int,
        def: lua_Number,
    ) -> lua_Number;
}
extern "C" {
    pub fn luaL_checkinteger(L: *mut lua_State, arg: ::std::os::raw::c_int) -> lua_Integer;
}
extern "C" {
    pub fn luaL_optinteger(
        L: *mut lua_State,
        arg: ::std::os::raw::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
}
extern "C" {
    pub fn luaL_checkstack(
        L: *mut lua_State,
        sz: ::std::os::raw::c_int,
        msg: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn luaL_checktype(L: *mut lua_State, arg: ::std::os::raw::c_int, t: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_checkany(L: *mut lua_State, arg: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_newmetatable(
        L: *mut lua_State,
        tname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_setmetatable(L: *mut lua_State, tname: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn luaL_testudata(
        L: *mut lua_State,
        ud: ::std::os::raw::c_int,
        tname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn luaL_checkudata(
        L: *mut lua_State,
        ud: ::std::os::raw::c_int,
        tname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn luaL_where(L: *mut lua_State, lvl: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_error(
        L: *mut lua_State,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_checkoption(
        L: *mut lua_State,
        arg: ::std::os::raw::c_int,
        def: *const ::std::os::raw::c_char,
        lst: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_fileresult(
        L: *mut lua_State,
        stat: ::std::os::raw::c_int,
        fname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_execresult(L: *mut lua_State, stat: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_ref(L: *mut lua_State, t: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_unref(L: *mut lua_State, t: ::std::os::raw::c_int, ref_: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const ::std::os::raw::c_char,
        sz: usize,
        name: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_loadstring(
        L: *mut lua_State,
        s: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_newstate() -> *mut lua_State;
}
extern "C" {
    pub fn luaL_len(L: *mut lua_State, idx: ::std::os::raw::c_int) -> lua_Integer;
}
extern "C" {
    pub fn luaL_gsub(
        L: *mut lua_State,
        s: *const ::std::os::raw::c_char,
        p: *const ::std::os::raw::c_char,
        r: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_getsubtable(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        fname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const ::std::os::raw::c_char,
        level: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn luaL_requiref(
        L: *mut lua_State,
        modname: *const ::std::os::raw::c_char,
        openf: lua_CFunction,
        glb: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luaL_Buffer {
    pub b: *mut ::std::os::raw::c_char,
    pub size: usize,
    pub n: usize,
    pub L: *mut lua_State,
    pub initb: [::std::os::raw::c_char; 8192usize],
}
extern "C" {
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
}
extern "C" {
    pub fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: usize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const ::std::os::raw::c_char, l: usize);
}
extern "C" {
    pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
}
extern "C" {
    pub fn luaL_pushresult(B: *mut luaL_Buffer);
}
extern "C" {
    pub fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: usize);
}
extern "C" {
    pub fn luaL_buffinitsize(
        L: *mut lua_State,
        B: *mut luaL_Buffer,
        sz: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_openlibs(L: *mut lua_State);
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CallInfo {
    pub _address: u8,
}
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
