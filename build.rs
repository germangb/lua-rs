extern crate bindgen;
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
        .files(libs)
        .file("lua/lauxlib.c")
        .compile("lua");

    use bindgen::builder;

    let bindings = builder()
        .header("wrapper.h")
        .layout_tests(false)
        .whitelist_function("^lua(L?)_(.*)")
        .whitelist_var("^LUA_(.*)")
        .clang_args(&[
            // TODO fix generally
            "-I/usr/lib/llvm-6.0/lib/clang/6.0.0/include",
            "-Ilua",
        ])
        .generate()
        .expect("Error generaring bindings");

    bindings
        .write_to_file("src/ffi/bindgen.rs")
        .expect("IO error");
}
