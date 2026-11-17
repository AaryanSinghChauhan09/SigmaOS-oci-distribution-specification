//! Integration tests for SigmaOS-oci-distribution-specification.

use sigmaos_oci_distribution_specification_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
