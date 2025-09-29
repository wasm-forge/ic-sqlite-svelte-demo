// This is a generated test setup file.
// Manual changes are possible, but you still need to make sure they are not lost, if the file is regenerated.
// If possible, it is best to keep any additional manual test preparation steps outside, in `tests.rs`,
// then this file can be regenerated without risk of losing work.
#![allow(dead_code)]

use ic_test::IcpTest;

use crate::bindings::backend::{self, BackendCanister};

pub(crate) struct Env {
    pub icp_test: IcpTest,
    pub backend: BackendCanister,
}

pub(crate) async fn setup(icp_test: IcpTest) -> Env {
    let icp_user = icp_test.icp.test_user(0);

    // initialize canisters

    let backend = backend::deploy(&icp_user).call().await;

    Env { icp_test, backend }
}
