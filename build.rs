extern crate cc;

fn main() {
    let core = &[
        "lua/lapi.c",
        "lua/lcode.c",
        "lua/lctype.c",
        "lua/ldebug.c",
        "lua/ldo.c",
        "lua/ldump.c",
        "lua/lfunc.c",
        "lua/lgc.c",
        "lua/linit.c",
        "lua/llex.c",
        "lua/lmem.c",
        "lua/lobject.c",
        "lua/lopcodes.c",
        "lua/lparser.c",
        "lua/lstate.c",
        "lua/lstring.c",
        "lua/ltable.c",
        "lua/ltests.c",
        "lua/ltm.c",
        //"lua/lua.c",
        "lua/lundump.c",
        "lua/lvm.c",
        "lua/lzio.c",
    ];

    let libs = &[
        "lua/lbaselib.c",
        "lua/lbitlib.c",
        "lua/lcorolib.c",
        "lua/ldblib.c",
        "lua/liolib.c",
        "lua/lmathlib.c",
        "lua/loadlib.c",
        "lua/loslib.c",
        "lua/lstrlib.c",
        "lua/ltablib.c",
        "lua/lutf8lib.c",
    ];

    cc::Build::new()
        .files(core)
        //#[cfg(feature = "libs")]
        .files(libs)
        .file("lua/lauxlib.c")
        .compile("lua");
}
