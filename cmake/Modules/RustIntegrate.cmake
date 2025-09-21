set(RUST_LIBRARY  ${CMAKE_SOURCE_DIR}/build/lib/)
set(RUST_STATIC_LIBRARY ${RUST_LIBRARY}/librust_src.a)

add_custom_target(
    built_rust
    command cargo build 
    --manifest-path=${CMAKE_SOURCE_DIR}/rust_src/Cargo.toml
    --target-dir=${CMAKE_SOURCE_DIR}/build/lib/
)

add_custom_target(
    copy_rust_lib
    COMMAND
        ${CMAKE_COMMAND} -E 
        copy
        ${CMAKE_SOURCE_DIR}/build/lib/debug/librust_src.a
        ${CMAKE_SOURCE_DIR}/build/lib/
)


add_dependencies(copy_rust_lib built_rust)