use std::path::PathBuf;

use crate::transpiler;

pub mod simple;

pub fn assert_transpiles_to(test_name: &str, module_name: &str) {
    let rust_src = std::fs::read_to_string(PathBuf::from(format!(
        "src/tests/{}/{}.rs",
        module_name, test_name
    )))
    .expect("Failed to read Rust source file");

    // TODO: implement transpilation from Rust to Lua
    let transpiled_src = transpiler::transpile(&rust_src).expect("Failed to transpile Rust source"); // Placeholder until transpilation is implemented

    let lua_src = std::fs::read_to_string(PathBuf::from(format!(
        "src/tests/{}/{}.lua",
        module_name, test_name
    )))
    .expect("Failed to read Lua source file");

    std::fs::write(
        format!("src/tests/{}/{}_gen.lua", module_name, test_name),
        &transpiled_src,
    )
    .unwrap();

    assert_eq!(transpiled_src, lua_src);
}
