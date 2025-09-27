use std::path::{Path, PathBuf};

const CXX_STANDARD: &str = "c++14";
const WRAPPER_SRC: &str = "cpp/wrapper.cpp";

fn locate_boost() -> PathBuf {
    const BOOST_ROOT: &str = "BOOST_ROOT";

    if let Ok(boost_root) = std::env::var(BOOST_ROOT) {
        let path = Path::new(&boost_root);
        return if path.join("boost/version.hpp").is_file() {
            Path::new(&boost_root).to_path_buf()
        } else if path.ends_with("boost") && path.join("version.hpp").is_file() {
            path.parent().unwrap().to_path_buf()
        } else if !path.is_dir() {
            panic!("{BOOST_ROOT} is set but does not point to a valid directory");
        } else {
            panic!("{BOOST_ROOT} is set but does not point to a valid Boost installation");
        };
    }

    let search_paths = if cfg!(target_os = "windows") {
        &["C:/local/include", "C:/vcpkg/installed/x64-windows/include"][..]
    } else {
        &["/usr/include", "/usr/local/include", "/opt/local/include"][..]
    };

    let mut valid_search_paths = Vec::new();
    for path_str in search_paths.iter() {
        let path = Path::new(path_str);
        if path.is_dir() {
            valid_search_paths.push(path_str);
        } else {
            continue;
        }

        if path.join("boost/version.hpp").exists() {
            return path.to_path_buf();
        }
    }

    panic!(
        "Boost headers not found (searched in: {:?}). Please set {BOOST_ROOT}.",
        valid_search_paths
    );
}

fn main() {
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .flag(if cfg!(debug_assertions) { "-O0" } else { "-O3" })
        .include("cpp")
        .include(locate_boost())
        .file(WRAPPER_SRC);

    if build.get_compiler().is_like_msvc() {
        build.flag(format!("/std:{CXX_STANDARD}"));
    } else {
        build.std(CXX_STANDARD);
    }

    build.compile("boost_math_wrapper");

    println!("cargo:rerun-if-changed={WRAPPER_SRC}");
    println!(
        "cargo:rerun-if-changed={}",
        WRAPPER_SRC.replace(".cpp", ".h")
    );
}
