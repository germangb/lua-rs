bindgen \
    --no-layout-tests \
    --whitelist-function "^lua(L?)_(.*)" \
    --whitelist-var "^LUA_(.*)" \
    wrapper.h \
    -- \
    -I/usr/lib/llvm-6.0/lib/clang/6.0.0/include \
    -Ilua > src/ffi/bindgen.rs

cat macros/lua.rs.txt >> src/ffi/bindgen.rs
