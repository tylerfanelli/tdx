// SPDX-License-Identifier: Apache-2.0

use kvm_ioctls::Kvm;

use tdx::launch::{TdxVcpu, TdxVm};
use tdx::tdvf;

#[test]
fn launch() {
    let mut kvm_fd = Kvm::new().unwrap();

    // create vm
    let tdx_vm = TdxVm::new(&kvm_fd, 100).unwrap();
    let caps = tdx_vm.get_capabilities().unwrap();
}
