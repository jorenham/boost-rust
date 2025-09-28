const CXX_STANDARD: &str = "c++23";

const BOOST_MATH_DIR: &str = "subprojects/boost_math/math";
const WRAPPER_CPP: &str = "wrapper.cpp";

fn main() {
    cc::Build::new()
        .cpp(true)
        .std(CXX_STANDARD)
        .flag_if_supported(format!("/std:{CXX_STANDARD}"))
        .includes([
            format!("{BOOST_MATH_DIR}/include"),
            format!("{BOOST_MATH_DIR}/src"),
        ])
        .warnings(true)
        .file(WRAPPER_CPP)
        .compile("wrapper");

    for &source_path in &["build.rs", WRAPPER_CPP, BOOST_MATH_DIR] {
        println!("cargo:rerun-if-changed={source_path}");
    }
}
