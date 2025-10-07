const CXX_STANDARD: &str = "c++20";

const BOOST_MATH_DIR: &str = "subprojects/boost_math/math";
const WRAPPER_CPP: &str = "wrapper.cpp";

fn main() {
    cc::Build::new()
        .cpp(true)
        .std(CXX_STANDARD)
        // windows: use the specified C++ standard
        .flag_if_supported(format!("/std:{CXX_STANDARD}"))
        // windows: enable C++ exception unwinding
        .flag_if_supported("/EHsc")
        // linux: boost/math/special_functions/detail/hypergeometric_series.hpp:244
        .flag_if_supported("-Wno-maybe-uninitialized")
        // macos: boost/math/special_functions/lambert_w.hpp:184
        .flag_if_supported("-Wno-unused-parameter")
        .warnings_into_errors(true)
        .include(format!("{BOOST_MATH_DIR}/include"))
        .file(WRAPPER_CPP)
        .compile("wrapper");

    for &source_path in &["build.rs", WRAPPER_CPP, BOOST_MATH_DIR] {
        println!("cargo:rerun-if-changed={source_path}");
    }
}
