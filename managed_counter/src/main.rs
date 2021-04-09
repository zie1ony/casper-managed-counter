#![no_main]

use managed_counter::deployer;

#[no_mangle]
pub extern "C" fn call() {
    deployer::deploy_manager_contract(String::from("manager"));
}
