const BOOST_ROOT: Option<&'static str> = option_env!("BOOST_ROOT");
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

    if let Some(boost_root) = BOOST_ROOT {
        build.include(boost_root);
    }

    build.compile("boost_math_wrapper");

    println!("cargo:rerun-if-changed={WRAPPER_FILE}");
    println!(
        "cargo:rerun-if-changed={}",
        WRAPPER_FILE.replace(".cpp", ".h")
    );
}
