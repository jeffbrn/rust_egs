use std::{fs, path::{Path, PathBuf}};
use std::process::Command;

pub fn cmake_build(cpp_project_path: &PathBuf, lib_name: &str) -> Vec<PathBuf> {
	println!("cargo:warning=*** Building dependent CPP library: {} @ {}", lib_name, cpp_project_path.display());

	//rebuild if any dependent source file changes
	let include_dir = PathBuf::from(cpp_project_path).join("include");
	let _inc_files = glob_files(&include_dir, "h");
	let _inc_files = glob_files(&include_dir, "hpp");
	let src_dir = PathBuf::from(cpp_project_path).join("src");
	let _src_files = glob_files(&src_dir, "cpp");

	let cpp_lib_file = PathBuf::from(cpp_project_path)
		.join("build")
		.join(format!("lib{}.a", lib_name));
	println!("cargo:rerun-if-changed={}", cpp_lib_file.display());

	let _cpp_build = Command::new("cmake")
		.arg("-S")
		.arg(cpp_project_path)
		.arg("-B")
		.arg(cpp_project_path.join("build"))
		.arg("-DCMAKE_BUILD_TYPE=Release")
		.arg("-DCMAKE_POSITION_INDEPENDENT_CODE=ON")
		.status()
		.expect("Failed to run cmake to configure C++ project");

		println!("cargo:rustc-link-lib=static={}", lib_name);
		vec![include_dir]
}

pub fn bindings_build(bindings_src: &impl AsRef<Path>, extn_includes: &Vec<PathBuf>, cpp_build: Option<CppBuildParms>) {
	let mut includes = extn_includes.clone();
	let sources = cpp_build.
		map(|cpp_p| {
			let inc_dir = cpp_p.include_dir.clone();
			let _include_files = glob_files(inc_dir.clone().into(), "h");
			let _include_files = glob_files(inc_dir.clone().into(), "hpp");
			includes.push(inc_dir.into());
			glob_src(cpp_p.src_dir.into(), "cpp")
		})
		.unwrap_or_default();

	println!("cargo:rerun-if-changed={}", bindings_src.as_ref().display());
	println!("cargo:warning=rerun-if-changed={}", bindings_src.as_ref().display());

	let mut b = autocxx_build::Builder::new(bindings_src, &includes)
		.build()
		.unwrap();
	b.flag_if_supported("-std=c++17")
		.files(sources)
		.compile("autocxx-demo");
}

pub struct CppBuildParms {
	pub include_dir: String,
	pub src_dir: String,
}

fn glob_files(dir: &Path, extn: &str) -> Vec<PathBuf> {
	let mut retval = Vec::<PathBuf>::new();
	walk_dirs(dir.as_path(), extn, &mut retval);
	retval
}

fn walk_dirs(dir: &Path, extn: &str, files: &mut Vec<PathBuf>) {
	if dir.is_dir() {
		for entry in fs::read_dir(dir).unwrap() {
			let entry = entry.unwrap();
			let path = entry.path();
			if path.is_dir() {
				walk_dirs(path.as_path(), extn, files);
			} else if path.extension().map(|s| s == extn).unwrap_or(false) {
				println!("cargo:rerun-if-changed={}", path.display());
				println!("cargo:warning=rerun-if-changed={}", path.display());
				files.push(path);
			}
		}
	}
}