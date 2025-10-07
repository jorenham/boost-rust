const CXX_STANDARD: &str = "c++20";

const BOOST_MATH_DIR: &str = "subprojects/boost_math/math";
const WRAPPER_CPP: &str = "wrapper.cpp";

fn main() {
    cc::Build::new()
        .cpp(true)
        .std(CXX_STANDARD)
        .flag_if_supported(format!("/std:{CXX_STANDARD}"))
        .warnings_into_errors(true)
        .include(format!("{BOOST_MATH_DIR}/include"))
        // boost/math/special_functions/detail/hypergeometric_series.hpp:244:20
        .flag_if_supported("-Wno-maybe-uninitialized")
        // boost/math/special_functions/lambert_w.hpp:184:46
        .flag_if_supported("-Wno-unused-parameter")
        .file(WRAPPER_CPP)
        .compile("wrapper");

    for &source_path in &["build.rs", WRAPPER_CPP, BOOST_MATH_DIR] {
        println!("cargo:rerun-if-changed={source_path}");
    }
}
