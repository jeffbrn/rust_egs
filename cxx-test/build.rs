mod cpp_build;
use std::path::PathBuf;

use cpp_build::{bindings_build, cmake_build, CppBuildParms};

fn main() -> miette::Result<()> {
    let cpp_proj_path = PathBuf::from("cpp/extn_lib");
    let cpp_lib_name = "eg_cpp";
    let build_cpp_path = PathBuf::from("cpp/wrapper");
    let cmake_includes = cmake_build(&cpp_proj_path, cpp_lib_name);
    bindings_build(
        &PathBuf::from("src/cpp_iface/bindings.rs"),
        &cmake_includes,
        Some(CppBuildParms {
            include_dir: build_cpp_path.join("include").to_string_lossy().to_string(),
            src_dir: build_cpp_path.join("src").to_string_lossy().to_string(),
        }),
    );

    Ok(())
}
