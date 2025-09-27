const CXX_STANDARD: &str = "c++14";
const WRAPPER_FILE: &str = "cpp/wrapper.cpp";

fn main() {
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .flag(if cfg!(debug_assertions) { "-O0" } else { "-O3" })
        .include("cpp")
        .file(WRAPPER_FILE);

    if build.get_compiler().is_like_msvc() {
        build.flag(format!("/std:{CXX_STANDARD}"));
    } else {
        build.std(CXX_STANDARD);
    }

    if let Ok(boost_root) = std::env::var("BOOST_ROOT") {
        build.include(boost_root);
    } else {
        let search_paths = if cfg!(target_os = "windows") {
            &["C:/local/include", "C:/vcpkg/installed/x64-windows/include"][..]
        } else {
            &["/usr/include", "/usr/local/include", "/opt/local/include"][..]
        };

        for path in search_paths {
            if std::path::Path::new(&format!("{path}/boost/version.hpp")).exists() {
                build.include(path);
                break;
            }
        }
    }

    build.compile("boost_math_wrapper");

    println!("cargo:rerun-if-changed={WRAPPER_FILE}");
    println!(
        "cargo:rerun-if-changed={}",
        WRAPPER_FILE.replace(".cpp", ".h")
    );
}
