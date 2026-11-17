// Integration test for SigmaOS-oci-distribution-specification
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use oci_distribution_specification as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
