mod cpp_build;
use std::path::PathBuf;

use cpp_build::{bindings_build, cmake_build, CppBuildParms};

fn main() -> miette::Result<()> {
    let cpp_proj_path = PathBuf::from("cpp/extn_lib");
    let cpp_lib_name = "eg_cpp";
    let cmake_includes = cmake_build(&cpp_proj_path, cpp_lib_name);
    bindings_build(&PathBuf::from("src/main.rs"), &cmake_includes, None);

    Ok(())
}
