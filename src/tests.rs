use crate::*;

#[test]
fn test_error_functions() {
    // erf(0) = 0
    assert!((special::erf(0.0) - 0.0).abs() < 1e-15);

    // erfc(0) = 1
    assert!((special::erfc(0.0) - 1.0).abs() < 1e-15);

    // erf + erfc = 1
    let x = 1.0;
    let erf_val = special::erf(x);
    let erfc_val = special::erfc(x);
    assert!((erf_val + erfc_val - 1.0).abs() < 1e-15);

    // erf is odd function: erf(-x) = -erf(x)
    assert!((special::erf(-1.0) + special::erf(1.0)).abs() < 1e-15);
}
